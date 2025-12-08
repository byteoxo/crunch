use super::progress_bar::init_progress_bar;
use anyhow::{Context, Result, bail};
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct BaseCompressOptions {
    pub output_extension: String,
    pub output_prefix: Option<String>,
}

impl BaseCompressOptions {
    /// mt: media_type
    pub fn new_with(mt: &str) -> Self {
        let output_extension = match mt {
            "video" => "webm".to_string(),
            "image" => "webp".to_string(),
            "audio" => "mp3".to_string(),
            _ => panic!("Unknow media type"),
        };
        Self {
            output_extension,
            output_prefix: None,
        }
    }
}

pub struct ImageCompressOptions {
    pub quality: u8,
    pub compression_level: u8,
    pub base: BaseCompressOptions,
}

impl Default for ImageCompressOptions {
    fn default() -> Self {
        Self {
            quality: 1,
            compression_level: 6,
            base: BaseCompressOptions {
                output_extension: "webp".into(),
                output_prefix: Some("compressed".to_string()),
            },
        }
    }
}

impl ImageCompressOptions {
    pub fn with_base(base: BaseCompressOptions) -> Self {
        Self {
            quality: 1,
            compression_level: 6,
            base,
        }
    }
}

pub struct VideoCompressOptions {
    pub crf: u8,        // Constant Rate Factor (0-51, lower is better quality). Default: 23
    pub preset: String, // ultrafast, superfast, veryfast, faster, fast, medium, slow, slower, veryslow
    pub video_codec: String, // e.g., "libx264", "libx265"
    pub base: BaseCompressOptions,
}

impl Default for VideoCompressOptions {
    fn default() -> Self {
        Self {
            crf: 42,
            preset: "good".to_string(),
            video_codec: "libvpx-vp9".to_string(),
            base: BaseCompressOptions {
                output_extension: "webm".to_string(),
                output_prefix: Some("compressed".to_string()),
            },
        }
    }
}

impl VideoCompressOptions {
    pub fn with_base(base: BaseCompressOptions) -> Self {
        Self {
            crf: 42,
            preset: "good".to_string(),
            video_codec: "libvpx-vp9".to_string(),
            base,
        }
    }
}

pub struct AudioCompressOptions {
    pub bitrate: String,          // e.g., "64k", "128k", "32k"
    pub audio_codec: String,      // e.g., "libmp3lame", "libopus", "aac"
    pub channels: Option<u8>,     // 1 = mono, 2 = stereo, None = keep original
    pub sample_rate: Option<u32>, // e.g., 44100, 22050, None = keep original
    pub base: BaseCompressOptions,
}

impl Default for AudioCompressOptions {
    fn default() -> Self {
        Self {
            bitrate: "64k".to_string(),
            audio_codec: "libmp3lame".to_string(),
            channels: None,
            sample_rate: None,
            base: BaseCompressOptions {
                output_extension: "mp3".to_string(),
                output_prefix: Some("compressed".to_string()),
            },
        }
    }
}

impl AudioCompressOptions {
    pub fn with_base(base: BaseCompressOptions) -> Self {
        Self {
            bitrate: "64k".to_string(),
            audio_codec: "libmp3lame".to_string(),
            channels: None,
            sample_rate: None,
            base,
        }
    }

    #[allow(unused)]
    pub fn set_bitrate(&mut self, bitrate: String) -> Self {
        Self {
            bitrate,
            audio_codec: self.audio_codec.clone(),
            channels: self.channels,
            sample_rate: self.sample_rate,
            base: BaseCompressOptions {
                output_prefix: Some("compressed".to_string()),
                output_extension: self.base.output_extension.clone(),
            },
        }
    }
}

