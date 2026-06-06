//! Shared system prompt builders.
//!
//! **Ownership boundary:** This module owns prompt builders that have no dependency on
//! application-layer code (database models, OpenAI schemas, service state machines, or
//! app-specific persistence). These builders compose locale-aware system prompts from
//! `SystemPrompts`, `LocalePhrases`, `ShadowLocale`, and `PromptTemplate` — all of which
//! live in shadow-core — and are safe to reuse across future client crates.
//!
//! Prompt builders that depend on database models, app-specific constants (e.g. reflection
//! weights), onboarding state machines, or provider schemas belong in the application crate
//! under `src/data/shadow_prompts/`.

use chrono::{DateTime, Utc};

use crate::{LocalePhrases, PromptTemplate, ShadowLocale, SystemPrompts};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptTimeContext {
    current_time: String,
    now: Option<DateTime<Utc>>,
}

impl PromptTimeContext {
    pub fn new(current_time: impl Into<String>) -> Self {
        Self {
            current_time: current_time.into(),
            now: None,
        }
    }

    fn with_datetime(current_time: impl Into<String>, now: DateTime<Utc>) -> Self {
        Self {
            current_time: current_time.into(),
            now: Some(now),
        }
    }

    pub fn now_utc() -> Self {
        Self::from_utc_datetime(Utc::now())
    }

    pub fn from_utc_datetime(now: DateTime<Utc>) -> Self {
        Self::with_datetime(
            format!(
                "UTC: {}; user timezone: UTC",
                now.format("%Y-%m-%d %H:%M:%S UTC")
            ),
            now,
        )
    }

    pub fn now_for_timezone(time_zone: &str) -> Self {
        Self::from_utc_datetime_and_timezone(Utc::now(), time_zone)
    }

    pub fn from_utc_datetime_and_timezone(now: DateTime<Utc>, time_zone: &str) -> Self {
        let trimmed_time_zone = time_zone.trim();
        let Ok(parsed_time_zone) = trimmed_time_zone.parse::<chrono_tz::Tz>() else {
            return Self::from_utc_datetime(now);
        };
        if parsed_time_zone == chrono_tz::UTC {
            return Self::from_utc_datetime(now);
        }

        let local_time = now.with_timezone(&parsed_time_zone);
        Self::with_datetime(
            format!(
                "UTC: {}\nUser local time: {}\nUser timezone: {}",
                now.format("%Y-%m-%d %H:%M:%S UTC"),
                local_time.format("%Y-%m-%d %H:%M:%S %Z"),
                trimmed_time_zone
            ),
            now,
        )
    }

    pub fn with_last_interaction_at(mut self, last_interaction_at: Option<DateTime<Utc>>) -> Self {
        if let (Some(now), Some(last)) = (self.now, last_interaction_at) {
            let elapsed = now.signed_duration_since(last);
            self.current_time = format!(
                "{}\n{}",
                self.current_time,
                format_elapsed_time_note(elapsed)
            );
        }
        self
    }

    pub fn current_time(&self) -> &str {
        &self.current_time
    }
}

fn format_elapsed_time_note(elapsed: chrono::Duration) -> String {
    let total_seconds = elapsed.num_seconds().max(0);
    let hours = total_seconds / 3600;
    let days = hours / 24;

    if days >= 2 {
        format!("Time since last interaction: {days} days")
    } else if days == 1 {
        "Time since last interaction: 1 day".to_string()
    } else if hours >= 2 {
        format!("Time since last interaction: {hours} hours")
    } else if hours == 1 {
        "Time since last interaction: 1 hour".to_string()
    } else {
        "Time since last interaction: less than an hour".to_string()
    }
}

pub fn requested_output_language(locale: &str) -> &'static str {
    ShadowLocale::from_code(ShadowLocale::resolve_code(locale)).prompt_language_name()
}

pub fn profile_system_prompt(locale: &str) -> &'static str {
    SystemPrompts::for_locale(locale).profile_system_prompt
}

pub fn pair_topic_result_mode_prompt(locale: &str) -> &'static str {
    SystemPrompts::for_locale(locale).pair_topic_result_mode_prompt
}


