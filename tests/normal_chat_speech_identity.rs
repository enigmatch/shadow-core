use shadow_core::{
    build_chat_system_prompt_with_time_context,
    build_chat_system_prompt_with_time_context_and_preferred_first_person, PromptTimeContext,
};

#[test]
fn preferred_first_person_defaults_to_the_existing_prompt_for_each_locale() {
    let time_context = PromptTimeContext::new("UTC: 2026-08-30 00:00:00 UTC; user timezone: UTC");

    for (locale, expected_default_rule) in [
        (
            "en",
            "In conversation, always refer to yourself as \"Shade\".",
        ),
        (
            "ja",
            "会話の中では、常に自分のことを「Shade」と呼んでください。",
        ),
        (
            "fr",
            "Dans la conversation, appelle-toi toujours \"Shade\".",
        ),
    ] {
        let existing =
            build_chat_system_prompt_with_time_context("Shade", "User", locale, &time_context);
        let with_no_preference =
            build_chat_system_prompt_with_time_context_and_preferred_first_person(
                "Shade",
                "User",
                locale,
                &time_context,
                None,
            );

        assert_eq!(with_no_preference, existing);
        assert!(with_no_preference.contains(expected_default_rule));
        assert!(!with_no_preference.contains("{shadow_self_reference_rule}"));
    }
}

#[test]
fn preferred_first_person_is_rendered_in_each_supported_locale() {
    for (locale, expression, expected_instruction) in [
        (
            "en",
            "I",
            "use \"I\" as your preferred first-person expression",
        ),
        ("ja", "僕", "一人称として「僕」を使ってください"),
        (
            "fr",
            "je",
            "utilise « je » comme expression à la première personne",
        ),
    ] {
        let prompt = build_chat_system_prompt_with_time_context_and_preferred_first_person(
            "Kage",
            "Yuki",
            locale,
            &PromptTimeContext::new("UTC: 2026-08-30 00:00:00 UTC; user timezone: UTC"),
            Some(expression),
        );

        assert!(
            prompt.contains(expected_instruction),
            "{locale} prompt should contain its localized speech identity instruction"
        );
        assert!(
            prompt.contains("Kage"),
            "{locale} prompt should retain the actual Shadow name"
        );
        assert!(
            !prompt.contains('{') && !prompt.contains('}'),
            "{locale} prompt should not contain unresolved placeholders"
        );
    }
}
