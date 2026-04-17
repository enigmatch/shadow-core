pub const PROFILE_SYSTEM_PROMPT: &str = include_str!("prompts/profile_system_prompt.txt");
pub const PREVIEW_SYSTEM_PROMPT: &str = include_str!("prompts/preview_system_prompt.txt");
pub const CHAT_SYSTEM_PROMPT: &str = include_str!("prompts/chat_system_prompt.txt");
pub const ONBOARDING_TURN_TWO_SYSTEM_PROMPT: &str = include_str!("prompts/onboarding_turn_two.txt");
pub const ONBOARDING_TURN_THREE_SYSTEM_PROMPT: &str =
    include_str!("prompts/onboarding_turn_three.txt");
pub const SHADOW_CORE_PERSONA_PROMPT: &str = include_str!("prompts/shadow_core_persona.txt");
pub const ONBOARDING_MODE_PROMPT: &str = include_str!("prompts/onboarding_mode.txt");
pub const NORMAL_CHAT_MODE_PROMPT: &str = include_str!("prompts/normal_chat_mode.txt");
pub const OUTPUT_STYLE_PROMPT: &str = include_str!("prompts/output_style.txt");

#[cfg(test)]
mod tests {
    use super::{
        CHAT_SYSTEM_PROMPT, NORMAL_CHAT_MODE_PROMPT, ONBOARDING_MODE_PROMPT,
        ONBOARDING_TURN_THREE_SYSTEM_PROMPT, ONBOARDING_TURN_TWO_SYSTEM_PROMPT,
        OUTPUT_STYLE_PROMPT, PREVIEW_SYSTEM_PROMPT, PROFILE_SYSTEM_PROMPT,
        SHADOW_CORE_PERSONA_PROMPT,
    };

    #[test]
    fn prompt_assets_are_non_empty() {
        assert!(!PROFILE_SYSTEM_PROMPT.trim().is_empty());
        assert!(!PREVIEW_SYSTEM_PROMPT.trim().is_empty());
        assert!(!CHAT_SYSTEM_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_TURN_TWO_SYSTEM_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_TURN_THREE_SYSTEM_PROMPT.trim().is_empty());
        assert!(!SHADOW_CORE_PERSONA_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_MODE_PROMPT.trim().is_empty());
        assert!(!NORMAL_CHAT_MODE_PROMPT.trim().is_empty());
        assert!(!OUTPUT_STYLE_PROMPT.trim().is_empty());
    }

    #[test]
    fn profile_prompt_keeps_output_contract_private() {
        assert!(PROFILE_SYSTEM_PROMPT.contains("append the exact output contract separately"));
        assert!(!PROFILE_SYSTEM_PROMPT.contains("\"headline\""));
        assert!(!PROFILE_SYSTEM_PROMPT.contains("Return JSON only with this exact shape"));
    }
}
