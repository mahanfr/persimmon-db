use std::{fs};
use std::io::{Write};
use serde_json::{Value, Number};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;
mod file_system;

#[derive(Debug)]
pub struct Database {
    pub name: String,
    directory: String,
    // TODO: add a collection map
    
}   

// Database struct methods
impl Database {

    // create a new database
    // creates a folder for the database and returns a Database struct
    pub fn create(name: &str) -> Database {
        let dir_name: &str = &("data/db-".to_owned() + name);

        file_system::create_database(name);

        Database {
            name: name.to_owned(),
            directory: dir_name.to_owned(),
        }
    }

    // creates a collection file inside the database directory
    pub fn create_collection(&self, name: &str) {
        file_system::create_collection(&self.name, name);
    }

    // insert data inside collection file
    pub fn insert(& mut self, collection: &str, document: &str) {
        let file_name: &str = &(self.directory.to_owned() +"/" +&collection.to_owned() + &".paradb".to_owned());
        match fs::OpenOptions::new().append(true).open(file_name) {
            Err(why) => panic!("couldn't open file: {}", why),
            Ok(mut file) => {
                
                let mut value : Value = serde_json::from_str(document).unwrap();
                match value {
                    Value::Object(ref mut map) => {
                        map.insert("_id".to_owned(), Value::Number(Number::from(create_unique_id(collection))));
                    },
                    _ => {
                        panic!("failed to add id: document is not an object");
                    }
                }
                
                // let len = file.metadata().unwrap().len();
                

                // TODO: make main db file also searchable just in case!
                match file.write_all(value.to_string().as_bytes()) {
                    Err(why) => panic!("couldn't write to file: {}", why),
                    Ok(_) => println!("successfully wrote to file"),
                }
            }
        }
    }

}

fn create_unique_id(collection: &str) -> u64 {
    let mut s = DefaultHasher::new();
    collection.hash(&mut s);
    s.write_u32(process::id());
    s.write_u128(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis());
    s.finish()
}
