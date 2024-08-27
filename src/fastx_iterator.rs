use std::rc::Rc;

pub struct SimpleFastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    lines_iterator: I,
}

impl<I> SimpleFastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    pub fn from(lines_iterator: I) -> Self {
        Self {
            lines_iterator,
        }
    }
}

impl<I> Iterator for SimpleFastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let _first = self.lines_iterator.next()?;
        let second = self.lines_iterator.next()?;
        Some(second)
    }
}

pub struct FastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    lines_iterator: I,
    next_line: Option<Rc<String>>,
    end_reached: bool,
}

impl<I> FastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    pub fn from(lines_iterator: I) -> Self {
        Self {
            lines_iterator,
            next_line: None,
            end_reached: false,
        }
    }
}

impl<I> Iterator for FastaReadsIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    type Item = Rc<String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end_reached {
            return None;
        }

        let _first = match &self.next_line {
            None => self.lines_iterator.next()?,
            Some(x) => x.clone(),
        };
        self.next_line = None;

        let mut second: Vec<Rc<String>> = vec![self.lines_iterator.next()?];
        let mut second_size = second[0].len();
        // check if this read is on multiple lines
        loop {
            match self.lines_iterator.next() {
                Some(x) => {
                    if x.starts_with('>') {
                        self.next_line = Some(x);
                        break;
                    } else {
                        second.push(x.clone());
                        second_size += x.len();
                    }
                }
                None => {
                    self.end_reached = true;
                    break;
                }
            }
        }
        // merge
        let mut merge: String = String::with_capacity(second_size);
        for x in second {
            merge.push_str(&x.to_string());
        }

        Some(Rc::new(merge))
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
            lines_iterator,
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
