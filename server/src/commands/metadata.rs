use crate::commands::error::CommandError;
use crate::primitives::YoutubeUrl;
use tokio::process::Command;

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

pub async fn get_metadata(youtube_url: YoutubeUrl) -> Result<YoutubeVideoMetadata, CommandError> {
    let output = Command::new("yt-dlp")
        .arg("--get-title")
        .arg("--get-duration")
        .arg(String::from(youtube_url))
        .output()
        .await
        .map_err(|_| CommandError::CommandFailed)?;

    let output = String::from_utf8(output.stdout).map_err(|_| CommandError::CommandFailed)?;
    let title = output
        .lines()
        .next()
        .ok_or(CommandError::CommandFailed)?
        .to_string();
    let duration = output.lines().nth(1).ok_or(CommandError::CommandFailed)?;

    let duration = parse_duration(duration).unwrap_or(0);

    Ok(YoutubeVideoMetadata {
        title,
        duration_s: duration,
    })
}