pub fn compress_audio(
    ffmpeg: &Path,
    input: &Path,
    options: &AudioCompressOptions,
) -> Result<PathBuf> {
    if !ffmpeg.exists() {
        bail!("FFmpeg executable not found at: {}", ffmpeg.display());
    }

    // empty path as fallback
    let parent = input.parent().unwrap_or(Path::new(""));

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let prefix = match &options.base.output_prefix {
        Some(p) => format!("{}_", p),
        None => String::new(),
    };

    let new_filename = format!("{}{}.{}", prefix, stem, options.base.output_extension);
    let mut output = parent.join(new_filename);

    // Avoid overwriting input file.
    if input == output {
        let new_filename = format!(
            "compressed_{}{}.{}",
            prefix, stem, options.base.output_extension
        );
        output = parent.join(new_filename);
    }

    let mut args = vec![
        "-i".to_string(),
        input.to_str().context("Invalid input path")?.to_string(),
        "-c:a".to_string(),
        options.audio_codec.clone(),
        "-b:a".to_string(),
        options.bitrate.clone(),
    ];

    // Add channels if specified
    if let Some(channels) = options.channels {
        args.push("-ac".to_string());
        args.push(channels.to_string());
    }

    // Add sample rate if specified
    if let Some(sample_rate) = options.sample_rate {
        args.push("-ar".to_string());
        args.push(sample_rate.to_string());
    }

    // Overwrite output and add output path
    args.push("-y".to_string());
    args.push(output.to_str().context("Invalid output path")?.to_string());

    let result = Command::new(ffmpeg)
        .args(&args)
        .output()
        .context("Failed to execute ffmpeg")?;

    if result.status.success() {
        Ok(output)
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("Failed to compress {}: {}", input.display(), stderr)
    }
}

pub fn compress_all_audios(
    ffmpeg: &Path,
    audios: &Vec<PathBuf>,
    options: &AudioCompressOptions,
) -> Result<Vec<Result<PathBuf>>> {
    let count = audios.len() as u64;
    let pb = init_progress_bar(count);

    let results: Vec<Result<PathBuf>> = audios
        .par_iter()
        .map(|audio| {
            let name = audio.file_name().unwrap_or_default().to_string_lossy();
            pb.println(format!("Processing: {}", name));

            let start = std::time::Instant::now();
            let res = compress_audio(ffmpeg, audio, options);
            let duration = start.elapsed();

            match &res {
                Ok(_) => pb.println(format!("Finished: {} (took {:.1?})", name, duration)),
                Err(e) => {
                    pb.println(format!("FAILED: {} \nReason: {}", name, e));
                }
            }

            res
        })
        .progress_with(pb.clone())
        .collect();

    pb.finish_with_message("Audio Compression complete");

    Ok(results)
}

pub fn compress_image(
    ffmpeg: &Path,
    input: &Path,
    options: &ImageCompressOptions,
) -> Result<PathBuf> {
    if !ffmpeg.exists() {
        bail!("FFmpeg executable not found at: {}", ffmpeg.display());
    }

    let parent = input.parent().unwrap_or(Path::new(""));

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");
    let prefix = match &options.base.output_prefix {
        Some(p) => format!("{}_", p),
        None => String::new(),
    };

    let new_filename = format!("{}{}.{}", prefix, stem, options.base.output_extension);
    let mut output = parent.join(new_filename);

    // Avoid overwriting output path.
    if input == output {
        let new_filename = format!(
            "compressed_{}{}.{}",
            prefix, stem, options.base.output_extension
        );
        output = parent.join(new_filename);
    }

    let result = Command::new(ffmpeg)
        .args([
            "-i",
            input.to_str().context("Invalid input path")?,
            "-c:v",
            "libwebp",
            "-quality",
            &options.quality.to_string(),
            "-compression_level",
            &options.compression_level.to_string(),
            "-y",
            output.to_str().context("Invalid output path")?,
        ])
        .output()
        .context("Failed to execute ffmpeg")?;

    if result.status.success() {
        Ok(output)
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("Failed to compress {}: {}", input.display(), stderr)
    }
}

pub fn compress_all_images(
    ffmpeg: &Path,
    images: &Vec<PathBuf>,
    options: &ImageCompressOptions,
) -> Result<Vec<Result<PathBuf>>> {
    // Initialize the progress bar
    let count = images.len() as u64;
    let pb = init_progress_bar(count);

    let results: Vec<Result<PathBuf>> = images
        .par_iter()
        .map(|image| {
            // Note: Avoid using println! here as it interferes with the progress bar
            let name = image.file_name().unwrap_or_default().to_string_lossy();
            pb.println(format!("Processing: {}", name));

            let start = std::time::Instant::now();
            let res = compress_image(ffmpeg, image, options);
            let duration = start.elapsed();

            match &res {
                Ok(_) => pb.println(format!("Finished: {} (took {:.1?})", name, duration)),
                Err(e) => {
                    pb.println(format!("FAILED: {} \nReason: {}", name, e));
                }
            }

            res
        })
        .progress_with(pb.clone()) // Attach the progress bar to rayon iterator
        .collect();

    pb.finish_with_message("Images Compression complete");

    Ok(results)
}

