use std::rc::Rc;

pub struct KmerIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    fasta_reads_iterator: I,
    k: usize,
    i: usize,
    current_read: Option<Rc<String>>,
}

impl<I> KmerIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    pub fn from(fasta_reads_iterator: I, k: usize) -> Self {
        Self {
            fasta_reads_iterator,
            k,
            i: 0,
            current_read: None,
        }
    }
}

impl<I> Iterator for KmerIterator<I>
where
    I: Iterator<Item = Rc<String>>,
{
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_read.is_none() {
            self.current_read = Some(self.fasta_reads_iterator.next()?);
        }

        let mut read = self.current_read.clone().unwrap();

        while self.i + self.k >= read.len() {
            // we reached the end of the read we are extracting k-mers from
            self.i = 0;
            read = self.fasta_reads_iterator.next()?;
        }

        let start = self.i;
        let end = self.i + self.k;
        let kmer: String = read.chars().take(end).skip(start).collect();
        self.i += 1;
        Some(kmer)
    }
}
