//! Register Nori (Korean) analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;

use pizza_engine::analysis::AnalysisFactory;
use pizza_engine::analysis::Analyzer;

use crate::{
    KoreanStopFilter, NoriDecompoundMode, NoriPartOfSpeechFilter,
    NoriReadingformFilter, NoriTokenizer,
};

/// Register Nori tokenizers, token filters, and analyzers.
///
/// Matches Elasticsearch's analysis-nori plugin registration:
/// - Tokenizer: `nori_tokenizer` (discard decompound mode by default)
/// - Filters: `nori_part_of_speech`, `nori_readingform`, `ko_stop`
/// - Analyzer: `nori` (KoreanAnalyzer pipeline: pos → readingform)
pub fn register_all(factory: &mut AnalysisFactory) {
    // Tokenizers
    factory.register_tokenizer("nori_tokenizer", Box::new(NoriTokenizer::new(NoriDecompoundMode::Discard)));

    // Token filters
    factory.register_token_filter("nori_part_of_speech", Box::new(NoriPartOfSpeechFilter::with_defaults()));
    factory.register_token_filter("nori_readingform", Box::new(NoriReadingformFilter::new()));
    factory.register_token_filter("ko_stop", Box::new(KoreanStopFilter::new()));

    // Analyzer: nori (matches Lucene KoreanAnalyzer pipeline)
    // Pipeline: tokenizer(discard) → part_of_speech → readingform
    factory.register_analyzer(
        "nori",
        Analyzer::new(
            vec![],
            Box::new(NoriTokenizer::new(NoriDecompoundMode::Discard)),
            vec![
                Box::new(NoriPartOfSpeechFilter::with_defaults()),
                Box::new(NoriReadingformFilter::new()),
            ],
        ),
    );
}
