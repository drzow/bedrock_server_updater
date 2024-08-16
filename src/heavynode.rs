use cloudflare_workers::kvstore;
use confy;
use reqwest::blocking::Client;
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::{net::TcpStream, io::{Write, Read}, path::PathBuf, fs::File};
use ssh2::Session;
use std::path::Path;

use webprocessing;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    server_name: String,
    server_port: i32,
    username: String,
    password: String,
    api_key: String,
    base_url: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { server_name: "Server.fqdn".into(), server_port: 1234, username: "User".into(), password: "pw".into(), api_key: "key".into(), base_url: "url".into() } }
}

pub fn main(stripped_zipfile: PathBuf) -> Result<(), String> {
    // Load configuration
    let myconfig: MyConfig = confy::load("bedrock_server_updater", None).expect("Error loading configuration");
    //confy::store("bedrock_server_updater", None, cfg);

    // Upload to HeavyNode account
    upload_zip_file(&myconfig, &stripped_zipfile);

    // Stop HeavyNode server
    stop_server(&myconfig);

    // Instruct HeavyNode to unzip it
    unzip_on_server(&myconfig, &stripped_zipfile);

    // Start HeavyNode server
    start_server(&myconfig);

    // Save version to KV store
    save_version(&myconfig, &stripped_zipfile);


    // Profit
    Ok(())
}

fn upload_zip_file(myconfig: &MyConfig, zip_file: &PathBuf) {
    let mut server_string = String::from(&myconfig.server_name);
    server_string.push(':');
    let server_port_string = String::from(myconfig.server_port.to_string());
    server_string.push_str(&server_port_string);
    println!("Attempting to connect to {}", &server_string);
    let tcp = TcpStream::connect(server_string).expect("Unable to connect to remote server");
    let mut session = Session::new().expect("Unable to open new ssh session");
    session.set_tcp_stream(tcp);
    session.handshake().expect("Unable to handshake ssh session");
    session.userauth_password(&myconfig.username, &myconfig.password).expect("Unable to authenticate as user with pass");
    // Connect to the ssh agent
    // let agent = session.agent().expect("Unable to connect to ssh agent");
    // Loop over all the identities in the agent until we find one that works
    // let mut success = false;
    // for identity in agent.identities().expect("Unable to get identities from ssh agent") {
    //     println!("Attempting to connect using {}", identity.comment());
    //     let auth_result = agent.userauth(&myconfig.username, &identity);
    //     if auth_result.is_err() { continue; }
    //     success = true;
    // }
    // If we did not find one that worked, PANIC!
    // assert!(success, "Unable to find an identity to connect to server");
    let sftp = session.sftp().expect("Unable to get sftp connection from ssh connection");
    let filename = zip_file.file_name().expect("Unable to get file name of zip file")
                                            .to_str().expect("Unable to convert zip file name to str");
    let mut file_contents = Vec::new();
    let mut source_file = File::open(zip_file).expect("Unable to open source zip file");
    source_file.read_to_end(&mut file_contents).expect("Unable to read contents of zip file");
    let mut remote_file = sftp.create(&Path::new(filename)).expect("Unable to create remote file");
    remote_file.write_all(&file_contents).expect("Unable to write contents to remote zip");
}

fn power_server(myconfig: &MyConfig, command: String) {
    let request_url = format!("{}/power", myconfig.base_url);
    println!("Sending {} request",command);
    let command_body = json!({"signal": command});
    let bearer_token = format!("Bearer {}", &myconfig.api_key);
    let response = Client::new()
        .post(request_url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer_token)
        .json(&command_body)
        .send()
        .expect("Error sending server power request");
    if ! response.status().is_success() {
        panic!("Error sending power request: {}", response.status().as_str());
    }
}

fn stop_server(myconfig: &MyConfig) {
    power_server(myconfig, String::from("stop"));
}

fn unzip_on_server(myconfig: &MyConfig, zip_file: &PathBuf) {
    let request_url = format!("{}/files/decompress", myconfig.base_url);
    println!("Sending decompress request");
    let command_body = json!({"root": "/", "file": zip_file.file_name()
        .expect("Unable to get filename from uploaded file path")
        .to_str().expect("Unable to convert uploaded filename to str")});
    let bearer_token = format!("Bearer {}", &myconfig.api_key);
    let response = Client::new()
        .post(request_url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("Authorization", bearer_token)
        .json(&command_body)
        .send()
        .expect("Error sending server decompress request");
    if ! response.status().is_success() {
        panic!("Error sending decompress request: {}", response.status().as_str());
    }
}

fn start_server(myconfig: &MyConfig) {
    power_server(myconfig, String::from("start"));
} 

fn save_version(myconfig: &MyConfig, zip_file: &PathBuf) {
    // Get version number from path
    let path_str: String = String::from(zip_file.as_os_str());
    let version: String = webprocessing::get_version_from_string(path_str);
}