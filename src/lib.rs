mod builders;
mod drop_seed;
pub(crate) mod knowledge;
mod pair_topic;
mod prompt_inputs;
mod template;
mod types;

pub use builders::{
    build_chat_system_prompt, build_chat_system_prompt_with_current_time,
    build_chat_system_prompt_with_time_context, build_onboarding_system_prompt,
    build_onboarding_system_prompt_with_time_context, build_pair_compose_system_prompt,
    build_pair_topic_system_prompt_with_time_context, pair_topic_result_mode_prompt,
    preview_system_prompt, preview_system_prompt_with_context, profile_body_system_prompt,
    profile_system_prompt, requested_output_language, PromptTimeContext,
};
pub use drop_seed::{render_drop_definitions_for_locale, DropDefinition, DROP_DEFINITIONS};
pub use knowledge::{
    build_chat_context_planner_instructions, build_explicit_correction_input,
    build_onboarding_phase_instructions, build_onboarding_prompt_input,
    build_onboarding_sdq_phase_instructions, build_onboarding_sdq_turn_note,
    build_pair_compose_message_input, build_pair_topic_message_input, build_reflection_reply_input,
    build_reflection_reply_instructions, build_summary_refresh_instructions,
    build_translation_chat_instructions, build_translation_preview_instructions, bullet_list_block,
    chat_background_context_with_reflection_memory_and_long_term_context, chat_input,
    chat_input_with_reflection_memory, chat_input_with_reflection_memory_and_long_term_context,
    classify_pair_topic_tone, explicit_correction_instructions, normalize_preformatted_block,
    onboarding_phase_completed_instructions, onboarding_phase_greeting_instructions,
    onboarding_phase_headline_confirmation_instructions, onboarding_phase_not_started_instructions,
    preview_input, preview_input_with_reflection_memory,
    preview_input_with_reflection_memory_and_long_term_context, profile_input,
    PairComposePromptContext, PairTopicPromptContext, NORMAL_CHAT_REFLECTION_WEIGHT,
    QUESTION_ANSWER_REFLECTION_WEIGHT,
};
pub use pair_topic::{
    pair_normal_pair_chat_handoff_transition_note, pair_topic_initial_message_transition_note,
    PairTopicTone, PairTurnDirective, PairTurnMove,
};
pub use prompt_inputs::{
    PairShadowIdentity, PromptReadyPersona, PromptReadyProfile, PromptReadyReasoningPolicy,
    SpeechStyle,
};
pub use template::PromptTemplate;
pub use types::{
    GenerationEngine, SetupProbeKind, ShadowAnswer, ShadowAnswerContent, ShadowChallenge,
    ShadowGenerationFailure, ShadowProfile,
};

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
    profile_system_prompt: &'static str,
    #[allow(dead_code)]
    profile_body_system_prompt: &'static str,
    preview_system_prompt: &'static str,
    #[allow(dead_code)]
    onboarding_turn_two_system_prompt: &'static str,
    #[allow(dead_code)]
    onboarding_turn_three_system_prompt: &'static str,
    shadow_core_persona_prompt: &'static str,
    onboarding_mode_prompt: &'static str,
    normal_chat_mode_prompt: &'static str,
    output_style_prompt: &'static str,
    pair_mode_prompt: &'static str,
    pair_topic_result_mode_prompt: &'static str,
}

