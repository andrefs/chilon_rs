# chilon_rs

> A fast, parallel Rust tool for namespace-based summarization of massive RDF graphs — extracts structure, infers namespaces, normalizes triples, and generates interactive visualizations.

[![Rust](https://github.com/andrefs/chilon_rs/actions/workflows/rust.yml/badge.svg)](https://github.com/andrefs/chilon_rs/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust 1.77+](https://img.shields.io/badge/rust-1.77+-blue.svg)](https://www.rust-lang.org)
[![Version](https://img.shields.io/crates/v/chilon_rs.svg?cache=1)](https://crates.io/crates/chilon_rs)
[![CLI Version](https://img.shields.io/crates/v/chilon_cli.svg)](https://crates.io/crates/chilon-cli)

[![oxigraph](https://img.shields.io/crates/v/oxigraph?label=oxigraph)](https://crates.io/crates/oxigraph)
[![qp-trie](https://img.shields.io/crates/v/qp-trie?label=qp-trie)](https://crates.io/crates/qp-trie)
[![clap](https://img.shields.io/crates/v/clap?label=clap)](https://crates.io/crates/clap)
[![oxrdf](https://img.shields.io/crates/v/oxrdf?label=oxrdf)](https://crates.io/crates/oxrdf)
[![rayon](https://img.shields.io/crates/v/rayon?label=rayon)](https://crates.io/crates/rayon)
[![serde](https://img.shields.io/crates/v/serde?label=serde)](https://crates.io/crates/serde)

## Table of Contents

- [Background](#background)
- [Install](#install)
- [Quick Start](#quick-start)
- [Usage](#usage)
  - [Basic Processing](#basic-processing)
  - [CLI Options](#cli-options)
  - [Multiple Files](#multiple-files)
- [Outputs & Visualization](#outputs--visualization)
- [Auxiliary Commands](#auxiliary-commands)
- [Validation](#validation)
- [Citation](#citation)
- [Contributing](#contributing)
- [License](#license)

## Background

RDF graphs at web scale (hundreds of millions to billions of triples) are difficult to explore and understand. **chilon_rs** addresses this by:

1. **Namespace Inference** — automatically discovers namespace prefixes from IRIs using a community prefix table and statistical segmentation.
2. **Triple Normalization** — groups triples by inferred namespace, counts occurrences, and filters by minimum frequency to produce a compact summary.
3. **Visualization** — emits JSON data and a self-contained HTML/JS visualization for interactive exploration of the summary.

The algorithm is based on: *dos Santos & Leal (2023). "Summarization of Massive RDF Graphs Using Identifier Classification." ICCS 2023.*

## Install

### From Source (Recommended)

Requires **Rust 1.77+**.

```bash
git clone https://github.com/andrefs/chilon_rs.git
cd chilon_rs
cargo build --release
```

The binary will be at `cli/target/release/chilon_rs`.

### Cargo Install (when published)

```bash
cargo install chilon_rs
```

## Quick Start

```bash
# Build
cargo build --release

# Process an RDF file (Turtle format)
./cli/target/release/chilon_rs mygraph.ttl
```

This creates a dated folder under `results/YYYYMMDD/` containing:
- Normalized triples grouped by namespace
- Namespace prefix table
- `summary.json` + `visualization.html` for interactive browsing

## Usage

### Basic Processing

```bash
# Process a single file with defaults (namespace inference ON, strict mode)
chilon_rs graph.ttl
```

### CLI Options

| Flag | Description |
|------|-------------|
| `--no-infer-ns` | Disable namespace inference; use only the built-in community prefix table. |
| `-i, --ignore-unknown` | Skip triples whose namespace cannot be resolved (instead of erroring). |
| `-h, --help` | Show help message. |
| `-V, --version` | Print version. |

### Multiple Files

```bash
# Process multiple files in one run
chilon_rs file1.ttl file2.ttl file3.ttl
```

Worker threads are auto-tuned: `max(2, min(files + 1, CPU cores - 2))`.

## Outputs & Visualization

After processing, a folder `results/YYYYMMDD-N/` is created with:

| File | Description |
|------|-------------|
| `output.ttl` | Normalized triples grouped by namespace (Turtle). |
| `vis-data.json` | Visualization data (nodes, edges, aliases). |
| `namespaces.tsv` | Inferred + community prefix table (prefix, URI, count). |
| `normalized.tsv` | Normalized triples: `subject\tpredicate\tobject\tcount`. |
| `tasks.json` | Processing metadata (timings, triple counts, stages). |
| `chilon.log` | Detailed execution log. |

### Viewing the Visualization

The visualization is a JavaScript module that **must be served over HTTP** (browsers block ES modules on `file://`). Two steps:

1. **Build the visualization assets** (requires Node.js ≥ 18 + yarn):
   ```bash
   cargo run --bin gen-viz -- results/20260909-9
   ```
   This runs `vite build` inside `chilon-viz/` and copies `dist/` into the results folder.

2. **Serve the `dist/` folder** and open in browser:
   ```bash
   cd results/20260909-9/dist && python3 -m http.server 8000
   # Then open http://localhost:8000
   ```

## Auxiliary Commands

Two additional binaries ship with the crate:

```bash
# Generate visualization from an existing results folder
cargo run --bin gen-viz -- results/20260909-9

# Quick test/debug: parse RDF and show basic stats without full pipeline
cargo run --bin test-files -- file1.ttl file2.ttl
```

## Validation

`chilon_rs` has been validated on **11 real-world RDF graphs** spanning from a few MB / <1M triples to **90+ GB / billions of triples**:

| Dataset | Domain |
|---------|--------|
| ClaimsKG | Fact-checking claims |
| CrunchBase | Company/startup data |
| DbKwik | Wikipedia infobox extraction |
| DBLP | Bibliography / publications |
| DBpedia | Wikipedia structured data |
| KBpedia | Knowledge base ontology |
| LinkedMDB | Movie database |
| OpenCyc | Common-sense knowledge base |
| Wikidata | General knowledge graph |
| WordNet | Lexical database |
| YAGO | Ontology from Wikipedia |

Summaries and visualizations: <https://andrefs.github.io/chilon_rs>

## Citation

If you use `chilon_rs` or the underlying algorithm in your work, please cite:

> dos Santos, A.F., Leal, J.P. (2023). *Summarization of Massive RDF Graphs Using Identifier Classification.* In: Ojeda-Aciego, M., Sauerwald, K., Jäschke, R. (eds) Graph-Based Representation and Reasoning. ICCS 2023. Lecture Notes in Computer Science. Springer, Cham. [https://doi.org/10.1007/978-3-031-40960-8_8](https://doi.org/10.1007/978-3-031-40960-8_8)

## Datasets

The 11 corpora used for validation are publicly available:

- [ClaimsKG](https://claimskg.org/)
- [CrunchBase](https://crunchbase.com/)
- [DbKwik](https://dbkwik.com/)
- [DBLP](https://dblp.org/)
- [DBpedia](https://dbpedia.org/)
- [KBpedia](https://kbpedia.org/)
- [LinkedMDB](https://linkedmdb.org/)
- [OpenCyc](https://opencyc.org/)
- [Wikidata](https://www.wikidata.org/)
- [WordNet](https://wordnet.princeton.edu/)
- [YAGO](https://yago.org/)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:

- Setting up the development environment (`cargo test`, `cargo clippy --all-targets -- -D warnings`)
- Code style and commit conventions
- Opening issues and pull requests

## License

Released under the **MIT License** — see [LICENSE](LICENSE) for details.

Copyright (c) 2023 Alexandre F. dos Santos