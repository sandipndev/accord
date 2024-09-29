use crate::commands::error::CommandError;
use crate::primitives::{TrackId, YoutubeUrl};

use std::process::Stdio;
use tokio::process::Command;

pub async fn download_track(
    track_id: TrackId,
    youtube_url: YoutubeUrl,
    home_absolute_path: &String,
) -> Result<(), CommandError> {
    let download_file_at = format!("{}/{}.mp3", home_absolute_path, track_id);

    let status = Command::new("yt-dlp")
        .arg("-xk")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(download_file_at)
        .arg(&String::from(youtube_url))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await;

    match status {
        Ok(status) if status.success() => Ok(()),
        Ok(_) | Err(_) => {
            eprintln!("yt-dlp errored for track_id: {}", track_id);
            Err(CommandError::CommandFailed)
        }
    }
}
