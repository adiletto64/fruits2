use std::fs;
use std::io::Write;
use std::path::Path;


const FILE_NAME: &str = "records.txt";


pub fn is_record(score: u32) -> bool {
    if score > read_file() {
        return true 
    }
    false
}


pub fn read_file() -> u32 {
    let file_exists = Path::new(FILE_NAME).is_file();
    if !file_exists {
        return 0;
    }
    
    let record = fs::read_to_string(FILE_NAME).unwrap();
    record.parse::<u32>().unwrap_or(0)
}


pub fn write_record(record: u32)  {
    let mut file: fs::File;
    
    
    let file_exists = Path::new(FILE_NAME).is_file();
    if file_exists {
        file = fs::OpenOptions::new().write(true).open(FILE_NAME).unwrap();
    } else {

        file = fs::OpenOptions::new().write(true).create(true).open(FILE_NAME).unwrap();
    }

    file.write_all(record.to_string().as_str().as_bytes()).unwrap();
}



#[cfg(test)]
mod tests {
    use std::{path::Path, fs};
    use crate::utils::record::is_record;
    use super::{write_record, read_file, FILE_NAME};

    #[test]
    fn test_it_writes_record() {
        write_record(100);

        let result = read_file();

        assert_eq!(result, 100);
        cleanup();
    }

    #[test]
    fn test_it_checks_record() {
        write_record(100);

        assert!(!is_record(50));
        assert!(is_record(150));

        cleanup();
    }

    fn cleanup() {
        if Path::new(FILE_NAME).is_file() {
            fs::remove_file(FILE_NAME).unwrap();
        }
    }
}
