//! Korean stop word filter.
//!
//! Removes common Korean stop words (particles, copulas, etc.)
//! that add little value to full-text search.

use hashbrown::HashSet;

use pizza_engine::analysis::{Token, TokenFilter};

/// Default Korean stop words (common particles, endings, copulas).
pub const KOREAN_STOP_WORDS: &[&str] = &[
    "이", "그", "저", "것", "수", "등", "들", "및", "에", "의",
    "가", "를", "으로", "로", "에서", "와", "과", "도", "는", "은",
    "만", "에게", "까지", "부터", "이다", "하다", "되다", "있다",
    "없다", "않다", "같다", "위하다", "대하다", "통하다", "따르다",
    "에서의", "로서", "로써", "라고", "이라고",
];

/// Removes Korean stop words from the token stream.
///
/// Equivalent to Elasticsearch's `nori_stoptags` combined with basic stop words.
#[derive(Clone)]
pub struct KoreanStopFilter {
    stop_words: HashSet<String>,
}

impl KoreanStopFilter {
    /// Create with default Korean stop words.
    pub fn new() -> Self {
        Self {
            stop_words: KOREAN_STOP_WORDS
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// Create with custom stop words.
    pub fn with_words(words: Vec<String>) -> Self {
        Self {
            stop_words: words.into_iter().collect(),
        }
    }
}

impl Default for KoreanStopFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenFilter for KoreanStopFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let deleted = self.stop_words.contains(token.term.as_ref());
        (deleted, None)
    }
}