pub fn compress_video(
    ffmpeg: &Path,
    input: &Path,
    options: &VideoCompressOptions,
) -> Result<PathBuf> {
    if !ffmpeg.exists() {
        bail!("FFmpeg executable not found at: {}", ffmpeg.display());
    }

    // empty path as fallback
    let parent = input.parent().unwrap_or(Path::new(""));

    let prefix = match &options.base.output_prefix {
        Some(p) => format!("{}_", p),
        None => String::new(),
    };

    let stem = input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("output");

    let new_filename = format!("{}{}.{}", prefix, stem, options.base.output_extension);
    let mut output = parent.join(new_filename);

    // Avoid overwriting output path
    if input == output {
        let new_filename = format!(
            "compressed_{}{}.{}",
            prefix, stem, options.base.output_extension
        );
        output = parent.join(new_filename);
    }

    //  Ensure input path is valid
    let input_str = input.to_str().context("Invalid input path")?;
    let output_str = output.to_str().context("Invalid output path")?;

    let result = Command::new(ffmpeg)
        .args([
            "-i",
            input_str,
            // Video Codec
            "-c:v",
            &options.video_codec,
            // CRITICAL: Force pixel format for Chrome/Web compatibility
            "-pix_fmt",
            "yuv420p",
            // CRITICAL: VP9 requires -b:v 0 for CRF to work
            "-b:v",
            "0",
            "-crf",
            &options.crf.to_string(),
            // Performance settings (makes encoding faster than default)
            "-deadline",
            &options.preset,
            "-cpu-used",
            "4", // Range 0-5. 4 is a good balance of speed/size
            "-row-mt",
            "1", // Enable row-based multithreading
            // Audio Codec
            "-c:a",
            "libopus",
            "-b:a",
            "64k",
            "-y", // Overwrite output
            output_str,
        ])
        .output()
        .context("Failed to execute ffmpeg process")?;

    if result.status.success() {
        Ok(output)
    } else {
        let stderr = String::from_utf8_lossy(&result.stderr);
        bail!("Failed to compress {}: {}", input.display(), stderr)
    }
}

pub fn compress_all_videos(
    ffmpeg: &Path,
    videos: &Vec<PathBuf>,
    options: &VideoCompressOptions,
) -> Result<Vec<Result<PathBuf>>> {
    let count = videos.len() as u64;
    println!("Found {} videos to compress", count);

    if count == 0 {
        return Ok(vec![]);
    }

    // 1. Verify FFmpeg path exists before starting
    if !ffmpeg.exists() {
        bail!("FFmpeg executable not found at: {}", ffmpeg.display());
    }

    let pb = init_progress_bar(count);

    // 2. Process videos in parallel
    let results: Vec<Result<PathBuf>> = videos
        .par_iter()
        .map(|video| {
            let name = video.file_name().unwrap_or_default().to_string_lossy();
            pb.println(format!("Processing: {}", name));

            let start = std::time::Instant::now();
            let res = compress_video(ffmpeg, video, options);
            let duration = start.elapsed();

            match &res {
                Ok(_) => {
                    pb.println(format!("Finished: {} (took {:.1?})", name, duration));
                }
                Err(e) => {
                    // Print the actual error to the console so you can see it
                    pb.println(format!("FAILED: {} \nReason: {}", name, e));
                }
            }
            res
        })
        .progress_with(pb.clone())
        .collect();

    pb.finish_with_message("Video compression complete");

    // 3. Check if everything failed
    let failures = results.iter().filter(|r| r.is_err()).count();
    if failures > 0 {
        println!(
            "\nWARNING: {} videos failed to compress. Check the logs above.",
            failures
        );
    }
    Ok(results)
}

#[allow(unused)]
pub fn compress_all_with_progress(
    ffmpeg: &Path,
    images: &Vec<PathBuf>,
    options: &ImageCompressOptions,
) -> Result<(Vec<PathBuf>, Vec<anyhow::Error>)> {
    let results = compress_all_images(ffmpeg, images, options)?;

    let mut successes = Vec::new();
    let mut failures = Vec::new();

    for result in results {
        match result {
            Ok(path) => successes.push(path),
            Err(e) => failures.push(e),
        }
    }

    Ok((successes, failures))
}
