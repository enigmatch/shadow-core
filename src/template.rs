use crate::error::{find_unmatched_placeholder, ShadowCoreError};

pub struct PromptTemplate<'a> {
    template: &'a str,
}

impl<'a> PromptTemplate<'a> {
    pub fn new(template: &'a str) -> Self {
        Self { template }
    }

    pub fn render(&self, vars: &[(&str, &str)]) -> Result<String, ShadowCoreError> {
        let mut result = self.template.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{key}}}"), value);
        }
        if let Some(unmatched) = find_unmatched_placeholder(&result) {
            return Err(ShadowCoreError::TemplateMissingPlaceholder(unmatched));
        }
        Ok(result)
    }
}
