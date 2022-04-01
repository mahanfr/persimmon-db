#[cfg(test)]
mod tests {
  use crate::storage_manager::StorageManager;

  #[test]
  fn create_page() {
    // initial
    assert!(StorageManager::collect("test/test.test").is_ok());
    // with file already exists
    assert!(StorageManager::collect("test/test.test").is_ok());
    // cleanup
    assert!(std::fs::remove_file("test/test.test").is_ok());
  }

}
