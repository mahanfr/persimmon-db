use std::{fs::File, io::{Write, Read}};

pub(crate) struct DataBase {
    key: String,
    value: String,
    path: String,
}

impl DataBase {
    pub fn create(key:&str,value:&str) -> DataBase {
        let path = format!("db/objects/{}.txt",key);

        let dir = std::path::Path::new(&path).parent().unwrap();
        if !dir.exists() {
            std::fs::create_dir_all(dir).unwrap();
        }
        match File::create(path) {
            Ok(file) => file,
            Err(error) => panic!("Could not create file: {}", error),
        };

        let db = DataBase {
            key: key.to_string(),
            value: value.to_string(),
            path: format!("db/objects/{}.txt",key).to_string(),
        };
        db
    }

    // inserting to a database by replacing contents of struct and file
    pub fn insert(&mut self, value:String) {
        self.set_value(value);
        let mut file = File::options().read(true).write(true).open(self.get_path()).unwrap();
        let _ = file.write_all(self.get_value().as_bytes());
    }

    //remove the database files and only keep it in memory
    pub fn remove(&self) {
        // Remove a file
        std::fs::remove_file(&self.path).unwrap();
    }

    // get data stored in database from file
    pub fn get_stored_value(&self) -> String {
        let mut file = File::options().read(true).open(self.get_path()).unwrap();
        let mut buffer = String::new();
        file.read_to_string(&mut buffer).unwrap();
        buffer
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    fn set_value(&mut self, value:String) {
        self.value = value;
    }

}