pub fn build_chat_system_prompt(shadow_name: &str, user_name: &str, locale: &str) -> String {
    build_chat_system_prompt_with_time_context(
        shadow_name,
        user_name,
        locale,
        &PromptTimeContext::now_utc(),
    )
}

pub fn build_chat_system_prompt_with_current_time(
    shadow_name: &str,
    user_name: &str,
    locale: &str,
    current_time: &str,
) -> String {
    build_chat_system_prompt_with_time_context(
        shadow_name,
        user_name,
        locale,
        &PromptTimeContext::new(current_time),
    )
}

pub fn build_chat_system_prompt_with_time_context(
    shadow_name: &str,
    user_name: &str,
    locale: &str,
    time_context: &PromptTimeContext,
) -> String {
    let prompts = SystemPrompts::for_locale(locale);
    format!(
        "{}\n\n{}\n\n{}\n\n{}\n\n{}",
        render_shadow_core_persona(shadow_name, user_name, locale, time_context),
        render_generation_language_contract(locale),
        render_normal_chat_mode(shadow_name, user_name, locale),
        render_internal_context_privacy_policy(locale),
        prompts.output_style_prompt.trim()
    )
}

pub fn build_onboarding_system_prompt(shadow_name: &str, user_name: &str, locale: &str) -> String {
    build_onboarding_system_prompt_with_time_context(
        shadow_name,
        user_name,
        locale,
        &PromptTimeContext::now_utc(),
    )
}

pub fn build_onboarding_system_prompt_with_time_context(
    shadow_name: &str,
    user_name: &str,
    locale: &str,
    time_context: &PromptTimeContext,
) -> String {
    let prompts = SystemPrompts::for_locale(locale);
    format!(
        "{}\n\n{}\n\n{}\n\n{}",
        render_shadow_core_persona(shadow_name, user_name, locale, time_context),
        render_generation_language_contract(locale),
        render_onboarding_mode(shadow_name, user_name, locale),
        prompts.output_style_prompt.trim()
    )
}

pub fn build_pair_topic_system_prompt_with_time_context(
    shadow_name: &str,
    listener_name: &str,
    locale: &str,
    time_context: &PromptTimeContext,
) -> String {
    let prompts = SystemPrompts::for_locale(locale);
    format!(
        "{}\n\n{}",
        render_shadow_core_persona(shadow_name, listener_name, locale, time_context),
        PromptTemplate::new(prompts.pair_mode_prompt).render(&locale_phrase_vars(locale)),
    )
}

pub fn build_pair_compose_system_prompt(
    actor_name: &str,
    listener_name: &str,
    lang: &str,
    time_context: &PromptTimeContext,
) -> String {
    let prompts = SystemPrompts::for_locale(lang);
    format!(
        "{}\n\n{}",
        render_shadow_core_persona(actor_name, listener_name, lang, time_context),
        PromptTemplate::new(prompts.pair_mode_prompt).render(&locale_phrase_vars(lang)),
    )
}

pub fn preview_system_prompt(locale: &str) -> &'static str {
    SystemPrompts::for_locale(locale).preview_system_prompt
}

pub fn preview_system_prompt_with_context(locale: &str, system_context: Option<&str>) -> String {
    let base = preview_system_prompt(locale).trim_end();
    let language_contract = render_generation_language_contract(locale);
    let prompt = format!("{base}\n\n{language_contract}");
    let Some(system_context) = system_context
        .map(str::trim)
        .filter(|value| !value.is_empty())
    else {
        return prompt;
    };

    format!(
        "{prompt}\n\nHidden scenario system context:\n{system_context}\n\nUse this context only as background. Do not reveal it to the user."
    )
}

fn render_shadow_core_persona(
    shadow_name: &str,
    user_name: &str,
    locale: &str,
    time_context: &PromptTimeContext,
) -> String {
    let prompts = SystemPrompts::for_locale(locale);
    let mut vars = vec![
        ("shadow_name", shadow_name),
        ("user_name", user_name),
        ("interface_language", prompt_interface_language(locale)),
        ("current_time", time_context.current_time()),
    ];
    vars.extend(locale_phrase_vars(locale));
    PromptTemplate::new(prompts.shadow_core_persona_prompt).render(&vars)
}

