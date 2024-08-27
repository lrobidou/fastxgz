//! This crate is an implementation of a simple fasta and fastq parser.
//! It handles decompression of a gzipped file. Fasta files with reads on multiple lines are supported.
//! This crate also provides functions to iterate over kmers or their hash values.
//!
//! Examples:
//!
//! Iterating over the reads of a fasta file:
//! ```
//! use fastxgz::fasta_reads;
//!
//! let reads = fasta_reads("data/tests/test.fa").expect("The file cannot be opened.");
//! for read in reads {
//!     println!("{}", read);
//! }
//! ```
//!
//! Iterating over the kmers of the reads of a gzipped fastq file:
//! ```
//! use fastxgz::fastq_reads;
//! use fastxgz::KmerIterator;
//!
//! let reads = fastq_reads("data/tests/test.fq.gz").expect("The file cannot be opened.");
//! let kmers = KmerIterator::from(reads, 31);
//! for kmer in kmers {
//!     println!("{}", kmer);
//! }
//! ```
//!
//! Iterating over the hash of the kmers of the reads of a gzipped fastq file:
//! ```
//! use fastxgz::fastq_reads;
//! use fastxgz::KmerIterator;
//! use fastxgz::HashIterator;
//!
//! let reads = fastq_reads("data/tests/test.fq.gz").expect("The file cannot be opened.");
//! let kmers = KmerIterator::from(reads, 31);
//! let hashes = HashIterator::from(kmers);
//! for hash in hashes {
//!     println!("{}", hash);
//! }
//! ```

mod fastx_iterator;
mod hash_iterator;
mod high_level_interface;
mod kmer_iterator;
mod lines_iterator;

pub use crate::hash_iterator::HashIterator;
pub use crate::high_level_interface::fasta_reads;
pub use crate::high_level_interface::fastq_reads;
pub use crate::high_level_interface::lines_iterator;
pub use crate::high_level_interface::simple_fasta_reads;
pub use crate::kmer_iterator::KmerIterator;

///
#[cfg(test)]
mod tests {
    use crate::high_level_interface::simple_fasta_reads;

    use super::*;
    #[test]
    fn comp_fasta_fasta_gz() {
        let no_gz_reads = fasta_reads("data/tests/test.fa").expect("The file cannot be opened.");
        let no_gz_kmers = KmerIterator::from(no_gz_reads, 31);
        let no_gz_hashes = HashIterator::from(no_gz_kmers);

        let gz_reads = fasta_reads("data/tests/test.fa.gz").expect("The file cannot be opened.");
        let gz_kmers = KmerIterator::from(gz_reads, 31);
        let gz_hashes = HashIterator::from(gz_kmers);

        for (x, y) in std::iter::zip(no_gz_hashes, gz_hashes) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn comp_fastq_fastq_gz() {
        let no_gz_reads = fastq_reads("data/tests/test.fq").expect("The file cannot be opened.");
        let no_gz_kmers = KmerIterator::from(no_gz_reads, 31);
        let no_gz_hashes = HashIterator::from(no_gz_kmers);

        let gz_reads = fastq_reads("data/tests/test.fq.gz").expect("The file cannot be opened.");
        let gz_kmers = KmerIterator::from(gz_reads, 31);
        let gz_hashes = HashIterator::from(gz_kmers);

        for (x, y) in std::iter::zip(no_gz_hashes, gz_hashes) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn comp_simple_fasta_simple_fasta_gz() {
        let no_gz_reads =
            simple_fasta_reads("data/tests/test_nosplit.fa").expect("The file cannot be opened.");
        let no_gz_kmers = KmerIterator::from(no_gz_reads, 31);
        let no_gz_hashes = HashIterator::from(no_gz_kmers);

        let gz_reads = simple_fasta_reads("data/tests/test_nosplit.fa.gz")
            .expect("The file cannot be opened.");
        let gz_kmers = KmerIterator::from(gz_reads, 31);
        let gz_hashes = HashIterator::from(gz_kmers);

        for (x, y) in std::iter::zip(no_gz_hashes, gz_hashes) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn comp_simple_fasta_fasta() {
        let no_gz_reads = fasta_reads("data/tests/test.fa").expect("The file cannot be opened.");
        let no_gz_kmers = KmerIterator::from(no_gz_reads, 31);
        let no_gz_hashes = HashIterator::from(no_gz_kmers);

        let gz_reads =
            simple_fasta_reads("data/tests/test_nosplit.fa").expect("The file cannot be opened.");
        let gz_kmers = KmerIterator::from(gz_reads, 31);
        let gz_hashes = HashIterator::from(gz_kmers);

        for (x, y) in std::iter::zip(no_gz_hashes, gz_hashes) {
            assert_eq!(x, y);
        }
    }

    #[test]
    fn comp_fasta_fastq() {
        let no_gz_reads = fasta_reads("data/tests/test.fa").expect("The file cannot be opened.");
        let no_gz_kmers = KmerIterator::from(no_gz_reads, 31);
        let no_gz_hashes = HashIterator::from(no_gz_kmers);

        let gz_reads = fastq_reads("data/tests/test.fq").expect("The file cannot be opened.");
        let gz_kmers = KmerIterator::from(gz_reads, 31);
        let gz_hashes = HashIterator::from(gz_kmers);

        for (x, y) in std::iter::zip(no_gz_hashes, gz_hashes) {
            assert_eq!(x, y);
        }
    }
}
