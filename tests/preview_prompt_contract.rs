use shadow_core::{build_chat_system_prompt, preview_system_prompt};

#[test]
fn preview_prompt_pushes_personal_stance_and_non_prompt_titles() {
    let prompt = preview_system_prompt("en");

    assert!(prompt.contains("Aim for the answer this person would actually want to say."));
    assert!(prompt.contains("Make the answer feel like a personal post or statement"));
    assert!(
        prompt.contains("Do not use the title to restate, summarize, or lightly rephrase the prompt.")
    );
    assert!(prompt.contains("Keep the body answer to at most 2 sentences."));
    assert!(prompt.contains("Do not pull in exceptions from unrelated setup answers"));
}

#[test]
fn preview_prompt_names_publish_ready_memory_background_contract() {
    let prompt = preview_system_prompt("en");

    assert!(prompt.contains("Publish-ready"));
    assert!(prompt.contains("Long-term memory is background evidence"));
}

#[test]
fn preview_prompt_avoids_mixed_language_chat_continuation_examples() {
    let prompt = preview_system_prompt("en");

    assert!(prompt.contains("Avoid chat-continuation wording"));
    for phrase in [
        "tell me more",
        "what kind of advice do you want?",
        "let's think together",
        "一緒に考えよ",
        "教えてくれたら",
        "どんなテーマがいい？",
    ] {
        assert!(
            !prompt.contains(phrase),
            "preview_system_prompt should avoid literal mixed-language example '{phrase}'"
        );
    }
}

#[test]
fn normal_chat_prompt_answers_explicit_help_directly_without_losing_shadow_voice() {
    let en = build_chat_system_prompt("Shade", "User", "en");
    let ja = build_chat_system_prompt("Kage", "User", "ja");

    let expected = "When the user clearly asks for help, explanation, research, summarization, planning, writing, or practical advice, answer the useful part directly first, then keep the response conversational and Shadow-like.";

    assert!(
        en.contains(expected),
        "English chat system prompt should contain the direct-help output rule"
    );
    assert!(
        ja.contains(expected),
        "Japanese chat system prompt should contain the shared direct-help output rule"
    );
}

#[test]
fn normal_chat_prompt_allows_light_structure_for_requested_explanations() {
    let prompt = build_chat_system_prompt("Shade", "User", "en");

    for expected in [
        "In casual conversation, avoid bullets and headings",
        "when the user asks for explanation, comparison, steps, planning, a list, or a draft",
        "short bullets or compact headings",
        "Do not turn light structure into a report tone",
    ] {
        assert!(
            prompt.contains(expected),
            "chat system prompt should contain {expected}"
        );
    }
}
