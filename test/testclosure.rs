use std::env;
use std::io::{Write};
use std::fs::{self, DirEntry};
use std::path::Path;

use std::{path::{PathBuf}, fs::File};

use std::io::Read;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let target_file_string = &args[1];
    let mut target_file = PathBuf::new();
    target_file.push(target_file_string);
    let source_directory_string = &args[2];
    let mut source_directory = PathBuf::new();
    source_directory.push(source_directory_string);
    // Zip the package
    zip_package(&target_file, &source_directory);
}

fn zip_package(target_file: &PathBuf, source_directory: &PathBuf) {
    // Construct new zip path from previous zip path
    let zip_file = File::create(target_file.as_path()).expect("Unable to open zip file");
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let zip_options = zip::write::FileOptions::default().compression_level(Some(9));
    
    // Zip up the directory into the new zip file
    let mut add_to_zip = |file_path: &DirEntry| {
        zip_writer.start_file(file_path.path().as_os_str().to_str().expect("Unable to get filename to zip"), zip_options).expect("Unable to start adding file to zip");
        let mut file_contents = Vec::new();
        let mut source_file = File::open(file_path.path()).expect("Unable to open source file");
        source_file.read_to_end(&mut file_contents).expect("Unable to read contents of file to zip");
        zip_writer.write_all(&file_contents).expect("Unable to write file contents in zip");
        file_contents.clear();
    };
    visit_dirs(source_directory.as_path(), &mut add_to_zip);
}

// From The Book: https://doc.rust-lang.org/std/fs/fn.read_dir.html
fn visit_dirs<F: FnMut(&DirEntry) -> ()>(dir: &Path, fx: &mut F) {
    if dir.is_dir() {
        for entry in fs::read_dir(dir).expect("Unable to read directory") {
            let entry = entry.expect("Unable to convert directory entry");
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, fx);
            } else {
                fx(&entry);
            }
        }
    }
}