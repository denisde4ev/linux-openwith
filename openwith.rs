#!/usr/bin/env rust-script

// AI generated

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::io::{self, BufRead};

fn main() {
	let program_name = env::args().next().unwrap_or_else(|| "openwith".to_string());
	
	// Check for --help flag
	if env::args().nth(1).as_deref() == Some("--help") {
		use std::io::Write;
		let stdout = std::io::stdout();
		let mut handle = stdout.lock();
		
		let result = writeln!(handle, "Usage: {} --<type> -- <URI|file>", program_name)
			.and_then(|_| writeln!(handle))
			.and_then(|_| writeln!(handle, "Arguments:"))
			.and_then(|_| writeln!(handle, "  --<type>     MIME type or protocol (e.g., http, https, text/plain)"))
			.and_then(|_| writeln!(handle, "  --           Separator between options and target"))
			.and_then(|_| writeln!(handle, "  <URI|file>   The file path or URI to open"))
			.and_then(|_| writeln!(handle))
			.and_then(|_| writeln!(handle, "Examples:"))
			.and_then(|_| writeln!(handle, "  {} --http -- https://google.com", program_name))
			.and_then(|_| writeln!(handle, "  {} --text/plain -- document.txt", program_name))
			.and_then(|_| writeln!(handle, "  {} --application/pdf -- document.pdf", program_name));
		
		if result.is_ok() {
			std::process::exit(0);
		} else {
			std::process::exit(1);
		}
	}
	
	// Parse command line arguments
	let (type_arg, target) = parse_arguments(&program_name);
	
	// Find applications that can handle this type
	let apps = find_applications(&type_arg);
	
	// Show selection dialog and get user choice
	let selected_app = show_selection_dialog(&apps, &target);
	
	// Launch the selected application
	execute_application(&selected_app, &target);
}

/// Parse command line arguments and return (type_arg, target)
fn parse_arguments(program_name: &str) -> (String, String) {
	let args: Vec<String> = env::args().collect();
	let mut type_arg = String::new();
	let mut target = String::new();
	let mut found_separator = false;

	// Parse arguments: ./openwith.rs --<type> -- <URI|file>
	for arg in &args[1..] {
		if arg == "--" {
			found_separator = true;
			continue;
		}
		if found_separator {
			target = arg.clone();
			break; // Only take the first argument after --
		} else if arg.starts_with("--") {
			type_arg = arg[2..].to_string();
		}
	}

	if type_arg.is_empty() || target.is_empty() {
		eprintln!("Usage: {} --<type> -- <URI|file>", program_name);
		std::process::exit(1);
	}

	(type_arg, target)
}

