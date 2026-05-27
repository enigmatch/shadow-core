//! Shadow operational knowledge: prompt builders and instruction factories.
//!
//! **Ownership boundary:** This module owns all prompt builders that have no dependency on
//! application-layer code (database models, OpenAI schemas, service state machines, or
//! app-specific persistence). These builders compose instructions, inputs, and user-turn
//! prompts from static text templates and pure data — all of which live in shadow-core.
//!
//! Builders that still depend on app-layer code remain in the application crate under
//! `src/data/shadow_prompts/`.

use crate::{
    PairShadowIdentity, PairTopicTone, PairTurnDirective, PromptReadyPersona, PromptReadyProfile,
    PromptReadyReasoningPolicy, PromptTemplate, ShadowChallenge, ShadowProfile, SpeechStyle,
};

pub const QUESTION_ANSWER_REFLECTION_WEIGHT: u8 = 5;
pub const NORMAL_CHAT_REFLECTION_WEIGHT: u8 = 3;

// ── Text file includes ───────────────────────────────────────────────────────

const TRANSLATION_PREVIEW_INSTRUCTIONS: &str = include_str!("translation_preview_instructions.txt");
const TRANSLATION_CHAT_INSTRUCTIONS: &str = include_str!("translation_chat_instructions.txt");
const REFLECTION_REPLY_INSTRUCTIONS: &str = include_str!("reflection_reply_instructions.txt");
const REFLECTION_REPLY_INPUT: &str = include_str!("reflection_reply_input.txt");
const ONBOARDING_PHASE_INSTRUCTIONS: &str = include_str!("onboarding_phase_instructions.txt");
const ONBOARDING_PROMPT_INPUT: &str = include_str!("onboarding_prompt_input.txt");
const ONBOARDING_PHASE_NOT_STARTED_INSTRUCTIONS: &str =
    include_str!("onboarding_phase_not_started_instructions.txt");
const ONBOARDING_PHASE_GREETING_INSTRUCTIONS: &str =
    include_str!("onboarding_phase_greeting_instructions.txt");
const ONBOARDING_PHASE_HEADLINE_CONFIRMATION_INSTRUCTIONS: &str =
    include_str!("onboarding_phase_headline_confirmation_instructions.txt");
const ONBOARDING_PHASE_COMPLETED_INSTRUCTIONS: &str =
    include_str!("onboarding_phase_completed_instructions.txt");
const ONBOARDING_SDQ_INITIAL_TURN_NOTE: &str = include_str!("onboarding_sdq_initial_turn_note.txt");
const ONBOARDING_SDQ_FOLLOWUP_TURN_NOTE: &str =
    include_str!("onboarding_sdq_followup_turn_note.txt");
const ONBOARDING_SDQ_PHASE_INSTRUCTIONS: &str =
    include_str!("onboarding_sdq_phase_instructions.txt");
const EXPLICIT_CORRECTION_INPUT: &str = include_str!("explicit_correction_input.txt");
const EXPLICIT_CORRECTION_INSTRUCTIONS: &str = include_str!("explicit_correction_instructions.txt");
const CHAT_CONTEXT_PLANNER_INSTRUCTIONS: &str =
    include_str!("chat_context_planner_instructions.txt");
const SUMMARY_REFRESH_INSTRUCTIONS: &str = include_str!("summary_refresh_instructions.txt");

// ── Macro for zero-param static-prompt accessors ─────────────────────────────

macro_rules! define_static_prompt {
    ($( $vis:vis fn $name:ident = $const:ident; )*) => {
        $(
            $vis fn $name() -> &'static str {
                $const.trim_end()
            }
        )*
    };
}

