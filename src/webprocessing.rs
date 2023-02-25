extern crate regex;
use regex::Regex;
extern crate scraper;
use scraper::{Html, Selector};
extern crate ureq;
use ureq::{Agent, AgentBuilder};

const SERVER_URL: &'static str = "https://www.minecraft.net/en-us/download/server/bedrock";

pub fn main() -> Result<(), String> {
    // Check Bedrock Server Page
    let http_agent = get_agent();
    let server_page = check_bedrock_server_page(http_agent, SERVER_URL);

    // Find current Bedrock Server version
    let download_link = get_download_link(server_page);/*
    let server_version = get_server_version(download_link);

    // If it is equal to or less than the latest one we have, exit
    let current_version = get_current_version();
    let new_version = is_newer_version(current_version, new_version);
    if (! new_version) {
        exit Ok(());
    }

    // Download the server
    let package_file = download_server_package(server_page); */
    Ok(())
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
/*
fn get_server_version(server_page: Html) -> String {
    let selector = Selector::parse(r#"a[aria-label="Download Minecraft Dedicated Server software for Ubuntu (Linux)"]"#).unwrap();
    let download_link = server_page.select(&selector).next().unwrap().value().attr("href");
    let version_regex = Regex::new(r"bedrock-server-(\d+\.\d+\.\d+)\.").unwrap();
    let version_cap = version_regex.captures(download_link).unwrap();
    version_cap.get(1);
}

fn get_current_version() -> String {
    // TODO
    // If the file does not exist, return "1.0.0"
    // If it does exist, open it, read the contents, and return them
}

fn is_newer_version(current_version: String, new_version:String) -> bool {
    // TODO
}

fn download_server_package(server_page) -> FileRef {
    // TODO
}
*/

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
            .get("/", Box::new(|request: web_server::Request, response: web_server::Response|
                page_contents.into()))
            .launch(8012);
            //.unwrap();
        // Fetch page
        let test_page = check_bedrock_server_page(get_agent(), SERVER_URL);

        // Verify we got the right content
        assert_eq!(test_page, Html::parse_document(&page_copy));
        Ok(())
    }
}