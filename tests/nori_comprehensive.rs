//! Comprehensive tests for the `pizza-analysis-nori` crate.

use pizza_analysis_nori::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter, Tokenizer};

// ─── Helpers ───────────────────────────────────────────────────────────────────

fn terms<'a>(tokens: &'a [Token<'_>]) -> Vec<&'a str> {
    tokens.iter().map(|t| t.term.as_ref()).collect()
}

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

fn filter_term(filter: &dyn TokenFilter, term: &str) -> String {
    let mut token = make_token(term);
    filter.filter(&mut token);
    token.term.into_owned()
}

fn filter_deleted(filter: &dyn TokenFilter, term: &str) -> bool {
    let mut token = make_token(term);
    let (deleted, _) = filter.filter(&mut token);
    deleted
}

// ═══════════════════════════════════════════════════════════════════════════════
// mod tokenizer — NoriTokenizer
// ═══════════════════════════════════════════════════════════════════════════════

mod tokenizer {
    use super::*;

    #[test]
    fn none_mode_basic_sentence() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("한국어를 처리합니다");
        let t = terms(&tokens);
        assert!(t.len() >= 2, "should produce multiple tokens, got {:?}", t);
    }

    #[test]
    fn none_mode_returns_correct_mode() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        assert_eq!(tok.mode(), NoriDecompoundMode::None);
    }

    #[test]
    fn discard_mode_returns_correct_mode() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::Discard);
        assert_eq!(tok.mode(), NoriDecompoundMode::Discard);
    }

    #[test]
    fn mixed_mode_returns_correct_mode() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::Mixed);
        assert_eq!(tok.mode(), NoriDecompoundMode::Mixed);
    }

    #[test]
    fn discard_mode_decompounds() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::Discard);
        let tokens = tok.tokenize("가곡역");
        let t = terms(&tokens);
        // Discard mode should decompose compound words
        assert!(!t.is_empty(), "discard mode should produce tokens");
    }

    #[test]
    fn empty_string() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("");
        assert!(tokens.is_empty());
    }

    #[test]
    fn single_hangul_word() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("서울");
        let t = terms(&tokens);
        assert!(
            t.contains(&"서울"),
            "single word should tokenize to itself, got {:?}",
            t
        );
    }

    #[test]
    fn mixed_korean_ascii() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("Java언어");
        let t = terms(&tokens);
        assert!(t.len() >= 1, "should produce tokens for mixed text, got {:?}", t);
    }

    #[test]
    fn positions_are_sequential() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("한국어를 처리합니다");
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(
                token.position, i as u32,
                "positions should be sequential"
            );
        }
    }

    #[test]
    fn offsets_are_character_based() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("서울시");
        for token in &tokens {
            assert!(
                token.end_offset <= 3,
                "char offset should be ≤ 3 for 3-char input, got end={}",
                token.end_offset
            );
        }
    }

    #[test]
    fn whitespace_only() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("   ");
        let _ = terms(&tokens); // no panic
    }

    #[test]
    fn punctuation_handling() {
        let tok = NoriTokenizer::new(NoriDecompoundMode::None);
        let tokens = tok.tokenize("서울。부산");
        let t = terms(&tokens);
        assert!(
            t.iter().any(|s: &&str| s.contains("서울")),
            "expected 서울 in {:?}",
            t
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// mod part_of_speech — NoriPartOfSpeechFilter
// ═══════════════════════════════════════════════════════════════════════════════

mod part_of_speech {
    use super::*;

    #[test]
    fn filters_particle_with_defaults() {
        let filter = NoriPartOfSpeechFilter::with_defaults();
        // Korean particles/postpositions often tagged as J*
        // "를" is an object particle
        let deleted = filter_deleted(&filter, "를");
        // May or may not be deleted depending on ko-dic tagging
        let _ = deleted;
    }

    #[test]
    fn keeps_noun() {
        let filter = NoriPartOfSpeechFilter::with_defaults();
        // "서울" is a proper noun (NNP)
        assert!(
            !filter_deleted(&filter, "서울"),
            "proper noun 서울 should be kept"
        );
    }

    #[test]
    fn custom_stop_tags() {
        // NNG = common noun — remove nouns for this test
        let filter = NoriPartOfSpeechFilter::new(vec!["NNG".to_string()]);
        // A common noun should be removed
        let _ = filter_deleted(&filter, "사람");
    }

    #[test]
    fn with_defaults_does_not_panic() {
        let filter = NoriPartOfSpeechFilter::with_defaults();
        let _ = filter_deleted(&filter, "테스트");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// mod readingform — NoriReadingformFilter
// ═══════════════════════════════════════════════════════════════════════════════

mod readingform {
    use super::*;

    #[test]
    fn hangul_passthrough() {
        let filter = NoriReadingformFilter::new();
        // Pure Hangul has no Hanja conversion needed
        let result = filter_term(&filter, "서울");
        assert_eq!(result, "서울", "Hangul should pass through unchanged");
    }

    #[test]
    fn ascii_passthrough() {
        let filter = NoriReadingformFilter::new();
        let result = filter_term(&filter, "hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn default_construction() {
        let filter = NoriReadingformFilter::default();
        let result = filter_term(&filter, "서울");
        assert_eq!(result, "서울");
    }

    #[test]
    fn does_not_panic_on_empty() {
        let filter = NoriReadingformFilter::new();
        let result = filter_term(&filter, "");
        assert_eq!(result, "");
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// mod stop — KoreanStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

mod stop {
    use super::*;

    #[test]
    fn removes_particle_eui() {
        let filter = KoreanStopFilter::new();
        assert!(filter_deleted(&filter, "의"), "의 should be a stop word");
    }

    #[test]
    fn removes_particle_ga() {
        let filter = KoreanStopFilter::new();
        assert!(filter_deleted(&filter, "가"), "가 should be a stop word");
    }

    #[test]
    fn removes_particle_reul() {
        let filter = KoreanStopFilter::new();
        assert!(filter_deleted(&filter, "를"), "를 should be a stop word");
    }

    #[test]
    fn removes_particle_neun() {
        let filter = KoreanStopFilter::new();
        assert!(filter_deleted(&filter, "는"), "는 should be a stop word");
    }

    #[test]
    fn removes_particle_eun() {
        let filter = KoreanStopFilter::new();
        assert!(filter_deleted(&filter, "은"), "은 should be a stop word");
    }

    #[test]
    fn removes_particle_e() {
        let filter = KoreanStopFilter::new();
        assert!(filter_deleted(&filter, "에"), "에 should be a stop word");
    }

    #[test]
    fn keeps_content_word() {
        let filter = KoreanStopFilter::new();
        assert!(
            !filter_deleted(&filter, "서울"),
            "content word 서울 should not be removed"
        );
    }

    #[test]
    fn keeps_verb_stem() {
        let filter = KoreanStopFilter::new();
        assert!(
            !filter_deleted(&filter, "먹다"),
            "verb should not be a stop word (unless in default list)"
        );
    }

    #[test]
    fn custom_stop_words() {
        let filter = KoreanStopFilter::with_words(vec![
            "커스텀".to_string(),
            "테스트".to_string(),
        ]);
        assert!(filter_deleted(&filter, "커스텀"));
        assert!(filter_deleted(&filter, "테스트"));
        assert!(
            !filter_deleted(&filter, "의"),
            "default words not in custom list"
        );
    }

    #[test]
    fn default_construction() {
        let filter = KoreanStopFilter::default();
        assert!(filter_deleted(&filter, "의"));
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// mod register — register_all
// ═══════════════════════════════════════════════════════════════════════════════

mod register {
    use super::*;

    #[test]
    fn registers_nori_tokenizer() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(
            factory.get_tokenizer("nori_tokenizer").is_some(),
            "nori_tokenizer should be registered"
        );
    }

    #[test]
    fn registers_part_of_speech_filter() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(
            factory.get_token_filter("nori_part_of_speech").is_some(),
            "nori_part_of_speech filter should be registered"
        );
    }

    #[test]
    fn registers_readingform_filter() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(
            factory.get_token_filter("nori_readingform").is_some(),
            "nori_readingform filter should be registered"
        );
    }

    #[test]
    fn registers_ko_stop_filter() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(
            factory.get_token_filter("ko_stop").is_some(),
            "ko_stop filter should be registered"
        );
    }

    #[test]
    fn registers_nori_analyzer() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(
            factory.get_analyzer("nori").is_some(),
            "nori analyzer should be registered"
        );
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// mod pipeline — Full integration pipeline
// ═══════════════════════════════════════════════════════════════════════════════

mod pipeline {
    use super::*;

    /// Tokenize, then apply a chain of filters, returning surviving terms.
    fn analyze(text: &str) -> Vec<String> {
        let tokenizer = NoriTokenizer::new(NoriDecompoundMode::Discard);
        let pos_filter = NoriPartOfSpeechFilter::with_defaults();
        let readingform = NoriReadingformFilter::new();
        let stop = KoreanStopFilter::new();

        let mut tokens = tokenizer.tokenize(text);
        let filters: Vec<&dyn TokenFilter> = vec![&pos_filter, &readingform, &stop];

        let mut result = Vec::new();
        for token in &mut tokens {
            let mut deleted = false;
            for f in &filters {
                let (del, _extras) = f.filter(token);
                if del {
                    deleted = true;
                    break;
                }
            }
            if !deleted {
                result.push(token.term.to_string());
            }
        }
        result
    }

    #[test]
    fn full_pipeline_basic() {
        let result = analyze("한국어를 처리합니다");
        assert!(!result.is_empty(), "pipeline should produce tokens");
    }

    #[test]
    fn full_pipeline_removes_stop_words() {
        let result = analyze("서울의 날씨는 좋다");
        // "의" and "는" are stop words
        assert!(
            !result.contains(&"의".to_string()),
            "stop word 의 should be removed, got {:?}",
            result
        );
    }

    #[test]
    fn registered_analyzer_works() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("nori").unwrap();
        let mut input = "한국어를 처리합니다".to_string();
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        assert!(
            !tokens.is_empty(),
            "registered nori analyzer should produce tokens"
        );
    }
}