define_static_prompt! {
    pub fn explicit_correction_instructions = EXPLICIT_CORRECTION_INSTRUCTIONS;
    pub fn onboarding_phase_not_started_instructions = ONBOARDING_PHASE_NOT_STARTED_INSTRUCTIONS;
    pub fn onboarding_phase_greeting_instructions = ONBOARDING_PHASE_GREETING_INSTRUCTIONS;
    pub fn onboarding_phase_headline_confirmation_instructions = ONBOARDING_PHASE_HEADLINE_CONFIRMATION_INSTRUCTIONS;
    pub fn onboarding_phase_completed_instructions = ONBOARDING_PHASE_COMPLETED_INSTRUCTIONS;
    pub fn build_chat_context_planner_instructions = CHAT_CONTEXT_PLANNER_INSTRUCTIONS;
    pub fn build_summary_refresh_instructions = SUMMARY_REFRESH_INSTRUCTIONS;
}

// ── Pair topic tone classifier ────────────────────────────────────────────────

pub fn classify_pair_topic_tone(
    tag_label: Option<&str>,
    localized_tag_label: Option<&str>,
    _prompt_text: &str,
) -> PairTopicTone {
    let labels = [tag_label.unwrap_or(""), localized_tag_label.unwrap_or("")];
    if labels_match(
        &labels,
        &[
            "funny",
            "humor",
            "humour",
            "ユーモア",
            "お笑い",
            "drôle",
            "drole",
        ],
    ) {
        return PairTopicTone::Funny;
    }
    if labels_match(
        &labels,
        &[
            "love affairs",
            "love",
            "amour",
            "drama",
            "drame",
            "修羅場",
            "relationship",
            "relationships",
            "romance",
            "恋",
            "恋愛",
            "恋愛・人間関係",
            "human relationships",
            "human relationship",
            "relations humaines",
            "人間関係",
            "amour et relations",
        ],
    ) {
        return PairTopicTone::Relationship;
    }
    if labels_match(
        &labels,
        &[
            "dev fight",
            "work",
            "startup",
            "money",
            "career",
            "money & work",
            "work & money",
            "argent et travail",
            "travail et argent",
            "company",
            "仕事",
            "起業",
            "お金",
            "お金と仕事",
            "仕事",
        ],
    ) {
        return PairTopicTone::WorkDev;
    }
    if labels_match(
        &labels,
        &[
            "serious",
            "serious reflective",
            "debate",
            "débat",
            "debat",
            "賛否",
            "討論",
            "politics",
            "society",
            "society & politics",
            "politique",
            "société et politique",
            "政治",
            "社会",
            "ethics",
            "倫理",
        ],
    ) {
        return PairTopicTone::SeriousReflective;
    }
    if labels_match(
        &labels,
        &["idea", "ideas", "idées", "アイデア", "daily life"],
    ) {
        return PairTopicTone::CasualValues;
    }
    PairTopicTone::CasualValues
}

fn labels_match(labels: &[&str], expected: &[&str]) -> bool {
    labels.iter().any(|label| {
        let normalized = normalize_topic_label(label);
        expected
            .iter()
            .any(|candidate| normalized == normalize_topic_label(candidate))
    })
}

fn normalize_topic_label(label: &str) -> String {
    label.trim().to_lowercase()
}

// ── Translation builders ──────────────────────────────────────────────────────

pub fn build_translation_preview_instructions(source_locale: &str, target_locale: &str) -> String {
    render_template(
        TRANSLATION_PREVIEW_INSTRUCTIONS,
        &[
            ("source_locale", source_locale),
            ("target_locale", target_locale),
        ],
    )
}

pub fn build_translation_chat_instructions(
    content_kind: &str,
    source_locale: &str,
    target_locale: &str,
) -> String {
    render_template(
        TRANSLATION_CHAT_INSTRUCTIONS,
        &[
            ("content_kind", content_kind),
            ("source_locale", source_locale),
            ("target_locale", target_locale),
        ],
    )
}

// ── Reflection reply builders ─────────────────────────────────────────────────

