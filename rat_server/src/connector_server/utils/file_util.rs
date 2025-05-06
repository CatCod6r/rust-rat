use std::path::Path;

use tokio::fs::{self, File};

pub async fn create_file(file_path: &str) {
    if let Some(parent) = Path::new(file_path).parent() {
        // Create the directory and ignore error if it already exists
        let _ = fs::create_dir_all(parent).await;
    }

    // Open or create the file
    let _ = File::create(file_path).await;
}
