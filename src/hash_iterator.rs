use xxhash_rust::xxh3::xxh3_64;

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)]
pub struct Hash(pub u64);

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
    type Item = Hash;

    fn next(&mut self) -> Option<Self::Item> {
        let kmer = self.kmers.next()?;
        Some(Hash(xxh3_64(kmer.as_bytes())))
    }
}
