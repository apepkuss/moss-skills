use clap::Parser;
use regex::Regex;
use std::fs;
use std::process::{self, Command};
use std::sync::LazyLock;

static VIDEO_ID_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:youtube\.com/watch\?.*v=|youtu\.be/)([\w-]{11})").unwrap()
});

static TIMESTAMP_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\d{2}:\d{2}:\d{2}\.\d{3}\s-->\s\d{2}:\d{2}:\d{2}\.\d{3}").unwrap()
});

static HTML_TAG_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"<[^>]+>").unwrap()
});

fn extract_video_id(url: &str) -> Option<&str> {
    VIDEO_ID_RE.captures(url).map(|caps| caps.get(1).unwrap().as_str())
}

fn clean_vtt(content: &str) -> String {
    let mut text_lines: Vec<String> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line == "WEBVTT" || line.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if TIMESTAMP_RE.is_match(line) {
            continue;
        }
        if line.starts_with("NOTE") || line.starts_with("STYLE") {
            continue;
        }
        if line.starts_with("Kind:") || line.starts_with("Language:") {
            continue;
        }

        let line = HTML_TAG_RE.replace_all(line, "").to_string();

        if text_lines.last().map_or(false, |last| last == &line) {
            continue;
        }

        text_lines.push(line);
    }

    text_lines.join("\n")
}

#[derive(Parser)]
#[command(name = "moss-youtube-captions")]
#[command(about = "Fetch YouTube video transcript as plain text")]
struct Cli {
    /// YouTube video URL
    url: String,
}

fn main() {
    let cli = Cli::parse();

    // Validate URL
    if extract_video_id(&cli.url).is_none() {
        eprintln!("Error: could not extract video ID from URL");
        process::exit(1);
    }

    let temp_dir = match tempfile::tempdir() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error creating temp directory: {e}");
            process::exit(1);
        }
    };

    let output = Command::new("yt-dlp")
        .args([
            "--write-subs",
            "--write-auto-subs",
            "--skip-download",
            "--sub-lang", "en",
            "--output", "subs",
            &cli.url,
        ])
        .current_dir(temp_dir.path())
        .output();

    match output {
        Ok(result) if !result.status.success() => {
            eprintln!(
                "Error running yt-dlp: {}",
                String::from_utf8_lossy(&result.stderr)
            );
            process::exit(1);
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            eprintln!("Error: yt-dlp not found. Please install it.");
            process::exit(1);
        }
        Err(e) => {
            eprintln!("Error running yt-dlp: {e}");
            process::exit(1);
        }
        Ok(_) => {}
    }

    // Find .vtt file
    let vtt_path = match fs::read_dir(temp_dir.path())
        .ok()
        .and_then(|entries| {
            entries
                .filter_map(|e| e.ok())
                .find(|e| e.path().extension().map_or(false, |ext| ext == "vtt"))
                .map(|e| e.path())
        })
    {
        Some(path) => path,
        None => {
            eprintln!("No subtitles found.");
            process::exit(1);
        }
    };

    let content = match fs::read_to_string(&vtt_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading subtitle file: {e}");
            process::exit(1);
        }
    };

    let clean_text = clean_vtt(&content);
    println!("{clean_text}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_standard_url() {
        assert_eq!(
            extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ")
        );
    }

    #[test]
    fn test_extract_short_url() {
        assert_eq!(
            extract_video_id("https://youtu.be/dQw4w9WgXcQ"),
            Some("dQw4w9WgXcQ")
        );
    }

    #[test]
    fn test_extract_with_extra_params() {
        assert_eq!(
            extract_video_id("https://www.youtube.com/watch?v=dQw4w9WgXcQ&t=120"),
            Some("dQw4w9WgXcQ")
        );
    }

    #[test]
    fn test_extract_invalid_url() {
        assert_eq!(extract_video_id("https://example.com"), None);
    }

    #[test]
    fn test_clean_vtt_removes_header_and_timestamps() {
        let vtt = "WEBVTT\n\n1\n00:00:01.000 --> 00:00:04.000\nHello world\n\n2\n00:00:04.000 --> 00:00:08.000\nThis is a test";
        let result = clean_vtt(vtt);
        assert_eq!(result, "Hello world\nThis is a test");
    }

    #[test]
    fn test_clean_vtt_removes_html_tags() {
        let vtt = "WEBVTT\n\n00:00:01.000 --> 00:00:04.000\n<b>Bold text</b> and <i>italic</i>";
        let result = clean_vtt(vtt);
        assert_eq!(result, "Bold text and italic");
    }

    #[test]
    fn test_clean_vtt_deduplicates_consecutive_lines() {
        let vtt = "WEBVTT\n\n00:00:01.000 --> 00:00:02.000\nSame line\n\n00:00:02.000 --> 00:00:03.000\nSame line\n\n00:00:03.000 --> 00:00:04.000\nDifferent line";
        let result = clean_vtt(vtt);
        assert_eq!(result, "Same line\nDifferent line");
    }
}
