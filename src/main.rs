use std::env;
use std::fs;
mod entry;

fn get_libraries() -> Vec<String> {
    fs::read_dir("./")
        .unwrap()
        .into_iter()
        .map(|entry| entry.unwrap().path().to_str().unwrap().into())
        .filter(|the: &String| the.contains(" Library.txt"))
        .collect()
}

/// File format of a library below:
/// [tags, tag1, tag tag tag another_tag more tags The entire string will show up who cares how its separated]
/// { source links separated by whatever you want I guess it shows the whole string lol }
/// Main content--plain text--with potentially indefinite number of newlines.
/// An entry only "ends" because end of file or another entry began.
/// The syntax is very forgiving just have the right starting and ending characters.
fn main() {
    // First args is target\debug\library_reader.exe when cargo run the project.
    // Otherwise args is just what is inputted as expected.
    let args: Vec<String> = env::args().map(|item| item.to_lowercase()).collect();

    // User provided no tags to search for
    // For some reason the first arg is always the executable.
    if args.len() == 1 {
        println!("Usage: sf Tag1 Tag2 Tag3....");
        println!("Current folder should contain file(s) named similar to \"* Library.txt\"");

        return;
    }

    let libs = get_libraries();
    if libs.len() == 0 {
        println!("No libraries found in current folder");

        return;
    }

    // Gather all "* Library.txt" files into the entries Vec.
    let mut entries: Vec<entry::Entry> = Vec::new();
    for each in libs {
        entries.extend(entry::get_entries(&each));
    }

    let sorted = entry::sort_by_tags(entries, &args);
    if sorted.iter().all(|item| item.score == 0) {
        print!("No matches for: ");
        for arg in env::args().skip(1) {
            print!("{arg}");
        }

        println!("\nIs the correct library in this folder?");
        
        return;
    }
    
    for entry in sorted.iter().filter(|entry| entry.score > 0) {
        println!("{entry}");
    }
}
