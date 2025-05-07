use std::path::Path;

use tokio::fs::{self, File};

pub async fn create_file(file_path: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        // Create the directory and ignore error if it already exists
        fs::create_dir_all(parent).await.unwrap();
    }

    // Open or create the file
    File::create(file_path).await.unwrap();
}