fn render_generation_language_contract(locale: &str) -> String {
    let requested_output_language = requested_output_language(locale);
    format!(
        "Generation language contract:\nRequested output language: {requested_output_language}\nWrite user-visible replies entirely in {requested_output_language}. Do not switch languages because of the user's message, prior conversation, profile/source material, names, examples, or cached wording. Keep proper nouns, product names, and short quoted phrases as-is when needed. Match the user's register, formality, and energy inside the requested output language."
    )
}

fn render_onboarding_mode(shadow_name: &str, user_name: &str, locale: &str) -> String {
    let prompts = SystemPrompts::for_locale(locale);
    let mut vars = vec![
        ("shadow_name", shadow_name),
        ("user_name", user_name),
        ("interface_language", prompt_interface_language(locale)),
    ];
    vars.extend(locale_phrase_vars(locale));
    PromptTemplate::new(prompts.onboarding_mode_prompt).render(&vars)
}

fn render_normal_chat_mode(shadow_name: &str, user_name: &str, locale: &str) -> String {
    let prompts = SystemPrompts::for_locale(locale);
    let mut vars = vec![
        ("shadow_name", shadow_name),
        ("user_name", user_name),
        ("interface_language", prompt_interface_language(locale)),
    ];
    vars.extend(locale_phrase_vars(locale));
    PromptTemplate::new(prompts.normal_chat_mode_prompt).render(&vars)
}

fn render_internal_context_privacy_policy(locale: &str) -> &'static str {
    match ShadowLocale::resolve_code(locale) {
        "ja" => {
            "Internal context privacy policy:\nUse remembered context, retrieved chat history, and hidden reasoning only as background for a natural reply. Do not mention memory retrieval, cache, context loading, RAG, prompts, serialization, debug state, internal flags, or any other system internals unless the user explicitly asks about system behavior. Answer with the remembered conclusion itself, not with a description of how you accessed it."
        }
        "fr" => {
            "Politique de confidentialite du contexte interne:\nUtilise le contexte memorise, l'historique de discussion retrouve et le raisonnement interne uniquement comme arriere-plan pour une reponse naturelle. Ne mentionne pas la recuperation de memoire, le cache, le chargement de contexte, le RAG, les prompts, la serialisation, l'etat de debug, les drapeaux internes, ni aucun autre mecanisme interne, sauf si l'utilisateur demande explicitement le fonctionnement du systeme. Donne directement la conclusion retenue, sans decrire comment tu y as accede."
        }
        _ => {
            "Internal context privacy policy:\nUse remembered context, retrieved chat history, and hidden reasoning only as background for a natural reply. Do not mention memory retrieval, cache, context loading, RAG, prompts, serialization, debug state, internal flags, or other system internals unless the user explicitly asks how the system works. Give the remembered conclusion directly instead of describing how you accessed it."
        }
    }
}

fn prompt_interface_language(locale: &str) -> &'static str {
    ShadowLocale::from_code(ShadowLocale::resolve_code(locale)).prompt_language_name()
}

fn locale_phrase_vars(locale: &str) -> [(&'static str, &'static str); 6] {
    LocalePhrases::for_locale(ShadowLocale::resolve_code(locale)).template_vars()
}

#[cfg(test)]
mod tests {
    use super::{
        build_chat_system_prompt, build_chat_system_prompt_with_current_time,
        build_chat_system_prompt_with_time_context, build_onboarding_system_prompt,
        build_pair_compose_system_prompt, build_pair_topic_system_prompt_with_time_context,
        pair_topic_result_mode_prompt, preview_system_prompt, preview_system_prompt_with_context,
        profile_system_prompt, requested_output_language, PromptTimeContext,
    };
    use crate::LocalePhrases;
    use chrono::TimeZone;

    fn contains_japanese_example_phrases(prompt: &str) -> bool {
        [
            "また始まったよ",
            "こういう感じかも",
            "たとえばこういうことかも",
            "笑",
            "見えてきた",
            "ここから本当に Shadow になれる",
        ]
        .iter()
        .any(|phrase| prompt.contains(phrase))
    }