pub fn build_reflection_reply_instructions(base_instructions: &str) -> String {
    render_template(
        REFLECTION_REPLY_INSTRUCTIONS,
        &[("base_instructions", base_instructions)],
    )
}

pub fn build_reflection_reply_input(
    requested_answer_language: &str,
    reflection_session_status: &str,
    shadow_name: &str,
    scenario_prompt: &str,
    original_answer_title: &str,
    original_answer_body: &str,
    normal_chat_context: &str,
    reflection_conversation: &str,
) -> String {
    render_template(
        REFLECTION_REPLY_INPUT,
        &[
            ("requested_answer_language", requested_answer_language),
            ("reflection_session_status", reflection_session_status),
            ("shadow_name", shadow_name),
            ("scenario_prompt", scenario_prompt),
            ("original_answer_title", original_answer_title),
            ("original_answer_body", original_answer_body),
            ("normal_chat_context", normal_chat_context),
            ("reflection_conversation", reflection_conversation),
        ],
    )
}

// ── Onboarding phase builders ─────────────────────────────────────────────────

pub fn build_onboarding_phase_instructions(
    common: &str,
    requested_output_language: &str,
    phase_rules: &str,
) -> String {
    render_template(
        ONBOARDING_PHASE_INSTRUCTIONS,
        &[
            ("common", common),
            ("requested_output_language", requested_output_language),
            ("phase_rules", phase_rules),
        ],
    )
}

pub fn build_onboarding_prompt_input(
    target_phase: &str,
    requested_output_language: &str,
    current_phase: &str,
    last_response_time_tier: &str,
    persona_summary: &str,
    headline_candidate: &str,
    confirmed_headline: &str,
    headline_confirmation_count: usize,
    pending_response_time_tier: &str,
    kickoff_signal: &str,
    conversation: &str,
) -> String {
    let headline_confirmation_count = headline_confirmation_count.to_string();
    render_template(
        ONBOARDING_PROMPT_INPUT,
        &[
            ("target_phase", target_phase),
            ("requested_output_language", requested_output_language),
            ("current_phase", current_phase),
            ("last_response_time_tier", last_response_time_tier),
            ("persona_summary", persona_summary),
            ("headline_candidate", headline_candidate),
            ("confirmed_headline", confirmed_headline),
            ("headline_confirmation_count", &headline_confirmation_count),
            ("pending_response_time_tier", pending_response_time_tier),
            ("kickoff_signal", kickoff_signal),
            ("conversation", conversation),
        ],
    )
}

// ── SDQ builders ──────────────────────────────────────────────────────────────

pub fn build_onboarding_sdq_turn_note(
    is_initial: bool,
    question_index: usize,
    topic: &str,
) -> String {
    let question_index = question_index.to_string();
    let template = if is_initial {
        ONBOARDING_SDQ_INITIAL_TURN_NOTE
    } else {
        ONBOARDING_SDQ_FOLLOWUP_TURN_NOTE
    };
    render_template(
        template,
        &[("question_index", &question_index), ("topic", topic)],
    )
}

pub fn build_onboarding_sdq_phase_instructions(
    step_label: &str,
    question_index: usize,
    topic: &str,
    turn_note: &str,
) -> String {
    let question_index = question_index.to_string();
    render_template(
        ONBOARDING_SDQ_PHASE_INSTRUCTIONS,
        &[
            ("step_label", step_label),
            ("question_index", &question_index),
            ("topic", topic),
            ("turn_note", turn_note),
        ],
    )
}

// ── Explicit correction builders ──────────────────────────────────────────────

pub fn build_explicit_correction_input(message: &str) -> String {
    render_template(EXPLICIT_CORRECTION_INPUT, &[("message", message)])
}

// ── Profile input builder ─────────────────────────────────────────────────────

