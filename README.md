# File Organizer CLI (Forg)

Introducing Forg! A fast, efficient command-line tool written in Rust for organizing files by type, date, or custom rules. Perfect for cleaning up messy directories and maintaining organized file systems.


https://github.com/user-attachments/assets/7c3948bf-2e9a-41c3-87e6-eb587dade09a

## Features

- **Organize by file type**: Automatically sorts files into folders based on their extensions
- **Organize by date**: Groups files by creation/modification date (year, month, or day)
- **Clean empty directories**: Removes empty folders after organization
- **File statistics**: Shows detailed breakdown of file types and sizes
- **Dry run mode**: Preview changes before executing
- **Colorized output**: Easy-to-read terminal output with colored text labels
- **Configurable**: Customize file type mappings and ignore patterns

## Installation

### From Source

```bash
git clone https://github.com/lazzerex/forg-file-organizer
cd forg-file-organizer
cargo build --release
```

The binary will be available at `target/release/forg`.

### Add to PATH (Optional)

```bash
# Add to your shell profile (.bashrc, .zshrc, etc.)
export PATH="$PATH:/path/to/forg-file-organizer/target/release"
```

## Usage

### Organize by File Type

```bash
# Organize current directory
forg by-type

# Organize specific directory
forg by-type -s /path/to/messy/folder

# Organize to different target directory
forg by-type -s /path/to/source -t /path/to/target

# Preview changes without moving files
forg by-type -s /path/to/folder --dry-run
```

### Organize by Date

```bash
# Organize by month (default)
forg by-date -s /path/to/photos

# Organize by year
forg by-date -s /path/to/documents -f year

# Organize by day
forg by-date -s /path/to/files -f day --dry-run
```

### Clean Empty Directories

```bash
# Clean current directory
forg clean

# Clean specific directory
forg clean -d /path/to/directory --dry-run
```

### Show File Statistics

```bash
# Analyze current directory
forg stats

# Analyze specific directory
forg stats -d /path/to/analyze
```

### Generate Configuration File

```bash
# Generate default config
forg config

# Generate config to specific path
forg config -o my-config.json
```

## File Type Categories

The tool automatically categorizes files into these folders:

- **Images**: jpg, png, gif, svg, webp, etc.
- **Documents**: pdf, doc, docx, txt, rtf, etc.
- **Code**: rs, py, js, html, css, cpp, java, etc.
- **Videos**: mp4, avi, mkv, mov, webm, etc.
- **Audio**: mp3, wav, flac, aac, ogg, etc.
- **Archives**: zip, rar, 7z, tar, gz, etc.
- **Applications**: exe, msi, deb, rpm, pkg, etc.
- **Others**: Files that don't match known categories

## Configuration

Generate a configuration file to customize behavior:

```bash
forg config -o forg-config.json
```

Example configuration:

```json
{
  "file_type_mappings": {
    "jpg": "Photos",
    "png": "Photos",
    "pdf": "PDFs",
    "rs": "Rust-Code"
  },
  "ignore_patterns": [
    ".DS_Store",
    "Thumbs.db",
    "*.tmp",
    ".git"
  ],
  "date_formats": {
    "year": "%Y",
    "month": "%Y-%m",
    "day": "%Y-%m-%d"
  }
}
```

## Examples

### Before Organization
```
messy-folder/
├── photo1.jpg
├── photo2.png
├── document.pdf
├── script.py
├── song.mp3
├── movie.mp4
└── archive.zip
```

### After `forg by-type`
```
messy-folder/
├── Images/
│   ├── photo1.jpg
│   └── photo2.png
├── Documents/
│   └── document.pdf
├── Code/
│   └── script.py
├── Audio/
│   └── song.mp3
├── Videos/
│   └── movie.mp4
└── Archives/
    └── archive.zip
```

### After `forg by-date -f month`
```
photos/
├── 2024-01/
│   ├── vacation1.jpg
│   └── vacation2.jpg
├── 2024-02/
│   ├── birthday.jpg
│   └── party.jpg
└── 2024-03/
    └── spring.jpg
```

## Command Reference

| Command | Description | Options |
|---------|-------------|---------|
| `by-type` | Organize files by extension | `-s/--source`, `-t/--target`, `--dry-run` |
| `by-date` | Organize files by date | `-s/--source`, `-t/--target`, `-f/--format`, `--dry-run` |
| `clean` | Remove empty directories | `-d/--directory`, `--dry-run` |
| `stats` | Show file statistics | `-d/--directory` |
| `config` | Generate config file | `-o/--output` |

## Development

### Prerequisites

- Rust 1.70+ 
- Cargo

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Debug Info

```bash
RUST_LOG=debug cargo run -- by-type --dry-run
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [ ] Custom rules engine for complex organization logic
- [ ] Integration with cloud storage (Dropbox, Google Drive)
- [ ] GUI version using Tauri
- [ ] Plugin system for custom file processors
- [ ] Batch operations and scheduling
- [ ] File deduplication features
- [ ] WebAssembly version for browser usage

## Performance

Benchmarked on a directory with 10,000 mixed files:
- Organization by type: ~2.1 seconds
- Organization by date: ~2.3 seconds
- Statistics generation: ~1.8 seconds

## Troubleshooting

### Permission Errors
Make sure you have read/write permissions for both source and target directories.

### Large Directory Performance
For directories with 100k+ files, consider using the `--dry-run` flag first to estimate processing time.

### File Name Conflicts
If files with the same name exist in the target location, the tool will skip them and report an error. Consider renaming files before organizing.

## Similar Tools

- [organize-cli](https://github.com/tfeldmann/organize) - Python-based file organizer
- [fclones](https://github.com/pkolaczk/fclones) - Rust duplicate file finder
- [fd](https://github.com/sharkdp/fd) - Fast file finder alternative to `find`

---

Made with ❤️ and ⚡ Rust
