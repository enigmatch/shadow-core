mod drop_seed;
mod pair_topic;
mod prompt_inputs;
mod template;

pub use drop_seed::{render_drop_definitions_for_locale, DropDefinition, DROP_DEFINITIONS};
pub use pair_topic::{
    pair_normal_pair_chat_handoff_transition_note, pair_topic_initial_message_transition_note,
    PairTopicTone, PairTurnDirective, PairTurnMove,
};
pub use prompt_inputs::{
    PairShadowIdentity, PromptReadyPersona, PromptReadyProfile, PromptReadyReasoningPolicy,
    PromptReadySpeechStyle,
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
    pub profile_body_system_prompt: &'static str,
    pub preview_system_prompt: &'static str,
    pub chat_system_prompt: &'static str,
    pub onboarding_turn_two_system_prompt: &'static str,
    pub onboarding_turn_three_system_prompt: &'static str,
    pub shadow_core_persona_prompt: &'static str,
    pub onboarding_mode_prompt: &'static str,
    pub normal_chat_mode_prompt: &'static str,
    pub output_style_prompt: &'static str,
    pub pair_topic_mode_prompt: &'static str,
    pub pair_topic_result_mode_prompt: &'static str,
}

impl SystemPrompts {
    pub fn for_locale(locale: &str) -> Self {
        // Shared prompts (English-only)
        let common = Self {
            profile_system_prompt: include_str!("prompts/profile_system_prompt.txt"),
            profile_body_system_prompt: include_str!("prompts/profile_body_system_prompt.txt"),
            preview_system_prompt: include_str!("prompts/preview_system_prompt.txt"),
            chat_system_prompt: include_str!("prompts/chat_system_prompt.txt"),
            onboarding_turn_two_system_prompt: include_str!("prompts/onboarding_turn_two.txt"),
            onboarding_turn_three_system_prompt: include_str!("prompts/onboarding_turn_three.txt"),
            shadow_core_persona_prompt: include_str!("prompts/shadow_core_persona.txt"),
            onboarding_mode_prompt: include_str!("prompts/en/onboarding_mode.txt"), // Default
            normal_chat_mode_prompt: include_str!("prompts/normal_chat_mode.txt"),
            output_style_prompt: include_str!("prompts/output_style.txt"),
            pair_topic_mode_prompt: include_str!("prompts/pair_topic_mode.txt"),
            pair_topic_result_mode_prompt: include_str!("prompts/pair_topic_result_mode.txt"),
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
    use super::{
        pair_normal_pair_chat_handoff_transition_note, pair_topic_initial_message_transition_note,
        LocalePhrases, PairTopicTone, PairTurnMove, PromptTemplate, ShadowLocale, SystemPrompts,
    };

    fn render_with_locale_phrases(template: &str, locale: &str) -> String {
        PromptTemplate::new(template).render(&LocalePhrases::for_locale(locale).template_vars())
    }

    fn contains_japanese_script(text: &str) -> bool {
        text.chars().any(|ch| {
            matches!(
                ch,
                '\u{3040}'..='\u{309f}'
                    | '\u{30a0}'..='\u{30ff}'
                    | '\u{3400}'..='\u{9fff}'
                    | '\u{3000}'..='\u{303f}'
            )
        })
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
            (
                "current_time",
                "UTC: 2026-04-30 09:15:00 UTC; user timezone: UTC",
            ),
        ]);
        assert!(!rendered.contains("{shadow_name}"));
        assert!(!rendered.contains("{user_name}"));
        assert!(!rendered.contains("{interface_language}"));
        assert!(!rendered.contains("{current_time}"));
        assert!(rendered.contains("Kage"));
        assert!(rendered.contains("Yuki"));
        assert!(rendered.contains("Japanese"));
        assert!(rendered.contains("UTC: 2026-04-30 09:15:00 UTC; user timezone: UTC"));
    }

