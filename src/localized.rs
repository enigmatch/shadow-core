//! Values written separately for each language.
//!
//! A language users can pick must have its own entry in every table; each
//! table's test fails otherwise, instead of the Shadow silently getting the
//! default language's text.

use crate::{ShadowLocale, DEFAULT_LANGUAGE};

pub(crate) struct Localized<T: 'static> {
    entries: &'static [(&'static str, T)],
}

impl<T: Copy> Localized<T> {
    pub(crate) const fn new(entries: &'static [(&'static str, T)]) -> Self {
        Self { entries }
    }

    /// The value for `locale`, or the default language's for unknown and
    /// hidden codes.
    pub(crate) fn for_locale(&self, locale: &str) -> T {
        self.entry(ShadowLocale::resolve_code(locale))
            .or_else(|| self.entry(DEFAULT_LANGUAGE.code))
            .expect("every localized table has a default language entry")
    }

    fn entry(&self, code: &str) -> Option<T> {
        self.entries
            .iter()
            .find(|(entry_code, _)| *entry_code == code)
            .map(|(_, value)| *value)
    }

    /// Shown languages without their own entry. Tables check this is empty.
    #[cfg(test)]
    pub(crate) fn missing_shown_languages(&self) -> Vec<&'static str> {
        crate::shown_languages()
            .map(|language| language.code)
            .filter(|code| self.entry(code).is_none())
            .collect()
    }
}
