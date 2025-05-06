pub async fn create_path(&self) {
    if let Some(parent) = Path::new(USERS_PATH).parent() {
        // Create the directory and ignore error if it already exists
        let _ = fs::create_dir_all(parent).await;
    }

    // Open or create the file
    let _ = File::create(USERS_PATH).await;
}
