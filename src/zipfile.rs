extern crate tempdir;
use regex::Regex;
use tempdir::TempDir;

use std::{path::{Path, PathBuf}, fs::{self, DirEntry, File}, io::{Write, Read}};

pub fn main(server_download: PathBuf) -> Result<PathBuf, String> {

    // Unzip the package
    let unzip_directory = unzip_package_file(&server_download);

    // Remove configuration files
    remove_conf_files(&unzip_directory);

    // Zip the package
    let zip_file = zip_package(&server_download, &unzip_directory);

    // Profit
    Ok(zip_file)
}

fn unzip_package_file(server_download: &PathBuf) -> TempDir {
    // Create a temporary directory
    let tmp_dir = TempDir::new("bedrock-server").expect("Unable to create temporary directory");
    println!("Unzipping to {}", tmp_dir.path().as_os_str().to_str().expect("Unable to get str from temp path"));
    // Unzip the contents there
    let orig_file = File::open(server_download).expect("Unable to open downloaded file");
    let mut orig_zip = zip::ZipArchive::new(orig_file).expect("Unable to recognize file as zip archive");
    orig_zip.extract(&tmp_dir).expect("Unable to extract zip archive");
    // Return the path to the temporary directory
    tmp_dir
}

fn remove_conf_files(unzip_directory: &TempDir) {
    // We do not want to include allowlist.json, config/default/permissions.json,
    // permissions.json, and server.properties
    const TO_DELETE: &'static [&'static str] = &["allowlist.json", "config/default/permissions.json", "permissions.json", "server.properties"];
    for file_to_delete in TO_DELETE.iter() {
        let mut file_path = PathBuf::new();
        file_path.push(unzip_directory.path());
        file_path.push(file_to_delete);
        std::fs::remove_file(file_path).expect("Error removing file");
    }
}

fn zip_package(server_download: &PathBuf, unzip_directory: &TempDir) -> PathBuf {
    // Construct new zip path from previous zip path
    let download_filename = server_download.file_name().expect("Unable to get filename of downloaded file");
    let download_filename_str = download_filename.to_str().expect("Unable to convert filename to string");
    let download_filename_string = String::from(download_filename_str);
    let filename_regex = Regex::new(r"(bedrock-server-\d+\.\d+\.\d+)\.zip").unwrap();
    let filename_cap = filename_regex.captures(&download_filename_string).unwrap();
    let filename_base = filename_cap.get(1).unwrap().as_str().to_string();
    let mut stripped_filename = String::from(filename_base);
    stripped_filename.push_str("-stripped.zip");
    // let mut stripped_path = PathBuf::from(server_download.parent().expect("Unable to get directory of downloaded file"));
    // stripped_path.push(stripped_filename);
    let stripped_path = PathBuf::from(stripped_filename);
    let mut source_dir_pathbuf = PathBuf::new();
    source_dir_pathbuf.push(unzip_directory.path());
    zip_command(&stripped_path, &source_dir_pathbuf);
    stripped_path
}

fn zip_command(target_file: &PathBuf, source_directory: &PathBuf) {
    // Construct new zip path from previous zip path
    let zip_file = File::create(target_file.as_path()).expect("Unable to open zip file");
    let mut zip_writer = zip::ZipWriter::new(zip_file);
    let zip_options = zip::write::FileOptions::default().compression_level(Some(9));
    
    // Zip up the directory into the new zip file
    let mut add_to_zip = |file_path: &DirEntry| {
        let full_file_path_string = String::from(file_path.path().as_os_str().to_str().expect("Unable to get filename to zip"));
        let relative_file_path_str_slash = full_file_path_string.strip_prefix(source_directory.to_str().expect("Unable to get path of temp dir as string")).expect("Unable to strip prefix");
        let relative_file_path_str = relative_file_path_str_slash.strip_prefix("/").expect("Unable to strip slash prefix");
        zip_writer.start_file(relative_file_path_str, zip_options).expect("Unable to start adding file to zip");
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