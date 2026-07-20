use shadow_core::build_chat_system_prompt;

#[test]
fn normal_chat_prompt_keeps_referenced_text_separate_from_shadow_commentary() {
    let english = build_chat_system_prompt("Shade", "User", "en");
    let japanese = build_chat_system_prompt("Kage", "User", "ja");
    let french = build_chat_system_prompt("Ombre", "User", "fr");

    for (locale, prompt, reference_rule, commentary_rule) in [
        (
            "en",
            english,
            "put only the referenced source text inside a Markdown blockquote",
            "Put your own interpretation, advice, or reply in a separate paragraph",
        ),
        (
            "ja",
            japanese,
            "参照元のテキストだけを Markdown の引用ブロック内に置いてください",
            "あなた自身の解釈、助言、返信は別の段落に置いてください",
        ),
        (
            "fr",
            french,
            "place uniquement le texte source référencé dans un bloc de citation Markdown",
            "Place ton interprétation, tes conseils ou ta réponse dans un paragraphe séparé",
        ),
    ] {
        assert!(
            prompt.contains(reference_rule),
            "{locale} normal-chat prompt should define a reference-only blockquote"
        );
        assert!(
            prompt.contains(commentary_rule),
            "{locale} normal-chat prompt should keep Shadow commentary outside the reference"
        );
    }
}