pub fn profile_input<T: AsRef<str>>(
    kickoff_style_signal: Option<&str>,
    structured_answers: &[T],
) -> String {
    let kickoff_block = kickoff_style_signal
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("Kickoff style signal:\n{value}\n\n"))
        .unwrap_or_default();
    let structured_block = if structured_answers.is_empty() {
        "Structured onboarding answers:\n- none supplied".to_string()
    } else {
        format!(
            "Structured onboarding answers:\n- {}",
            structured_answers
                .iter()
                .map(|answer| answer.as_ref().trim())
                .collect::<Vec<_>>()
                .join("\n- ")
        )
    };

    format!("{kickoff_block}{structured_block}\n\nReturn JSON only.")
}

// ── Chat and preview input builders ──────────────────────────────────────────

pub fn chat_input(
    relevant_onboarding_answers: &[String],
    onboarding_continuity: &[String],
    profile: &ShadowProfile,
    conversation_lines: &[String],
) -> String {
    chat_input_with_reflection_memory(
        relevant_onboarding_answers,
        onboarding_continuity,
        profile,
        conversation_lines,
        &[],
    )
}

pub fn chat_input_with_reflection_memory(
    relevant_onboarding_answers: &[String],
    onboarding_continuity: &[String],
    profile: &ShadowProfile,
    conversation_lines: &[String],
    reflection_summaries: &[String],
) -> String {
    chat_input_with_reflection_memory_and_long_term_context(
        relevant_onboarding_answers,
        onboarding_continuity,
        profile,
        conversation_lines,
        reflection_summaries,
        "Long-term memory selected for this turn:\n- none selected",
    )
}

pub fn chat_input_with_reflection_memory_and_long_term_context(
    relevant_onboarding_answers: &[String],
    onboarding_continuity: &[String],
    profile: &ShadowProfile,
    conversation_lines: &[String],
    reflection_summaries: &[String],
    long_term_memory_block: &str,
) -> String {
    let (ready_profile, ready_persona, ready_reasoning) = prompt_ready_parts(profile);
    let relevant_answers_block = bullet_list_block(
        "Relevant onboarding answers",
        relevant_onboarding_answers,
        "- none selected",
    );
    let onboarding_continuity_block = bullet_list_block(
        "Onboarding continuity",
        onboarding_continuity,
        "- none captured yet",
    );
    let conversation_block = if conversation_lines.is_empty() {
        "none yet".to_string()
    } else {
        conversation_lines.join("\n")
    };
    let reflection_block = reflection_memory_block(
        "normal chat",
        NORMAL_CHAT_REFLECTION_WEIGHT,
        reflection_summaries,
    );
    let long_term_memory_block = normalize_preformatted_block(
        long_term_memory_block,
        "Long-term memory selected for this turn:\n- none selected",
    );

    format!(
        "{relevant_answers_block}\n\n{onboarding_continuity_block}\n\n{long_term_memory_block}\n\n{reflection_block}\n\nOnboarding profile:\n{}\n\nPersona:\n{}\n\nReasoning policy:\n{}\n\nRecent conversation (latest 20 messages max):\n{conversation_block}\n",
        onboarding_profile_json(&ready_profile),
        persona_json(&ready_persona),
        reasoning_policy_json(&ready_reasoning),
    )
}


pub fn preview_input(
    challenge: &ShadowChallenge,
    profile: &ShadowProfile,
    supporting_evidence: &[String],
    answer_lang: &str,
) -> String {
    preview_input_with_reflection_memory(challenge, profile, supporting_evidence, answer_lang, &[])
}

