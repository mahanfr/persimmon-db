use std::{
    fs,
    io::{Read, Write, Error},
    vec
};

#[path = "./test.rs"]
mod test;

const PAGE_SIZE: usize = 4096;

struct StorageManager {
    dictionary: [u8; PAGE_SIZE],
    file: fs::File,
}

impl StorageManager {
    fn collect(path: &str) -> Result<StorageManager, Error> {
      let mut file = fs::OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
        
      let mut page_dictionary : [u8; PAGE_SIZE] = [0; PAGE_SIZE];
      
      if file.metadata()?.len() == 0  {
        file.write_all(vec![0; PAGE_SIZE].as_slice())?;
      }else{
        file.read_exact(&mut page_dictionary).unwrap();
      }

      Ok(StorageManager {
        dictionary: page_dictionary,
        file,
      })
    }

    fn append_page(mut self) -> Result<(),Error> {
        // fill 4k file with null bytes

        let mut buffer = vec![0; PAGE_SIZE];
        self.file.write_all(&mut buffer).unwrap();
        Ok(())
    }

}