/// Find all applications that can handle the given type
fn find_applications(type_arg: &str) -> Vec<(String, String, String)> {
	let mime_type = if type_arg.contains('/') {
		type_arg.to_string()
	} else {
		format!("x-scheme-handler/{}", type_arg)
	};

	println!("Looking for applications for mime type: {}", mime_type);

	// Build list of directories to search
	let mut dirs = vec![
		Some(std::path::PathBuf::from("/usr/local/share/applications")),
		Some(std::path::PathBuf::from("/usr/share/applications")),
	];
	
	if let Ok(home) = env::var("HOME") {
		dirs.insert(0, Some(std::path::PathBuf::from(home).join(".local/share/applications")));
	}

	let mut apps: Vec<(String, String, String)> = Vec::new();
	let mut found_in_cache = false;

	// Try to find in mimeinfo.cache first
	for dir in dirs.iter().flatten() {
		let cache_path = dir.join("mimeinfo.cache");
		if cache_path.exists() {
			let cached_files = parse_mimeinfo_cache(&cache_path, &mime_type);
			for desktop_name in cached_files {
				found_in_cache = true;
				// Try to find the desktop file in any of the search directories
				if let Some(path) = find_desktop_file(&desktop_name, &dirs) {
					if let Some(app) = parse_desktop_file(&path, &mime_type) {
						apps.push(app);
					}
				}
			}
		}
	}

	// If nothing found in cache, scan directories
	if !found_in_cache || apps.is_empty() {
		// Only print fallback message if we actually tried cache and failed to get results
		if found_in_cache {
			println!("Cache entries found but failed to load applications, falling back to full scan...");
		}
		
		for dir in dirs.into_iter().flatten() {
			if dir.exists() {
				visit_dirs(&dir, &mime_type, &mut apps);
			}
		}
	}

	if apps.is_empty() {
		eprintln!("No applications found for {}", mime_type);
		std::process::exit(1);
	}

	// Sort and deduplicate
	apps.sort_by(|a, b| a.0.cmp(&b.0));
	apps.dedup_by(|a, b| a.0 == b.0);

	// Prepend Editor options
	if let Ok(editor) = env::var("EDITOR") {
		if !editor.is_empty() {
			// Determine terminal emulator
			let terminal = env::var("TERMINAL").unwrap_or_else(|_| "x-terminal-emulator".to_string());
			
			// note gnome-terminal uses `--` and not `-e`, but I dont use it!
			// or make your own wrapper
			// DONT: let term_flag = "--";
			let term_flag = "-e";

			// Option 1: EDITOR=<editor>
			// Command: $TERMINAL -e $EDITOR -- %f
			// We use shell_escape for paths to handle spaces/quotes
			let editor_cmd = format!("{} {} {} -- %f", shell_escape(&terminal), term_flag, shell_escape(&editor));
			apps.insert(0, (format!("$ EDITOR={}", editor), editor_cmd, "custom".to_string()));

			// Option 2: echo pipe edit
			// Command: $TERMINAL -e sh -c 'printf "%s\n" "$1" | $EDITOR' -- %f
			// We pass %f as an argument to sh -c to avoid quoting hell.
			// The command string inside sh -c needs $EDITOR to be properly escaped/quoted.
			let pipe_cmd = format!(
				"{} {} sh -c 'printf \"%s\\n\" \"$1\" | {}' -- %f", 
				shell_escape(&terminal), 
				term_flag, 
				shell_escape(&editor)
			);
			apps.insert(1, ("$ echo $@ | $EDITOR".to_string(), pipe_cmd, "custom".to_string()));
		}
	}

	// Clipboard option
	let clipboard_cmd = if env::var("WAYLAND_DISPLAY").is_ok() && check_command_exists("wl-copy") {
		Some("wl-copy -n")
	} else if env::var("DISPLAY").is_ok() && check_command_exists("xclip") {
		Some("xclip -sel clip -r")
	} else if check_command_exists("termux-clipboard-set") {
		Some("termux-clipboard-set")
	} else {
		None
	};

	if let Some(cmd) = clipboard_cmd {
		// Command: sh -c 'printf "%s" "$1" | <clipboard_cmd>' -- %f
		// We use printf "%s" (no newline) as usually desired for clipboard
		let full_cmd = format!("sh -c 'printf \"%s\" \"$1\" | {}' -- %f", cmd);
		apps.insert(0, (format!("$ {}", cmd), full_cmd, "custom".to_string()));
	}

	apps
}

fn check_command_exists(cmd: &str) -> bool {
	return
		Command::new("sh")
		.arg("-c")
		.arg(format!("command -v {}", cmd))
		.output()
		.map(|o| o.status.success())
		.unwrap_or(false)
	;
}

/// Show kdialog selection menu and return the selected application
fn show_selection_dialog(apps: &[(String, String, String)], target: &str) -> (String, String, String) {
	let mut kdialog_args = vec![
		"--menu".to_string(),
		format!("Select application for {}", target),
	];

	for (i, app) in apps.iter().enumerate() {
		kdialog_args.push(i.to_string());
		kdialog_args.push(app.0.clone());
	}

	let output = Command::new("kdialog")
		.args(&kdialog_args)
		.output()
		.unwrap_or_else(|e| {
			eprintln!("Failed to run kdialog: {}", e);
			std::process::exit(127);
		});

	if !output.status.success() {
		eprintln!("Selection cancelled or failed.");
		std::process::exit(1);
	}

	let selection = String::from_utf8_lossy(&output.stdout).trim().to_string();
	let index = selection.parse::<usize>().unwrap_or_else(|_| {
		eprintln!("Invalid selection");
		std::process::exit(1);
	});

	apps.get(index).cloned().unwrap_or_else(|| {
		eprintln!("Invalid selection index");
		std::process::exit(1);
	})
}

/// Execute the selected application with the target
fn execute_application(app: &(String, String, String), target: &str) {
	let exec_template = &app.1;
	launch_app(exec_template, target);
}