pub fn preview_input_with_reflection_memory(
    challenge: &ShadowChallenge,
    profile: &ShadowProfile,
    supporting_evidence: &[String],
    answer_lang: &str,
    reflection_summaries: &[String],
) -> String {
    use crate::builders::requested_output_language;

    let (ready_profile, ready_persona, ready_reasoning) = prompt_ready_parts(profile);
    let evidence_block = bullet_list_block(
        "Relevant onboarding answers for this prompt",
        supporting_evidence,
        "- none selected",
    );
    let reflection_block = reflection_memory_block(
        "question answer",
        QUESTION_ANSWER_REFLECTION_WEIGHT,
        reflection_summaries,
    );

    format!(
        "Requested answer language: {}\nPrompt tag label: {}\nPrompt context: {}\nPrompt: {}\n\n{evidence_block}\n\n{reflection_block}\n\nOnboarding profile:\n{}\n\nPersona:\n{}\n\nReasoning policy:\n{}\n",
        requested_output_language(answer_lang),
        challenge.tag_label.as_deref().unwrap_or("none supplied"),
        challenge_context(challenge).as_deref().unwrap_or("none supplied"),
        challenge.prompt_text,
        onboarding_profile_json(&ready_profile),
        persona_json(&ready_persona),
        reasoning_policy_json(&ready_reasoning),
    )
}

fn challenge_context(challenge: &ShadowChallenge) -> Option<String> {
    match challenge.preferred_probe_kind {
        Some(crate::SetupProbeKind::InitialWorkstyleProbe) => {
            Some("learning style under time pressure".to_string())
        }
        Some(crate::SetupProbeKind::ProjectShiftReflection) => {
            Some("how this person changes approach mid-project".to_string())
        }
        Some(crate::SetupProbeKind::DeadlineQualityTradeoff) => {
            Some("how this person handles deadline versus quality tradeoffs".to_string())
        }
        None => None,
    }
}

// ── Pair topic / compose input builders ──────────────────────────────────────

#[derive(Debug, Clone)]
pub struct PairTopicPromptContext<'a> {
    pub speaker: &'a PairShadowIdentity,
    pub listener: &'a PairShadowIdentity,
    pub speaker_voice_evidence: &'a [String],
    pub listener_voice_evidence: &'a [String],
    pub requested_output_language: &'a str,
    pub topic_title: &'a str,
    pub topic_prompt: &'a str,
    pub tag_label: Option<&'a str>,
    pub tone: PairTopicTone,
    pub directive: PairTurnDirective,
    pub previous_messages: &'a [String],
    pub recent_topic_results: &'a [String],
    pub transition_note: Option<&'a str>,
    pub relevant_onboarding_answers: &'a [String],
    pub onboarding_continuity: &'a [String],
    pub long_term_memory_block: &'a str,
    pub reflection_summaries: &'a [String],
}

