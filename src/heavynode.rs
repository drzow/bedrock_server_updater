use confy;
use serde::{Serialize, Deserialize};
use std::{net::TcpStream, io::{Write, Read}, path::PathBuf, fs::File};
use ssh2::{Session};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct MyConfig {
    server_name: String,
    server_port: i32,
    username: String,
    password: String,
    api_key: String,
}

impl ::std::default::Default for MyConfig {
    fn default() -> Self { Self { server_name: "Server.fqdn".into(), server_port: 1234, username: "User".into(), password: "pw".into(), api_key: "key".into() } }
}

pub fn main(stripped_zipfile: PathBuf) -> Result<(), String> {
    // Load configuration
    let myconfig: MyConfig = confy::load("bedrock_server_updater", None).expect("Error loading configuration");
    //confy::store("bedrock_server_updater", None, cfg);

    // Upload to HeavyNode account
    upload_zip_file(&myconfig, &stripped_zipfile);
/*
    // Stop HeavyNode server
    stop_server();

    // Instruct HeavyNode to unzip it
    unzip_on_server(zip_file);

    // Start HeavyNode server
    start_server();
*/
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
    //session.userauth_agent(&myconfig.username).expect("Unable to authenticate as user");
    session.userauth_password(&myconfig.username, &myconfig.password).expect("Unable to authenticate as user with pass");
    let sftp = session.sftp().expect("Unable to get sftp connection from ssh connection");
    let filename = zip_file.file_name().expect("Unable to get file name of zip file")
                                             .to_str().expect("Unable to convert zip file name to str");
    let mut file_contents = Vec::new();
    let mut source_file = File::open(zip_file).expect("Unable to open source zip file");
    source_file.read_to_end(&mut file_contents).expect("Unable to read contents of zip file");
    let mut remote_file = sftp.create(&Path::new(filename)).expect("Unable to create remote file");
    remote_file.write_all(&file_contents).expect("Unable to write contents to remote zip");
}
/*
fn stop_server() {
    // TODO
}

fn unzip_on_server(zip_file) {
    // TODO
}

fn start_server() {
    // TODO
} */