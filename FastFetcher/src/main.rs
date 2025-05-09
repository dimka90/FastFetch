use std::{env, task::Context, thread::spawn};
use reqwest::{self, Error};
fn command_line_input() -> Vec<String>{

    let command_line_input: Vec<String> = env::args().skip(1).collect();

    println!("{:?}", command_line_input);
    command_line_input
}


async  fn save_file(content: String) -> Result<(), Error>{
let _ = tokio::fs::write("dimka.txt", content.as_bytes()).await;

Ok(())
}
async fn download_file(url: String) -> Result<String, reqwest::Error>{

    let get_file = reqwest::get(url)
                    .await?
                    .text()
                    .await?;
                println!("{:?}", get_file);

                Ok(get_file)
}
#[tokio::main]
async  fn main() {


let urls = command_line_input();

for i in urls.into_iter(){

    let result = tokio::spawn(download_file(i)).await;
    let content_to_save = result.unwrap().unwrap();
    let save_result = tokio::spawn(save_file(content_to_save)).await;

}


}
