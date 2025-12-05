#!/bin/bash
# Install desktop files and update their Exec paths

set -e

# Parse options
exec_path=""

# Check if we have at least 2 arguments
case $1 in --help)
	printf %s\\n \
		"Usage: $0 [-e EXEC_PATH] <desktop-file>... <target>" \
		"" \
		"Options:" \
		"  -e EXEC_PATH    Path to the executable (default: auto-detect openwith.rs)" \
		"" \
		"Example: $0 desktop-files/openwith-*.desktop ~/.local/share/applications/" \
		"Example: $0 -e /usr/bin/openwith desktop-files/openwith-*.desktop /usr/share/applications/" \
	;
	exit
esac

# Parse -e option
while getopts "e:" opt; do
	case $opt in
		e) exec_path="$OPTARG";;
		\?) echo "Error: Invalid option -$OPTARG" >&2; exit 1;;
	esac
done
shift $((OPTIND-1))

case $1 in
	--) shift;;
	-*) echo "Error: Invalid option $1" >&2; exit 1;;
esac

# Get the last argument as the target
target="${@: -1}"

# Get all arguments except the last one as source files
source_files=("${@:1:$#-1}")


# Determine if target is a directory or file
if [ -d "$target" ]; then
	# Target is a directory
	target_is_dir=true
	target_dir="$target"
elif [ "${#source_files[@]}" -eq 1 ]; then
	# Target is a file (or doesn't exist yet), and we have only one source
	target_is_dir=false
	target_dir="$(dirname "$target")"
	target_file="$target"
	
	# Create target directory if it doesn't exist
	mkdir -p "$target_dir"
else
	# Multiple source files but target is not a directory
	echo "Error: When copying multiple files, target must be a directory" >&2
	exit 1
fi

# Get the executable path
if [ -z "$exec_path" ]; then
	# Auto-detect: get the absolute path to openwith.rs (assumes it's in the same directory as this script)
	script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
	exec_path="$script_dir/openwith.rs"
	
	if [ ! -f "$exec_path" ]; then
		echo "Error: openwith.rs not found at $exec_path" >&2
		echo "Use -e option to specify executable path" >&2
		exit 1
	fi
	
	# Make openwith.rs executable
	chmod +x "$exec_path"
fi

# Process each desktop file
for desktop_file in "${source_files[@]}"; do
	if [ ! -f "$desktop_file" ]; then
		echo "Warning: '$desktop_file' is not a file, skipping" >&2
		continue
	fi
	
	# Determine the target file path
	if [ "$target_is_dir" = true ]; then
		basename="$(basename "$desktop_file")"
		dest_file="$target_dir/$basename"
	else
		dest_file="$target_file"
	fi
	
	echo "Installing $(basename "$desktop_file") to $dest_file"
	
	# Replace /path/to/openwith.rs with the actual path and copy
	sed "s|/path/to/openwith.rs|$exec_path|g" "$desktop_file" > "$dest_file"
	
	# Make sure the installed file has correct permissions
	chmod 644 "$dest_file"
done

echo "Done! Installed ${#source_files[@]} desktop file(s)"
echo "Updating desktop database..."

# Update desktop database if the command exists
if command -v update-desktop-database &> /dev/null; then
	update-desktop-database "$target_dir" 2>/dev/null || true
	echo "Desktop database updated"
else
	echo "Note: update-desktop-database not found, you may need to run it manually"
fi
