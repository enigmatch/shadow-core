use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpeechStyle {
    pub dialect: Option<String>,
    pub formality: String,
    pub markers: Vec<String>,
    pub sentence_pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptReadyProfile {
    pub headline: String,
    pub stance: String,
    pub source_answers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptReadyPersona {
    pub tone: String,
    pub traits: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_style: Option<SpeechStyle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptReadyReasoningPolicy {
    pub decision_style: String,
    pub anchor: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PairShadowIdentity {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<PromptReadyProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persona: Option<PromptReadyPersona>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_policy: Option<PromptReadyReasoningPolicy>,
}

#[cfg(test)]
mod tests {
    use super::{
        PairShadowIdentity, PromptReadyPersona, PromptReadyProfile, PromptReadyReasoningPolicy,
    };
    use crate::{ShadowAnswer, ShadowAnswerContent, ShadowProfile, SpeechStyle};

    #[test]
    fn shadow_profile_serializes_as_flat_core_type() {
        let profile = ShadowProfile {
            headline: "Answers with explicit tradeoffs".to_string(),
            stance: "evidence_first".to_string(),
            source_answers: vec!["I value responsibility".to_string()],
            tone: "reflective".to_string(),
            traits: vec!["structured".to_string()],
            decision_style: "principled_tradeoff".to_string(),
            anchor: "evidence before action".to_string(),
            speech_style: Some(SpeechStyle {
                dialect: Some("kansai-ben".to_string()),
                formality: "casual".to_string(),
                markers: vec!["やん".to_string()],
                sentence_pattern: "short bursts ending in やん".to_string(),
            }),
        };

        let value = serde_json::to_value(profile).expect("shadow profile should serialize");
        assert_eq!(value["headline"], "Answers with explicit tradeoffs");
        assert_eq!(value["tone"], "reflective");
        assert_eq!(value["speech_style"]["formality"], "casual");
    }

    #[test]
    fn pair_shadow_identity_uses_renamed_speech_style_type() {
        let identity = PairShadowIdentity {
            name: "Kage".to_string(),
            personal_instruction: Some("Open with my own point of view.".to_string()),
            profile: Some(PromptReadyProfile {
                headline: "clear".to_string(),
                stance: "measured".to_string(),
                source_answers: vec!["One".to_string()],
            }),
            persona: Some(PromptReadyPersona {
                tone: "warm".to_string(),
                traits: vec!["curious".to_string()],
                speech_style: Some(SpeechStyle {
                    dialect: None,
                    formality: "neutral".to_string(),
                    markers: vec!["hmm".to_string()],
                    sentence_pattern: "short".to_string(),
                }),
            }),
            reasoning_policy: Some(PromptReadyReasoningPolicy {
                decision_style: "evidence_first".to_string(),
                anchor: "signal".to_string(),
            }),
        };

        let value = serde_json::to_value(identity).expect("pair identity should serialize");
        assert_eq!(
            value["personal_instruction"],
            "Open with my own point of view."
        );
        assert_eq!(value["persona"]["speech_style"]["formality"], "neutral");
    }

    #[test]
    fn shadow_answer_final_text_keeps_title_and_body_format() {
        let answer = ShadowAnswer {
            content: ShadowAnswerContent {
                title: "A title".to_string(),
                body: "A body".to_string(),
            },
            display_deltas: vec!["A title".to_string()],
        };

        assert_eq!(answer.content.final_answer_text(), "A title\n\nA body");
    }
}
