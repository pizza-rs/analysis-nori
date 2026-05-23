# pizza-analysis-nori

Korean morphological analysis for the [Pizza](https://pizza.rs) search engine. Wraps the [Lindera](https://github.com/lindera/lindera) library with the ko-dic dictionary for tokenization, reading form conversion, and POS-based filtering.

## Components

| Name | Type | Description |
|------|------|-------------|
| `nori_tokenizer` | Tokenizer | Korean morphological tokenizer with decompound modes |
| `nori_part_of_speech` | Token Filter | Remove tokens by part-of-speech (POS) tags |
| `nori_readingform` | Token Filter | Convert Hanja (漢字) to Hangul reading |
| `ko_stop` | Token Filter | Remove Korean stop words |
| `nori` | Analyzer | Full Korean pipeline |

## Usage

### Full Analyzer

The `nori` analyzer combines all components into a standard Korean analysis pipeline:

```json
{
  "analyzer": {
    "type": "nori"
  }
}
```

Pipeline: `nori_tokenizer` → `nori_part_of_speech` → `nori_readingform`

### Decompound Modes

| Mode | Description |
|------|-------------|
| `none` | No decompounding of compound nouns |
| `discard` | Decompound and discard the original compound form (default) |
| `mixed` | Decompound and keep both the original and sub-tokens |

### Examples

**Input:** `가거도항`

| Mode | Output |
|------|--------|
| None | `가거도항` |
| Discard | `가거도`, `항` |
| Mixed | `가거도항`, `가거도`, `항` |

**Input:** `碩765765`

| Component | Output |
|-----------|--------|
| Tokenizer | `碩`, `765765` |
| + Readingform | `석`, `765765` |

### Custom Analyzer

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "nori_tokenizer",
    "filter": ["nori_part_of_speech", "nori_readingform", "ko_stop"]
  }
}
```

## Stop Words

Default Korean stop words include common particles and postpositions:
`이`, `그`, `저`, `것`, `수`, `등`, `들`, `및`, `에`, `의`, `가`, `으로`, `에서`, `를`, `은`, `는`, `도`, `와`, `과`, `하다`, ...

## POS Tags Filtered by Default

Particles, suffixes, and punctuation:
- `E` — Verbal endings
- `IC` — Interjections
- `J` — Particles (postpositions)
- `MAG` — General adverbs
- `SP` — Spaces
- `SSC`/`SSO` — Brackets
- `SC`/`SE`/`SF`/`SY` — Punctuation/symbols
- `XPN`/`XSA`/`XSN`/`XSV` — Prefixes/suffixes

## Data Sources

- **Dictionary**: mecab-ko-dic — the same dictionary used by Apache Lucene's Nori analyzer
- **Embedded via**: `lindera` 3.0 with `embed-ko-dic` feature

## Features

- `embed-dict` (default) — Embeds the ko-dic dictionary at compile time

## License

Apache-2.0
