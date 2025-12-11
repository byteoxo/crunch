# Crunch v0.1.0 - Initial Release 🦀

> A blazingly fast, parallel media compression tool powered by FFmpeg

We're excited to announce the first release of **Crunch** — a command-line tool designed for non-programmers who need to compress media files quickly and efficiently.

## Highlights

- 🚀 **Parallel Processing** — Leverages all CPU cores for maximum performance using Rayon
- 🎥 **Video Compression** — VP9/WebM encoding with configurable quality levels
- 🖼️ **Image Compression** — WebP output with adjustable compression settings
- 🎵 **Audio Compression** — MP3 encoding with bitrate control
- 📦 **Embedded FFmpeg** — No external dependencies required; FFmpeg is bundled for Windows, Linux, and macOS
- 📊 **Real-time Progress** — Visual progress bars powered by Indicatif

## Features

### Supported Input Formats

| Type | Formats |
|------|---------|
| **Video** | mp4, avi, mkv, mov, wmv, flv, webm, ogv, mpeg, mpg, m4v, 3gp, 3g2, vob, ts, m2ts, mts, mxf, r3d, braw |
| **Image** | jpg, jpeg, png, gif, bmp, webp, tiff, tif, ico, svg, avif, heic, heif, raw, cr2, nef, arw, dng |
| **Audio** | mp3, wav, aac, ogg, flac, wma, opus, m4a, aiff, aif, alac, ape, wv, m4b, m4r, amr, mid, midi, ac3, dts, eac3, mka, ra, rm, au, gsm, voc, tta, snd |

### Default Output Formats

- **Video** → WebM (VP9 codec with Opus audio)
- **Image** → WebP
- **Audio** → MP3

### Compression Levels

Three preset compression levels to balance quality and file size:

| Level | Description | Use Case |
|-------|-------------|----------|
| `low` | Minimal compression, high quality | Archival, professional work |
| `medium` | Balanced compression (default) | General purpose |
| `high` | Maximum compression, smaller files | Web delivery, storage savings |

## Installation

No external FFmpeg installation required — Crunch includes embedded FFmpeg binaries!

### Step 1: Download

Download the appropriate binary for your platform from the [Releases](https://github.com/byteoxo/crunch/releases/tag/v0.1.0) page:

| Platform | File |
|----------|------|
| macOS (Intel) | `crunch-macos-x64` |
| macOS (Apple Silicon) | `crunch-macos-arm64` |
| Linux (x64) | `crunch-linux-x64` |
| Windows (x64) | `crunch-windows-x64.exe` |

### Step 2: Add to PATH

#### macOS / Linux

```bash
# Move the binary to a directory in your PATH
chmod +x crunch-*
sudo mv crunch-* /usr/local/bin/crunch

# Or add to your user bin directory
mkdir -p ~/.local/bin
mv crunch-* ~/.local/bin/crunch
chmod +x ~/.local/bin/crunch

# Add to PATH (add this line to ~/.bashrc or ~/.zshrc)
export PATH="$HOME/.local/bin:$PATH"
```

#### Windows

1. Create a directory for the binary, e.g., `C:\Program Files\crunch\`
2. Move `crunch-windows-x64.exe` to that directory and rename it to `crunch.exe`
3. Add the directory to your system PATH:
   - Open **Settings** → **System** → **About** → **Advanced system settings**
   - Click **Environment Variables**
   - Under **System variables**, find and select **Path**, then click **Edit**
   - Click **New** and add `C:\Program Files\crunch\`
   - Click **OK** to save

### Step 3: Verify Installation

```bash
crunch --version
crunch --help
```

## Usage

### Basic Commands

```bash
# Compress all media with default settings
crunch --default

# Compress videos only (default format: webm)
crunch --videos

# Compress images only (default format: webp)
crunch --images

# Compress audio only (default format: mp3)
crunch --audios
```

### Specify Output Formats

```bash
crunch --videos=mp4
crunch --images=avif
crunch --audios=opus
```

### Specify Working Directory

```bash
crunch --default -p /path/to/media
```

### Compression Levels

```bash
# High quality, larger files
crunch --default --level=low

# Balanced (default)
crunch --default --level=medium

# Smaller files, lower quality
crunch --default --level=high
```

### Custom Output Prefix

```bash
# Output files will be named: myprefix_originalname.ext
crunch --default --prefix=myprefix
```

## Technical Details

- **Video Encoding**: VP9 codec (`libvpx-vp9`) with CRF-based quality control
  - Pixel format: `yuv420p` for maximum browser compatibility
  - Row-based multithreading enabled for faster encoding
  - Audio track encoded with Opus at 64kbps

- **Image Encoding**: WebP with configurable quality and compression level

- **Audio Encoding**: MP3 (`libmp3lame`) with configurable bitrate (64k - 192k)

## Platform Support

| Platform | Architecture | Status |
|----------|--------------|--------|
| macOS | x64 / ARM64 | ✅ Supported |
| Linux | x64 | ✅ Supported |
| Windows | x64 | ✅ Supported |

## Dependencies

- [Clap](https://github.com/clap-rs/clap) — Command line argument parsing
- [Rayon](https://github.com/rayon-rs/rayon) — Data parallelism
- [Indicatif](https://github.com/console-rs/indicatif) — Progress bars
- [WalkDir](https://github.com/BurntSushi/walkdir) — Recursive directory traversal
- [Anyhow](https://github.com/dtolnay/anyhow) — Error handling

## Known Limitations

- Recursive directory scanning is always enabled
- Output files are placed in the same directory as input files
- No GPU acceleration (planned for future releases)

## What's Next

Future releases will include:

- [ ] GPU acceleration (NVENC, VideoToolbox)
- [ ] Watch mode for auto-compressing new files
- [ ] Additional output formats (AV1, AVIF)
- [ ] Configurable output directory
- [ ] Config file support
- [ ] Dry-run mode

## License

MIT License — see [LICENSE](LICENSE) for details.

---

**Full Changelog**: https://github.com/byteoxo/crunch/commits/v0.1.0

Made with 🦀 and ❤️ by [byteoxo](https://github.com/byteoxo)