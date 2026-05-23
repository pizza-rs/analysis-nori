//! Nori-compatible Korean morphological tokenizer.
//!
//! Wraps lindera with ko-dic dictionary. Supports three decompound modes:
//! - **None:** No decompounding — preserve original compounds
//! - **Discard:** Decompound and discard the original compound form
//! - **Mixed:** Emit both the original compound and decompounded parts

use alloc::borrow::Cow;
use alloc::sync::Arc;

use lindera::dictionary::load_dictionary;
use lindera::mode::Mode as LinderaMode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer as LinderaTokenizer;

use pizza_engine::analysis::{Token, Tokenizer};

/// Decompound mode for Korean morphological analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NoriDecompoundMode {
    /// No decompounding — keep compounds as-is.
    None,
    /// Decompound and discard the original compound.
    Discard,
    /// Emit both compound and decompounded parts (graph token).
    Mixed,
}

/// Korean morphological tokenizer using ko-dic dictionary via lindera.
///
/// Equivalent to Elasticsearch's `nori_tokenizer`.
#[derive(Clone)]
pub struct NoriTokenizer {
    inner: Arc<LinderaTokenizer>,
    mode: NoriDecompoundMode,
}

impl NoriTokenizer {
    /// Create a new Korean tokenizer with the specified decompound mode.
    pub fn new(mode: NoriDecompoundMode) -> Self {
        let lindera_mode = match mode {
            NoriDecompoundMode::None => LinderaMode::Normal,
            NoriDecompoundMode::Discard | NoriDecompoundMode::Mixed => {
                LinderaMode::Decompose(Default::default())
            }
        };

        let dictionary = load_dictionary("embedded://ko-dic")
            .expect("failed to load embedded ko-dic dictionary");
        let segmenter = Segmenter::new(lindera_mode, dictionary, None);
        let tokenizer = LinderaTokenizer::new(segmenter);
        Self {
            inner: Arc::new(tokenizer),
            mode,
        }
    }

    /// Get the current decompound mode.
    pub fn mode(&self) -> NoriDecompoundMode {
        self.mode
    }
}

impl Tokenizer for NoriTokenizer {
    fn tokenize<'a>(&self, text: &'a str) -> Vec<Token<'a>> {
        let tokens = match self.inner.tokenize(text) {
            Ok(t) => t,
            Err(_) => return Vec::new(),
        };

        let mut result = Vec::with_capacity(tokens.len());
        let mut position = 0u32;

        for token in tokens {
            let surface = token.surface.as_ref();
            if surface.is_empty() {
                continue;
            }

            // byte offsets → char-based offsets
            let byte_start = token.byte_start;
            let byte_end = token.byte_end;
            let start_offset = text[..byte_start].chars().count() as u32;
            let end_offset = text[..byte_end].chars().count() as u32;

            result.push(Token {
                term: Cow::Owned(surface.to_string()),
                start_offset,
                end_offset,
                position,
            });
            position += 1;
        }

        result
    }
}
