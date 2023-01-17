use entry::scored_entries;
use std::env;
mod entry;

fn main() {
    // First args is target\debug\library_reader.&exe when cargo run the project.
    // Otherwise args is just what is inputted as expected.
    let args: Vec<String> = env::args().map(|item| item.to_lowercase()).collect();

    // // User provided no tags to search for
    // // For some reason the first arg is always the executable.
    if args.len() == 1 {
        println!("Usage: sf Tag1 Tag2 Tag3....");
        println!("Current folder should have file with tag line(s) \"[tag1 tag2 tag3]\" with sources line next \"{{source1 source2 source3}}\"");
        return;
    }

    // Print out those with score higher than zero.
    for entry in scored_entries(&args, "./") {
        println!("{entry}");
    }
}
