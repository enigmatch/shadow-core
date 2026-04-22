pub struct PromptTemplate<'a> {
    template: &'a str,
}

impl<'a> PromptTemplate<'a> {
    pub fn new(template: &'a str) -> Self {
        Self { template }
    }

    pub fn render(&self, vars: &[(&str, &str)]) -> String {
        let mut result = self.template.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{key}}}"), value);
        }
        result
    }
}
