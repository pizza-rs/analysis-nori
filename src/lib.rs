#![cfg_attr(not(feature = "std"), no_std)]
//! Korean morphological analysis for Pizza search engine.
//!
//! This crate provides Nori-compatible Korean text analysis using
//! [lindera](https://github.com/lindera/lindera) with the ko-dic dictionary.
//!
//! # Components
//!
//! - [`NoriTokenizer`] — Korean morphological tokenizer (none/discard/mixed decompound modes)
//! - [`NoriPartOfSpeechFilter`] — Remove tokens by part-of-speech tags
//! - [`NoriReadingformFilter`] — Convert Hanja to Hangul reading
//! - [`KoreanStopFilter`] — Korean stop word removal
extern crate alloc;
mod tokenizer;
mod part_of_speech;
mod readingform;
mod stop;

pub use tokenizer::{NoriTokenizer, NoriDecompoundMode};
pub use part_of_speech::NoriPartOfSpeechFilter;
pub use readingform::NoriReadingformFilter;
pub use stop::KoreanStopFilter;
pub mod register;
pub use register::register_all;
