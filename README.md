<div align="center">

# 🦀 Crunch

**A easiest, blazingly fast, parallel media compression tool for normal people**

[![Crates.io](https://img.shields.io/crates/v/crunch.svg)](https://crates.io/crates/crunch)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/github/actions/workflow/status/yourname/crunch/ci.yml?branch=main)](https://github.com/yourname/crunch/actions)
[![Downloads](https://img.shields.io/crates/d/crunch.svg)](https://crates.io/crates/crunch)

[Installation](#installation) •
[Usage](#usage) •
[Examples](#examples) •
[Configuration](#configuration) •
[Contributing](#contributing)

</div>

---

## ✨ Features

- 🚀 **Blazingly Fast** — Parallel & concurrent processing utilizing all CPU cores
- 🎬 **Video Compression** — H.265/HEVC, VP9, AV1 encoding with optimal settings
- 🖼️ **Image Compression** — WebP, AVIF, optimized JPEG/PNG output
- 🎵 **Audio Compression** — Opus, AAC, MP3 with configurable bitrates
- 📁 **Batch Processing** — Compress entire directories with one command
- 🔄 **Recursive Mode** — Process nested subdirectories
- 📊 **Progress Display** — Real-time progress bars and statistics
- ⚙️ **Highly Configurable** — Fine-tune quality, format, and performance
- 🛡️ **Safe Defaults** — Sensible presets that just work

---

## 📦 Installation
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

## 🚀 Usage

### Quick Start

```bash
# Compress all media with default settings
crunch --default

# Compress only videos
crunch --videos

# Compress videos and images with specific formats
crunch --videos=mp4 --images=avif

# Specify input/output directories
crunch --default -i ./raw -o ./compressed
```

### Command Line Options

```
Usage: crunch [OPTIONS]

Options:
      --default
          Use default settings (videos=webm, images=webp)

  -p, --path [<PATH>]
          Path to process (default: current directory)

      --level [<LEVEL>]
          Compress leve

      --prefix [<PREFIX>]


      --videos [<VIDEOS>]
          Video format. Use --videos for default(webm) or --videos=FORMAT

      --images [<IMAGES>]
          Image format. Use --images for default(webp) or --images=FORMAT

      --audios [<AUDIOS>]
          Audios format. Use --audios for default(webp) or --audios=FORMAT

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

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
```
---

## 📖 Examples

### Basic Usage

```bash
# Compress all videos in current directory to WebM
crunch --videos

# Compress with maximum quality
crunch --videos --level=low

# Fast compression with lower quality
crunch --videos --level=high

# Full compression pipeline with custom settings
crunch \
  --videos=mp4 \
  --images=avif \
  --audio=opus \
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         Crunch                               │
├──────────────┬──────────────┬──────────────┬────────────────┤
│   Scanner    │   Scheduler  │   Workers    │   Reporter     │
│              │              │              │                │
│  - Walk dirs │  - Job queue │  - FFmpeg    │  - Progress    │
│  - Filter    │  - Priority  │  - Parallel  │  - Statistics  │
│  - Classify  │  - Balance   │  - Async I/O │  - Errors      │
└──────────────┴──────────────┴──────────────┴────────────────┘
                              │
                              ▼
                    ┌──────────────────┐
                    │     FFmpeg       │
                    │  (subprocess)    │
                    └──────────────────┘
```

---

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourname/crunch.git
cd crunch

# Install development dependencies
cargo build

# Run with debug output
RUST_LOG=debug cargo run -- --default

# Format code
cargo fmt

# Lint
cargo clippy
```

### Commit Convention

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add AV1 encoding support
fix: handle spaces in filenames
docs: update installation instructions
perf: optimize parallel job scheduling
refactor: split video module
test: add compression quality tests
```

### Roadmap

- [ ] GPU acceleration (NVENC, VideoToolbox)
- [ ] Watch mode (auto-compress new files)
- [ ] Web UI dashboard
- [ ] Cloud storage integration (S3, GCS)
- [ ] Custom FFmpeg preset files

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [FFmpeg](https://ffmpeg.org/) — The powerhouse behind all media processing
- [Rayon](https://github.com/rayon-rs/rayon) — Data parallelism library for Rust
- [Clap](https://github.com/clap-rs/clap) — Command line argument parser
- [Indicatif](https://github.com/console-rs/indicatif) — Progress bar library

---

<div align="center">

**If you find Crunch useful, please consider giving it a ⭐**

Made with 🦀 and ❤️

</div>

