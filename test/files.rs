#[cfg(test)]
mod tests {
  use crate::db;

  #[test]
  fn create_db() {
    let db = db::DataBase::create("test", "test");
    assert_eq!(db.get_key(), "test");
    assert_eq!(db.get_value(), "test");
    assert_eq!(db.get_path(), "db/objects/test.txt");
    db.remove();
  }

  #[test]
  fn check_if_db_file_exists() {
    let db = db::DataBase::create("test1", "test1");
    assert!(std::path::Path::new(db.get_path()).exists());
    db.remove();
  }
  
  #[test]
  fn check_if_db_gets_removed() {
    let db = db::DataBase::create("test2", "test2");
    db.remove();
    assert!(!std::path::Path::new(db.get_path()).exists());
  }

  #[test]
  fn insert_value_to_db() {
    let mut db = db::DataBase::create("test3", "test3");
    db.insert("Hello".to_string());
    assert_eq!(db.get_value(), "Hello");
    db.remove();
  }

  #[test]
  fn get_stored_value() {
    let mut db = db::DataBase::create("test4", "test4");
    db.insert("Hello".to_string());
    assert_eq!(db.get_stored_value(), "Hello".to_string());
    db.remove();
  }

  #[test]
  fn check_if_value_equals_stored_value() {
    let mut db = db::DataBase::create("test5", "test5");
    db.insert("Hello".to_string());
    assert_eq!(db.get_value(), "Hello");
    db.remove();
  }

}
