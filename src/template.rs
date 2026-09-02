pub struct PromptTemplate<'a> {
    template: &'a str,
}

impl<'a> PromptTemplate<'a> {
    pub fn new(template: &'a str) -> Self {
        Self { template }
    }

    pub fn render(&self, vars: &[(&str, &str)]) -> String {
        let mut result = String::with_capacity(self.template.len());
        let mut cursor = 0;

        while let Some(open_offset) = self.template[cursor..].find('{') {
            let open = cursor + open_offset;
            result.push_str(&self.template[cursor..open]);

            let Some(close_offset) = self.template[open + 1..].find('}') else {
                result.push_str(&self.template[open..]);
                cursor = self.template.len();
                break;
            };
            let close = open + 1 + close_offset;
            let key = &self.template[open + 1..close];

            if let Some((_, value)) = vars.iter().rev().find(|(candidate, _)| *candidate == key) {
                result.push_str(value);
            } else {
                result.push_str(&self.template[open..=close]);
            }
            cursor = close + 1;
        }

        if cursor < self.template.len() {
            result.push_str(&self.template[cursor..]);
        }
        result
    }
}
