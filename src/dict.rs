//! ko-dic dictionary resolution for the nori plugin.
//!
//! Resolution order (matching `pizza_engine::analysis::dict`):
//!
//! 1. An external lindera ko-dic dictionary directory at
//!    `<analysis dict dir>/nori/ko-dic`.
//! 2. The embedded ko-dic dictionary — only compiled in with the `embed-dict`
//!    feature.
//!
//! Nori analyzers are registered lazily, so this only runs when a schema
//! actually uses a nori component.

use lindera::dictionary::load_dictionary;
use lindera::dictionary::Dictionary;

/// Load the ko-dic dictionary using the external-first, embedded-fallback policy.
pub(crate) fn load_kodic() -> Dictionary {
    #[cfg(feature = "std")]
    {
        if let Some(path) = pizza_engine::analysis::dict::resolve("nori", "ko-dic") {
            let p = path.to_str().expect("non-UTF-8 nori dictionary path");
            return load_dictionary(p)
                .unwrap_or_else(|e| panic!("failed to load ko-dic dictionary from {p}: {e}"));
        }
    }

    #[cfg(feature = "embed-dict")]
    {
        crate::dict::load_kodic()
    }

    #[cfg(not(feature = "embed-dict"))]
    {
        panic!(
            "nori ko-dic dictionary not available: place a lindera ko-dic dictionary directory \
             at <analysis dict dir>/nori/ko-dic and configure the analysis dict path, or build \
             pizza-analysis-nori with the 'embed-dict' feature"
        )
    }
}
