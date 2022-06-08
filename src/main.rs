use std::env;
mod entry;

fn main() {
    // First args is target\debug\library_reader.exe when cargo run the project.
    // Otherwise args is just what is inputted as expected.
    let args: Vec<String> = env::args().map(|item| item.to_lowercase()).collect();

    // User provided no tags to search for
    // For some reason the first arg is always the executable.
    if args.len() == 1 {
        println!("Usage: sf Tag1 Tag2 Tag3....");
        println!("Current folder should have file with tag line(s) \"[tag1 tag2 tag3]\" and source line(s) next \"{{source1 source2 source3}}\"");

        return;
    }

    let libs = entry::get_library_file_names();
    if libs.len() == 0 {
        println!("No libraries found in current folder");
        return;
    }

    // Gather all .txt files that are libraries into the entries Vec.
    let entries: Vec<entry::Entry> = libs
        .into_iter()
        .flat_map(|name| entry::get_entries(&name))
        .collect();

    let sorted = entry::sort_by_tags(entries, &args);
    if sorted.iter().all(|item| item.score == 0) {
        print!("No matches for: ");
        for arg in env::args().skip(1) {
            print!("{arg}");
        }

        return;
    }

    for entry in sorted.iter().filter(|entry| entry.score > 0) {
        println!("{entry}");
    }
}
