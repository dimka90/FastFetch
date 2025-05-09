mod fastfetcher;

use fastfetcher::fastfetcher::{command_line_input, download_file, save_file};

#[tokio::main]
async fn main() {
    let urls = command_line_input();

    let mut handles = Vec::new();

    for url in urls {
        let handle = tokio::spawn(async move {
            let content = download_file(url).await.unwrap();

            save_file(content)
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap().await.unwrap();
    }
}
