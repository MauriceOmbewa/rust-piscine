use std::fs::File;

pub fn open_file(s: &str) -> File {
    let example_file = File::open(s);

    match example_file{
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    }
}