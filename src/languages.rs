//! The single list of languages SBT knows about.
//!
//! Every other place that needs "which languages exist" reads from here, so a
//! new language is added by one entry instead of edits scattered across the
//! codebase that silently fall back to English when one is missed.

/// Whether users can pick the language yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    /// Offered in the language switcher, accepted in URLs, and given its own
    /// prompts.
    Shown,
    /// Its catalog loads, but users are not offered it yet and prompts treat
    /// it as the default language. Used while a language is still in progress.
    Hidden,
}

/// The writing system a language uses, for rules that depend on script
/// rather than on one specific language (typography, length limits).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Script {
    Latin,
    Japanese,
}

#[derive(Debug, PartialEq, Eq)]
pub struct LanguageSpec {
    /// Short code used in URLs, cookies and stored data, e.g. `ja`.
    pub code: &'static str,
    /// Fluent catalog folder name, e.g. `ja-JP`.
    pub catalog_id: &'static str,
    /// Name shown in the language switcher, written in the language itself.
    pub switcher_label: &'static str,
    /// Flag emoji shown on the language button, e.g. 🇯🇵.
    pub flag: &'static str,
    /// English name of the language, as told to the LLM.
    pub prompt_name: &'static str,
    pub script: Script,
    pub visibility: Visibility,
    /// Whether new content is translated into this language as soon as it is
    /// created. When false, readers get the original with an on-demand
    /// translate option, so adding a language does not multiply translation
    /// jobs for every new message, answer and question.
    pub auto_translate: bool,
}

impl LanguageSpec {
    #[must_use]
    pub fn is_shown(&self) -> bool {
        self.visibility == Visibility::Shown
    }

    /// Hidden languages are never auto-translated, whatever their flag says.
    #[must_use]
    pub fn is_auto_translated(&self) -> bool {
        self.is_shown() && self.auto_translate
    }
}

pub const LANGUAGES: &[LanguageSpec] = &[
    LanguageSpec {
        code: "en",
        catalog_id: "en-US",
        switcher_label: "English",
        flag: "🇺🇸",
        prompt_name: "English",
        script: Script::Latin,
        visibility: Visibility::Shown,
        auto_translate: true,
    },
    LanguageSpec {
        code: "ja",
        catalog_id: "ja-JP",
        switcher_label: "日本語",
        flag: "🇯🇵",
        prompt_name: "Japanese",
        script: Script::Japanese,
        visibility: Visibility::Shown,
        auto_translate: true,
    },
    LanguageSpec {
        code: "fr",
        catalog_id: "fr-FR",
        switcher_label: "Français",
        flag: "🇫🇷",
        prompt_name: "French",
        script: Script::Latin,
        visibility: Visibility::Shown,
        auto_translate: true,
    },
    LanguageSpec {
        code: "de",
        catalog_id: "de-DE",
        switcher_label: "Deutsch",
        flag: "🇩🇪",
        prompt_name: "German",
        script: Script::Latin,
        visibility: Visibility::Hidden,
        auto_translate: false,
    },
];

/// The language used whenever a code is unknown or not shown.
pub const DEFAULT_LANGUAGE: &LanguageSpec = &LANGUAGES[0];

/// Any registered language, shown or hidden. Matching is exact: callers that
/// accept tags such as `fr-FR` reduce them to the short code themselves.
#[must_use]
pub fn find_language(code: &str) -> Option<&'static LanguageSpec> {
    LANGUAGES.iter().find(|language| language.code == code)
}

/// A language users can pick, or `None` for unknown and hidden codes.
#[must_use]
pub fn find_shown_language(code: &str) -> Option<&'static LanguageSpec> {
    find_language(code).filter(|language| language.is_shown())
}

/// Languages users can pick, in switcher order.
pub fn shown_languages() -> impl Iterator<Item = &'static LanguageSpec> {
    LANGUAGES.iter().filter(|language| language.is_shown())
}

#[cfg(test)]
mod tests {
    use super::{
        find_language, find_shown_language, shown_languages, Visibility, DEFAULT_LANGUAGE,
        LANGUAGES,
    };
    use std::collections::HashSet;

    #[test]
    fn current_shown_languages_are_all_auto_translated() {
        let codes = LANGUAGES
            .iter()
            .filter(|language| language.is_auto_translated())
            .map(|language| language.code)
            .collect::<Vec<_>>();
        assert_eq!(codes, vec!["en", "ja", "fr"]);
    }

    #[test]
    fn hidden_languages_are_never_auto_translated() {
        for language in LANGUAGES.iter().filter(|language| !language.is_shown()) {
            assert!(
                !language.is_auto_translated(),
                "{} is hidden, so new content must not be translated into it",
                language.code
            );
        }
    }

    #[test]
    fn shown_languages_keep_the_current_switcher_order() {
        let codes = shown_languages()
            .map(|language| language.code)
            .collect::<Vec<_>>();
        assert_eq!(codes, vec!["en", "ja", "fr"]);
    }

    #[test]
    fn default_language_is_shown_english() {
        assert_eq!(DEFAULT_LANGUAGE.code, "en");
        assert_eq!(DEFAULT_LANGUAGE.catalog_id, "en-US");
        assert_eq!(DEFAULT_LANGUAGE.visibility, Visibility::Shown);
    }

    #[test]
    fn hidden_language_is_found_but_not_shown() {
        assert_eq!(find_language("de").map(|l| l.catalog_id), Some("de-DE"));
        assert_eq!(find_shown_language("de"), None);
        assert!(shown_languages().all(|language| language.code != "de"));
    }

    #[test]
    fn lookup_is_exact_and_rejects_unknown_codes() {
        assert_eq!(find_shown_language("ja").map(|l| l.code), Some("ja"));
        assert_eq!(find_language("ja-JP"), None);
        assert_eq!(find_language("JA"), None);
        assert_eq!(find_language("xx"), None);
        assert_eq!(find_language(""), None);
    }

    #[test]
    fn shown_languages_keep_the_current_flags() {
        let flags = shown_languages()
            .map(|language| (language.code, language.flag))
            .collect::<Vec<_>>();
        assert_eq!(flags, vec![("en", "🇺🇸"), ("ja", "🇯🇵"), ("fr", "🇫🇷")]);
    }

    #[test]
    fn every_flag_is_one_regional_indicator_pair() {
        let is_regional_indicator = |c: char| ('\u{1F1E6}'..='\u{1F1FF}').contains(&c);
        for language in LANGUAGES {
            let chars = language.flag.chars().collect::<Vec<_>>();
            assert!(
                chars.len() == 2 && chars.iter().copied().all(is_regional_indicator),
                "{} flag {:?} must be a single country flag emoji",
                language.code,
                language.flag
            );
        }
    }

    #[test]
    fn codes_and_catalog_ids_are_unique() {
        let codes = LANGUAGES.iter().map(|l| l.code).collect::<HashSet<_>>();
        let catalogs = LANGUAGES
            .iter()
            .map(|l| l.catalog_id)
            .collect::<HashSet<_>>();
        assert_eq!(codes.len(), LANGUAGES.len());
        assert_eq!(catalogs.len(), LANGUAGES.len());
    }

    #[test]
    fn catalog_id_starts_with_its_code() {
        for language in LANGUAGES {
            assert!(
                language
                    .catalog_id
                    .starts_with(&format!("{}-", language.code)),
                "{} catalog id {} must start with its code",
                language.code,
                language.catalog_id
            );
        }
    }
}
