mod compress;
mod progress_bar; 

use crate::consts::FFMPEG_BINARY;
use anyhow::Result;
pub use compress::{
    AudioCompressOptions, ImageCompressOptions, VideoCompressOptions, compress_all_audios,
    compress_all_images, compress_all_videos,
};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tempfile::tempdir;

pub fn get_ffmpeg() -> Result<PathBuf> {
    let temp_dir = tempdir()?;

    #[cfg(target_os = "windows")]
    let ffmpeg_path = temp_dir.path().join("ffmpeg.exe");

    #[cfg(not(target_os = "windows"))]
    let ffmpeg_path = temp_dir.path().join("ffmpeg");

    let mut file = fs::File::create(&ffmpeg_path)?;
    file.write_all(FFMPEG_BINARY)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&ffmpeg_path, fs::Permissions::from_mode(0o755))?;
    }

    // Leak tempdir so it doesn't get deleted
    Box::leak(Box::new(temp_dir));

    Ok(ffmpeg_path)
}
