#[cfg(test)]
mod tests {
  use crate::storage_manager;

  #[test]
  fn create_page() {
    storage_manager::create_page("test/test.test");
    assert!(std::path::Path::new("test/test.test").exists());
    // cleanup
    assert!(std::fs::remove_file("test/test.test").is_ok());
  }

  #[test]
  fn get_page() {
    storage_manager::create_page("test/test2.test");
    let page = storage_manager::get_page("test/test2.test", 0);
    assert_eq!(page.len(), 4096);
    assert_eq!(page, vec![0; 4096]);
    // cleanup
    assert!(std::fs::remove_file("test/test2.test").is_ok());
  }


}
