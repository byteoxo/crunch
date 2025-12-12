mod consts;
mod ffmpeg;
mod utilities;

use anyhow::Result;
use clap::Parser;
use ffmpeg::{
    AudioCompressOptions, BaseCompressOptions, ImageCompressOptions, VideoCompressOptions,
    compress_all_audios, compress_all_images, compress_all_videos, get_ffmpeg,
};
use std::path::{Path, PathBuf};
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
    #[arg(short = 'i', long, num_args = 0..=1, default_value = ".", default_missing_value = ".")]
    input_path: PathBuf,

    #[arg(short = 'o', long, num_args = 0..=1, default_value = "./crunch_compressed", default_missing_value = "./crunch_compressed")]
    output_path: PathBuf,

    /// Compress leve
    #[arg(long, num_args = 0..=1, default_value="midium", default_missing_value="midium")]
    level: String,

    #[arg(long, num_args= 0..=1, default_missing_value = "compressed")]
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

fn process_images(ffmpeg: &Path, base_options: BaseCompressOptions) -> Result<()> {
    let path = base_options.input_path.as_path();
    let images = get_image_files(path);
    let count = images.len();

    if count == 0 {
        println!("No images found to compress");
        return Ok(());
    }

    println!(
        "Found {} images to compress to {}",
        count, base_options.output_extension
    );

    let options = ImageCompressOptions::with_base(base_options);
    compress_all_images(ffmpeg, &images, &options)?;

    println!("Successfully compressed {} images", count);
    Ok(())
}

fn process_videos(ffmpeg: &Path, base_options: BaseCompressOptions) -> Result<()> {
    let path = base_options.input_path.as_path();
    let videos = get_video_files(path);
    let count = videos.len();

    if count == 0 {
        println!("No videos found to compress");
        return Ok(());
    }

    println!(
        "Found {} videos to compress to {}",
        count, base_options.output_extension
    );

    let options = VideoCompressOptions::with_base(base_options);
    compress_all_videos(ffmpeg, &videos, &options)?;

    println!("Successfully compressed {} videos", count);
    Ok(())
}

fn process_audios(ffmpeg: &Path, base_options: BaseCompressOptions) -> Result<()> {
    let path = base_options.input_path.as_path();
    let audios = get_audio_files(path);
    let count = audios.len();

    if count == 0 {
        println!("No audios found to compress");
        return Ok(());
    }

    println!(
        "Found {} audios to compress to {}",
        count, base_options.output_extension
    );

    let options = AudioCompressOptions::with_base(base_options);
    compress_all_audios(ffmpeg, &audios, &options)?;

    println!("Successfully compressed {} audios", count);
    Ok(())
}

fn main() -> Result<()> {
    let ffmpeg = get_ffmpeg()?;
    let args = Args::parse();

    // Resolve the working path
    let path = args.input_path.as_path();

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    // Determine what to process
    let (is_process_videos, video_base_options) = if args.default {
        (true, BaseCompressOptions::new_with("video"))
    } else {
        (
            args.videos.is_some(),
            BaseCompressOptions {
                input_path: args.input_path.clone(),
                output_path: args.output_path.clone(),
                output_extension: args.videos.clone().unwrap_or_default(),
                output_prefix: args.prefix.clone(),
                level: args.level.clone(),
            },
        )
    };

    let (is_process_images, image_base_options) = if args.default {
        (true, BaseCompressOptions::new_with("image"))
    } else {
        (
            args.images.is_some(),
            BaseCompressOptions {
                input_path: args.input_path.clone(),
                output_path: args.output_path.clone(),
                output_extension: args.images.clone().unwrap_or_default(),
                output_prefix: args.prefix.clone(),
                level: args.level.clone(),
            },
        )
    };

    let (is_process_audios, audio_base_options) = if args.default {
        (true, BaseCompressOptions::new_with("audio"))
    } else {
        (
            args.audios.is_some(),
            BaseCompressOptions {
                input_path: args.input_path.clone(),
                output_path: args.output_path.clone(),
                output_extension: args.audios.clone().unwrap_or_default(),
                output_prefix: args.prefix.clone(),
                level: args.level.clone(),
            },
        )
    };

    // Check if anything to do
    if !is_process_videos && !is_process_images && !is_process_audios {
        println!("No conversion specified. Use --help for usage.");
        return Ok(());
    }

    // Process media
    if is_process_images {
        process_images(&ffmpeg, image_base_options)?;
    }

    if is_process_videos {
        process_videos(&ffmpeg, video_base_options)?;
    }

    if is_process_audios {
        process_audios(&ffmpeg, audio_base_options)?;
    }

    Ok(())
}
