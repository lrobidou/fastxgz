use flate2::read::GzDecoder;

use std::{
    fs::File,
    io::{self, prelude::*},
    rc::Rc,
};

enum FileType {
    GZ(io::BufReader<GzDecoder<File>>),
    NOGZ(io::BufReader<File>),
}

pub struct ResultLinesIterator {
    reader: FileType,
    buf: Rc<String>,
}

fn new_buf() -> Rc<String> {
    Rc::new(String::with_capacity(1024)) // Custom capacity
}

impl ResultLinesIterator {
    pub fn open(path: impl AsRef<std::path::Path>) -> io::Result<Self> {
        match path.as_ref().extension() {
            Some(ext) if ext == "gz" => {
                let file = File::open(path)?;
                let gz_decoder = GzDecoder::new(file);

                let reader = io::BufReader::new(gz_decoder);
                let buf = new_buf();

                Ok(Self {
                    reader: FileType::GZ(reader),
                    buf,
                })
            }
            _ => {
                let file = File::open(path)?;
                let reader = io::BufReader::new(file);
                let buf = new_buf();

                Ok(Self {
                    reader: FileType::NOGZ(reader),
                    buf,
                })
            }
        }
    }
}

impl Iterator for ResultLinesIterator {
    type Item = io::Result<Rc<String>>;

    fn next(&mut self) -> Option<Self::Item> {
        let buf = match Rc::get_mut(&mut self.buf) {
            Some(buf) => {
                buf.clear();
                buf
            }
            None => {
                self.buf = new_buf();
                Rc::make_mut(&mut self.buf)
            }
        };

        let size_read = match &mut self.reader {
            FileType::GZ(x) => x.read_line(buf),
            FileType::NOGZ(x) => x.read_line(buf),
        };

        match size_read {
            Err(e) => Some(Err(e)),
            Ok(size) => match size {
                0 => None,
                _ => {
                    if buf.ends_with("\n") {
                        buf.pop();
                    }
                    Some(Ok(Rc::clone(&self.buf)))
                }
            },
        }
    }
}

pub struct LinesIterator<I>
where
    I: Iterator<Item = io::Result<Rc<String>>>,
{
    lines_iterator: I,
}

impl<I> LinesIterator<I>
where
    I: Iterator<Item = io::Result<Rc<String>>>,
{
    pub fn from(lines_iterator: I) -> Self {
        Self {
            lines_iterator: lines_iterator,
        }
    }
}

impl<I> Iterator for LinesIterator<I>
where
    I: Iterator<Item = io::Result<Rc<String>>>,
{
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let line_or_error = self.lines_iterator.next()?;
        let line = line_or_error.expect("A line could not be read due to an IO error.");
        Some(line)
    }
}
// use flate2::read::GzDecoder;
// use std::fs::File;
// use std::io::{self, BufRead, BufReader, Read};
// use std::path::Path;

// // Define a custom enum to represent either a regular file or a gzipped file.
// enum FileType {
//     RegularFile(File),
//     GzippedFile(GzDecoder<File>),
// }

// // Implement an iterator that yields lines from either regular or gzipped files.
// struct LineIterator {
//     file: FileType,
// }

// impl LineIterator {
//     // Create a new LineIterator from a file path.
//     fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
//         let file = match path.as_ref().extension() {
//             Some(ext) if ext == "gz" => {
//                 let file = File::open(path)?;
//                 FileType::GzippedFile(GzDecoder::new(file))
//             }
//             _ => {
//                 let file = File::open(path)?;
//                 FileType::RegularFile(file)
//             }
//         };

//         Ok(LineIterator { file })
//     }
// }

// impl Iterator for LineIterator {
//     type Item = io::Result<String>;

//     fn next(&mut self) -> Option<Self::Item> {
//         let mut buffer = String::new();
//         let result = match &mut self.file {
//             FileType::RegularFile(file) => {
//                 if file.read_line(&mut buffer).is_ok() {
//                     Ok(buffer.clone())
//                 } else {
//                     Err(io::Error::new(
//                         io::ErrorKind::InvalidData,
//                         "Failed to read line from regular file",
//                     ))
//                 }
//             }
//             FileType::GzippedFile(file) => {
//                 if file.read_line(&mut buffer).is_ok() {
//                     Ok(buffer.clone())
//                 } else {
//                     Err(io::Error::new(
//                         io::ErrorKind::InvalidData,
//                         "Failed to read line from gzipped file",
//                     ))
//                 }
//             }
//         };

//         match result {
//             Ok(line) => Some(Ok(line)),
//             Err(err) => Some(Err(err)),
//         }
//     }
// }

// fn main() -> io::Result<()> {
//     // Replace 'your_file_path' with the path to your file.
//     let file_path = "your_file_path";

//     let line_iterator = LineIterator::new(file_path)?;

//     for line in line_iterator {
//         match line {
//             Ok(line) => println!("{}", line),
//             Err(err) => eprintln!("Error: {}", err),
//         }
//     }

//     Ok(())
// }