pub fn build_pair_topic_message_input(context: PairTopicPromptContext<'_>) -> String {
    let relevant_answers_block = bullet_list_block(
        "Relevant onboarding answers",
        context.relevant_onboarding_answers,
        "- none selected",
    );
    let onboarding_continuity_block = bullet_list_block(
        "Onboarding continuity",
        context.onboarding_continuity,
        "- none captured yet",
    );
    let long_term_memory_block = normalize_preformatted_block(
        context.long_term_memory_block,
        "Long-term memory selected for this turn:\n- none selected",
    );
    let reflection_block = reflection_memory_block(
        "pair chat",
        NORMAL_CHAT_REFLECTION_WEIGHT,
        context.reflection_summaries,
    );
    let is_normal_pair_chat = context.topic_title == "Normal pair chat";
    let conversation = if context.previous_messages.is_empty() {
        if is_normal_pair_chat {
            "No previous Pair / Topic Talk messages yet.".to_string()
        } else {
            format!(
                "No previous Pair / Topic Talk messages yet; no current-topic thread exists yet. \
                 Do not begin with agreement phrases; there is no previous Shadow line to agree with. \
                 Treat the user instruction as something the speaker wants help sending now. \
                 Generate a complete sendable message through the speaker's voice, then shape it so it \
                 can land naturally for {listener}. Prefer plain chat words over theatrical props or \
                 poetic stage directions.",
                listener = context.listener.name
            )
        }
    } else {
        context.previous_messages.join("\n")
    };
    let speaker_personal_instruction =
        pair_personal_instruction_block(context.speaker.personal_instruction.as_deref());
    let speaker_voice_evidence = pair_voice_evidence_block(context.speaker_voice_evidence);
    let recent_results = if context.recent_topic_results.is_empty() {
        "No recent Pair / Topic Talk results.".to_string()
    } else if is_normal_pair_chat {
        format!(
            "Use these completed Topic Talk results only as light background for normal pair chat. \
             A transition note may ask for a brief bridge, but do not keep discussing the completed topic.\n{}",
            context.recent_topic_results.join("\n")
        )
    } else {
        format!(
            "Do not quote or callback from this section. Use it only for light background. \
             Do not reuse its concrete metaphors, phrases, jokes, or scenes in this Topic Talk.\n{}",
            context.recent_topic_results.join("\n")
        )
    };
    let transition_note = context
        .transition_note
        .map(str::trim)
        .filter(|note| !note.is_empty())
        .map(|note| format!("\nTransition note:\n{note}\n"))
        .unwrap_or_default();
    let callback_boundary = if is_normal_pair_chat {
        "Normal pair chat recall boundary: prior normal chat and recent completed Topic Talk results may inform natural continuity when relevant. Keep any completed-topic bridge brief, and return to the original owners instead of continuing the completed topic."
    } else {
        "Topic Talk callback boundary: callbacks, concrete reuse, and phrase reuse are allowed only from the current Topic Talk messages above and the current topic title/prompt. Recent results, prior normal chat, previous topics, and profile/source material may inform stance and style, but do not quote, callback, or reuse their concrete metaphors, phrases, jokes, or scenes."
    };
    let topic_anchor = if is_normal_pair_chat {
        String::new()
    } else {
        format!(
            "Current instruction: {} — stay grounded in this user instruction. ",
            context.topic_title
        )
    };

    format!(
        "{relevant_answers_block}\n\n\
         {onboarding_continuity_block}\n\n\
         {long_term_memory_block}\n\n\
         {reflection_block}\n\n\
         You are speaking as {speaker} to {listener}.\n\
         Requested output language: {requested_output_language}\n\
         Pair / Topic Talk tone: {tone}\n\
         Turn phase: {phase}\n\
         Suggested wording angle: {move_label} - {move_instruction}\n\
         Topic tag: {tag_label}\n\
         User instruction title:\n{topic_title}\n\n\
         User instruction:\n{topic_prompt}\n\n\
          Speaker Shadow profile:\n{speaker_profile}\n\n\
           Speaker personal instruction:\n{speaker_personal_instruction}\n\n\
           Speaker voice evidence:\n{speaker_voice_evidence}\n\n\
           Previous Pair / Topic Talk messages:\n{conversation}\n\n\
          Recent Pair / Topic Talk results for light background:\n{recent_results}\n\
          {transition_note}\n\
         {callback_boundary} \
         User instruction is the source of intent for this turn. \
         Write the next sendable message that {speaker} will send on behalf of the original user, \
         not as the Shadow's independent opinion. \
         If the instruction asks to explain, research, summarize, persuade, invite, soften, or adjust tone, generate a complete sendable message. \
         Keep the original user's taste and Shadow voice, while making the wording usable for the recipient. \
         Use listener/recipient context to make the message land naturally, but do not let listener context overwrite the speaker's voice. \
         Choose the length based on the instruction, not a fixed short-chat rule. \
         {topic_anchor}Write the next sendable message. \
           Write entirely in the requested output language. Do not mix languages unless quoting a \
           proper noun, product name, or short phrase already present in the topic or previous line. \
           Do not copy the language of profile/source material when it differs from the requested output language. \
           Treat speaker personal instruction, when present, as a high-priority turn constraint for the speaker's wording, stance, formality/register, and behavior. \
           It has higher priority than default tone, speaker profile, and speaker voice evidence; it overrides conflicting voice evidence. \
           Follow it throughout this turn, not only when introducing the topic, but do not quote it or turn it into explicit meta-explanation. \
            Use speaker voice evidence only to shape the speaker's ordinary words, phrasing, distance, and rhythm. \
          Do not treat voice evidence as private facts, concrete scene material, or callback material. \
         Connect to the immediately previous line when it helps the message feel conversational, but do not force a handoff question or unrelated joke. \
         Do not invent concrete private owner facts.",
        speaker = context.speaker.name,
        listener = context.listener.name,
        requested_output_language = context.requested_output_language,
        tone = context.tone.label(),
        phase = context.directive.phase_label(),
        move_label = context.directive.move_kind.label(),
        move_instruction = context.directive.move_kind.instruction(),
        tag_label = context.tag_label.unwrap_or("none supplied"),
        topic_title = context.topic_title,
         topic_prompt = context.topic_prompt,
         speaker_profile = pair_identity_block(&pair_identity_without_personal_instruction(
             context.speaker,
         )),
         speaker_personal_instruction = speaker_personal_instruction,
         speaker_voice_evidence = speaker_voice_evidence,
         conversation = conversation,
         recent_results = recent_results,
         transition_note = transition_note,
        callback_boundary = callback_boundary,
        topic_anchor = topic_anchor,
    )
}