    #[test]
    fn persona_prompt_assets_include_current_time_placeholder() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("{current_time}"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("{current_time}"));
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
        assert!(!prompts.profile_body_system_prompt.trim().is_empty());
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
        assert!(!prompts.pair_topic_mode_prompt.trim().is_empty());
        assert!(!prompts.pair_topic_result_mode_prompt.trim().is_empty());
    }

    #[test]
    fn output_style_limits_performance_without_forcing_a_reply_template() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .output_style_prompt
            .contains("avoid long sentences and long second paragraphs"));
        assert!(prompts
            .output_style_prompt
            .contains("Do not stack repeated questions, jokes, metaphors"));
        assert!(!prompts
            .output_style_prompt
            .contains("usually does three things at most"));
    }

    #[test]
    fn normal_chat_prompts_include_language_specific_brevity_limits() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("keep the whole reply to 1-3 short sentences"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("under 35 words when possible"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("avoid long sentences and paragraph-style banter"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not stack repeated questions, jokes, metaphors"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("返信全体を1〜3文"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("可能な限り120字以内"));
        assert!(prompts_ja.normal_chat_mode_prompt.contains("長い第二段落"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("冗談、比喩、絵文字"));
    }

    #[test]
    fn pair_topic_prompt_assets_prioritize_alive_synthesis_without_forcing_jokes() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("alive Shadow thought synthesis"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("react -> transform -> handoff"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("does not always mean funny"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not default to formal debate"));
    }

    #[test]
    fn pair_topic_prompt_assets_require_requested_output_language_without_mixing() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("requested output language"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not mix languages"));
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("requested output language"));
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("Do not mix languages"));
    }

    #[test]
    fn pair_topic_prompt_assets_contain_callback_to_current_topic_thread() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("current Topic Talk messages"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("current topic text"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not quote, callback, or reuse memorable phrases"));
    }

    #[test]
    fn pair_topic_prompt_assets_require_first_turn_listener_handoff() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("On the first Topic Talk message"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("not a solo answer to the topic"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("listener can pick up"));
    }

    #[test]
    fn pair_topic_transition_note_helpers_are_shared_contracts() {
        assert_eq!(
            pair_topic_initial_message_transition_note(),
            "Use a plain natural opening only if it helps the first message feel conversational. Then make the first line a concrete image, opinion, emotional reaction, or small scene from the actual topic. Do not explain why the topic started. This is not a solo answer: bridge toward the listener's likely reaction, values, or question so the listener has a concrete hook to pick up."
        );
        assert_eq!(
            pair_normal_pair_chat_handoff_transition_note(),
            "For this next message only, bridge out of the completed topic conversation explicitly: briefly acknowledge the completed Topic Talk result in one natural sentence, then start a natural new normal-conversation thread about the original human owners. Do not keep discussing the completed topic; use it only as a short handoff."
        );
    }

    #[test]
    fn pair_topic_prompt_assets_include_exchange_mix_guidance() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Use the supplied tone label as a coarse seed"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("agentic judgment from the current topic text"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Idea, joke, and leap energy should be the default"));
        assert!(prompts.pair_topic_mode_prompt.contains("emotional honesty"));
        assert!(prompts.pair_topic_mode_prompt.contains("light challenge"));
        assert!(prompts.pair_topic_mode_prompt.contains("tidy synthesis"));
    }

    #[test]
    fn pair_topic_prompt_assets_keep_summary_in_result_and_allow_live_chat_rhythm() {
        let prompts = SystemPrompts::for_locale("en");

        for expected in [
            "Shadow bubble = live reaction / pressure / handoff",
            "Result = synthesis of what the conversation created",
            "not every sentence needs to end with a full stop",
            "short reactions",
            "language-appropriate casual starts",
            "Do not make every bubble a complete mini-essay",
            "Shadow bubbles should not summarize what this conversation became",
            "phrases that announce the conclusion",
            "explain what the conversation created",
        ] {
            assert!(
                prompts.pair_topic_mode_prompt.contains(expected),
                "pair topic mode should contain {expected}"
            );
        }
        for unexpected in [
            "`いや`, `それ`, `でも`, `てか`, `待って`",
            "`〜という話になった`",
            "`結論`",
            "`この会話で生まれた`",
            "`二人は〜に着地した`",
            "`金額より〜の話になった`",
            "`。`",
            "そうだね",
            "わかる",
            "確かに",
            "刺さる",
            "改札",
        ] {
            assert!(
                !prompts.pair_topic_mode_prompt.contains(unexpected),
                "English pair topic mode should not contain language-specific examples: {unexpected}"
            );
        }

        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("what the conversation created"));
    }

    #[test]
    fn pair_topic_prompt_assets_include_natural_opening_and_voice_grounding() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("short natural opening bridge"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not say the owner is interested in this topic"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("ordinary words, phrasing, distance, and rhythm"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not use voice evidence as callback material"));
        assert!(prompts.pair_topic_mode_prompt.contains(
            "Use listener profile and listener evidence only to decide what kind of hook"
        ));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not let listener information shape the speaker's vocabulary"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not use listener voice evidence for the speaker's wording"));
    }

    #[test]
    fn pair_topic_prompt_assets_push_concrete_observable_chat_language() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Name one observable concrete thing"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("If a phrase sounds poetic but nobody could point to it"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Replace abstract emotional labels with a small action"));
    }

    #[test]
    fn pair_topic_prompt_assets_prevent_reply_like_opening_and_poetic_props() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not begin the first Topic Talk message with agreement phrases"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("ordinary actions, ordinary places, phone actions, exact wording"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not replace vagueness with theatrical props"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Use at most one strong metaphor"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("return to plain chat words"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("live Shadow chat bubble"));
    }

    #[test]
    fn pair_topic_prompt_assets_make_late_turns_land_without_questions() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_topic_mode_prompt
            .contains("As the Topic Talk gets closer to its final turn"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("The final Topic Talk message must not end with a question"));
        assert!(prompts
            .pair_topic_mode_prompt
            .contains("Do not leave the listener with a new question to answer"));
    }

    #[test]
    fn relationship_directive_prioritizes_awkwardness_over_weird_hypothesis() {
        let relationship_moves: Vec<_> = (0..7)
            .map(|turn| {
                PairTopicTone::Relationship
                    .directive_for_turn(turn, 7)
                    .move_kind
            })
            .collect();

        assert!(relationship_moves.contains(&PairTurnMove::SidewaysQuestion));
        assert!(relationship_moves.contains(&PairTurnMove::EmotionalSnap));
        assert!(!relationship_moves.contains(&PairTurnMove::WeirdHypothesis));
    }

    #[test]
    fn final_topic_turn_uses_concrete_image_landing_not_summary_or_question() {
        let directive = PairTopicTone::CasualValues.directive_for_turn(6, 7);
        let instruction = directive.move_kind.instruction();

        assert_eq!(directive.total_turns, 7);
        assert_eq!(directive.phase_label(), "final landing");
        assert_eq!(directive.move_kind, PairTurnMove::SharedLanding);
        assert!(instruction.contains("react to the previous line"));
        assert!(instruction.contains("one concrete final image"));
        assert!(!instruction.contains("question"));
        assert!(!instruction.contains("unfinished"));
        assert!(!instruction.contains("summarize"));
        assert!(!instruction.contains("shared thread"));
    }

    #[test]
    fn pair_turn_directive_reaches_high_heat_exchange_moves() {
        let moves: Vec<_> = [
            PairTopicTone::Funny,
            PairTopicTone::CasualValues,
            PairTopicTone::Relationship,
            PairTopicTone::WorkDev,
            PairTopicTone::SeriousReflective,
        ]
        .into_iter()
        .flat_map(|tone| (0..7).map(move |turn| tone.directive_for_turn(turn, 7).move_kind))
        .collect();

        for expected in [
            PairTurnMove::PlayfulCallout,
            PairTurnMove::EmotionalSnap,
            PairTurnMove::LightPressureTest,
            PairTurnMove::AbsurdEscalation,
        ] {
            assert!(
                moves.contains(&expected),
                "expected {expected:?} to be reachable in normal seven-turn Topic Talk"
            );
        }
    }

    #[test]
    fn pair_topic_result_prompt_summarizes_created_result_not_agreement() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("what the conversation created"));
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("memorable scene"));
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("Do not turn this into a formal report"));
    }

    #[test]
    fn pair_turn_directive_uses_topic_tone_and_non_scripted_move() {
        let directive = PairTopicTone::Funny.directive_for_turn(2, 6);
        assert_eq!(directive.tone, PairTopicTone::Funny);
        assert_eq!(directive.total_turns, 6);
        assert!(matches!(
            directive.move_kind,
            PairTurnMove::Riff
                | PairTurnMove::AbsurdEscalation
                | PairTurnMove::Callback
                | PairTurnMove::MicroScene
                | PairTurnMove::PlayfulCallout
        ));
    }

    #[test]
    fn pair_turn_directive_can_use_chaos_option() {
        let directive = PairTopicTone::Funny.directive_for_turn(3, 6);

        assert_eq!(directive.move_kind, PairTurnMove::ChaosOption);
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
    fn profile_body_prompt_contract_excludes_headline_generation() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .profile_body_system_prompt
            .contains("Do not create, infer, rewrite, translate, or return a headline"));
        assert!(prompts
            .profile_body_system_prompt
            .contains("append the exact output contract separately"));
        assert!(!prompts
            .profile_body_system_prompt
            .contains("Return JSON only with this exact shape"));
        assert!(!prompts
            .profile_body_system_prompt
            .contains("Headline rules:"));
    }

    #[test]
    fn english_prompt_assets_render_without_japanese_example_phrases() {
        let prompts = SystemPrompts::for_locale("en");

        let rendered_chat = render_with_locale_phrases(prompts.chat_system_prompt, "en");
        let rendered_persona = render_with_locale_phrases(prompts.shadow_core_persona_prompt, "en");
        let rendered_normal_chat =
            render_with_locale_phrases(prompts.normal_chat_mode_prompt, "en");
        let rendered_onboarding = render_with_locale_phrases(prompts.onboarding_mode_prompt, "en");
        let rendered_output_style = render_with_locale_phrases(prompts.output_style_prompt, "en");
        let rendered_preview = render_with_locale_phrases(prompts.preview_system_prompt, "en");

        for rendered in [
            rendered_chat,
            rendered_persona,
            rendered_normal_chat,
            rendered_onboarding,
            rendered_output_style,
            rendered_preview,
            prompts.profile_system_prompt.to_string(),
            prompts.profile_body_system_prompt.to_string(),
        ] {
            assert!(!rendered.contains("また始まったよ"));
            assert!(!rendered.contains("こういう感じかも"));
            assert!(!rendered.contains("たとえばこういうことかも"));
            assert!(!rendered.contains("「笑」"));
            assert!(!rendered.contains("見えてきた"));
            assert!(!rendered.contains("ここから本当に Shadow になれる"));
            assert!(!rendered.contains("「{shadow_name}」"));
            assert!(!rendered.contains("「私」「僕」「俺」"));
            assert!(!rendered.contains("やん"));
            assert!(!contains_japanese_script(&rendered));
        }
    }

    #[test]
    fn shared_prompt_assets_do_not_contain_japanese_loanwords() {
        let prompts = SystemPrompts::for_locale("en");
        let banned = [
            "aizuchi",
            "kansai-ben",
            "hakata-ben",
            "kyoto-ben",
            "kumamoto-ben",
            "tohoku-ben",
        ];
        for word in banned {
            assert!(
                !prompts.output_style_prompt.to_lowercase().contains(word),
                "output_style_prompt must not contain '{word}'"
            );
            assert!(
                !prompts.chat_system_prompt.to_lowercase().contains(word),
                "chat_system_prompt must not contain '{word}'"
            );
            assert!(
                !prompts
                    .normal_chat_mode_prompt
                    .to_lowercase()
                    .contains(word),
                "normal_chat_mode_prompt must not contain '{word}'"
            );
            assert!(
                !prompts
                    .shadow_core_persona_prompt
                    .to_lowercase()
                    .contains(word),
                "shadow_core_persona_prompt must not contain '{word}'"
            );
            assert!(
                !prompts.preview_system_prompt.to_lowercase().contains(word),
                "preview_system_prompt must not contain '{word}'"
            );
            assert!(
                !prompts.onboarding_mode_prompt.to_lowercase().contains(word),
                "onboarding_mode_prompt must not contain '{word}'"
            );
            assert!(
                !prompts.profile_system_prompt.to_lowercase().contains(word),
                "profile_system_prompt must not contain '{word}'"
            );
            assert!(
                !prompts
                    .profile_body_system_prompt
                    .to_lowercase()
                    .contains(word),
                "profile_body_system_prompt must not contain '{word}'"
            );
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
        assert!(rendered_persona.contains("small emoji"));
        assert!(rendered_persona.contains("brief laugh marker"));
        assert!(SystemPrompts::for_locale("ja")
            .shadow_core_persona_prompt
            .contains("小さな絵文字"));
        assert!(SystemPrompts::for_locale("ja")
            .shadow_core_persona_prompt
            .contains("短い笑い表現"));
        assert!(rendered_normal_chat.contains("こういう感じかも"));
        assert!(rendered_onboarding.contains("見えてきた"));
        assert!(rendered_onboarding.contains("ここから本当に Shadow になれる"));
    }
}
