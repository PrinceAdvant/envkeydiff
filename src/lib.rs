//! Compare the variable names in two dotenv style files.
//!
//! `envkeydiff` reads a real `.env` file and a committed `.env.example`
//! and tells you which keys are missing from the real file and which keys
//! exist there but are not documented in the example. Values are never
//! printed, so it is safe to run in CI logs.
//!
//! ```
//! let example = "DATABASE_URL=\nREDIS_URL=\n";
//! let actual = "DATABASE_URL=postgres://db/app\nDEBUG=1\n";
//! let d = envkeydiff::diff(&envkeydiff::keys(example), &envkeydiff::keys(actual));
//! assert_eq!(d.missing, vec!["REDIS_URL".to_string()]);
//! assert_eq!(d.extra, vec!["DEBUG".to_string()]);
//! ```
//!
//! For how Compose itself loads `.env`, `env_file` and `environment`, see
//! [environment variables in Docker Compose](https://peon.sh/blogs/environment-variables-docker-compose).
//! Built by a contributor to [Peon](https://peon.sh/), an open-source
//! self-hosted deployment platform.

use std::collections::BTreeSet;

/// Extract variable names from dotenv style text.
///
/// Blank lines and lines starting with `#` are ignored, an optional
/// leading `export ` is stripped, and only the part before the first `=`
/// is kept.
pub fn keys(text: &str) -> BTreeSet<String> {
    text.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .filter_map(|l| {
            let l = l.strip_prefix("export ").unwrap_or(l);
            let k = l.split('=').next()?.trim();
            if k.is_empty() || !l.contains('=') {
                None
            } else {
                Some(k.to_string())
            }
        })
        .collect()
}

/// The difference between expected and actual keys.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Diff {
    /// Keys present in the example but not in the actual file.
    pub missing: Vec<String>,
    /// Keys present in the actual file but not in the example.
    pub extra: Vec<String>,
}

impl Diff {
    /// True when both files declare exactly the same keys.
    pub fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.extra.is_empty()
    }
}

/// Compare expected keys (from the example file) with actual keys.
pub fn diff(expected: &BTreeSet<String>, actual: &BTreeSet<String>) -> Diff {
    Diff {
        missing: expected.difference(actual).cloned().collect(),
        extra: actual.difference(expected).cloned().collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_comments_export_and_blanks() {
        let k = keys("# comment\n\nexport A=1\nB = 2\nnot a pair\n=nokey\n");
        assert_eq!(k.into_iter().collect::<Vec<_>>(), vec!["A", "B"]);
    }

    #[test]
    fn clean_when_equal() {
        let a = keys("A=1\nB=2\n");
        assert!(diff(&a, &a).is_clean());
    }
}
