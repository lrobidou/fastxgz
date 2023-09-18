use std::rc::Rc;

pub struct FastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    lines_iterator: I,
}

impl<I> FastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    pub fn from(lines_iterator: I) -> Self {
        Self {
            lines_iterator: lines_iterator,
        }
    }
}

impl<I> Iterator for FastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = self.lines_iterator.next()?;
        while line.starts_with(">") {
            line = self.lines_iterator.next()?;
        }
        Some(line)
    }
}

pub struct FastqReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    lines_iterator: I,
}

impl<I> FastqReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    pub fn from(lines_iterator: I) -> Self {
        Self {
            lines_iterator: lines_iterator,
        }
    }
}

impl<I> Iterator for FastqReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let _first = self.lines_iterator.next()?;
        let second = self.lines_iterator.next()?;
        let _third = self.lines_iterator.next()?;
        let _fourth = self.lines_iterator.next()?;
        Some(second)
    }
}
