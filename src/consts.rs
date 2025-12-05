pub const VIDEO_EXTENSIONS: &[&str] = &[
    // Common formats
    "mp4", "avi", "mkv", "mov", "wmv", "flv", // Web formats
    "webm", "ogv", // Other formats
    "mpeg", "mpg", "m4v", "3gp", "3g2", "vob", // Broadcast/transport formats
    "ts", "m2ts", "mts", "mxf", // Professional/raw formats
    "r3d", "braw",
];

pub const IMAGE_EXTENSIONS: &[&str] = &[
    // Common formats
    "jpg", "jpeg", "png", "gif", "bmp", "webp", // Other formats
    "tiff", "tif", "ico", "svg", // Modern formats
    "avif", "heic", "heif", // Raw formats
    "raw", "cr2", "nef", "arw", "dng",
];

pub const AUDIO_EXTENSIONS: &[&str] = &[
    // Common formats
    "mp3", "wav", "aac", "ogg", "flac", "wma", // Modern/web formats
    "opus", "m4a", // Lossless formats
    "aiff", "aif", "alac", "ape", "wv", // Other formats
    "m4b", "m4r", "amr", "mid", "midi", // Surround/professional formats
    "ac3", "dts", "eac3", "mka", // Legacy/other formats
    "ra", "rm", "au", "gsm", "voc", "tta", "snd",
];

#[cfg(target_os = "windows")]
pub const FFMPEG_BINARY: &[u8] = include_bytes!("../assets/ffmpeg.exe");

#[cfg(target_os = "linux")]
pub const FFMPEG_BINARY: &[u8] = include_bytes!("../assets/ffmpeg-linux");

#[cfg(target_os = "macos")]
pub const FFMPEG_BINARY: &[u8] = include_bytes!("../assets/ffmpeg-macos");