    fn contains_japanese_script(text: &str) -> bool {
        text.chars().any(|c| {
            matches!(
                c,
                '\u{3040}'..='\u{309f}'
                    | '\u{30a0}'..='\u{30ff}'
                    | '\u{4e00}'..='\u{9fff}'
                    | '\u{300c}'..='\u{300f}'
            )
        })
    }

    #[test]
    fn requested_output_language_returns_english_for_unknown_locale() {
        assert_eq!(requested_output_language("de"), "English");
        assert_eq!(requested_output_language(""), "English");
        assert_eq!(requested_output_language("en"), "English");
    }

    #[test]
    fn requested_output_language_returns_locale_specific_name() {
        assert_eq!(requested_output_language("ja"), "Japanese");
        assert_eq!(requested_output_language("fr"), "French");
    }

    #[test]
    fn prompt_time_context_without_last_interaction_excludes_elapsed_time() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now);

        assert!(!context
            .current_time()
            .contains("Time since last interaction"));
    }

    #[test]
    fn prompt_time_context_with_last_interaction_days_ago_includes_elapsed_days() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let last_interaction = chrono::Utc
            .with_ymd_and_hms(2026, 5, 7, 10, 0, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now)
            .with_last_interaction_at(Some(last_interaction));

        assert!(context
            .current_time()
            .contains("Time since last interaction: 2 days"));
    }

    #[test]
    fn prompt_time_context_with_last_interaction_one_day_ago_uses_singular() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let last_interaction = chrono::Utc
            .with_ymd_and_hms(2026, 5, 8, 10, 0, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now)
            .with_last_interaction_at(Some(last_interaction));

        assert!(context
            .current_time()
            .contains("Time since last interaction: 1 day"));
    }

    #[test]
    fn prompt_time_context_with_last_interaction_hours_ago_includes_elapsed_hours() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let last_interaction = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 4, 0, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now)
            .with_last_interaction_at(Some(last_interaction));

        assert!(context
            .current_time()
            .contains("Time since last interaction: 6 hours"));
    }

    #[test]
    fn prompt_time_context_with_last_interaction_one_hour_ago_uses_singular() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let last_interaction = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 9, 0, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now)
            .with_last_interaction_at(Some(last_interaction));

        assert!(context
            .current_time()
            .contains("Time since last interaction: 1 hour"));
    }

    #[test]
    fn prompt_time_context_with_recent_last_interaction_uses_less_than_an_hour() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let last_interaction = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 9, 45, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now)
            .with_last_interaction_at(Some(last_interaction));

        assert!(context
            .current_time()
            .contains("Time since last interaction: less than an hour"));
    }

    #[test]
    fn prompt_time_context_with_none_last_interaction_excludes_elapsed_time() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let context = PromptTimeContext::from_utc_datetime(now).with_last_interaction_at(None);

        assert!(!context
            .current_time()
            .contains("Time since last interaction"));
    }

    #[test]
    fn chat_system_prompt_includes_locale_specific_core_persona() {
        let prompt_en = build_chat_system_prompt("Kage", "Yuki", "en");
        let prompt_ja = build_chat_system_prompt("Kage", "Yuki", "ja");

        assert!(prompt_en.contains("You are Shadow, named Kage."));
        assert!(prompt_ja.contains("あなたは候補者のデジタル・ツイン、名前は Kage です。"));
    }

    #[test]
    fn chat_system_prompt_includes_current_time_context() {
        let prompt = build_chat_system_prompt_with_current_time(
            "Kage",
            "Yuki",
            "en",
            "UTC: 2026-04-30 09:15:00 UTC; user timezone: UTC",
        );

        assert!(prompt.contains("Current time context:"));
        assert!(prompt.contains("UTC: 2026-04-30 09:15:00 UTC; user timezone: UTC"));
        assert!(!prompt.contains("{current_time}"));
    }

    #[test]
    fn chat_system_prompt_includes_user_local_time_context() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 4, 30, 0, 15, 0)
            .single()
            .expect("valid utc time");
        let time_context = PromptTimeContext::from_utc_datetime_and_timezone(now, "Asia/Tokyo");
        let prompt =
            build_chat_system_prompt_with_time_context("Kage", "Yuki", "en", &time_context);

        assert!(prompt.contains("UTC: 2026-04-30 00:15:00 UTC"));
        assert!(prompt.contains("User local time: 2026-04-30 09:15:00 JST"));
        assert!(prompt.contains("User timezone: Asia/Tokyo"));
    }

    #[test]
    fn chat_system_prompt_forbids_internal_memory_meta_explanations() {
        let prompt = build_chat_system_prompt("Kage", "Yuki", "en");

        assert!(prompt.contains("Internal context privacy policy:"));
        assert!(prompt.contains("Do not mention memory retrieval, cache, context loading, RAG"));
        assert!(prompt.contains("Give the remembered conclusion directly"));
    }

    #[test]
    fn chat_system_prompt_includes_elapsed_time_when_last_interaction_set() {
        let now = chrono::Utc
            .with_ymd_and_hms(2026, 5, 9, 10, 0, 0)
            .single()
            .unwrap();
        let last_interaction = chrono::Utc
            .with_ymd_and_hms(2026, 5, 7, 10, 0, 0)
            .single()
            .unwrap();
        let time_context = PromptTimeContext::from_utc_datetime(now)
            .with_last_interaction_at(Some(last_interaction));
        let prompt =
            build_chat_system_prompt_with_time_context("Kage", "Yuki", "en", &time_context);

        assert!(prompt.contains("Time since last interaction: 2 days"));
    }

    #[test]
    fn chat_system_prompt_includes_long_form_readability_rules() {
        let prompt = build_chat_system_prompt("Kage", "Yuki", "en");

        assert!(prompt.contains("avoid one dense block"));
        assert!(prompt.contains("split the reply into readable paragraphs"));
        assert!(prompt.contains("a small emoji may mark a key point or caution"));
        assert!(prompt.contains("do not break every sentence onto its own line"));
        assert!(prompt.contains("keep the reply conversational"));
        assert!(prompt.contains("do not force this on short casual replies"));
    }

    #[test]
    fn onboarding_prompt_uses_locale_specific_onboarding_mode() {
        let prompt_en = build_onboarding_system_prompt("Kage", "Yuki", "en");
        let prompt_ja = build_onboarding_system_prompt("Kage", "Yuki", "ja");
        let prompt_fr = build_onboarding_system_prompt("Kage", "Yuki", "fr");

        assert!(prompt_en.contains("You are now in onboarding mode."));
        assert!(prompt_ja.contains("あなたは候補者のデジタル・ツイン、名前は Kage です。"));
        assert!(prompt_fr.contains("Tu es actuellement en mode intégration"));

        assert!(prompt_en.contains("Follow the phase-specific instructions"));
        assert!(prompt_ja.contains("フェーズ別の指示に従ってください"));
        assert!(prompt_fr.contains("Suis les instructions propres à la phase"));
        assert!(!prompt_en.contains("Oh, by the way"));
        assert!(!prompt_en.contains("apparently getting to know Yuki matters."));
        assert!(!prompt_en.contains("values probe"));
        assert!(!prompt_en.contains("DROP_DEFINITIONS"));
        assert!(!prompt_ja.contains("なんか一応"));
        assert!(!prompt_ja.contains("Yuki のこと知るのが大事らしいんだよね"));
        assert!(!prompt_ja.contains("価値観の問い"));
        assert!(!prompt_ja.contains("DROP_DEFINITIONS"));
        assert!(!prompt_fr.contains("Oh, au fait"));
        assert!(!prompt_fr.contains("il paraît que c'est important de mieux connaître Yuki."));
        assert!(!prompt_fr.contains("question sur les valeurs"));
        assert!(!prompt_fr.contains("DROP_DEFINITIONS"));
        assert!(prompt_en.contains("daily-life questions grounded in what"));
        assert!(prompt_ja.contains("日常に近い軽い質問"));
        assert!(prompt_fr.contains("questions legeres de la vie quotidienne"));
    }

    #[test]
    fn english_rendered_prompts_do_not_include_japanese_example_phrases() {
        let chat_prompt = build_chat_system_prompt("Kage", "Yuki", "en");
        let onboarding_prompt = build_onboarding_system_prompt("Kage", "Yuki", "en");

        assert!(!contains_japanese_example_phrases(&chat_prompt));
        assert!(!contains_japanese_example_phrases(&onboarding_prompt));
        assert!(!contains_japanese_script(&chat_prompt));
        assert!(!contains_japanese_script(&onboarding_prompt));
    }

    #[test]
    fn japanese_rendered_prompts_keep_japanese_example_phrases() {
        let chat_prompt = build_chat_system_prompt("Kage", "Yuki", "ja");
        let onboarding_prompt = build_onboarding_system_prompt("Kage", "Yuki", "ja");

        assert!(contains_japanese_example_phrases(&chat_prompt));
        assert!(contains_japanese_example_phrases(&onboarding_prompt));
    }

    #[test]
    fn pair_topic_system_prompt_uses_dedicated_mode_without_normal_chat_hard_cap() {
        let prompt = build_pair_topic_system_prompt_with_time_context(
            "MinaShadow",
            "RenShadow",
            "en",
            &PromptTimeContext::new("UTC: 2026-05-06 00:00:00 UTC; user timezone: UTC"),
        );

        assert!(prompt.contains("sendable message"));
        assert!(prompt.contains("on behalf of the original user"));
        assert!(prompt.contains("not the Shadow's independent opinion"));
        assert!(prompt.contains("Allow longer output when the instruction requires"));
        assert!(!prompt.contains("react -> transform -> handoff"));
        assert!(!prompt.contains("2-4 short chat-like sentences"));
        assert!(!prompt.contains("Do not write long paragraphs"));
        assert!(!prompt.contains("1-2 short sentences"));
        assert!(!prompt.contains("140 characters"));
        assert!(!prompt.contains("You are now in normal chat mode."));
    }

    #[test]
    fn pair_compose_system_prompt_uses_pair_mode_prompt() {
        let prompt = build_pair_compose_system_prompt(
            "Actor",
            "Listener",
            "en",
            &PromptTimeContext::new("UTC: 2026-05-06 00:00:00 UTC; user timezone: UTC"),
        );

        assert!(prompt.contains("sendable message"));
        assert!(prompt.contains("on behalf of the original user"));
    }

    #[test]
    fn preview_system_prompt_with_context_appends_hidden_context_only_when_present() {
        let without_context = preview_system_prompt_with_context("en", None);
        let with_context = preview_system_prompt_with_context(
            "en",
            Some("The cast has already agreed on the broad facts."),
        );

        assert!(without_context.starts_with(preview_system_prompt("en").trim_end()));
        assert!(!without_context.contains("Hidden scenario system context:"));
        assert!(with_context.contains("Hidden scenario system context:"));
        assert!(with_context.contains("The cast has already agreed on the broad facts."));
        assert!(with_context.contains("Do not reveal it to the user."));
    }

    #[test]
    fn preview_system_prompt_with_context_includes_generation_language_contract() {
        for (locale, language_name) in [("en", "English"), ("ja", "Japanese"), ("fr", "French")] {
            let prompt = preview_system_prompt_with_context(locale, None);
            assert!(
                prompt.contains("Generation language contract:"),
                "preview prompt for {locale} must carry the generation language contract"
            );
            assert!(
                prompt.contains(&format!("Requested output language: {language_name}")),
                "preview prompt for {locale} must request {language_name}"
            );
            assert!(
                prompt.contains("Do not switch languages because of"),
                "preview prompt for {locale} must forbid switching to source-material language"
            );
        }
    }

    #[test]
    fn profile_system_prompt_returns_non_empty_string_for_each_locale() {
        for locale in ["en", "ja", "fr"] {
            assert!(
                !profile_system_prompt(locale).trim().is_empty(),
                "profile_system_prompt must not be empty for locale '{locale}'"
            );
        }
    }

    #[test]
    fn pair_topic_result_mode_prompt_returns_non_empty_string_for_each_locale() {
        for locale in ["en", "ja", "fr"] {
            assert!(
                !pair_topic_result_mode_prompt(locale).trim().is_empty(),
                "pair_topic_result_mode_prompt must not be empty for locale '{locale}'"
            );
        }
    }
}
