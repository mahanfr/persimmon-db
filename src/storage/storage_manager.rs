use std::{fs, io::{Write, SeekFrom, Seek, Read}, vec, convert::TryInto};


#[path="./test.rs"]
mod test;

const PAGE_SIZE: usize = 4096;

fn create_page(path: &str) {
    // fill 4k file with null bytes
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)
        .unwrap();
    let mut buffer = vec![0; PAGE_SIZE];
    file.write_all(&mut buffer).unwrap();
}

fn get_page(path: &str , page_number: usize) -> vec::Vec<u8> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();
    let mut buffer = vec![0; PAGE_SIZE];
    file.seek(SeekFrom::Start((page_number * PAGE_SIZE).try_into().unwrap())).unwrap();
    file.read_exact(&mut buffer).unwrap();
    buffer
}