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

        // Semitone shifts from -10 to -1 and +1 to +10
        let semitone_shifts: Vec<i32> = (-10..=10).filter(|&s| s != 0).collect();

        // Limit the number of concurrent FFmpeg processes
        let max_concurrent_processes = 4;
        let semaphore = Arc::new(Semaphore::new(max_concurrent_processes));

        // Use FuturesUnordered for efficient asynchronous iteration
        let mut futures = FuturesUnordered::new();

        for semitone in semitone_shifts {
            // Clone variables for use in the async block
            let downloaded_file_at = downloaded_file_at.clone();
            let home_path = self.config.home_absolute_path.clone();
            let process_id = process.id.clone();
            let semaphore = semaphore.clone();

            // Create an async task for each semitone shift
            futures.push(async move {
                // Acquire a permit before starting the process
                let _permit = semaphore.acquire().await;

                // Calculate the pitch factor
                let pitch_factor = 2f64.powf(semitone as f64 / 12.0);

                // Construct the output file path
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

                // Build the FFmpeg command
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
                    Ok(status) if status.success() => {
                        self.update_status(process.id, ProcessStatus::Converted)
                            .await?;

                        self.update_status(process.id, ProcessStatus::Done).await?;

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
            });
        }

        // Process the futures as they complete
        while let Some(result) = futures.next().await {
            if let Err(e) = result {
                eprintln!("Error during conversion: {}", e);
                return Err(e);
            }
        }

        println!(
            "All pitch-shifted files have been generated successfully for process {}",
            process.id
        );

        Ok(())
    }
}
