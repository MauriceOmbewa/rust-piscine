use std::path::Path;
use std::io::Write;
use std::fs::OpenOptions;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let file = OpenOptions::new().append(true).create(true).open(path);

    match file{
        Ok(mut okay_result) => {
            let words = writeln!(okay_result, "{}", content);

            match words{
                Ok(_) => {},
                Err(_) => panic!(),
            }
        }
        Err(_) => panic!(),
    }
}