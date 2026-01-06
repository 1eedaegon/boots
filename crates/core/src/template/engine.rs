use std::collections::HashMap;

pub struct TemplateEngine {
    variables: HashMap<String, String>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) -> &mut Self {
        self.variables.insert(key.to_string(), value.to_string());
        self
    }

    pub fn render(&self, template: &str) -> String {
        let mut result = template.to_string();
        for (key, value) in &self.variables {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }
}

impl Default for TemplateEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple() {
        let mut engine = TemplateEngine::new();
        engine.set("name", "test-project");

        let result = engine.render("Project: {{name}}");
        assert_eq!(result, "Project: test-project");
    }

    #[test]
    fn test_render_multiple_variables() {
        let mut engine = TemplateEngine::new();
        engine.set("name", "my-app");
        engine.set("version", "0.1.0");

        let template = "name = \"{{name}}\"\nversion = \"{{version}}\"";
        let result = engine.render(template);

        assert!(result.contains("name = \"my-app\""));
        assert!(result.contains("version = \"0.1.0\""));
    }
}