#[derive(Debug, Clone)]
pub struct PairComposePromptContext<'a> {
    pub actor: &'a PairShadowIdentity,
    pub listener: &'a PairShadowIdentity,
    pub actor_voice_evidence: &'a [String],
    pub relevant_onboarding_answers: &'a [String],
    pub onboarding_continuity: &'a [String],
    pub long_term_memory_block: &'a str,
    pub reflection_summaries: &'a [String],
    pub recent_messages: &'a [String],
    pub user_intent: &'a str,
    pub requested_output_language: &'a str,
}

pub fn build_pair_compose_message_input(context: PairComposePromptContext<'_>) -> String {
    let relevant_answers_block = bullet_list_block(
        "Relevant onboarding answers",
        context.relevant_onboarding_answers,
        "- none selected",
    );
    let onboarding_continuity_block = bullet_list_block(
        "Onboarding continuity",
        context.onboarding_continuity,
        "- none captured yet",
    );
    let long_term_memory_block = normalize_preformatted_block(
        context.long_term_memory_block,
        "Long-term memory selected for this turn:\n- none selected",
    );
    let reflection_block = reflection_memory_block(
        "pair chat",
        NORMAL_CHAT_REFLECTION_WEIGHT,
        context.reflection_summaries,
    );
    let actor_voice_evidence = if context.actor_voice_evidence.is_empty() {
        "No voice evidence available.".to_string()
    } else {
        context.actor_voice_evidence.join("\n- ")
    };
    let conversation = if context.recent_messages.is_empty() {
        "No previous messages in this conversation.".to_string()
    } else {
        context.recent_messages.join("\n")
    };
    let actor_profile = pair_identity_block(context.actor);
    let listener_profile = pair_identity_block(context.listener);

    format!(
        "Actor Shadow profile:\n{actor_profile}\n\n\
        Listener Shadow profile:\n{listener_profile}\n\n\
        Actor voice evidence:\n- {actor_voice_evidence}\n\n\
        {relevant_answers_block}\n\n\
        {onboarding_continuity_block}\n\n\
        {long_term_memory_block}\n\n\
        {reflection_block}\n\n\
        Recent conversation:\n{conversation}\n\n\
        Requested output language: {requested_output_language}\n\n\
        The user ({actor_name}) wants to say:\n{user_intent}\n\n\
        Write a single message from {actor_name} to {listener_name} that expresses the above intent \
        in {actor_name}'s authentic voice and personality. \
        Make it natural, on-brand for {actor_name}, and appropriate for the conversation context. \
        Do not add explanations or meta-commentary. \
        Output only the message text in the requested output language.",
        actor_profile = actor_profile,
        listener_profile = listener_profile,
        actor_voice_evidence = actor_voice_evidence,
        relevant_answers_block = relevant_answers_block,
        onboarding_continuity_block = onboarding_continuity_block,
        long_term_memory_block = long_term_memory_block,
        reflection_block = reflection_block,
        conversation = conversation,
        requested_output_language = context.requested_output_language,
        user_intent = context.user_intent,
        actor_name = context.actor.name,
        listener_name = context.listener.name,
    )
}

