use rust_embed::Embed;

#[derive(Embed)]
#[folder = "../../templates/"]
pub struct Templates;

impl Templates {
    pub fn get_template(path: &str) -> Option<String> {
        Self::get(path).map(|f| String::from_utf8_lossy(&f.data).to_string())
    }

    pub fn list_templates(prefix: &str) -> Vec<String> {
        Self::iter()
            .filter(|path| path.starts_with(prefix))
            .map(|path| path.to_string())
            .collect()
    }
}
