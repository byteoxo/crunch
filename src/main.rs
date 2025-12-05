mod consts;
mod ffmpeg;
mod utilities;

use anyhow::Result;
use clap::Parser;
use ffmpeg::{
    AudioCompressOptions, ImageCompressOptions, VideoCompressOptions, compress_all_audios,
    compress_all_images, compress_all_videos, get_ffmpeg,
};
use std::path::Path;
use utilities::{get_audio_files, get_image_files, get_video_files};

#[derive(Parser, Debug)]
#[command(name = "crunch")]
#[command(
    version,
    author,
    about = "A fast, parallel media compression tool for non-programmer powered by FFmpeg",
    long_about = "\
A fast, parallel media compression tool for non-programmer powered by FFmpeg.

Compress videos, images, and audio files in bulk with optimal settings.
Supports concurrent processing for maximum performance.",
    after_help = "\
EXAMPLES:
    crunch --default                     Compress all media with default formats (video: webm, image: webp, audio: mp3)
    crunch --videos                      Compress videos only (default: webm)
    crunch --videos=webm                 Compress videos to webm
    crunch --images=webp                 Compress images to webp
    crunch --audio=mp3                   Compress audios to mp3

SUPPORTED FORMATS:
    Videos: webm, mp4, mkv, av1, etc.
    Images: webp, avif, jpg, png, etc.
    Audio:  opus, mp3, aac, flac, etc.

MORE INFO:
    https://github.com/byteoxo/crunch",
    arg_required_else_help = true
)]
struct Args {
    /// Use default settings (videos=webm, images=webp)
    #[arg(long)]
    default: bool,

    /// Path to process (default: current directory)
    #[arg(short = 'p', long, num_args = 0..=1, default_missing_value = ".")]
    path: Option<String>,

    #[arg(long, num_args= 0..1, default_missing_value = "compressed")]
    prefix: Option<String>,

    /// Video format. Use --videos for default(webm) or --videos=FORMAT
    #[arg(long, num_args = 0..=1, default_missing_value = "webm")]
    videos: Option<String>,

    /// Image format. Use --images for default(webp) or --images=FORMAT
    #[arg(long, num_args = 0..=1, default_missing_value = "webp")]
    images: Option<String>,

    /// Audios format. Use --audios for default(webp) or --audios=FORMAT
    #[arg(long, num_args = 0..=1, default_missing_value = "mp3")]
    audios: Option<String>,
}

fn process_images(ffmpeg: &Path, path: &Path, format: &str) -> Result<()> {
    let images = get_image_files(path);
    let count = images.len();

    if count == 0 {
        println!("No images found to compress");
        return Ok(());
    }

    println!("Found {} images to compress to {}", count, format);

    let options = ImageCompressOptions::default().set_output_ext(format.into());
    compress_all_images(ffmpeg, &images, &options)?;

    println!("Successfully compressed {} images", count);
    Ok(())
}

fn process_videos(ffmpeg: &Path, path: &Path, format: &str) -> Result<()> {
    let videos = get_video_files(path);
    let count = videos.len();

    if count == 0 {
        println!("No videos found to compress");
        return Ok(());
    }

    println!("Found {} videos to compress to {}", count, format);

    let options = VideoCompressOptions::default().set_output_ext(format.into());
    compress_all_videos(ffmpeg, &videos, &options)?;

    println!("Successfully compressed {} videos", count);
    Ok(())
}

fn process_audios(ffmpeg: &Path, path: &Path, format: &str) -> Result<()> {
    let audios = get_audio_files(path);
    let count = audios.len();

    if count == 0 {
        println!("No audios found to compress");
        return Ok(());
    }

    println!("Found {} audios to compress to {}", count, format);

    let options = AudioCompressOptions::default().set_output_ext(format.into());
    compress_all_audios(ffmpeg, &audios, &options)?;

    println!("Successfully compressed {} audios", count);
    Ok(())
}

fn main() -> Result<()> {
    let ffmpeg = get_ffmpeg()?;
    let args = Args::parse();

    // Resolve the working path
    let path_str = args.path.as_deref().unwrap_or("./");
    let path = Path::new(path_str);

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Determine what to process
    let (is_process_videos, video_fmt) = if args.default {
        (true, "webm".to_string())
    } else {
        (
            args.videos.is_some(),
            args.videos.clone().unwrap_or_default(),
        )
    };

    let (is_process_images, image_fmt) = if args.default {
        (true, "webp".to_string())
    } else {
        (
            args.images.is_some(),
            args.images.clone().unwrap_or_default(),
        )
    };

    let (is_process_audios, audio_fmt) = if args.default {
        (true, "mp3".to_string())
    } else {
        (
            args.audios.is_some(),
            args.audios.clone().unwrap_or_default(),
        )
    };

    // Check if anything to do
    if !is_process_videos && !is_process_images && !is_process_audios {
        println!("No conversion specified. Use --help for usage.");
        return Ok(());
    }

    // Process media
    if is_process_images {
        process_images(&ffmpeg, path, &image_fmt)?;
    }

    if is_process_videos {
        process_videos(&ffmpeg, path, &video_fmt)?;
    }

    if is_process_audios {
        process_audios(&ffmpeg, path, &audio_fmt)?;
    }

    Ok(())
}
