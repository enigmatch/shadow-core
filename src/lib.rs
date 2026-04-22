mod prompt_inputs;
mod template;

pub use prompt_inputs::{
    PromptReadyPersona, PromptReadyProfile, PromptReadyReasoningPolicy, PromptReadySpeechStyle,
};
pub use template::PromptTemplate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalePhrases {
    pub soft_example_phrase: &'static str,
    pub soft_example_phrase_alt: &'static str,
    pub lazy_remark: &'static str,
    pub laugh_marker: &'static str,
    pub closing_insight_phrase: &'static str,
    pub closing_identity_phrase: &'static str,
}

impl LocalePhrases {
    pub fn for_locale(locale: &str) -> Self {
        match locale {
            "ja" => Self {
                soft_example_phrase: "「こういう感じかも」",
                soft_example_phrase_alt: "「たとえばこういうことかも」",
                lazy_remark: "また始まったよ...",
                laugh_marker: "「笑」",
                closing_insight_phrase: "「見えてきた」",
                closing_identity_phrase: "「ここから本当に Shadow になれる」",
            },
            _ => Self {
                soft_example_phrase: "\"something like this\"",
                soft_example_phrase_alt: "\"maybe it's more like this\"",
                lazy_remark: "\"Here we go again...\"",
                laugh_marker: "haha",
                closing_insight_phrase: "\"starting to take shape\"",
                closing_identity_phrase: "\"this is where it becomes real\"",
            },
        }
    }

    pub fn template_vars(&self) -> [(&'static str, &'static str); 6] {
        [
            ("soft_example_phrase", self.soft_example_phrase),
            ("soft_example_phrase_alt", self.soft_example_phrase_alt),
            ("lazy_remark", self.lazy_remark),
            ("laugh_marker", self.laugh_marker),
            ("closing_insight_phrase", self.closing_insight_phrase),
            ("closing_identity_phrase", self.closing_identity_phrase),
        ]
    }
}

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
                shadow_core_persona_prompt: include_str!("prompts/ja/shadow_core_persona.txt"),
                chat_system_prompt: include_str!("prompts/ja/chat_system_prompt.txt"),
                normal_chat_mode_prompt: include_str!("prompts/ja/normal_chat_mode.txt"),
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
    use super::{LocalePhrases, PromptTemplate, ShadowLocale, SystemPrompts};

    fn render_with_locale_phrases(template: &str, locale: &str) -> String {
        PromptTemplate::new(template).render(&LocalePhrases::for_locale(locale).template_vars())
    }

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
        assert!(!prompts
            .onboarding_turn_three_system_prompt
            .trim()
            .is_empty());
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

    #[test]
    fn english_prompt_assets_render_without_japanese_example_phrases() {
        let prompts = SystemPrompts::for_locale("en");

        let rendered_chat = render_with_locale_phrases(prompts.chat_system_prompt, "en");
        let rendered_persona = render_with_locale_phrases(prompts.shadow_core_persona_prompt, "en");
        let rendered_normal_chat =
            render_with_locale_phrases(prompts.normal_chat_mode_prompt, "en");
        let rendered_onboarding = render_with_locale_phrases(prompts.onboarding_mode_prompt, "en");

        for rendered in [
            rendered_chat,
            rendered_persona,
            rendered_normal_chat,
            rendered_onboarding,
        ] {
            assert!(!rendered.contains("また始まったよ"));
            assert!(!rendered.contains("こういう感じかも"));
            assert!(!rendered.contains("たとえばこういうことかも"));
            assert!(!rendered.contains("「笑」"));
            assert!(!rendered.contains("見えてきた"));
            assert!(!rendered.contains("ここから本当に Shadow になれる"));
        }
    }

    #[test]
    fn japanese_prompt_assets_render_with_japanese_example_phrases() {
        let prompts = SystemPrompts::for_locale("en");

        let rendered_chat = render_with_locale_phrases(prompts.chat_system_prompt, "ja");
        let rendered_persona = render_with_locale_phrases(prompts.shadow_core_persona_prompt, "ja");
        let rendered_normal_chat =
            render_with_locale_phrases(prompts.normal_chat_mode_prompt, "ja");
        let rendered_onboarding = render_with_locale_phrases(prompts.onboarding_mode_prompt, "ja");

        assert!(rendered_chat.contains("また始まったよ"));
        assert!(rendered_persona.contains("こういう感じかも"));
        assert!(rendered_persona.contains("たとえばこういうことかも"));
        assert!(rendered_persona.contains("「笑」"));
        assert!(rendered_normal_chat.contains("こういう感じかも"));
        assert!(rendered_onboarding.contains("見えてきた"));
        assert!(rendered_onboarding.contains("ここから本当に Shadow になれる"));
    }
}
