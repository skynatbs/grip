use clap::Parser;
use std::io::{self, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg(short, long)]
    pattern: String,

    /// The file to search in
    #[arg(short, long)]
    file: Option<String>,

    /// The folder to search in
    #[arg(short = 'd', long)]
    folder: Option<String>,

    /// Case insensitive search
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    insensitive: bool,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> io::Result<()> {
    let args = Args::parse();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    if let Some(file) = args.file {
        search_in_file(Path::new(&file), &args.pattern, args.insensitive, &mut stdout)?;
    } else {
        let folder = args.folder.as_deref().unwrap_or(".");
        search_in_folder(folder, &args.pattern, args.insensitive, &mut stdout)?;
    }

    Ok(())
}

fn search_in_file(file: &Path, pattern: &str, insensitive: bool, stdout: &mut StandardStream) -> io::Result<()> {
    let file_contents = match std::fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file.display(), e);
            return Ok(());
        }
    };

    let pattern = if insensitive { pattern.to_lowercase() } else { pattern.to_string() };

    for (line_number, line) in file_contents.lines().enumerate() {
        let line_lower = if insensitive { line.to_lowercase() } else { line.to_string() };

        if line_lower.contains(&pattern) {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
            write!(stdout, "{}", file.display())?;
            stdout.reset()?;
            writeln!(stdout, ":{}", line_number + 1)?;

            let mut start = 0;
            let mut line_with_highlights = String::new();
            while let Some(idx) = line_lower[start..].find(&pattern) {
                let idx = idx + start;
                line_with_highlights.push_str(&line[start..idx]);
                line_with_highlights.push_str("\x1b[31m");
                line_with_highlights.push_str(&line[idx..idx + pattern.len()]);
                line_with_highlights.push_str("\x1b[0m");
                start = idx + pattern.len();
            }
            line_with_highlights.push_str(&line[start..]);

            if let Err(e) = writeln!(stdout, "{}", line_with_highlights){
                eprintln!("Error while writing to stdout: {}", e);
            }
        }
    }
    Ok(())
}

fn search_in_folder(folder: &str, pattern: &str, insensitive: bool, stdout: &mut StandardStream) -> io::Result<()> {
    for entry in WalkDir::new(folder) {
        let entry = entry?;
        if entry.file_type().is_file() {
            search_in_file(entry.path(), pattern, insensitive, stdout)?;
        }
    }
    Ok(())
}

