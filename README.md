# fastxgz

fastx parser for rust. Supports both Gz and not gz files.

## Description
This parser can iterate over the reads of a fasta/fastq file, potentially compressed (gz). Compressed files should end in ".gz".
This parser can also iterate over the k-mers of every read, or even the hash of these k-mers.

Please see the [rust documentation](https://docs.rs/fastxgz/latest/fastxgz/) for details and examples.

This parser is meant to be convenient to use. Though not meant to be fast, its speed for enumerating the lines of a fastq file from an SSD on a laptop is only 28% slower than wc -l.
<!-- 45.25 GB -->

## How to use this
Simply add the following to your Cargo.toml file:
```
[dependencies]
fastxgz = "{X}.{Y}.{Z}"  # Please check and use the newest version
```
then:
```rust
use fastxgz::fasta_reads;

let reads = fasta_reads("data/tests/test.fa").expect("The file cannot be opened.");
for read in reads {
    println!("{}", read);
}
```
<!-- > time target/release/fastxgz xxx -->
<!-- 190831996 -->
<!-- target/release/fastxgz xxx0  35,38s user 11,20s system 78% cpu 58,973 total -->

<!-- > time wc -l /home/lrobidou/TARA/205SUR1QQSS11/BHN_AGKIOSF_3_1_C73H0ACXX.IND10_clean.fastq -->
<!-- 763327984 /home/lrobidou/TARA/205SUR1QQSS11/BHN_AGKIOSF_3_1_C73H0ACXX.IND10_clean.fastq -->
<!-- wc -l   0,57s user 10,02s system 23% cpu 45,746 total -->