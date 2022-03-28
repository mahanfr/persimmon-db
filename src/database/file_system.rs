use std::{path::Path, fs};

const DB_DIR_PREFIX: &str = "data/db-";

#[allow(dead_code)]
fn get_collection_dir(db_name: &str, collection_name: &str) -> String {
    let dir_name: &str = &(DB_DIR_PREFIX.to_owned() + db_name + "/"+ collection_name +".paradb");
    return dir_name.to_owned();
}

pub fn create_database(db_name: &str) {
    let dir_name: &str = &(DB_DIR_PREFIX.to_owned() + db_name);
    match fs::create_dir_all(dir_name) {
        Err(why) => println!("couldn't create directory: {}", why),
        Ok(_) => println!("successfully created database directory"),
    }
}

pub fn collection_exists(db_name: &str, collection_name: &str) -> bool {
    Path::new(&get_collection_dir(db_name,collection_name)).exists() 
}

pub fn create_collection(db_name: &str,collection_name: &str) {

    // check if a file exist inside the directory
    if !collection_exists(db_name, collection_name){
        // create the collection file
        println!("{}", get_collection_dir(db_name, collection_name));
        match fs::File::create(get_collection_dir(db_name,collection_name)) {
            Err(why) => println!("database couldn't create init file: {}", why),
            Ok(_) => println!("database init file created"),
        }
    }
}
