use std::path::Path;
use std::fs::OpenOptions;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)   // Append to the file.
        .create(true)   // Create the file if it doesn't exist.
        .open(path)
        .expect("Failed to open or create the file");

    writeln!(file, "{}", content)
        .expect("Failed to write to the file");
}

// pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
//     let file = OpenOptions::new().append(true).create(true).open(path);

//     match file{
//         Ok(mut okay_result) => {
//             let words = writeln!(okay_result, "{}", content);

//             match words{
//                 Ok(_) => {},
//                 Err(_) => panic!(),
//             }
//         }
//         Err(_) => panic!(),
//     }
// }