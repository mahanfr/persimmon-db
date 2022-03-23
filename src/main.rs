#[path = "../test/files.rs"]
mod test;

mod database;
use database::Database;


fn main() {
    // create a database
    // command: use <db-name>
    let db = Database::create("test");
    println!("switched to {}",db.name);
    // create a collection
    db.create_collection("students");
    // insert some documents
    db.insert("students", "{\"_id\":123,\"name\":\"John Doe\",\"age\":25}");
    // find some documents

    // update some documents

    // delete some documents

    // drop the database

    // drop the collection
}
