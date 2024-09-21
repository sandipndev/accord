use crate::process::ProcessStatus;

use super::{error::ProcessError, ProcessId, Processes};

use futures::stream::{FuturesUnordered, StreamExt};
use std::{process::Stdio, sync::Arc};
use tokio::{process::Command, sync::Semaphore};

impl Processes {
    pub async fn download(&self, process_id: ProcessId) -> Result<(), ProcessError> {
        let process = self.get(process_id).await?;

        self.update_status(process.id, ProcessStatus::Downloading)
            .await?;
        println!(
            "Downloading video: {} for process ID {}",
            process.youtube_url, process.id
        );

        let download_file_at = format!("{}/{}.mp3", self.config.home_absolute_path, process.id);

        let status = Command::new("yt-dlp")
            .arg("-x")
            .arg("--audio-format")
            .arg("mp3")
            .arg("-o")
            .arg(download_file_at)
            .arg(&process.youtube_url.to_string())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .await;

        match status {
            Ok(status) if status.success() => {
                self.update_status(process.id, ProcessStatus::Downloaded)
                    .await?;

                crate::job::spawn_convert_job(&self.pool, process.id)
                    .await
                    .map_err(|_| ProcessError::JobSpawnFailed)?;

                Ok(())
            }
            Ok(status) => {
                eprintln!("yt-dlp exited with status code: {}", status);
                Err(ProcessError::CommandFailed)
            }
            Err(e) => {
                eprintln!("Failed to execute yt-dlp: {}", e);
                Err(ProcessError::CommandFailed)
            }
        }
    }

    pub async fn convert(&self, process_id: ProcessId) -> Result<(), ProcessError> {
        let process = self.get(process_id).await?;

        self.update_status(process.id, ProcessStatus::Converting)
            .await?;
        println!(
            "Converting video: {} for process ID {}",
            process.youtube_url, process.id
        );

        let downloaded_file_at = format!("{}/{}.mp3", self.config.home_absolute_path, process.id);

        if !std::path::Path::new(&downloaded_file_at).exists() {
            return Err(ProcessError::FileNotFound);
        }

        let semitone_shifts: Vec<i32> = (-10..=10).filter(|&s| s != 0).collect();

        let max_concurrent_processes = 4;
        let semaphore = Arc::new(Semaphore::new(max_concurrent_processes));

        let mut futures = FuturesUnordered::new();

        for semitone in semitone_shifts {
            let downloaded_file_at = downloaded_file_at.clone();
            let home_path = self.config.home_absolute_path.clone();
            let process_id = process.id.clone();
            let semaphore = semaphore.clone();

            // Create an async task for each semitone shift
            futures.push(async move {
                // Acquire a permit before starting the process
                let _permit = semaphore.acquire().await;

                let pitch_factor = 2f64.powf(semitone as f64 / 12.0);

                let output_file = format!(
                    "{}/{}_{}_ST.mp3",
                    home_path,
                    process_id,
                    if semitone > 0 {
                        format!("+{}", semitone)
                    } else {
                        semitone.to_string()
                    }
                );

                let status = Command::new("ffmpeg")
                    .arg("-y") // Overwrite output files without asking
                    .arg("-i")
                    .arg(&downloaded_file_at)
                    .arg("-filter:a")
                    .arg(format!("rubberband=pitch={}", pitch_factor))
                    .arg(&output_file)
                    .stdout(Stdio::null())
                    .stderr(Stdio::null())
                    .status()
                    .await;

                match status {
                    Ok(status) if status.success() => Ok(()),
                    Ok(status) => {
                        eprintln!("ffmpeg exited with status code: {}", status);
                        Err(ProcessError::CommandFailed)
                    }
                    Err(e) => {
                        eprintln!("Failed to execute ffmpeg: {}", e);
                        Err(ProcessError::CommandFailed)
                    }
                }
            });
        }

        while let Some(result) = futures.next().await {
            if let Err(e) = result {
                eprintln!("Error during conversion: {}", e);
                return Err(e);
            }
        }

        self.update_status(process.id, ProcessStatus::Converted)
            .await?;

        self.update_status(process.id, ProcessStatus::Done).await?;

        println!(
            "All pitch-shifted files have been generated successfully for process {}",
            process.id
        );

        Ok(())
    }

    pub async fn get_metadata(
        &self,
        youtube_url: &str,
    ) -> Result<YoutubeVideoMetadata, ProcessError> {
        let output = Command::new("yt-dlp")
            .arg("--get-title")
            .arg("--get-duration")
            .arg(youtube_url)
            .output()
            .await
            .map_err(|_| ProcessError::CommandFailed)?;

        let output = String::from_utf8(output.stdout).map_err(|_| ProcessError::CommandFailed)?;
        let title = output
            .lines()
            .next()
            .ok_or(ProcessError::CommandFailed)?
            .to_string();
        let duration = output.lines().nth(1).ok_or(ProcessError::CommandFailed)?;

        let duration = parse_duration(duration).unwrap_or(0);

        Ok(YoutubeVideoMetadata {
            title,
            duration_s: duration,
        })
    }
}

pub struct YoutubeVideoMetadata {
    pub title: String,
    pub duration_s: u32,
}

fn parse_duration(duration: &str) -> Option<u32> {
    let parts: Vec<&str> = duration.split(':').collect();

    match parts.len() {
        1 => {
            // SS
            let seconds = parts[0].parse::<u32>().ok()?;
            Some(seconds)
        }
        2 => {
            // MM:SS
            let minutes = parts[0].parse::<u32>().ok()?;
            let seconds = parts[1].parse::<u32>().ok()?;
            Some(minutes * 60 + seconds)
        }
        3 => {
            // HH:MM:SS
            let hours = parts[0].parse::<u32>().ok()?;
            let minutes = parts[1].parse::<u32>().ok()?;
            let seconds = parts[2].parse::<u32>().ok()?;
            Some(hours * 3600 + minutes * 60 + seconds)
        }
        _ => None, // Invalid format
    }
}
