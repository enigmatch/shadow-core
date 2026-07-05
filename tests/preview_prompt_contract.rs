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
fn preview_prompt_requires_profile_specific_answers() {
    let prompt = preview_system_prompt("en");

    for expected in [
        "Do not write an answer that could be said by a generic assistant, an average person, or another Shadow.",
        "The answer must depend on this Shadow's own profile",
        "what only this Shadow would notice, resist, care about, or prioritize",
        "If the answer would still make sense after removing this Shadow's profile, it is too generic.",
    ] {
        assert!(
            prompt.contains(expected),
            "preview_system_prompt should contain profile-specific answer contract: {expected}"
        );
    }
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

fn contract_profile() -> shadow_core::ShadowProfile {
    shadow_core::ShadowProfile {
        headline: "Ships fast, admits mistakes".to_string(),
        stance: "Bias for action".to_string(),
        source_answers: vec![],
        tone: "direct".to_string(),
        traits: vec!["decisive".to_string()],
        decision_style: "gut first, verify after".to_string(),
        anchor: "keep promises".to_string(),
        speech_style: None,
    }
}

fn contract_challenge() -> shadow_core::ShadowChallenge {
    shadow_core::ShadowChallenge {
        title: None,
        prompt_text: "Submit on time or fix the mistake?".to_string(),
        tag_label: Some("#startup".to_string()),
        system_context: None,
        preferred_probe_kind: None,
    }
}

#[test]
fn preview_input_renders_same_question_feedback_as_top_priority_directive() {
    let input = shadow_core::preview_input_with_reflection_memory_and_long_term_context(
        &contract_challenge(),
        &contract_profile(),
        &[],
        "en",
        &["User reflection feedback: keep the tradeoff explicit.".to_string()],
        "Long-term memory selected for this turn:\n- none selected",
        &["User reflection feedback: be bolder, neutral answers are boring.".to_string()],
    );

    let directive_index = input
        .find("User feedback on your previous answer to this exact prompt")
        .expect("same-question feedback directive should be present");
    assert!(input.contains("be bolder, neutral answers are boring."));
    let generic_index = input
        .find("Reflection memory for question answer (influence 5/10)")
        .expect("generic reflection memory block should stay");
    assert!(
        directive_index < generic_index,
        "directive must appear before the generic reflection memory block"
    );
    let evidence_index = input
        .find("Relevant onboarding answers for this prompt")
        .expect("evidence block should stay");
    assert!(
        directive_index < evidence_index,
        "directive must appear before the onboarding evidence block"
    );
    assert!(
        !input.contains("Reflection memory for question answer (influence 5/10):\n- User reflection feedback: be bolder"),
        "same-question feedback must not be duplicated into the generic block"
    );
}

#[test]
fn preview_input_omits_directive_without_same_question_feedback() {
    let input = shadow_core::preview_input_with_reflection_memory_and_long_term_context(
        &contract_challenge(),
        &contract_profile(),
        &[],
        "en",
        &["User reflection feedback: keep the tradeoff explicit.".to_string()],
        "Long-term memory selected for this turn:\n- none selected",
        &[],
    );

    assert!(
        !input.contains("User feedback on your previous answer to this exact prompt"),
        "directive header must not render when there is no same-question feedback"
    );
    assert!(input.contains("Reflection memory for question answer (influence 5/10)"));
    assert!(input.contains("keep the tradeoff explicit."));
}
