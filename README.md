
# Note:

* I have integrated my command `/bin/$` that evaluates my .bashrc to get the value of `$EDITOR` when missing
* `echo | $EDITOR` is known to work only for `micro` editor (I use it)
* depends on `kdialog` (KDE). {todo: there was some wrapper that works for gnome's one and detects on what system you are, I'll use it in future}


----


from here on AI generated markdown:

# Rust OpenWith

A Rust program that detects and launches applications for specific protocols or file types on Linux systems using `kdialog` for user selection.

## Features

- Detects available applications for protocols (http, https, etc.) and MIME types
- Presents a user-friendly selection dialog using `kdialog`
- Works with `rust-script` - no compilation needed
- No external crates required

## Usage

### Direct Usage

```bash
./openwith.rs --<type> -- <URI|file>
```

**Examples:**
```bash
# Open a URL with HTTP handler selection
./openwith.rs --http -- https://google.com

# Open a text file with text editor selection
./openwith.rs --text/plain -- document.txt

# Open a PDF with PDF viewer selection
./openwith.rs --application/pdf -- document.pdf
```

### Desktop Integration

The project includes `.desktop` files for common protocols and MIME types:

- `openwith-http.desktop` - HTTP URLs
- `openwith-https.desktop` - HTTPS URLs
- `openwith-text.desktop` - Plain text files
- `openwith-html.desktop` - HTML files
- `openwith-pdf.desktop` - PDF files
- `openwith-image.desktop` - Image files (PNG, JPEG, GIF, SVG, WebP, BMP)
- `openwith-video.desktop` - Video files (MP4, WebM, OGG, AVI, MKV)
- `openwith-audio.desktop` - Audio files (MP3, OGG, WAV, FLAC, AAC)
- `openwith-directory.desktop` - Directories
- `openwith-archive.desktop` - Archives (ZIP, TAR, GZ, 7Z, etc.)
- `openwith-code.desktop` - Code and Data files (XML, JSON, YAML, etc.)
- `openwith-document.desktop` - Office documents (ODT, DOCX, XLSX, etc.)
- `openwith-font.desktop` - Fonts (TTF, OTF, etc.)
- `openwith-package.desktop` - Packages (DEB, RPM, AppImage, etc.)
- `openwith-torrent.desktop` - Torrents and Magnet links
- `openwith-geo.desktop` - Geo locations
- `openwith-communication.desktop` - Communication links (Mailto, SSH, IRC, etc.)
- `openwith-crypto.desktop` - Keys and Certificates
- `openwith-ebook.desktop` - E-books and Comics (EPUB, CBZ, etc.)
- `openwith-design.desktop` - Design files (Visio, Krita, etc.)

### Installation

1. Make `openwith.rs` executable:
   ```bash
   chmod +x openwith.rs
   ```

2. Update the `Exec` paths in all `.desktop` files to point to the absolute path of `openwith.rs`:
   ```bash
   # Replace /path/to/ with the actual path
   ./install-desktop-file.sh -e /full/path/to/openwith.rs desktop-files/openwith-*.desktop ~/.local/share/applications/
   ```

3. Update the desktop database:
   ```bash
   update-desktop-database ~/.local/share/applications/
   ```

4. Set the openwith as the default handler for specific types (optional):
   ```bash
   xdg-mime default openwith-http.desktop x-scheme-handler/http
   xdg-mime default openwith-https.desktop x-scheme-handler/https
   xdg-mime default openwith-text.desktop text/plain
   ```

## How It Works

1. **Argument Parsing**: Extracts the type and target (URI/file) from command-line arguments
2. **MIME Type Detection**: Converts protocol names to `x-scheme-handler/<protocol>` format
3. **Desktop File Scanning**: Searches standard Linux application directories:
   - `~/.local/share/applications`
   - `/usr/local/share/applications`
   - `/usr/share/applications`
4. **Application Matching**: Parses `.desktop` files to find applications that support the MIME type
5. **User Selection**: Displays matching applications in a `kdialog` menu
6. **Execution**: Launches the selected application with the target URI/file

## Requirements

- Linux system
- `rust-script` installed
- `kdialog` installed (usually part of KDE Plasma)
- Standard `.desktop` files in system application directories

## Notes

- The program uses only Rust standard library (no external crates) to ensure compatibility with `rust-script`
- Desktop file `Exec` field codes (`%f`, `%F`, `%u`, `%U`) are handled automatically
- Applications with `NoDisplay=true` are filtered out from the selection
