# chilon_rs

A namespace-based summarization tool for RDF graphs.

[![Rust](https://github.com/andrefs/chilon_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/andrefs/chilon_rs/actions/workflows/rust.yml)

## Installation

```bash
cargo build --release
```

## Quick Start

```bash
# Build and run on an RDF file
cargo build --release
./target/release/chilon_rs mygraph.ttl
```

## Features

- **Namespace Inference**: Automatically detects and infers namespaces from RDF files
- **Triple Normalization**: Standardizes triple formats for consistent processing
- **Visualization**: Generates JSON and HTML visualizations of the processed graph
- **Batch Processing**: Process multiple RDF files efficiently

## Usage

### Basic Example

```bash
# Create a sample RDF file and process it
cargo build --release
./target/release/chilon_rs sample.ttl
```

### With CLI Flags

```bash
# Parse RDF files with custom options
chilon_rs --help

# Process multiple files
chilon_rs file1.ttl file2.ttl file3.ttl
```

## Validation

`chilon` has been validated by applying it to 11 RDF graphs, with sizes ranging from a few megabytes and less than 1 million triples, to over 90 gigabytes and thousands of millions of triples:

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

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [ClaimsKG](https://claimskg.org/)
- [CrunchBase](https://crunchbase.com/)
- [DbKwik](https://dbkwik.com/)
- [DBLP](https://dblp.org/)
- [DBpedia](https://dbpedia.org/)
- [KBpedia](https://kbpedia.org/)
- [LinkedMDB](https://linkedmdb.org/)
- [OpenCyc](https://openCyc.org/)
- [Wikidata](https://www.wikidata.org/)
- [WordNet](https://wordnet.princeton.edu/)
- [Yago](https://yago.org/)
