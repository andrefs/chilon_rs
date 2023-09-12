# chilon_rs

Namespace-based summarization of RDF graphs.

[![Rust](https://github.com/andrefs/chilon_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/andrefs/chilon_rs/actions/workflows/rust.yml)

## Installation

```
cargo build --release
```

## Running

```
target/release/chilon_rs [OPTIONS] <RDF_FILE(S)>
```

Run `chilon_rs --help` to view available options.

## Validation

`chilon` has been validated by applying it to 11 RDF graphs, with sizes ranging from a few megabytes and less that 1 million triples, to over 90 gigabytes and thousands of millions of triples:

 1. ClaimsKG
 1. CrunchBase
 1. DbKwik
 1. DBLP
 1. DBpedia
 1. KBpedia
 1. LinkedMDB
 1. OpenCyc
 1. Wikidata
 1. WordNet
 1. Yago

The resulting summaries and visualizations can be explored here: https://andrefs.github.io/chilon_rs

## Citation
If you use `chilon` or the underlying algorithm on your work, please consider citing:

> dos Santos, A.F., Leal, J.P. (2023). Summarization of Massive RDF Graphs Using Identifier Classification. In: Ojeda-Aciego, M., Sauerwald, K., Jäschke, R. (eds) Graph-Based Representation and Reasoning. ICCS 2023. Lecture Notes in Computer Science(). Springer, Cham. https://doi.org/10.1007/978-3-031-40960-8_8

