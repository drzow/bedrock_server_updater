extern crate regex;
use regex::Regex;
extern crate scraper;
use scraper::{Html, Selector};
extern crate ureq;
use ureq::Agent;
use std::{fs::{self}, process::exit, path::PathBuf};

const SERVER_URL: &'static str = "https://www.minecraft.net/en-us/download/server/bedrock";

pub fn main(version_path: &str) -> Result<PathBuf, String> {
    // Check Bedrock Server Page
    let http_agent = get_agent();
    let server_page = check_bedrock_server_page(http_agent, SERVER_URL);

    // Find current Bedrock Server version
    let download_link = get_download_link(server_page);
    let server_version = get_server_version(&download_link);

    // If it is equal to or less than the latest one we have, exit
    let current_version = get_current_version(version_path);
    let new_version = is_newer_version(&current_version, &server_version);
    if ! new_version {
        println!("Server version {} is unchanged.", server_version);
        exit(0);
    }

    let path_out = download_server_package(download_link, server_version);
    Ok(path_out)
}

fn get_agent() -> Agent {
    ureq::AgentBuilder::new().build()
}

fn check_bedrock_server_page(http_agent: Agent, path: &str) -> Html {
    let body: String = http_agent.get(path)
      .call().unwrap()
      .into_string().unwrap();
    Html::parse_document(&body)
}

fn get_download_link(server_page: Html) -> String {
    let selector = Selector::parse(r#"a[aria-label="Download Minecraft Dedicated Server software for Ubuntu (Linux)"]"#).unwrap();
    let download_link = server_page.select(&selector).next().unwrap().value().attr("href").unwrap();
    download_link.to_string()
}

fn get_server_version(download_link: &String) -> String {
    get_version_from_string(download_link)
}

fn get_version_from_string(link_or_path: &String) -> String {
    let version_regex = Regex::new(r"bedrock-server-(\d+\.\d+\.\d+)[\.-]").unwrap();
    let version_cap = version_regex.captures(&link_or_path).unwrap();
    version_cap.get(1).unwrap().as_str().to_string()
}

fn get_current_version(version_path: &str) -> String {
    // If the file does not exist, return "1.0.0"
    // If it does exist, open it, read the contents, and return them
    let version_file_result = fs::read_to_string(version_path);
    let version_result = match version_file_result {
        Ok(version) => version,
        Err(_error) => "1.0.0".to_string(),
    };
    version_result
}

fn is_newer_version(current_version: &String, new_version: &String) -> bool {
    // Break down each version into major.minor.patch
    let version_regex: Regex = Regex::new(r"(\d+)\.(\d+)\.(\d+)").unwrap();
    let current_cap = version_regex.captures(&current_version).unwrap();
    let current_major: i32 = current_cap.get(1).unwrap().as_str().to_string().parse::<i32>().unwrap();
    let current_minor: i32 = current_cap.get(2).unwrap().as_str().to_string().parse::<i32>().unwrap();
    let current_patch: i32 = current_cap.get(3).unwrap().as_str().to_string().parse::<i32>().unwrap();
    let new_cap = version_regex.captures(&new_version).unwrap();
    let new_major: i32 = new_cap.get(1).unwrap().as_str().to_string().parse::<i32>().unwrap();
    let new_minor: i32 = new_cap.get(2).unwrap().as_str().to_string().parse::<i32>().unwrap();
    let new_patch: i32 = new_cap.get(3).unwrap().as_str().to_string().parse::<i32>().unwrap();
    
    // Then check each of major.minor.patch in order to see if any is greater for the new version
    // If so, then the new version is newer
    new_major > current_major || new_minor > current_minor || new_patch > current_patch
}

fn download_server_package(download_link: String, server_version: String) -> PathBuf {
    println!("Downloading new server version: {}", server_version);
    let resp = reqwest::blocking::get(download_link).expect("Request to download server failed");
    let body = resp.bytes().expect("Server download had invalid body");
    let file_name: String = String::from("bedrock-server-") + &server_version + &String::from(".zip");
    let path_out: PathBuf = PathBuf::from(&file_name);
    std::fs::write(&PathBuf::from(file_name), &body).expect("Error writing downloaded file");
    path_out
}


#[cfg(test)]
mod tests {
    use super::*;
    use web_server;

    #[test]
    fn test_check_bedrock_server_page_success() -> Result<(), String> {
        // Stand up server
        let page_contents: String = "<html><body><p>This is a simple webpage.</p></body></html>".to_string();
        let page_copy: String = page_contents.clone();
        web_server::new()
            .get("/", Box::new(|_request: web_server::Request, _response: web_server::Response|
                "<html><body><p>This is a simple webpage.</p></body></html>".to_string().into()))
            .launch(8012);
            //.unwrap();
        // Fetch page
        let test_page = check_bedrock_server_page(get_agent(), SERVER_URL);

        // Verify we got the right content
        assert_eq!(test_page, Html::parse_document(&page_copy));
        Ok(())
    }
}