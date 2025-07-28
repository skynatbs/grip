use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg(short, long)]
    pattern: String, 

    /// The file to search in
    #[arg(short, long)]
    file: String,

    /// Case sensitive search
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    insensitive: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let file_path = Path::new(&args.file);
    let file = File::open(&file_path)?;
    let reader = io::BufReader::new(file);
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let line_contains_pattern = if args.insensitive {
            line.to_lowercase().contains(&args.pattern.to_lowercase())
        } else {
            line.contains(&args.pattern)
        };

        if line_contains_pattern {
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)))?;
            write!(&mut stdout, "{}", line_number + 1)?;
            stdout.reset()?;
            write!(&mut stdout, ": ")?;

            let pattern = if args.insensitive {
                args.pattern.to_lowercase()
            } else {
                args.pattern.clone()
            };
            let lower_line = line.to_lowercase();

            let mut start = 0;
            let mut line_with_highlights = String::new();
            while let Some(idx) = lower_line[start..].find(&pattern) {
                let idx = idx + start;
                line_with_highlights.push_str(&line[start..idx]);
                line_with_highlights.push_str("\x1b[31m"); // Red color for the pattern
                line_with_highlights.push_str(&line[idx..idx + args.pattern.len()]);
                line_with_highlights.push_str("\x1b[0m"); // Reset color
                start = idx + args.pattern.len();
            }
            line_with_highlights.push_str(&line[start..]);

            writeln!(&mut stdout, "{}", line_with_highlights)?;
        }
    }

    Ok(())
        //Simple another 'Main" wor'
}
