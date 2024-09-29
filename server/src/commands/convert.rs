use crate::commands::error::CommandError;
use crate::primitives::TrackId;

use std::process::Stdio;
use tokio::process::Command;

pub async fn shift_pitch_of_track_by(
    track_id: TrackId,
    shift: i32,
    home_absolute_path: &String,
) -> Result<(), CommandError> {
    let downloaded_file_at = format!("{}/{}.mp3", home_absolute_path, track_id);
    let pitch_factor = 2f64.powf(shift as f64 / 12.0);
    let output_file = format!(
        "{}/{}_{}_ST.mp3",
        home_absolute_path,
        track_id,
        if shift > 0 {
            format!("+{}", shift)
        } else {
            shift.to_string()
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
        Ok(_) | Err(_) => {
            eprintln!(
                "ffmpeg conversion errored for track_id: {} and pitch shift: {}",
                track_id, shift
            );
            Err(CommandError::CommandFailed)
        }
    }
}
