use std::{env, task::Context, thread::spawn};
use reqwest::{self, Error};
fn command_line_input() -> Vec<String>{

    let command_line_input: Vec<String> = env::args().skip(1).collect();

    println!("{:?}", command_line_input);
    command_line_input
}


async  fn save_file(file_name: String, content: String) -> Result<(), Error>{
let _ = tokio::fs::write(file_name, content.as_bytes()).await;

Ok(())
}
async fn download_file(url: String) -> Result<Vec<String>, reqwest::Error>{
    let filename = url.split('/').last().unwrap_or("dimka.txt").to_string();
    let get_file = reqwest::get(url)
                    .await?
                    .text()
                    .await?;
                println!("{:?}", get_file);
let mut file_data = vec![];
file_data.push(filename);
file_data.push(get_file);
Ok(file_data)
}
#[tokio::main]
async  fn main() {


let urls = command_line_input();

let mut  handles = Vec::new();

for url in urls{

    let handle = tokio::spawn(async move {

        let content = download_file(url).await.unwrap();

        save_file(content[0].clone(), content[1].clone())
    });

handles.push(handle);
}

for handle in handles{
    handle.await.unwrap().await.unwrap();
}
}


