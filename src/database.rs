use std::fs;
use std::io::Write;
use std::path::Path;
use serde_json::{Value, Number};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};
use std::process;

pub struct Database {
    pub name: String,
    directory: String,
}

impl Database {

    pub fn create(name: &str) -> Database {
        let dir_name: &str = &("data/db-".to_owned() + name);

        match fs::create_dir_all(dir_name) {
            Err(why) => panic!("couldn't create directory: {}", why),
            Ok(_) => println!("successfully created database directory"),
        }

        // check if a file exist inside the directory
        if Path::new(&(dir_name.to_owned() + &"/.init".to_owned())).exists() {
            println!("database init file already exists");
        }else {
            // create a init file
            match fs::File::create(&(dir_name.to_owned() + &"/.init".to_owned())) {
                Err(why) => panic!("database couldn't create init file: {}", why),
                Ok(_) => println!("database init file created"),
            }
        }

        Database {
            name: name.to_owned(),
            directory: dir_name.to_owned(),
        }
    }

    pub fn create_collection(&self, name: &str) {

        if Path::new(&(self.directory.to_owned() +"/"+ &name.to_owned() + &".paradb".to_owned())).exists() {
            println!("database init file already exists");
        }else{
            match fs::File::create(&(self.directory.to_owned() +"/" +&name.to_owned() + &".paradb".to_owned())) {
                Err(why) => panic!("database couldn't create init file: {}", why),
                Ok(_) => println!("database init file created"),
            }
        }

    }

    pub fn insert(&self, collection: &str, document: &str) {
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
    s.finish() 
        - SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() 
        + process::id() as u64
}