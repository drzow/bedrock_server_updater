use std::env;
mod heavynode;
mod zipfile;
mod webprocessing;

const VERSION_PATH: &'static str = "current_version.txt";

// This is a simple program that updates Bedrock Server on a HeavyNode instance
fn main() -> Result<(), String> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <download_url>", args[0]);
        std::process::exit(1);
    }
    let download_url = &args[1];

    // Pass the URL into webprocessing
    let server_download = webprocessing::main(VERSION_PATH, download_url).unwrap();
    let stripped_zipfile = zipfile::main(server_download).unwrap();
    heavynode::main(stripped_zipfile, VERSION_PATH).expect("Error while updating heavynode");

    Ok(())
}



