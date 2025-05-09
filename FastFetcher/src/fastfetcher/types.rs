pub struct DownloadResult {
    pub filename: String,
    pub content: String,
}

impl DownloadResult {
    pub fn new(filename: String, content: String) -> Self {
        Self { filename, content }
    }
}
