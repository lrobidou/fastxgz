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
