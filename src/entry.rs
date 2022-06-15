use std::fs::{read, self};

#[derive(Clone)]
pub struct Entry {
    pub score: u32,
    pub tags: Vec<u8>,
    pub sources: String,
    pub description: Vec<String>,
}

impl Entry {
    pub fn new() -> Entry {
        let empty = String::from("");

        Entry {
            score: 0,
            tags: vec![],
            sources: empty.clone(),
            description: vec![empty],
        }
    }
}

impl std::fmt::Display for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = write!(
            f,
            "{}\n{}",
            String::from_utf8_lossy(&self.tags),
            self.sources
        );
        for each in &self.description {
            println!("{}", each);
        }
        output
    }
}

pub fn get_entries(file: &str) -> Vec<Entry> {
    let input = lines(file);
    let mut output = Vec::new();

    for i in 0.. {
        if let Some(lines) = get_entry((i + 1) as i32, &input) {
            output.push(Entry::new());

            for line in lines {
                let current_line: String = String::from_utf8_lossy(&line).to_string();

                if is_tags(&line) {
                    output[i].tags = line;
                } else if is_sources(&line) {
                    output[i].sources = current_line;
                } else {
                    output[i].description.push(current_line);
                }
            }
        } else {
            break;
        }
    }

    output
}

/// Sorts input entries by count of how many tags the tag line contains.
/// Higher scoring entries are near the beginning!
pub fn sort_by_tags(input: Vec<Entry>, args: &Vec<String>) -> Vec<Entry> {
    let mut entries = input.clone();
    // Remove duplicate args so they can each only count once.
    let mut de_dup_args = args.clone();
    de_dup_args.sort();
    de_dup_args.dedup();

    for entry in &mut entries {
        // For all entries
        let mut str_chars = String::from_utf8_lossy(&entry.tags).to_lowercase();
        str_chars.pop(); // Remove newline at the end of the string.
        str_chars.pop(); // Remove ] at the end.
        str_chars.remove(0); // Remove [ at the start.

        // Remove duplicates from tags as well as input args.
        let mut tags: Vec<&str> = str_chars.split_whitespace().collect();
        tags.sort();
        tags.dedup();

        for arg in &de_dup_args {
            // Compare all args against
            for tag in &tags {
                // each tag in current entry.
                if *arg == *tag {
                    entry.score += 1;
                }
            }
        }
    }

    entries.sort_by_key(|item| item.score);
    entries.reverse();

    entries
}

pub fn get_library_file_names() -> Vec<String> {
    fs::read_dir("./")
        .unwrap()
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().into())
        .filter(|file_name: &String| is_library(file_name))
        .collect()
}

// Check if file contains even one tag line, with sources line afterwards.
pub fn is_library(file: &str) -> bool {
    if !file.contains(".txt") {
        return false;
    }
    
    let input = lines(file);

    for i in 1..input.len() {
        if is_tags(&input[i - 1]) && is_sources(&input[i]) {
            return true;
        }
    }

    false
}

/// Returns a Vec of Vec<u8> with each Vec<u8> being each line from the file.
fn lines(file: &str) -> Vec<Vec<u8>> {
    let mut input = read(file).expect("File not found");
    
    

    let (mut len, mut output, ln) = (input.len(), Vec::new(), '\n' as u8);

    // If file is empty return empty lines.
    if len == 0 {
        return vec![vec![]];
    }

    if input[len - 1] != ln {
        input.push(ln);
        len += 1;
    }

    let mut start = 0;
    for i in start..len {
        if input[i] == ln {
            let addition = input[start..i].to_vec();
            output.push(addition);

            start = i + 1;
        }
    }

    output
}

// Get entry from lines from guaranteed library file.
// Freezes indefinitely if ran on non-library file.
fn get_entry(target_entry: i32, lines: &Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>> {
    let mut output = Vec::new();
    if target_entry <= 0 {
        return None;
    }

    let (mut entry_index, mut entry_count) = (0, 0);
    while entry_count < target_entry {
        if entry_index >= lines.len() {
            return None;
        }

        if is_tags(&lines[entry_index]) {
            entry_count += 1;
        }

        entry_index += 1;
    }

    // The previous loop left entry_index ahead of the tags.
    entry_index -= 1;
    for i in entry_index..lines.len() {
        output.push(lines[i].clone());

        if i + 1 >= lines.len() || is_tags(&lines[i + 1]) {
            break;
        }
    }

    Some(output)
}

// Check the beginning and ending character to tell if something is tags or sources.
// We use len - 2 in both below because len - 1 is the newline at the end of the line.
fn is_tags(str: &Vec<u8>) -> bool {
    str[0] == '[' as u8 && str[str.len() - 2] == ']' as u8
}

fn is_sources(str: &Vec<u8>) -> bool {
    str[0] == '{' as u8 && str[str.len() - 2] == '}' as u8
}
