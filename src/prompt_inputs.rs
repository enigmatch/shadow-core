use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PromptReadySpeechStyle {
    pub dialect: Option<String>,
    pub formality: String,
    pub markers: Vec<String>,
    pub sentence_pattern: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PromptReadyProfile {
    pub headline: String,
    pub stance: String,
    pub source_answers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PromptReadyPersona {
    pub tone: String,
    pub traits: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_style: Option<PromptReadySpeechStyle>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct PromptReadyReasoningPolicy {
    pub decision_style: String,
    pub anchor: String,
}