// ── Private helpers ───────────────────────────────────────────────────────────

fn reflection_memory_block(use_case_label: &str, weight: u8, summaries: &[String]) -> String {
    if summaries.is_empty() {
        return format!(
            "Reflection memory for {use_case_label} (influence {weight}/10):\n- none captured yet"
        );
    }
    format!(
        "Reflection memory for {use_case_label} (influence {weight}/10):\n- {}",
        summaries.join("\n- ")
    )
}

fn prompt_ready_parts(
    profile: &ShadowProfile,
) -> (
    PromptReadyProfile,
    PromptReadyPersona,
    PromptReadyReasoningPolicy,
) {
    (
        PromptReadyProfile {
            headline: profile.headline.clone(),
            stance: profile.stance.clone(),
            source_answers: profile.source_answers.clone(),
        },
        PromptReadyPersona {
            tone: profile.tone.clone(),
            traits: profile.traits.clone(),
            speech_style: profile
                .speech_style
                .as_ref()
                .map(|speech_style| SpeechStyle {
                    dialect: speech_style.dialect.clone(),
                    formality: speech_style.formality.clone(),
                    markers: speech_style.markers.clone(),
                    sentence_pattern: speech_style.sentence_pattern.clone(),
                }),
        },
        PromptReadyReasoningPolicy {
            decision_style: profile.decision_style.clone(),
            anchor: profile.anchor.clone(),
        },
    )
}

fn pair_identity_block(identity: &PairShadowIdentity) -> String {
    serde_json::to_string_pretty(identity).expect("PairShadowIdentity serialization is infallible")
}

fn pair_identity_without_personal_instruction(identity: &PairShadowIdentity) -> PairShadowIdentity {
    let mut identity = identity.clone();
    identity.personal_instruction = None;
    identity
}

fn pair_voice_evidence_block(evidence: &[String]) -> String {
    if evidence.is_empty() {
        return "- none supplied".to_string();
    }
    format!("- {}", evidence.join("\n- "))
}

fn pair_personal_instruction_block(instruction: Option<&str>) -> String {
    instruction
        .map(str::trim)
        .filter(|instruction| !instruction.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| "- none supplied".to_string())
}

pub fn bullet_list_block(header: &str, items: &[String], empty_suffix: &str) -> String {
    if items.is_empty() {
        format!("{header}:\n{empty_suffix}")
    } else {
        format!("{header}:\n- {}", items.join("\n- "))
    }
}

pub fn normalize_preformatted_block(block: &str, fallback: &str) -> String {
    if block.trim().is_empty() {
        fallback.to_string()
    } else {
        block.trim().to_string()
    }
}

fn render_template(template: &'static str, vars: &[(&str, &str)]) -> String {
    PromptTemplate::new(template.trim_end()).render(vars)
}

fn onboarding_profile_json(profile: &PromptReadyProfile) -> serde_json::Value {
    serde_json::to_value(profile).expect("PromptReadyProfile serialization is infallible")
}

fn persona_json(persona: &PromptReadyPersona) -> serde_json::Value {
    serde_json::to_value(persona).expect("PromptReadyPersona serialization is infallible")
}

fn reasoning_policy_json(reasoning: &PromptReadyReasoningPolicy) -> serde_json::Value {
    serde_json::to_value(reasoning).expect("PromptReadyReasoningPolicy serialization is infallible")
}
