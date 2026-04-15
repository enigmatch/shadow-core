pub const PROFILE_SYSTEM_PROMPT: &str = include_str!("prompts/profile_system_prompt.txt");
pub const PREVIEW_SYSTEM_PROMPT: &str = include_str!("prompts/preview_system_prompt.txt");
pub const CHAT_SYSTEM_PROMPT: &str = include_str!("prompts/chat_system_prompt.txt");
pub const ONBOARDING_TURN_TWO_SYSTEM_PROMPT: &str = include_str!("prompts/onboarding_turn_two.txt");
pub const ONBOARDING_TURN_THREE_SYSTEM_PROMPT: &str =
    include_str!("prompts/onboarding_turn_three.txt");

#[cfg(test)]
mod tests {
    use super::{
        CHAT_SYSTEM_PROMPT, ONBOARDING_TURN_THREE_SYSTEM_PROMPT, ONBOARDING_TURN_TWO_SYSTEM_PROMPT,
        PREVIEW_SYSTEM_PROMPT, PROFILE_SYSTEM_PROMPT,
    };

    #[test]
    fn prompt_assets_are_non_empty() {
        assert!(!PROFILE_SYSTEM_PROMPT.trim().is_empty());
        assert!(!PREVIEW_SYSTEM_PROMPT.trim().is_empty());
        assert!(!CHAT_SYSTEM_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_TURN_TWO_SYSTEM_PROMPT.trim().is_empty());
        assert!(!ONBOARDING_TURN_THREE_SYSTEM_PROMPT.trim().is_empty());
    }

    #[test]
    fn profile_prompt_keeps_output_contract_private() {
        assert!(PROFILE_SYSTEM_PROMPT.contains("append the exact output contract separately"));
        assert!(!PROFILE_SYSTEM_PROMPT.contains("\"headline\""));
        assert!(!PROFILE_SYSTEM_PROMPT.contains("Return JSON only with this exact shape"));
    }
}
