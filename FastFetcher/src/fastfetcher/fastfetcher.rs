
use crate::fastfetcher::types::DownloadResult;
use reqwest::{self, Error};
use std::env;
pub fn command_line_input() -> Vec<String> {
    let command_line_input: Vec<String> = env::args().skip(1).collect();

    println!("{:?}", command_line_input);
    command_line_input
}

pub async fn save_file(content_to_save: DownloadResult) -> Result<(), Error> {
    let _ = tokio::fs::write(content_to_save.filename, content_to_save.content.as_bytes()).await;

    Ok(())
}
pub async fn download_file(url: String) -> Result<DownloadResult, reqwest::Error> {
    let filename = url.split('/').last().unwrap_or("dimka.txt").to_string();
    let get_file_content = reqwest::get(url).await?.text().await?;
    println!("{:?}", get_file_content);
    let file_data = DownloadResult::new(filename, get_file_content);

    Ok(file_data)

}
