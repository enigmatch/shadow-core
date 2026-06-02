use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ShadowCoreError {
    #[error("Rendering error: {0}")]
    RenderingError(String),

    #[error("Missing required template placeholder: {0}")]
    TemplateMissingPlaceholder(String),

    #[error("Validation error: {0}")]
    ValidationError(String),
}

pub(crate) fn find_unmatched_placeholder(text: &str) -> Option<String> {
    let bytes = text.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'{' {
            let start = i + 1;
            let mut name_end = start;
            while name_end < bytes.len()
                && (bytes[name_end].is_ascii_alphanumeric() || bytes[name_end] == b'_')
            {
                name_end += 1;
            }
            if name_end > start
                && name_end < bytes.len()
                && bytes[name_end] == b'}'
            {
                let name = unsafe { std::str::from_utf8_unchecked(&bytes[start..name_end]) };
                let first = name.as_bytes()[0];
                if first.is_ascii_alphabetic() || first == b'_' {
                    return Some(name.to_string());
                }
            }
            i = if name_end > start { name_end } else { i + 1 };
        } else {
            i += 1;
        }
    }
    None
}

pub(crate) fn serde_err_to_shadow(e: impl fmt::Display) -> ShadowCoreError {
    ShadowCoreError::RenderingError(e.to_string())
}