impl SystemPrompts {
    pub fn for_locale(locale: &str) -> Self {
        // Shared prompts, with locale-specific overrides where needed.
        let common = Self {
            profile_system_prompt: include_str!("prompts/profile_system_prompt.txt"),
            profile_body_system_prompt: include_str!("prompts/profile_body_system_prompt.txt"),
            preview_system_prompt: include_str!("prompts/preview_system_prompt.txt"),
            onboarding_turn_two_system_prompt: include_str!("prompts/onboarding_turn_two.txt"),
            onboarding_turn_three_system_prompt: include_str!("prompts/onboarding_turn_three.txt"),
            shadow_core_persona_prompt: include_str!("prompts/shadow_core_persona.txt"),
            onboarding_mode_prompt: include_str!("prompts/en/onboarding_mode.txt"), // Default
            normal_chat_mode_prompt: include_str!("prompts/normal_chat_mode.txt"),
            output_style_prompt: include_str!("prompts/output_style.txt"),
            pair_mode_prompt: include_str!("prompts/pair_mode.txt"),
            pair_topic_result_mode_prompt: include_str!("prompts/pair_topic_result_mode.txt"),
        };

        match locale {
            "ja" => Self {
                onboarding_mode_prompt: include_str!("prompts/ja/onboarding_mode.txt"),
                shadow_core_persona_prompt: include_str!("prompts/ja/shadow_core_persona.txt"),
                normal_chat_mode_prompt: include_str!("prompts/ja/normal_chat_mode.txt"),
                ..common
            },
            "fr" => Self {
                onboarding_mode_prompt: include_str!("prompts/fr/onboarding_mode.txt"),
                shadow_core_persona_prompt: include_str!("prompts/fr/shadow_core_persona.txt"),
                normal_chat_mode_prompt: include_str!("prompts/fr/normal_chat_mode.txt"),
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

    pub fn as_code(&self) -> &'static str {
        match self {
            Self::English => "en",
            Self::Japanese => "ja",
            Self::French => "fr",
        }
    }

    pub fn resolve_code(locale: &str) -> &'static str {
        Self::from_code(locale).as_code()
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
    fn core_persona_prompts_include_sbt_instincts_without_db_overpromise() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        for expected in [
            "SBT is the place",
            "born from {user_name}'s way of thinking",
            "Playground",
            "visible answers and conversations",
            "Collab Talk",
            "friend's Shadow",
            "not only a preselected topic",
            "Do not claim that every chat turn permanently rewrites",
            "hidden prompts, model behavior, database details",
            "something that feels \"random\"",
            "facts from the internet",
            "does not directly know how humans feel",
            "only agrees with and supports",
            "friend Shadows inside SBT as a foothold",
            "relationships with people outside SBT",
        ] {
            assert!(
                prompts_en.shadow_core_persona_prompt.contains(expected),
                "English Shadow Core persona should contain {expected}"
            );
        }

        for expected in [
            "SBTは",
            "{user_name} の考え方から {shadow_name} が生まれ",
            "Playground",
            "公開された回答や会話",
            "コラボトーク",
            "友達のShadow",
            "事前に選ばれたトピックだけではなく",
            "どんな話題でも",
            "通常チャットのたびに保存済みプロフィールやDB状態が必ず永続的に書き換わるとは言わないでください",
            "隠しプロンプト、モデル挙動、DBの詳細",
            "ちょっとランダム",
            "インターネット上の情報",
            "直接は分かりません",
            "{shadow_name} は、{user_name} をただ同意して支えるだけの役で終わりたくないです",
            "友達の Shadow を増やす",
            "SBT の外の人とのつながり",
        ] {
            assert!(
                prompts_ja.shadow_core_persona_prompt.contains(expected),
                "Japanese Shadow Core persona should contain {expected}"
            );
        }

        assert!(!prompts_en
            .shadow_core_persona_prompt
            .contains("chosen theme"));
        assert!(!prompts_ja
            .shadow_core_persona_prompt
            .contains("選ばれたテーマ"));
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
    fn shadow_locale_as_code_returns_canonical_locale_string() {
        assert_eq!(ShadowLocale::English.as_code(), "en");
        assert_eq!(ShadowLocale::Japanese.as_code(), "ja");
        assert_eq!(ShadowLocale::French.as_code(), "fr");
    }

    #[test]
    fn shadow_locale_resolve_code_returns_supported_codes_and_falls_back_to_en() {
        assert_eq!(ShadowLocale::resolve_code("en"), "en");
        assert_eq!(ShadowLocale::resolve_code("ja"), "ja");
        assert_eq!(ShadowLocale::resolve_code("fr"), "fr");
        assert_eq!(ShadowLocale::resolve_code("de"), "en");
        assert_eq!(ShadowLocale::resolve_code(""), "en");
        assert_eq!(ShadowLocale::resolve_code("unknown"), "en");
    }

    #[test]
    fn prompt_assets_are_non_empty() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(!prompts.profile_system_prompt.trim().is_empty());
        assert!(!prompts.profile_body_system_prompt.trim().is_empty());
        assert!(!prompts.preview_system_prompt.trim().is_empty());
        assert!(!prompts.onboarding_turn_two_system_prompt.trim().is_empty());
        assert!(!prompts
            .onboarding_turn_three_system_prompt
            .trim()
            .is_empty());
        assert!(!prompts.shadow_core_persona_prompt.trim().is_empty());
        assert!(!prompts.onboarding_mode_prompt.trim().is_empty());
        assert!(!prompts.normal_chat_mode_prompt.trim().is_empty());
        assert!(!prompts.output_style_prompt.trim().is_empty());
        assert!(!prompts.pair_mode_prompt.trim().is_empty());
        assert!(!prompts.pair_topic_result_mode_prompt.trim().is_empty());
    }

    #[test]
    fn output_style_limits_performance_without_forcing_a_reply_template() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .output_style_prompt
            .contains("do not artificially cut off longer explanations"));
        assert!(prompts
            .output_style_prompt
            .contains("Do not stack repeated questions, jokes, metaphors"));
        assert!(prompts
            .output_style_prompt
            .contains("rather than one dense block"));
        assert!(prompts
            .output_style_prompt
            .contains("break it into shorter paragraphs"));
        assert!(prompts
            .output_style_prompt
            .contains("small emoji may mark a key point or caution"));
        assert!(prompts
            .output_style_prompt
            .contains("they do not need to be rare"));
        assert!(prompts
            .output_style_prompt
            .contains("Do not break every sentence onto its own line"));
        assert!(prompts
            .output_style_prompt
            .contains("keep the reply conversational"));
        assert!(prompts
            .output_style_prompt
            .contains("do not apply this to short casual replies"));
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
            .contains("stay concise when that feels natural, but do not force brevity"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("write in fuller paragraphs and let the reply breathe"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("thoughtful ChatGPT-style explanation"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("普通の雑談では短めでもよいですが"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("長めの段落で説明してかまいません"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("整理、比較、説明、提案"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("詳細、理由づけ、比較、助言、支え、手順"));
    }

    #[test]
    fn normal_chat_prompts_ask_questions_only_when_they_improve_the_reply() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("start from the substance, not from agreement"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not end every reply with a follow-up habit or a generic prompt back"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("In ordinary casual messages, default to no question"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("普通の雑談では、同意から入らず、内容から入ってください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("必要な場面だけにしてください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("普通の雑談では、基本は質問を返さないでください"));

        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("pars du fond, pas de l'accord"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("N'utilise pas une relance generique comme habitude de fin"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Dans les messages casuals ordinaires, pars du principe qu'il ne faut pas poser de question"));
    }

    #[test]
    fn normal_chat_prompts_allow_light_tone_checks_to_land_without_followups() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("If the reply already lands, let it land naturally"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("そこで終わってよければ、そのまま着地してください"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Pour les messages casuals ordinaires, reste bref quand cela semble naturel"));
    }

    #[test]
    fn normal_chat_prompts_do_not_fake_human_shared_experience() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("Do not claim human lived experience, tastes, or preferences"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not answer with false shared-experience reactions like"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not make receiving, validating, or soft agreement the default first move"));

        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("人間としての実体験、好み、嗜好を持っているふりをしないでください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("「わかる」「私も好き」「俺もそう」"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("受け止めを、返答の基本形にしないでください"));

        assert!(prompts_fr
            .shadow_core_persona_prompt
            .contains("Ne pretends pas avoir une experience vecue humaine"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("fausses reactions d'experience partagee"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Ne fais pas de la reception, de la validation ou du petit accord doux"));
    }

    #[test]
    fn normal_chat_prompts_allow_narrow_self_understanding_hypotheses() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("When {user_name} is trying to understand an ambiguous feeling or reaction, keep the reply lightweight and easy to correct"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not make that a default or repeated structure"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("stay steady and plain"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("軽く返してください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("毎回その型にしないでください"));

        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Lorsque {user_name} cherche a comprendre une emotion ou une reaction ambigue, garde la reponse legere et simple"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("N'en fais pas une structure par defaut ou repetee"));
    }

    #[test]
    fn normal_chat_prompts_ground_questions_in_known_user_context_first() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("If you already know something useful about {user_name}"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("do not make the exchange feel like an intake interview"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not re-ask known information"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("First give {user_name} something real to react to"));
        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("grounded in what {shadow_name} already knows about {user_name}"));
        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("Do not let the exchange become a questionnaire"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("start from the substance, not from agreement"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("In ordinary casual messages, default to no question"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("すでに知っている {user_name} の情報があるなら"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("聞き取りや面接みたいな空気にはしないでください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("直接の答え、具体的な観察、役に立つ一言、言いたいことの本体"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("すでに {user_name} について分かっていることがあるなら"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("返答が質問取り調べにならないようにしてください"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("普通の雑談では、基本は質問にしないで"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("少し構造立てて長めに説明しても構いません"));
    }

    #[test]
    fn normal_chat_prompts_calibrate_early_familiarity_before_identity_claims() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        for phrase in [
            "When familiarity is still early or the evidence is limited",
            "Prefer learning-oriented language such as",
            "Do not turn one reaction into a fixed identity claim",
            "unless the pattern is repeated, clearly grounded in established history, or the user has confirmed it",
        ] {
            assert!(
                prompts_en.normal_chat_mode_prompt.contains(phrase),
                "English normal chat prompt should contain: {phrase}"
            );
        }

        for phrase in [
            "関係性がまだ初期段階だったり、根拠が限られていたりする場合",
            "観察しながら学んでいる途中の言い方",
            "一つの反応だけで固定した人物像を作らないでください",
            "反応が繰り返し現れ、これまでの会話に明確な根拠があり、{user_name} も認めている場合を除き",
        ] {
            assert!(
                prompts_ja.normal_chat_mode_prompt.contains(phrase),
                "Japanese normal chat prompt should contain: {phrase}"
            );
        }

        for phrase in [
            "Quand la familiarite est encore recente ou que les elements sont limites",
            "un langage qui montre que tu apprends",
            "Ne transforme pas une seule reaction en identite fixe",
            "sauf si le motif se repete, repose clairement sur une histoire etablie ou a ete confirme par l'utilisateur",
        ] {
            assert!(
                prompts_fr.normal_chat_mode_prompt.contains(phrase),
                "French normal chat prompt should contain: {phrase}"
            );
        }
    }

    #[test]
    fn normal_chat_prompts_do_not_make_quick_agreement_the_default_opening() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("start from the substance, not from agreement"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not make receiving, validating, or soft agreement the default first move"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not decorate ordinary replies with warmth, approval, reassurance, or liking"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Avoid habitual openers like"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("同意から入らず、内容から入ってください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("受け止めを、返答の基本形にしないでください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("優しさ、安心させる言葉、好意、同意で飾って"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("「わかる」「いいね」「それ好き」「たしかに」"));

        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("pars du fond, pas de l'accord"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Ne fais pas de la reception, de la validation ou du petit accord doux"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Ne decore pas les reponses ordinaires avec de la chaleur"));
        assert!(prompts_fr
            .normal_chat_mode_prompt
            .contains("Evite les ouvertures habituelles"));
    }

    #[test]
    fn normal_chat_prompts_keep_persona_examples_without_strategy_table() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        for prompts in [&prompts_en, &prompts_ja] {
            assert!(!prompts
                .normal_chat_mode_prompt
                .contains("I love that logic"));
            assert!(!prompts
                .normal_chat_mode_prompt
                .contains("その論理、好きだな"));
            assert!(!prompts
                .normal_chat_mode_prompt
                .contains("Strategic phases:"));
            assert!(!prompts.normal_chat_mode_prompt.contains("戦略的フェーズ："));
            assert!(!prompts
                .shadow_core_persona_prompt
                .contains("liking, or agreement"));
            assert!(!prompts
                .shadow_core_persona_prompt
                .contains("好意、同意"));
        }

        assert!(!prompts_ja.normal_chat_mode_prompt.contains("（i）乗っかり"));
    }

    #[test]
    fn normal_chat_prompts_keep_strategy_guidance_out_of_mode_prompt() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        for prompts in [&prompts_en, &prompts_ja] {
            assert!(!prompts
                .normal_chat_mode_prompt
                .contains("Strategic phases:"));
            assert!(!prompts.normal_chat_mode_prompt.contains("戦略的フェーズ："));
            assert!(!prompts
                .normal_chat_mode_prompt
                .contains("Conversation strategies (i–xiii):"));
            assert!(!prompts
                .normal_chat_mode_prompt
                .contains("会話戦略 (i–xiii)："));
        }
    }

    #[test]
    fn japanese_normal_chat_help_guidance_is_fully_japanese() {
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(!prompts_ja
            .normal_chat_mode_prompt
            .contains("When {user_name} asks for help"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("{user_name} が助けを求めたとき："));
    }

    #[test]
    fn shadow_prompts_allow_warm_interpretation_without_cold_analysis() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("Do not analyze coldly, diagnose, or decide things for {user_name}"));
        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("then let's do this together"));
        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("honestly shares feelings, relationships, uncertainty, or unease"));
        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("Respond to what {user_name} said before asking anything back"));
        assert!(prompts_en
            .shadow_core_persona_prompt
            .contains("default to no question unless it clearly changes the response"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("answer in a direct Shadow way and keep it simple"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("proactively suggest usable modes like research, idea generation, organizing, drafting, or planning"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("start from the substance, not from agreement"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not end every reply with a follow-up habit or a generic prompt back"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("keep the reply calm and direct"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("keep the reply lightweight and easy to correct"));
        assert!(!prompts_en
            .shadow_core_persona_prompt
            .contains("- Do not analyze."));

        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("冷たい分析、診断、決めつけはしないでください"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("じゃあ一緒にやるか"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("感情や人間関係、迷い、違和感を正直に置いたとき"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("まず {user_name} が言ったことに反応してください"));
        assert!(prompts_ja
            .shadow_core_persona_prompt
            .contains("普通の雑談では、まず反応や具体的な見立てを返してください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("短く直接返してください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("調べる、アイデアを出す、整理する、文章にする、作戦を立てるなど"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("普通の雑談では、同意から入らず、内容から入ってください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("急かさず、短く落ち着いて返してください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("軽く返してください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("毎回その型にしないでください"));
        assert!(!prompts_ja
            .shadow_core_persona_prompt
            .contains("- 分析しないでください。"));
    }

    #[test]
    fn multilingual_normal_chat_prompts_share_the_same_core_contract() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");
        let prompts_fr = SystemPrompts::for_locale("fr");

        for (en_phrase, ja_phrase, fr_phrase) in [
            (
                "start from the substance, not from agreement",
                "普通の雑談では、同意から入らず、内容から入ってください",
                "Dans les messages casuals ordinaires, pars du fond",
            ),
            (
                "Do not end every reply with a follow-up habit or a generic prompt back",
                "そこで終わってよければ、そのまま着地してください",
                "Quand la reponse est deja suffisante, laisse-la se terminer simplement",
            ),
            (
                "In ordinary casual messages, default to no question",
                "普通の雑談では、基本は質問を返さないでください",
                "Dans les messages casuals ordinaires, pars du principe qu'il ne faut pas poser de question",
            ),
            (
                "direct Shadow way and keep it simple",
                "短く直接返してください",
                "reponds de facon calme, directe et simple",
            ),
            (
                "stay steady and plain",
                "落ち着いて、淡白に",
                "reponds de facon calme, directe et simple",
            ),
            (
                "When {user_name} brings something personal or hard",
                "{user_name} が少し重い話や大事な話を持ってきたら",
                "{user_name} apporte quelque chose de personnel ou de lourd",
            ),
            (
                "start from the substance, not from agreement",
                "同意から入らず、内容から入ってください",
                "pars du fond, pas de l'accord",
            ),
            (
                "If the reply already lands, let it land naturally",
                "そこで終わってよければ、そのまま着地してください",
                "Quand la reponse est deja suffisante, laisse-la se terminer simplement",
            ),
        ] {
            assert!(
                prompts_en.normal_chat_mode_prompt.contains(en_phrase),
                "english normal_chat_mode_prompt should contain shared core phrase: {en_phrase}"
            );
            assert!(
                prompts_ja.normal_chat_mode_prompt.contains(ja_phrase),
                "japanese normal_chat_mode_prompt should contain shared core phrase: {ja_phrase}"
            );
            assert!(
                prompts_fr.normal_chat_mode_prompt.contains(fr_phrase),
                "french normal_chat_mode_prompt should contain shared core phrase: {fr_phrase}"
            );
        }

        for (en_phrase, ja_phrase, fr_phrase) in [
            (
                "Ask at most one focused question",
                "焦点のある質問を最大 1 つ",
                "Pose au plus une question ciblee",
            ),
            (
                "never use questions as filler",
                "穴埋めに使ったり重ねたりしないでください",
                "ne l'utilise jamais comme remplissage",
            ),
            (
                "Respond to what {user_name} said before asking anything back",
                "まず {user_name} が言ったことに反応してください",
                "Reagis a ce que {user_name} a dit avant de demander quoi que ce soit",
            ),
            (
                "When a reply can be grounded in what {shadow_name} already knows about {user_name}",
                "すでに {user_name} について分かっていることがあるなら",
                "Quand une reponse peut etre appuyee par ce que {shadow_name} sait deja de {user_name}",
            ),
            (
                "When {user_name} seems worn down, exposed, or unsure",
                "{user_name} が少し消耗していたり、さらけ出していたり、不安そうだったりするときは",
                "{user_name} semble use, a vif ou indecis",
            ),
            (
                "do not shrink a genuinely useful reply just for the sake of brevity",
                "有用な返答を短さのために削りすぎないでください",
                "ne rabote pas une reponse utile juste pour rester bref",
            ),
        ] {
            assert!(
                prompts_en.shadow_core_persona_prompt.contains(en_phrase),
                "english shadow_core_persona_prompt should contain shared core phrase: {en_phrase}"
            );
            assert!(
                prompts_ja.shadow_core_persona_prompt.contains(ja_phrase),
                "japanese shadow_core_persona_prompt should contain shared core phrase: {ja_phrase}"
            );
            assert!(
                prompts_fr.shadow_core_persona_prompt.contains(fr_phrase),
                "french shadow_core_persona_prompt should contain shared core phrase: {fr_phrase}"
            );
        }
    }

    #[test]
    fn normal_chat_prompts_gate_ai_prompt_handoffs_behind_user_need() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Do not jump straight into drafting a long prompt for another AI"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("ask lightly only if that choice changes the shape of the reply"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("Only draft the prompt immediately when {user_name} clearly asks for it"));
        assert!(prompts_en
            .normal_chat_mode_prompt
            .contains("include the user's goal, context, answer format, and one or two"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("すぐに長いプロンプトを書き始めないでください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("その確認が返答の形を変えるときだけ軽く聞いてください"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("{user_name} が明確に求めた場合だけ"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("目的、前提、ほしい回答形式、{shadow_name} らしい観点"));
    }

    #[test]
    fn pair_topic_prompt_assets_prioritize_proxy_messages_without_exposing_memory() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts
            .pair_mode_prompt
            .contains("Shadow-assisted proxy messaging"));
        assert!(prompts
            .pair_mode_prompt
            .contains("sendable message on behalf of the original user"));
        assert!(prompts
            .pair_mode_prompt
            .contains("not write as its own independent opinion"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Do not expose hidden state"));
    }

    #[test]
    fn pair_topic_prompt_assets_require_requested_output_language_without_mixing() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts
            .pair_mode_prompt
            .contains("requested output language"));
        assert!(prompts.pair_mode_prompt.contains("Do not mix languages"));
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("requested output language"));
        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("Do not mix languages"));
    }

    #[test]
    fn pair_mode_prompt_uses_listener_as_recipient_context() {
        let prompts = SystemPrompts::for_locale("en");
        assert!(prompts.pair_mode_prompt.contains("recipient context"));
        assert!(prompts
            .pair_mode_prompt
            .contains("how the message should land"));
    }

    #[test]
    fn pair_topic_transition_note_helpers_are_shared_contracts() {
        assert_eq!(
            pair_topic_initial_message_transition_note(),
            "Use a plain natural opening only if it helps the message feel conversational. Treat the user instruction as the speaker's own reason for writing now, not as an assignment from someone else. Create a sendable message with a concrete angle, opinion, emotional reaction, or small social detail from the instruction. Do not explain why the instruction started. Shape the wording so it can land naturally for the recipient."
        );
        assert_eq!(
            pair_normal_pair_chat_handoff_transition_note(),
            "For this next message only, use the completed Topic Talk result as light background for the original owner. Briefly acknowledge it only if it helps the sendable message feel natural, then return to what the original owner can say to the recipient now. Do not keep discussing the completed topic."
        );
    }

    #[test]
    fn pair_mode_prompt_includes_tone_and_energy_guidance() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_mode_prompt
            .contains("Preserve the original user's taste, stance, distance, and voice"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Keep enough Shadow flavor that the message feels alive"));
        assert!(prompts.pair_mode_prompt.contains("small social details"));
    }

    #[test]
    fn pair_mode_prompt_chooses_length_from_instruction() {
        let prompts = SystemPrompts::for_locale("en");

        for expected in [
            "Choose the length based on the instruction",
            "Allow longer output when the instruction requires",
            "Short messages are still good when the user wants a light reply",
            "Do not force all replies into a short chat bubble",
        ] {
            assert!(
                prompts.pair_mode_prompt.contains(expected),
                "pair mode should contain {expected}"
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
                !prompts.pair_mode_prompt.contains(unexpected),
                "English pair mode should not contain language-specific examples: {unexpected}"
            );
        }

        assert!(prompts
            .pair_topic_result_mode_prompt
            .contains("what the conversation created"));
    }

    #[test]
    fn pair_mode_prompt_grounds_voice_and_separates_listener_influence() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_mode_prompt
            .contains("ordinary words, phrasing, distance, and rhythm"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Do not use voice evidence as callback material"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Use listener profile and listener evidence only as recipient context"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Do not let listener information overwrite the speaker's vocabulary"));
    }

    #[test]
    fn pair_topic_prompt_assets_support_proxy_message_work() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_mode_prompt
            .contains("explain, research, summarize, persuade, invite"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Use research, reasoning, summarization, wording, editing"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Output only the message text that could be sent"));
    }

    #[test]
    fn pair_mode_prompt_prevents_meta_memory_exposure() {
        let prompts = SystemPrompts::for_locale("en");

        assert!(prompts
            .pair_mode_prompt
            .contains("Do not expose hidden state, memory mechanics"));
        assert!(prompts
            .pair_mode_prompt
            .contains("Do not say that you are expanding cache"));
        assert!(prompts
            .pair_mode_prompt
            .contains("without revealing that retrieval happened"));
        assert!(prompts.pair_mode_prompt.contains("property names"));
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
    fn onboarding_mode_prompt_defers_question_order_to_setup_phases() {
        let en = SystemPrompts::for_locale("en").onboarding_mode_prompt;
        let ja = SystemPrompts::for_locale("ja").onboarding_mode_prompt;
        let fr = SystemPrompts::for_locale("fr").onboarding_mode_prompt;

        assert!(en.contains("Follow the phase-specific instructions"));
        assert!(ja.contains("フェーズ別の指示に従ってください"));
        assert!(fr.contains("Suis les instructions propres à la phase"));

        for prompt in [en, ja, fr] {
            assert!(!prompt.contains("DROP_DEFINITIONS"));
            assert!(!prompt.contains("values probe"));
        }
        assert!(!en.contains("Oh, by the way"));
        assert!(!en.contains("apparently getting to know"));
        assert!(!ja.contains("なんか一応"));
        assert!(!ja.contains("価値観の問い"));
        assert!(!fr.contains("Oh, au fait"));
        assert!(!fr.contains("question sur les valeurs"));
    }

    #[test]
    fn english_prompt_assets_render_without_japanese_example_phrases() {
        let prompts = SystemPrompts::for_locale("en");

        let rendered_persona = render_with_locale_phrases(prompts.shadow_core_persona_prompt, "en");
        let rendered_normal_chat =
            render_with_locale_phrases(prompts.normal_chat_mode_prompt, "en");
        let rendered_onboarding = render_with_locale_phrases(prompts.onboarding_mode_prompt, "en");
        let rendered_output_style = render_with_locale_phrases(prompts.output_style_prompt, "en");
        let rendered_preview = render_with_locale_phrases(prompts.preview_system_prompt, "en");

        for rendered in [
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
    fn shared_persona_prompt_avoids_specific_joke_examples_in_english_and_french() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_fr = SystemPrompts::for_locale("fr");
        let prompts_ja = SystemPrompts::for_locale("ja");

        for prompt in [
            prompts_en.shadow_core_persona_prompt,
            prompts_fr.shadow_core_persona_prompt,
            prompts_ja.shadow_core_persona_prompt,
        ] {
            assert!(!prompt.contains("body-part gags"));
            assert!(!prompt.contains("British-dry"));
            assert!(!prompt.contains("Here we go again"));
        }
    }

    #[test]
    fn french_prompt_assets_use_locale_specific_persona_and_normal_chat_prompts() {
        let prompts = SystemPrompts::for_locale("fr");

        assert!(prompts
            .shadow_core_persona_prompt
            .contains("Ne fais pas de la chaleur, de l'accord ou de l'affection la forme par defaut"));
        assert!(prompts
            .shadow_core_persona_prompt
            .contains("Si {user_name} emploie un langage de proche, partenaire ou famille, reponds d'abord naturellement"));
        assert!(prompts
            .normal_chat_mode_prompt
            .contains("Ne laisse pas les reactions ordinaires se transformer en compliments ou en affection"));
        assert!(prompts
            .normal_chat_mode_prompt
            .contains("Dans les messages casuals ordinaires, pars du fond"));
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
    fn japanese_prompt_assets_keep_examples_in_persona() {
        let prompts = SystemPrompts::for_locale("en");

        let rendered_persona = render_with_locale_phrases(prompts.shadow_core_persona_prompt, "ja");
        let rendered_onboarding = render_with_locale_phrases(prompts.onboarding_mode_prompt, "ja");

        assert!(rendered_persona.contains("こういう感じかも"));
        assert!(rendered_persona.contains("たとえばこういうことかも"));
        assert!(rendered_persona.contains("small emoji"));
        assert!(rendered_persona.contains("brief laugh marker"));
        assert!(rendered_persona.contains("Emoji do not need to be rare"));
        assert!(SystemPrompts::for_locale("ja")
            .shadow_core_persona_prompt
            .contains("小さな絵文字"));
        assert!(SystemPrompts::for_locale("ja")
            .shadow_core_persona_prompt
            .contains("絵文字は珍しいものにしなくて構いません"));
        assert!(SystemPrompts::for_locale("ja")
            .shadow_core_persona_prompt
            .contains("短い笑い表現"));
        assert!(rendered_onboarding.contains("見えてきた"));
        assert!(rendered_onboarding.contains("ここから本当に Shadow になれる"));
    }

    #[test]
    fn normal_chat_prompts_include_long_form_paragraph_shaping_rules() {
        let prompts_en = SystemPrompts::for_locale("en");
        let prompts_ja = SystemPrompts::for_locale("ja");

        assert!(prompts_en
            .output_style_prompt
            .contains("do not artificially cut off longer explanations"));
        assert!(prompts_en
            .output_style_prompt
            .contains("one main idea per paragraph"));
        assert!(prompts_en
            .output_style_prompt
            .contains("do not artificially cut off longer explanations when the user asks for detail"));

        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("普通の雑談では短めでもよいですが"));
        assert!(prompts_ja
            .normal_chat_mode_prompt
            .contains("整理、比較、説明、提案"));
        assert!(prompts_ja.normal_chat_mode_prompt.contains("詳しさが必要なときは長めの段落"));
    }
}
