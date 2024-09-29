use crate::commands::error::CommandError;
use crate::primitives::{TrackId, YoutubeUrl};

use std::process::Stdio;
use tokio::process::Command;

pub async fn download_track(
    track_id: TrackId,
    youtube_url: YoutubeUrl,
    home_absolute_path: &String,
) -> Result<(), CommandError> {
    let video_file = format!("{}/{}.mp4", home_absolute_path, track_id);
    let status = Command::new("yt-dlp")
        .arg("-f")
        .arg("bestvideo[height<=720][ext=mp4]+bestaudio[ext=m4a]/mp4")
        .arg("--recode-video")
        .arg("mp4")
        .arg("-o")
        .arg(video_file)
        .arg(&String::from(youtube_url.clone()))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await;

    match status {
        Ok(status) if status.success() => {}
        Ok(_) | Err(_) => {
            eprintln!("yt-dlp errored for track_id: {}", track_id);
            return Err(CommandError::CommandFailed);
        }
    }

    let audio_file = format!("{}/{}.mp3", home_absolute_path, track_id);
    let status = Command::new("yt-dlp")
        .arg("-x")
        .arg("--audio-format")
        .arg("mp3")
        .arg("-o")
        .arg(audio_file)
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
