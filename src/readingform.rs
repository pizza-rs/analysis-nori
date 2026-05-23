//! Nori reading form filter.
//!
//! Converts Hanja (Chinese characters used in Korean) to their Hangul reading.
//! In ko-dic, the reading is stored in the detail fields.

use alloc::borrow::Cow;
use alloc::sync::Arc;

use lindera::dictionary::load_dictionary;
use lindera::mode::Mode as LinderaMode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer as LinderaTokenizer;

use pizza_engine::analysis::{Token, TokenFilter};

/// Converts Hanja tokens to their Hangul reading form.
///
/// Equivalent to Elasticsearch's `nori_readingform` filter.
#[derive(Clone)]
pub struct NoriReadingformFilter {
    inner: Arc<LinderaTokenizer>,
}

impl NoriReadingformFilter {
    /// Create a new reading form filter.
    pub fn new() -> Self {
        let dictionary = load_dictionary("embedded://ko-dic")
            .expect("failed to load embedded ko-dic dictionary");
        let segmenter = Segmenter::new(LinderaMode::Normal, dictionary, None);
        let tokenizer = LinderaTokenizer::new(segmenter);
        Self {
            inner: Arc::new(tokenizer),
        }
    }

    /// Check if a character is Hanja (CJK Unified Ideograph).
    fn is_hanja(ch: char) -> bool {
        let c = ch as u32;
        (0x4E00..=0x9FFF).contains(&c)     // CJK Unified Ideographs
            || (0x3400..=0x4DBF).contains(&c) // CJK Unified Ideographs Extension A
            || (0xF900..=0xFAFF).contains(&c) // CJK Compatibility Ideographs
    }

    /// Check if the token contains any Hanja characters.
    fn contains_hanja(text: &str) -> bool {
        text.chars().any(Self::is_hanja)
    }

    /// Get the Hangul reading from ko-dic details.
    fn get_reading(&self, surface: &str) -> Option<String> {
        if !Self::contains_hanja(surface) {
            return None;
        }

        let tokens = self.inner.tokenize(surface).ok()?;
        if tokens.len() != 1 {
            return None;
        }
        let token = &tokens[0];
        if let Some(ref details) = token.details {
            // ko-dic stores reading in one of the later detail fields
            // Typically detail[3] or detail[7] contains the reading
            for idx in [3, 7, 1] {
                if details.len() > idx && details[idx] != "*" && details[idx] != surface {
                    let reading = &details[idx];
                    // Verify the reading is actually Hangul
                    if reading.chars().all(|ch| {
                        let c = ch as u32;
                        (0xAC00..=0xD7AF).contains(&c) || (0x1100..=0x11FF).contains(&c)
                    }) {
                        return Some(reading.to_string());
                    }
                }
            }
        }
        None
    }
}

impl Default for NoriReadingformFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenFilter for NoriReadingformFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let surface = token.term.to_string();
        if let Some(reading) = self.get_reading(&surface) {
            token.term = Cow::Owned(reading);
        }
        (false, None)
    }
}
