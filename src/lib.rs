use std::{fs, str};

/// Returns all entries from all files in current directory that were parsable as a library.
pub fn scored_entries(args: &[String], in_dir: &str) -> Vec<String> {
    let files_here: Vec<String> = fs::read_dir(in_dir)
        .expect("Unable to open current directory!")
        .filter_map(|entry| entry.ok()?.path().to_str().map(String::from))
        .collect();

    let mut entries: Vec<(usize, String)> = files_here
        .iter()
        .filter_map(|file| entries_from(file))
        .flatten()
        .filter_map(|entry| {
            let tags = entry.lines().next().unwrap();
            let score = score_entry(args, tags);
            (score > 0).then_some((score, entry))
        })
        .collect();

    // Sort entries by score.
    entries.sort_by_key(|pair| pair.0);
    entries.into_iter().map(|(_, entry)| entry).rev().collect()
}

/// Creates list of entries from file.
/// Failure to read file, or then to interpret as utf8, returns `None`.
fn entries_from(file: &str) -> Option<Vec<String>> {
    let Ok(read_file) = fs::read(file) else {
        return None;
    };

    let Ok(file_str) = str::from_utf8(&read_file) else {
        return None;
    };

    let lines: Vec<&str> = file_str.lines().collect();
    let header_positions: Vec<usize> = lines
        .iter()
        .zip(lines.iter().skip(1))
        .map(|(prev, current)| (prev.trim(), current.trim()))
        .enumerate()
        .filter_map(|(i, (prev, current))| (is_tags(prev) && is_sources(current)).then_some(i))
        .collect();

    let mut ahead = header_positions.iter().skip(1);
    Some(
        header_positions
            .iter()
            .map(|index| {
                if let Some(end) = ahead.next() {
                    lines[*index..*end].join("\n")
                } else {
                    lines[*index..].join("\n")
                }
            })
            .collect(),
    )
}

fn is_tags(input: &str) -> bool {
    input.starts_with('[') && input.ends_with(']')
}

fn is_sources(input: &str) -> bool {
    input.starts_with('{') && input.ends_with('}')
}

/// Removes duplications in `args` and `tags`, then returns
/// how many args in args matches any of the tags in tags.
fn score_entry(args: &[String], tags: &str) -> usize {
    let mut tags: Vec<String> = tags
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split_ascii_whitespace()
        .map(|tag| tag.to_ascii_lowercase())
        .collect();
    tags.sort();
    tags.dedup();

    let mut args: Vec<String> = args.iter().map(|tag| tag.to_ascii_lowercase()).collect();
    args.sort();
    args.dedup();

    args.iter()
        .fold(0, |score, arg| score + (tags.contains(arg) as usize))
}

#[test]
fn test_scoring() {
    let tags = vec!["Github", "How", "To", "Push", "Changes"];
    let args: Vec<String> = tags.iter().map(|st| st.to_string()).collect();
    for entry in scored_entries(&args, "D:\\Documents\\Study") {
        println!("{entry}");
    }
}
