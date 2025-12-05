use crate::consts::{AUDIO_EXTENSIONS, IMAGE_EXTENSIONS, VIDEO_EXTENSIONS};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Get all image files in a directory (recursive)
pub fn get_image_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_image_file(entry.path()))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

/// Get all video files in a directory(recursive)
pub fn get_video_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_video_file(entry.path()))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

/// Get all audio files in a directory(recursive)
pub fn get_audio_files(dir: &Path) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| is_audio_file(entry.path()))
        .map(|entry| entry.path().to_path_buf())
        .collect()
}

/// Check if a file is an image based on extension
pub fn is_image_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| IMAGE_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Check if a file is an video based on extension
pub fn is_video_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| VIDEO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Check if a file is an audio based on extension
pub fn is_audio_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| AUDIO_EXTENSIONS.contains(&ext.to_lowercase().as_str()))
        .unwrap_or(false)
}

/// Get all image files (non-recursive, single directory only)
#[allow(unused)]
pub fn get_image_files_flat(dir: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(dir)
        .into_iter()
        .flatten()
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.is_file() && is_image_file(path))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{self, File};
    use tempfile::TempDir;

    /// Helper function to create a test file
    fn create_file(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        File::create(path).unwrap();
    }

    #[test]
    fn test_get_image_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let result = get_image_files(temp_dir.path());
        assert!(result.is_empty());
    }

    #[test]
    fn test_get_image_files_single_image() {
        let temp_dir = TempDir::new().unwrap();
        let image_path = temp_dir.path().join("test.jpg");
        create_file(&image_path);

        let result = get_image_files(temp_dir.path());

        assert_eq!(result.len(), 1);
        assert!(result.contains(&image_path));
    }

    #[test]
    fn test_get_image_files_multiple_extensions() {
        let temp_dir = TempDir::new().unwrap();

        let jpg_path = temp_dir.path().join("photo.jpg");
        let png_path = temp_dir.path().join("image.png");
        let gif_path = temp_dir.path().join("animation.gif");
        let webp_path = temp_dir.path().join("modern.webp");

        create_file(&jpg_path);
        create_file(&png_path);
        create_file(&gif_path);
        create_file(&webp_path);

        let result = get_image_files(temp_dir.path());

        assert_eq!(result.len(), 4);
        assert!(result.contains(&jpg_path));
        assert!(result.contains(&png_path));
        assert!(result.contains(&gif_path));
        assert!(result.contains(&webp_path));
    }

    #[test]
    fn test_get_image_files_ignores_non_images() {
        let temp_dir = TempDir::new().unwrap();

        let image_path = temp_dir.path().join("photo.jpg");
        let txt_path = temp_dir.path().join("readme.txt");
        let pdf_path = temp_dir.path().join("document.pdf");
        let no_ext_path = temp_dir.path().join("noextension");

        create_file(&image_path);
        create_file(&txt_path);
        create_file(&pdf_path);
        create_file(&no_ext_path);

        let result = get_image_files(temp_dir.path());

        assert_eq!(result.len(), 1);
        assert!(result.contains(&image_path));
    }

    #[test]
    fn test_get_image_files_recursive() {
        let temp_dir = TempDir::new().unwrap();

        // Root level image
        let root_image = temp_dir.path().join("root.jpg");
        create_file(&root_image);

        // Nested directory with image
        let nested_image = temp_dir.path().join("subdir/nested.png");
        create_file(&nested_image);

        // Deeply nested image
        let deep_image = temp_dir.path().join("a/b/c/deep.gif");
        create_file(&deep_image);

        let result = get_image_files(temp_dir.path());

        assert_eq!(result.len(), 3);
        assert!(result.contains(&root_image));
        assert!(result.contains(&nested_image));
        assert!(result.contains(&deep_image));
    }

    #[test]
    fn test_get_image_files_case_insensitive_extension() {
        let temp_dir = TempDir::new().unwrap();

        let lower = temp_dir.path().join("lower.jpg");
        let upper = temp_dir.path().join("upper.JPG");
        let mixed = temp_dir.path().join("mixed.JpG");

        create_file(&lower);
        create_file(&upper);
        create_file(&mixed);

        let result = get_image_files(temp_dir.path());

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_get_image_files_nonexistent_directory() {
        let path = Path::new("/nonexistent/path/that/does/not/exist");
        let result = get_image_files(path);
        assert!(result.is_empty());
    }

    #[test]
    fn test_is_image_file() {
        assert!(is_image_file(Path::new("test.jpg")));
        assert!(is_image_file(Path::new("test.JPG")));
        assert!(is_image_file(Path::new("test.png")));
        assert!(is_image_file(Path::new("test.gif")));
        assert!(!is_image_file(Path::new("test.txt")));
        assert!(!is_image_file(Path::new("test")));
        assert!(!is_image_file(Path::new("")));
    }
}
