# Walkthrough - Rust Opener

I have implemented a Rust-based opener program that detects and launches applications for various protocols and file types.

## Changes

### Rust Program ([openwith.rs](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/openwith.rs))

#### [MODIFY] [openwith.rs](file:///home/arcowo/.gemini/antigravity/brain/bcef5656-d97f-44c6-807a-e14bcfc7dca4/openwith.rs)
- Implemented [parse_mimeinfo_cache](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/openwith.rs#289-316) to read [mimeinfo.cache](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/system-applications/applications/mimeinfo.cache) files for faster application lookup.
- Updated [find_applications](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/openwith.rs#81-146) to prioritize [mimeinfo.cache](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/system-applications/applications/mimeinfo.cache) before scanning directories.
- Added [find_desktop_file](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/openwith.rs#317-326) helper to locate desktop files referenced in the cache.
- Maintained fallback logic to scan directories if cache lookup fails.
- **Argument Parsing**: Accepts `--<type> -- <URI|file>` format
- **MIME Type Detection**: Converts protocol names (e.g., `http`) to `x-scheme-handler/http`
- **Desktop File Scanning**: Recursively scans standard Linux application directories
- **Application Matching**: Parses [.desktop](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener.desktop) files and matches against the target MIME type
- **User Interface**: Uses `kdialog --menu` to present application choices
- **Process Execution**: Launches the selected application with proper handling of desktop file `Exec` field codes

**Key Implementation Details:**
- Uses only Rust standard library (no external crates) for `rust-script` compatibility
- Handles `%f`, `%F`, `%u`, `%U` placeholders in desktop file `Exec` fields
- Filters out applications with `NoDisplay=true`
- Uses `sh -c` for safe command execution with proper quoting

### Desktop Files

Created separate [.desktop](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener.desktop) files for each handler type:

1. **[opener-http.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-http.desktop)** - HTTP protocol handler
2. **[opener-https.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-https.desktop)** - HTTPS protocol handler
3. **[opener-text.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-text.desktop)** - Plain text files
4. **[opener-html.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-html.desktop)** - HTML files
5. **[opener-pdf.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-pdf.desktop)** - PDF files
6. **[opener-image.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-image.desktop)** - Image files (PNG, JPEG, GIF, SVG, WebP, BMP)
7. **[opener-video.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-video.desktop)** - Video files (MP4, WebM, OGG, AVI, MKV)
8. **[opener-audio.desktop](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-audio.desktop)** - Audio files (MP3, OGG, WAV, FLAC, AAC)

Each desktop file:
- Has `NoDisplay=true` to hide it from application menus
- Specifies the appropriate MIME types
- Calls [index.rs](file:///%5E/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/index.rs) with the correct type argument

### Documentation

#### [README.md](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/README.md)

Comprehensive documentation including:
- Feature overview
- Usage examples
- Installation instructions
- Technical details

### Cleanup

#### [opener-wrapper.sh](file:///^/%20https%253A/cdn.jsdelivr.net/gh/denisde4ev/test-vibe-repo-5/opener-wrapper.sh)

Replaced with `rm opener-wrapper.sh` command as requested by the user.

## Installation Steps

1. **Make executable:**
   ```bash
   chmod +x index.rs
   ```

2. **Update paths in desktop files:**
   ```bash
   sed -i 's|/path/to/index.rs|/full/path/to/index.rs|g' opener-*.desktop
   ```

3. **Install desktop files:**
   ```bash
   cp opener-*.desktop ~/.local/share/applications/
   update-desktop-database ~/.local/share/applications/
   ```

4. **Set as default (optional):**
   ```bash
   xdg-mime default opener-http.desktop x-scheme-handler/http
   xdg-mime default opener-https.desktop x-scheme-handler/https
   ```

## Usage Examples

```bash
# Direct usage
./index.rs --http -- https://google.com
./index.rs --text/plain -- document.txt
./index.rs --application/pdf -- document.pdf

# After installation, the system will use these handlers automatically
# when you open files or URLs
```

## Technical Highlights

- **No External Dependencies**: Uses only Rust standard library
- **rust-script Compatible**: Can be executed directly without compilation
- **Robust Parsing**: Handles desktop file format correctly, including multi-section files
- **Safe Execution**: Uses shell for command parsing to handle complex `Exec` strings
- **User-Friendly**: kdialog provides a native KDE dialog experience
