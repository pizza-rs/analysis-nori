//! Nori part-of-speech stop filter for Korean.
//!
//! Removes tokens whose part-of-speech matches configured stop tags.
//! ko-dic POS tags follow the Sejong tagset.

use hashbrown::HashSet;
use alloc::sync::Arc;

use lindera::dictionary::load_dictionary;
use lindera::mode::Mode as LinderaMode;
use lindera::segmenter::Segmenter;
use lindera::tokenizer::Tokenizer as LinderaTokenizer;

use pizza_engine::analysis::{Token, TokenFilter};

/// Default Korean stop POS tags (particles, suffixes, punctuation).
pub const DEFAULT_KOREAN_STOP_TAGS: &[&str] = &[
    "E",   // Verbal endings
    "IC",  // Interjections
    "J",   // Postpositions (particles)
    "MAG", // General adverbs
    "MAJ", // Connective adverbs
    "MM",  // Determiners
    "SP",  // Spacing
    "SSC", // Closing brackets
    "SSO", // Opening brackets
    "SC",  // Separators (comma, etc.)
    "SE",  // Ellipsis
    "XPN", // Prefix (nominal)
    "XSA", // Suffix (adjective-deriving)
    "XSN", // Suffix (noun-deriving)
    "XSV", // Suffix (verb-deriving)
    "UNA", // Unknown
    "NA",  // Unknown
    "VSV", // Unknown
];

/// Removes Korean tokens matching specified part-of-speech tags.
///
/// Equivalent to Elasticsearch's `nori_part_of_speech` filter.
#[derive(Clone)]
pub struct NoriPartOfSpeechFilter {
    inner: Arc<LinderaTokenizer>,
    stop_tags: HashSet<String>,
}

impl NoriPartOfSpeechFilter {
    /// Create with a set of POS stop tags.
    pub fn new(stop_tags: Vec<String>) -> Self {
        let dictionary = load_dictionary("embedded://ko-dic")
            .expect("failed to load embedded ko-dic dictionary");
        let segmenter = Segmenter::new(LinderaMode::Normal, dictionary, None);
        let tokenizer = LinderaTokenizer::new(segmenter);
        Self {
            inner: Arc::new(tokenizer),
            stop_tags: stop_tags.into_iter().collect(),
        }
    }

    /// Create with default Korean stop tags.
    pub fn with_defaults() -> Self {
        Self::new(DEFAULT_KOREAN_STOP_TAGS.iter().map(|s| s.to_string()).collect())
    }

    fn should_remove(&self, surface: &str) -> bool {
        let tokens = match self.inner.tokenize(surface) {
            Ok(t) => t,
            Err(_) => return false,
        };
        if tokens.len() != 1 {
            return false;
        }
        let token = &tokens[0];
        if let Some(ref details) = token.details {
            // ko-dic: detail[0] = POS tag
            if !details.is_empty() {
                let pos = &details[0];
                // Handle compound POS tags like "NNG+VCP+ETM"
                for tag_part in pos.split('+') {
                    if self.stop_tags.contains(tag_part) {
                        return true;
                    }
                }
                // Also check full tag
                if self.stop_tags.contains(pos.as_ref()) {
                    return true;
                }
            }
        }
        false
    }
}

impl TokenFilter for NoriPartOfSpeechFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let surface = token.term.to_string();
        let deleted = self.should_remove(&surface);
        (deleted, None)
    }
}
