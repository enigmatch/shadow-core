#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairTopicTone {
    Funny,
    CasualValues,
    Relationship,
    WorkDev,
    SeriousReflective,
}

impl PairTopicTone {
    pub fn label(self) -> &'static str {
        match self {
            Self::Funny => "Funny",
            Self::CasualValues => "Casual values",
            Self::Relationship => "Relationship",
            Self::WorkDev => "Work/dev",
            Self::SeriousReflective => "Serious reflective",
        }
    }

    pub fn directive_for_turn(self, turn_index: usize, total_turns: usize) -> PairTurnDirective {
        if turn_index + 1 >= total_turns {
            return PairTurnDirective {
                tone: self,
                move_kind: PairTurnMove::SharedLanding,
                turn_index,
                total_turns,
            };
        }

        let move_kind = match self {
            Self::Funny => match turn_index {
                0 | 1 => PairTurnMove::MicroScene,
                2 => PairTurnMove::Riff,
                3 => PairTurnMove::ChaosOption,
                4 => PairTurnMove::AbsurdEscalation,
                _ => PairTurnMove::GroundedPunchline,
            },
            Self::CasualValues => match turn_index {
                0 | 1 => PairTurnMove::MicroScene,
                2 => PairTurnMove::EmotionalSnap,
                3 => PairTurnMove::SidewaysQuestion,
                4 => PairTurnMove::LightPressureTest,
                _ => PairTurnMove::HandoffQuestion,
            },
            Self::Relationship => match turn_index {
                0 | 1 => PairTurnMove::PlayfulCallout,
                2 => PairTurnMove::SidewaysQuestion,
                3 => PairTurnMove::EmotionalSnap,
                4 => PairTurnMove::SoftRoast,
                _ => PairTurnMove::HandoffQuestion,
            },
            Self::WorkDev => match turn_index {
                0 | 1 => PairTurnMove::HotTake,
                2 => PairTurnMove::MicroScene,
                3 => PairTurnMove::GroundedPunchline,
                4 => PairTurnMove::WeirdHypothesis,
                _ => PairTurnMove::HandoffQuestion,
            },
            Self::SeriousReflective => match turn_index {
                0 | 1 => PairTurnMove::MicroScene,
                2 => PairTurnMove::HotTake,
                3 => PairTurnMove::SidewaysQuestion,
                4 => PairTurnMove::Callback,
                _ => PairTurnMove::HandoffQuestion,
            },
        };
        PairTurnDirective {
            tone: self,
            move_kind,
            turn_index,
            total_turns,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PairTurnMove {
    Riff,
    AbsurdEscalation,
    PlayfulCallout,
    WeirdHypothesis,
    Callback,
    MicroScene,
    HotTake,
    SidewaysQuestion,
    SoftRoast,
    ChaosOption,
    GroundedPunchline,
    EmotionalSnap,
    HandoffQuestion,
    LightPressureTest,
    SharedLanding,
}

impl PairTurnMove {
    pub fn label(self) -> &'static str {
        match self {
            Self::Riff => "Riff",
            Self::AbsurdEscalation => "Absurd escalation",
            Self::PlayfulCallout => "Playful callout",
            Self::WeirdHypothesis => "Weird hypothesis",
            Self::Callback => "Callback",
            Self::MicroScene => "Micro scene",
            Self::HotTake => "Hot take",
            Self::SidewaysQuestion => "Sideways question",
            Self::SoftRoast => "Soft roast",
            Self::ChaosOption => "Chaos option",
            Self::GroundedPunchline => "Grounded punchline",
            Self::EmotionalSnap => "Emotional snap",
            Self::HandoffQuestion => "Handoff question",
            Self::LightPressureTest => "Light pressure test",
            Self::SharedLanding => "Shared landing",
        }
    }

    pub fn instruction(self) -> &'static str {
        match self {
            Self::Riff => "build directly on the other Shadow's idea",
            Self::AbsurdEscalation => "push the shared idea one strange step further",
            Self::PlayfulCallout => "lightly call out the other Shadow's hidden motive",
            Self::WeirdHypothesis => "create a strange but interesting hypothesis",
            Self::Callback => "bring back an earlier phrase, image, or scene",
            Self::MicroScene => "make a small concrete scene the other Shadow can grab",
            Self::HotTake => "say a rough but memorable opinion",
            Self::SidewaysQuestion => "ask from an unexpected angle",
            Self::SoftRoast => "tease the other Shadow lightly without derailing",
            Self::ChaosOption => "offer the weirdest useful option",
            Self::GroundedPunchline => "turn a wild idea into a practical or dry punchline",
            Self::EmotionalSnap => "show a brief strong feeling, then leave room for reaction",
            Self::HandoffQuestion => "end with a small question or unfinished idea that slightly challenges or shifts the premise",
            Self::LightPressureTest => "gently test the assumption while keeping the thread alive",
            Self::SharedLanding => {
                "react to the previous line and leave one concrete final image for the result to pick up"
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PairTurnDirective {
    pub tone: PairTopicTone,
    pub move_kind: PairTurnMove,
    pub turn_index: usize,
    pub total_turns: usize,
}

impl PairTurnDirective {
    pub fn phase_label(self) -> &'static str {
        if self.turn_index == 0 {
            "opening spark"
        } else if self.turn_index + 1 >= self.total_turns {
            "final landing"
        } else if self.turn_index + 2 >= self.total_turns {
            "landing approach"
        } else {
            "continuation"
        }
    }
}

pub fn pair_topic_initial_message_transition_note() -> &'static str {
    "Use a plain natural opening only if it helps the message feel conversational. Treat the user instruction as the speaker's own reason for writing now, not as an assignment from someone else. Create a sendable message with a concrete angle, opinion, emotional reaction, or small social detail from the instruction. Do not explain why the instruction started. Shape the wording so it can land naturally for the recipient."
}

pub fn pair_normal_pair_chat_handoff_transition_note() -> &'static str {
    "For this next message only, use the completed Topic Talk result as light background for the original owner. Briefly acknowledge it only if it helps the sendable message feel natural, then return to what the original owner can say to the recipient now. Do not keep discussing the completed topic."
}
