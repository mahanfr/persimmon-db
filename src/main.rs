use std::env;
use database::Database;
use config::Config;

mod database;
mod shell;
mod config;
// #[path="storage/storage_manager.rs"]
// mod storage_manager;

fn main() {
    let mut args = env::args();
    args.next(); // skip program
    if let Some(command_or_option) = args.next() {
        todo!("{}",command_or_option);
    }else{
        shell::run();
    }
    // create config file
    // TODO: add config to all calls 
    Config::create();
    // create a database
    // command: use <db-name>
    let mut db = Database::create("test");
    println!("switched to {}",db.name);
    // create a collection
    db.create_collection("students");
    // insert some documents
    db.insert("students", "{\"name\":\"John Doe\",\"age\":25}");
    db.insert("students", "{\"name\":\"cummies\",\"age\":265}");
    db.insert("students", "{\"name\":\"asghar\",\"age\":1}");
    // find some documents

    // update some documents

    // delete some documents

    // drop the database

    // drop the collection
}