fn visit_dirs(dir: &Path, mime_type: &str, apps: &mut Vec<(String, String, String)>) {
	if let Ok(entries) = fs::read_dir(dir) {
		for entry in entries.flatten() {
			let path = entry.path();
			if path.is_dir() {
				visit_dirs(&path, mime_type, apps);
			} else if let Some(ext) = path.extension() {
				if ext == "desktop" {
					if let Some(app) = parse_desktop_file(&path, mime_type) {
						apps.push(app);
					}
				}
			}
		}
	}
}

fn parse_desktop_file(path: &Path, target_mime: &str) -> Option<(String, String, String)> {
	let file = fs::File::open(path).ok()?;
	let reader = io::BufReader::new(file);

	let mut name = String::new();
	let mut exec = String::new();
	let mut mime_types = Vec::new();
	let mut no_display = false;
	let mut is_desktop_entry = false;

	for line in reader.lines().flatten() {
		let line = line.trim();
		if line == "[Desktop Entry]" {
			is_desktop_entry = true;
			continue;
		}
		if line.starts_with('[') && line != "[Desktop Entry]" {
			is_desktop_entry = false; // Moved to another section
		}

		if !is_desktop_entry {
			continue;
		}

		if line.starts_with("Name=") {
			name = line[5..].to_string();
		} else if line.starts_with("Exec=") {
			exec = line[5..].to_string();
		} else if line.starts_with("MimeType=") {
			let mimes = line[9..].trim_end_matches(';');
			mime_types = mimes.split(';').map(|s| s.to_string()).collect();
		} else if line.starts_with("NoDisplay=true") {
			no_display = true;
		}
	}

	if no_display {
		return None;
	}

	if mime_types.iter().any(|m| m == target_mime) {
		if !name.is_empty() && !exec.is_empty() {
			return Some((name, exec, path.to_string_lossy().to_string()));
		}
	}

	None
}

fn launch_app(exec_template: &str, target: &str) {
	// Escape target for use inside double quotes in shell
	let target_quoted = double_quote_escape(target);
	
	let mut command_line = exec_template.to_string();
	
	command_line = command_line.replace("%f", &target_quoted);
	command_line = command_line.replace("%F", &target_quoted);
	command_line = command_line.replace("%u", &target_quoted);
	command_line = command_line.replace("%U", &target_quoted);
	
	if !exec_template.contains("%f") && !exec_template.contains("%F") && 
	   !exec_template.contains("%u") && !exec_template.contains("%U") {
		command_line.push_str(" ");
		command_line.push_str(&target_quoted);
	}

	println!("Executing: {}", command_line);
	
	let status = Command::new("sh")
		.arg("-c")
		.arg(command_line)
		.status();
		
	match status {
		Ok(s) => {
			if !s.success() {
				eprintln!("Application exited with error.");
			}
		},
		Err(e) => eprintln!("Failed to execute: {}", e),
	}
}

fn parse_mimeinfo_cache(path: &Path, target_mime: &str) -> Vec<String> {
	let file = match fs::File::open(path) {
		Ok(f) => f,
		Err(_) => return Vec::new(),
	};
	let reader = io::BufReader::new(file);
	let mut desktop_files = Vec::new();

	for line in reader.lines().flatten() {
		let line = line.trim();
		// Format is mime/type=file1.desktop;file2.desktop;
		if line.starts_with(target_mime) {
			if let Some(rest) = line.strip_prefix(target_mime) {
				if rest.starts_with('=') {
					let files_part = &rest[1..];
					for f in files_part.split(';') {
						let f = f.trim();
						if !f.is_empty() {
							desktop_files.push(f.to_string());
						}
					}
				}
			}
		}
	}
	desktop_files
}

fn find_desktop_file(name: &str, dirs: &[Option<std::path::PathBuf>]) -> Option<std::path::PathBuf> {
	for dir in dirs.iter().flatten() {
		let path = dir.join(name);
		if path.exists() {
			return Some(path);
		}
	}
	None
}

/// Escape string for use in shell (single quotes)
fn shell_escape(s: &str) -> String {
	format!("'{}'", s.replace("'", "'\\''"))
}

/// Escape string for use inside double quotes in shell
fn double_quote_escape(s: &str) -> String {
	let escaped = s.replace('\\', "\\\\")
		.replace('"', "\\\"")
		.replace('$', "\\$")
		.replace('`', "\\`");
	format!("\"{}\"", escaped)
}