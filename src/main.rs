mod heavynode;
mod zipfile;
mod webprocessing;

// This is a simple program that updates Bedrock Server on a HeavyNode instance
fn main() -> Result<(), String> {
    let result = webprocessing::main().unwrap();
    zipfile::main().unwrap();
    heavynode::main().unwrap();

    // Profit
    Ok(())
}



