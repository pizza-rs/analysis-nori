<div align="center">

# 🇰🇷 pizza-analysis-nori

**Korean morphological analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--nori-blue)](https://github.com/pizza-rs/analysis-nori)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

Korean morphological analyzer built on [lindera](https://github.com/lindera/lindera) with
the Korean dictionary. Provides tokenization with compound-word decomposition,
part-of-speech filtering, and reading form conversion — matching the Elasticsearch
`analysis-nori` plugin feature set.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| Tokenizer | `nori_tokenizer` | Korean morphological tokenizer with decompounding |
| TokenFilter | `nori_part_of_speech` | Remove tokens by POS tag (particles, punctuation) |
| TokenFilter | `nori_readingform` | Convert Hanja (漢字) to Hangul reading |
| TokenFilter | `ko_stop` | Korean stop words |
| Analyzer | `nori` | Full pipeline: nori_tokenizer → POS filter → readingform → stop |

### Decompound Modes

| Mode | Behavior | Example (가곡역) |
|:-----|:---------|:--------|
| `None` | Keep as-is | `가곡역` |
| `Discard` | Only emit parts | `가곡` + `역` |
| `Mixed` | Emit parts + original | `가곡` + `역` + `가곡역` |

## Example

```rust
use pizza_engine::analysis::Tokenizer;
use pizza_analysis_nori::{NoriTokenizer, NoriDecompoundMode};

let tk = NoriTokenizer::new(NoriDecompoundMode::Mixed);
let tokens = tk.tokenize("가곡역");
// Mixed mode: ["가곡", "역", "가곡역"]
```

## Installation

```toml
[dependencies]
pizza-analysis-nori = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["nori"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
