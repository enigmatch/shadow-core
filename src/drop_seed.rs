#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DropDefinition {
    pub id: &'static str,
    pub conflict_axis: &'static str,
    pub prompt_en: &'static str,
    pub prompt_ja: &'static str,
    pub prompt_fr: &'static str,
}

impl DropDefinition {
    #[must_use]
    pub fn prompt_for_locale(self, locale: &str) -> &'static str {
        match locale {
            "ja" => self.prompt_ja,
            "fr" => self.prompt_fr,
            _ => self.prompt_en,
        }
    }
}

pub const DROP_DEFINITIONS: [DropDefinition; 5] = [
    DropDefinition {
        id: "family_trip_or_friends",
        conflict_axis: "loyalty_vs_obligation",
        prompt_en: "If a family trip and a plan with close friends landed on the same day, which one would you protect first?",
        prompt_ja: "家族旅行と親しい友達との予定が同じ日に重なったら、まずどちらを優先する？",
        prompt_fr: "Si un voyage en famille et un plan avec des amis proches tombaient le meme jour, lequel protegerais-tu en premier ?",
    },
    DropDefinition {
        id: "truth_or_peace",
        conflict_axis: "honesty_vs_harmony",
        prompt_en: "If telling the full truth would hurt the atmosphere but hiding it would feel dishonest, where do you draw the line?",
        prompt_ja: "本当のことを言うと場の空気は悪くなるけど、隠すのも不誠実だと感じるとき、どこで線を引く？",
        prompt_fr: "Si dire toute la verite abime l'ambiance mais que la cacher te semble malhonnete, ou traces-tu la ligne ?",
    },
    DropDefinition {
        id: "speed_or_craft",
        conflict_axis: "speed_vs_quality",
        prompt_en: "When speed keeps everyone moving but careful work protects the result, which side do you betray last?",
        prompt_ja: "スピードがみんなを前に進める一方で、丁寧さが結果を守るとしたら、最後まで裏切りたくないのはどっち？",
        prompt_fr: "Quand la vitesse fait avancer tout le monde mais que le travail soigne protege le resultat, quel cote refuses-tu de trahir en dernier ?",
    },
    DropDefinition {
        id: "solo_recovery_or_being_reached",
        conflict_axis: "autonomy_vs_care",
        prompt_en: "When you are worn down, what feels more important to protect first: space to recover alone or being reached by someone who notices?",
        prompt_ja: "しんどいとき、先に守りたいのは一人で立て直す余白と、気づいて声をかけてもらえることのどっち？",
        prompt_fr: "Quand tu es epuise, qu'est-ce qui te semble le plus important a proteger d'abord : l'espace pour te reconstruire seul ou le fait que quelqu'un te tende la main ?",
    },
    DropDefinition {
        id: "fairness_or_special_person",
        conflict_axis: "fairness_vs_attachment",
        prompt_en: "If being fair to everyone clashes with protecting one irreplaceable person, which responsibility gets heavier for you?",
        prompt_ja: "みんなに公平でいることと、替えのきかない一人を守ることがぶつかったら、あなたにとって重い責任はどっち？",
        prompt_fr: "Si etre juste avec tout le monde entre en conflit avec proteger une personne irremplacable, quelle responsabilite pese le plus pour toi ?",
    },
];

#[must_use]
pub fn render_drop_definitions_for_locale(locale: &str, used_drop_ids: &[String]) -> String {
    DROP_DEFINITIONS
        .iter()
        .map(|definition| {
            let used_marker = if used_drop_ids.iter().any(|id| id == definition.id) {
                "used"
            } else {
                "available"
            };
            format!(
                "- id: {} | axis: {} | status: {} | question: {}",
                definition.id,
                definition.conflict_axis,
                used_marker,
                definition.prompt_for_locale(locale)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
