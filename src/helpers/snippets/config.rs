/// # Example output
///
/// ```json
/// "snippets name": {
///   "scope": "rust",
///   "prefix": "snippets prefix",
///   "body": [
///         "fn example(){",
///         "}
///     ],
///   "description": "snippets description",
/// }
/// ```
pub struct Config {
    scope: &'static str,
    prefix: String,
    body: Vec<String>,
    description: String,
}

impl Config {
    pub fn new(
        scope: &'static str,
        prefix: String,
        body: Vec<String>,
        description: String,
    ) -> Self {
        Config {
            scope,
            prefix,
            description,
            body,
        }
    }

    pub fn into_snippets(self) -> String {
        format!(
            "{{
              \"{}\": {{\n
              \"scope\": \"{}\",\n
              \"prefix\": \"{}\",\n
              \"body\": {:?},\n
              \"description\": \"{}\",\n
            }},
        }}
        ",
            self.prefix, self.scope, self.prefix, self.body, self.description
        )
    }

    pub fn get_name(&self) -> String {
        self.prefix.clone()
    }
}
