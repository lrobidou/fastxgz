use crate::lines_iterator::LinesIterator;
use crate::lines_iterator::ResultLinesIterator;

use crate::fastx_iterator::FastaReadsIterator;
use crate::fastx_iterator::FastqReadsIterator;

pub fn lines_iterator(
    path: impl AsRef<std::path::Path>,
) -> Result<LinesIterator<ResultLinesIterator>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    Ok(LinesIterator::from(f))
}

pub fn fastq_reads(
    path: impl AsRef<std::path::Path>,
) -> Result<FastqReadsIterator<LinesIterator<ResultLinesIterator>>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    let lines = LinesIterator::from(f);
    Ok(FastqReadsIterator::from(lines))
}

pub fn fasta_reads(
    path: impl AsRef<std::path::Path>,
) -> Result<FastaReadsIterator<LinesIterator<ResultLinesIterator>>, std::io::Error> {
    let f = ResultLinesIterator::open(path)?;
    let lines = LinesIterator::from(f);
    Ok(FastaReadsIterator::from(lines))
}
