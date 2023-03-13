mod heavynode;
mod zipfile;
mod webprocessing;

const VERSION_PATH: &'static str = "current_version.txt";

// This is a simple program that updates Bedrock Server on a HeavyNode instance
fn main() -> Result<(), String> {
    let server_download = webprocessing::main(VERSION_PATH).unwrap();
    let stripped_zipfile = zipfile::main(server_download).unwrap();
    heavynode::main(stripped_zipfile).expect("Error while updating heavynode");

    // Profit
    Ok(())
}



