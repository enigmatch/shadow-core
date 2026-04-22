mod prompt_inputs;
mod template;

pub use prompt_inputs::{
    PromptReadyPersona, PromptReadyProfile, PromptReadyReasoningPolicy, PromptReadySpeechStyle,
};
pub use template::PromptTemplate;

pub struct SystemPrompts {
    pub profile_system_prompt: &'static str,
    pub preview_system_prompt: &'static str,
    pub chat_system_prompt: &'static str,
    pub onboarding_turn_two_system_prompt: &'static str,
    pub onboarding_turn_three_system_prompt: &'static str,
    pub shadow_core_persona_prompt: &'static str,
    pub onboarding_mode_prompt: &'static str,
    pub normal_chat_mode_prompt: &'static str,
    pub output_style_prompt: &'static str,
}

impl SystemPrompts {
    pub fn for_locale(locale: &str) -> Self {
        // Shared prompts (English-only)
        let common = Self {
            profile_system_prompt: include_str!("prompts/profile_system_prompt.txt"),
            preview_system_prompt: include_str!("prompts/preview_system_prompt.txt"),
            chat_system_prompt: include_str!("prompts/chat_system_prompt.txt"),
            onboarding_turn_two_system_prompt: include_str!("prompts/onboarding_turn_two.txt"),
            onboarding_turn_three_system_prompt: include_str!("prompts/onboarding_turn_three.txt"),
            shadow_core_persona_prompt: include_str!("prompts/shadow_core_persona.txt"),
            onboarding_mode_prompt: include_str!("prompts/en/onboarding_mode.txt"), // Default
            normal_chat_mode_prompt: include_str!("prompts/normal_chat_mode.txt"),
            output_style_prompt: include_str!("prompts/output_style.txt"),
        };

        match locale {
            "ja" => Self {
                onboarding_mode_prompt: include_str!("prompts/ja/onboarding_mode.txt"),
                ..common
            },
            "fr" => Self {
                onboarding_mode_prompt: include_str!("prompts/fr/onboarding_mode.txt"),
                ..common
            },
            _ => common,
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::{PromptTemplate, ShadowLocale, SystemPrompts};

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
        let result =
            PromptTemplate::new("Hello {name}, your {unknown} is safe.").render(&[("name", "Alice")]);
        assert_eq!(result, "Hello Alice, your {unknown} is safe.");
    }

    #[test]
    fn prompt_template_replaces_placeholder_appearing_multiple_times() {
        let result = PromptTemplate::new("{x} and {x} again").render(&[("x", "foo")]);
        assert_eq!(result, "foo and foo again");
    }

    #[test]
    fn prompt_template_renders_real_persona_prompt_variables() {
        let prompts = SystemPrompts::for_locale("en");
        let rendered = PromptTemplate::new(prompts.shadow_core_persona_prompt).render(&[
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
        assert_eq!(
            ShadowLocale::from_code("en").prompt_language_name(),
            "English"
        );
    }

    #[test]
    fn shadow_locale_from_ja_code_returns_japanese_language_name() {
        assert_eq!(
            ShadowLocale::from_code("ja").prompt_language_name(),
            "Japanese"
        );
    }

    #[test]
    fn shadow_locale_from_fr_code_returns_french_language_name() {
        assert_eq!(
            ShadowLocale::from_code("fr").prompt_language_name(),
            "French"
        );
    }

    #[test]
    fn shadow_locale_falls_back_to_english_for_unknown_code() {
        assert_eq!(
            ShadowLocale::from_code("de").prompt_language_name(),
            "English"
        );
        assert_eq!(
            ShadowLocale::from_code("").prompt_language_name(),
            "English"
        );
    }

    #[test]
    fn prompt_assets_are_non_empty() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(!prompts.profile_system_prompt.trim().is_empty());
        assert!(!prompts.preview_system_prompt.trim().is_empty());
        assert!(!prompts.chat_system_prompt.trim().is_empty());
        assert!(!prompts.onboarding_turn_two_system_prompt.trim().is_empty());
        assert!(!prompts.onboarding_turn_three_system_prompt.trim().is_empty());
        assert!(!prompts.shadow_core_persona_prompt.trim().is_empty());
        assert!(!prompts.onboarding_mode_prompt.trim().is_empty());
        assert!(!prompts.normal_chat_mode_prompt.trim().is_empty());
        assert!(!prompts.output_style_prompt.trim().is_empty());
    }

    #[test]
    fn profile_prompt_keeps_output_contract_private() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts
            .profile_system_prompt
            .contains("append the exact output contract separately"));
        assert!(!prompts.profile_system_prompt.contains("\"headline\""));
        assert!(!prompts
            .profile_system_prompt
            .contains("Return JSON only with this exact shape"));
    }
}
