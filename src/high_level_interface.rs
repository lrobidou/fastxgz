use crate::lines_iterator::LinesIterator;
use crate::lines_iterator::ResultLinesIterator;

use crate::fastx_iterator::FastaReadsIterator;
use crate::fastx_iterator::FastqReadsIterator;
use crate::fastx_iterator::SimpleFastaReadsIterator;

/// Return an iterator over the lines of a (potentially gzipped) file.
pub fn lines_iterator(
    path: impl AsRef<std::path::Path>,
) -> Result<LinesIterator<ResultLinesIterator>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    Ok(LinesIterator::from(f))
}

/// Return an iterator over the reads of a (potentially gzipped) fastq file.
pub fn fastq_reads(
    path: impl AsRef<std::path::Path>,
) -> Result<FastqReadsIterator<LinesIterator<ResultLinesIterator>>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    let lines = LinesIterator::from(f);
    Ok(FastqReadsIterator::from(lines))
}

/// Return an iterator over the reads of a (potentially gzipped) fastq file. The reads must be on a single line.
pub fn simple_fasta_reads(
    path: impl AsRef<std::path::Path>,
) -> Result<SimpleFastaReadsIterator<LinesIterator<ResultLinesIterator>>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    let lines = LinesIterator::from(f);
    Ok(SimpleFastaReadsIterator::from(lines))
}

/// Return an iterator over the reads of a (potentially gzipped) fasta file. Reads can be on multiple lines.
pub fn fasta_reads(
    path: impl AsRef<std::path::Path>,
) -> Result<FastaReadsIterator<LinesIterator<ResultLinesIterator>>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    let lines = LinesIterator::from(f);
    Ok(FastaReadsIterator::from(lines))
}
