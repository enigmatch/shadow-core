use shadow_core::SystemPrompts;

#[test]
fn preview_prompt_pushes_personal_stance_and_non_prompt_titles() {
    let prompts = SystemPrompts::for_locale("en");

    assert!(prompts
        .preview_system_prompt
        .contains("Aim for the answer this person would actually want to say."));
    assert!(prompts
        .preview_system_prompt
        .contains("Make the answer feel like a personal post or statement"));
    assert!(prompts
        .preview_system_prompt
        .contains("Do not use the title to restate, summarize, or lightly rephrase the prompt."));
    assert!(prompts
        .preview_system_prompt
        .contains("Keep the body answer to at most 2 sentences."));
    assert!(prompts
        .preview_system_prompt
        .contains("Do not pull in exceptions from unrelated setup answers"));
}

#[test]
fn preview_prompt_names_publish_ready_memory_background_contract() {
    let prompts = SystemPrompts::for_locale("en");

    assert!(prompts.preview_system_prompt.contains("Publish-ready"));
    assert!(prompts
        .preview_system_prompt
        .contains("Long-term memory is background evidence"));
}

#[test]
fn preview_prompt_avoids_mixed_language_chat_continuation_examples() {
    let prompts = SystemPrompts::for_locale("en");

    assert!(prompts
        .preview_system_prompt
        .contains("Avoid chat-continuation wording"));
    for phrase in [
        "tell me more",
        "what kind of advice do you want?",
        "let's think together",
        "一緒に考えよ",
        "教えてくれたら",
        "どんなテーマがいい？",
    ] {
        assert!(
            !prompts.preview_system_prompt.contains(phrase),
            "preview_system_prompt should avoid literal mixed-language example '{phrase}'"
        );
    }
}
