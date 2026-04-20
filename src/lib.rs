pub const PROFILE_SYSTEM_PROMPT: &str = include_str!("prompts/profile_system_prompt.txt");
pub const PREVIEW_SYSTEM_PROMPT: &str = include_str!("prompts/preview_system_prompt.txt");
pub const CHAT_SYSTEM_PROMPT: &str = include_str!("prompts/chat_system_prompt.txt");
pub const ONBOARDING_TURN_TWO_SYSTEM_PROMPT: &str = include_str!("prompts/onboarding_turn_two.txt");
pub const ONBOARDING_TURN_THREE_SYSTEM_PROMPT: &str =
    include_str!("prompts/onboarding_turn_three.txt");
pub const SHADOW_CORE_PERSONA_PROMPT: &str = include_str!("prompts/shadow_core_persona.txt");
pub const ONBOARDING_MODE_PROMPT: &str = include_str!("prompts/onboarding_mode.txt");
pub const NORMAL_CHAT_MODE_PROMPT: &str = include_str!("prompts/normal_chat_mode.txt");
pub const OUTPUT_STYLE_PROMPT: &str = include_str!("prompts/output_style.txt");

pub enum ShadowLocale {
    English,
    Japanese,
    French,
}

impl ShadowLocale {
    pub fn from_code(code: &str) -> Self {
        match code {
            "ja" => Self::Japanese,
            "fr" => Self::French,
            _ => Self::English,
        }
    }

    pub fn prompt_language_name(&self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Japanese => "Japanese",
            Self::French => "French",
        }
    }
}

pub struct PromptTemplate<'a> {
    template: &'a str,
}

impl<'a> PromptTemplate<'a> {
    pub fn new(template: &'a str) -> Self {
        Self { template }
    }

    pub fn render(&self, vars: &[(&str, &str)]) -> String {
        let mut result = self.template.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{key}}}"), value);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{
        PromptTemplate, CHAT_SYSTEM_PROMPT, NORMAL_CHAT_MODE_PROMPT, ONBOARDING_MODE_PROMPT,
        ONBOARDING_TURN_THREE_SYSTEM_PROMPT, ONBOARDING_TURN_TWO_SYSTEM_PROMPT,
        OUTPUT_STYLE_PROMPT, PREVIEW_SYSTEM_PROMPT, PROFILE_SYSTEM_PROMPT,
        SHADOW_CORE_PERSONA_PROMPT,
    };

    #[test]
    fn prompt_template_replaces_single_variable() {
        let result = PromptTemplate::new("Hello, {name}!").render(&[("name", "World")]);
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn prompt_template_replaces_multiple_variables() {
        let result = PromptTemplate::new("Hi {user_name}, meet {shadow_name}.")
            .render(&[("user_name", "Alice"), ("shadow_name", "Kage")]);
        assert_eq!(result, "Hi Alice, meet Kage.");
    }

    #[test]
    fn prompt_template_leaves_unmatched_placeholders_intact() {
        let result = PromptTemplate::new("Hello {name}, your {unknown} is safe.")
            .render(&[("name", "Alice")]);
        assert_eq!(result, "Hello Alice, your {unknown} is safe.");
    }

    #[test]
    fn prompt_template_replaces_placeholder_appearing_multiple_times() {
        let result = PromptTemplate::new("{x} and {x} again").render(&[("x", "foo")]);
        assert_eq!(result, "foo and foo again");
    }

    #[test]
    fn prompt_template_renders_real_persona_prompt_variables() {
        let rendered = PromptTemplate::new(SHADOW_CORE_PERSONA_PROMPT).render(&[
            ("shadow_name", "Kage"),
            ("user_name", "Yuki"),
            ("interface_language", "Japanese"),
        ]);
        assert!(!rendered.contains("{shadow_name}"));
        assert!(!rendered.contains("{user_name}"));
        assert!(!rendered.contains("{interface_language}"));
        assert!(rendered.contains("Kage"));
        assert!(rendered.contains("Yuki"));
        assert!(rendered.contains("Japanese"));
    }

    #[test]
    fn shadow_locale_from_en_code_returns_english_language_name() {
        assert_eq!(super::ShadowLocale::from_code("en").prompt_language_name(), "English");
    }

    #[test]
    fn shadow_locale_from_ja_code_returns_japanese_language_name() {
        assert_eq!(super::ShadowLocale::from_code("ja").prompt_language_name(), "Japanese");
    }

    #[test]
    fn shadow_locale_from_fr_code_returns_french_language_name() {
        assert_eq!(super::ShadowLocale::from_code("fr").prompt_language_name(), "French");
    }

    #[test]
    fn shadow_locale_falls_back_to_english_for_unknown_code() {
        assert_eq!(super::ShadowLocale::from_code("de").prompt_language_name(), "English");
        assert_eq!(super::ShadowLocale::from_code("").prompt_language_name(), "English");
    }

    #[test]
    fn prompt_assets_are_non_empty() {
        assert!(!PROFILE_SYSTEM_PROMPT.trim().is_empty());
        assert!(!PREVIEW_SYSTEM_PROMPT.trim().is_empty());
        assert!(!CHAT_SYSTEM_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_TURN_TWO_SYSTEM_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_TURN_THREE_SYSTEM_PROMPT.trim().is_empty());
        assert!(!SHADOW_CORE_PERSONA_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_MODE_PROMPT.trim().is_empty());
        assert!(!NORMAL_CHAT_MODE_PROMPT.trim().is_empty());
        assert!(!OUTPUT_STYLE_PROMPT.trim().is_empty());
    }

    #[test]
    fn profile_prompt_keeps_output_contract_private() {
        assert!(PROFILE_SYSTEM_PROMPT.contains("append the exact output contract separately"));
        assert!(!PROFILE_SYSTEM_PROMPT.contains("\"headline\""));
        assert!(!PROFILE_SYSTEM_PROMPT.contains("Return JSON only with this exact shape"));
    }
}
