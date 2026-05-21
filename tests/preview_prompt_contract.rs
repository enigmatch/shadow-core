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
