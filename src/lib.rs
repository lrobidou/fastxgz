mod fastx_iterator;
mod hash_iterator;
mod high_level_interface;
mod kmer_iterator;
mod lines_iterator;

pub use crate::hash_iterator::Hash;
pub use crate::hash_iterator::HashIterator;
pub use crate::high_level_interface::fasta_reads;
pub use crate::high_level_interface::fastq_reads;
pub use crate::high_level_interface::lines_iterator;
pub use crate::kmer_iterator::KmerIterator;

pub enum FileType {
    FASTA,
    FASTQ,
}

#[cfg(test)]
mod tests {
    use crate::high_level_interface::fasta_reads;

    use super::*;
    #[test]
    fn open_fasta_test() {
        let no_gz_reads = fasta_reads("data/tests/data.fa").expect("The file cannot be opened.");
        let no_gz_kmers = KmerIterator::from(no_gz_reads, 31);
        let no_gz_hashes = HashIterator::from(no_gz_kmers);

        let gz_reads = fasta_reads("data/tests/data.fa.gz").expect("The file cannot be opened.");
        let gz_kmers = KmerIterator::from(gz_reads, 31);
        let gz_hashes = HashIterator::from(gz_kmers);

        for (x, y) in std::iter::zip(no_gz_hashes, gz_hashes) {
            assert_eq!(x, y);
        }
    }
}
