use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShadowProfile {
    pub headline: String,
    pub stance: String,
    pub source_answers: Vec<String>,
    pub tone: String,
    pub traits: Vec<String>,
    pub decision_style: String,
    pub anchor: String,
    #[serde(default)]
    pub speech_style: Option<crate::SpeechStyle>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowAnswerContent {
    pub title: String,
    pub body: String,
}

impl ShadowAnswerContent {
    #[must_use]
    pub fn final_answer_text(&self) -> String {
        if self.body.trim().is_empty() {
            self.title.trim().to_string()
        } else {
            format!("{}\n\n{}", self.title.trim(), self.body.trim())
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowAnswer {
    pub content: ShadowAnswerContent,
    pub display_deltas: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerationEngine {
    pub provider: &'static str,
    pub model: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadowGenerationFailure {
    pub code: String,
    pub message: String,
}

impl std::fmt::Display for ShadowGenerationFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ShadowGenerationFailure {}
