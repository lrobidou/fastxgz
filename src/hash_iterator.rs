use xxhash_rust::xxh3::xxh3_64;

/// An iterator taking a sequence of kmers and yielding their hash.
pub struct HashIterator<I>
where
    I: Iterator<Item = String>,
{
    kmers: I,
}

impl<I> HashIterator<I>
where
    I: Iterator<Item = String>,
{
    pub fn from(kmers: I) -> Self {
        Self { kmers }
    }
}

impl<I> Iterator for HashIterator<I>
where
    I: Iterator<Item = String>,
{
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let kmer = self.kmers.next()?;
        Some(xxh3_64(kmer.as_bytes()))
    }
}
