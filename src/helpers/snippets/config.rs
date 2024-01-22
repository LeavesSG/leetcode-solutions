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
        let headers = format!(
            "{{
  \"{}\": {{
    \"scope\": \"{}\",
    \"prefix\": \"fn {}\",
    \"body\":",
            self.prefix, self.scope, self.prefix
        );
        let body = self.body.iter().fold(String::from(""), |a, c| {
            return a + format!(
                "
      \"{}\",",
                c
            )
            .as_str();
        });
        let footer = format!(
            "\"description\": \"{}\"
  }},
}}",
            self.description
        );
        format!(
            "{}[
{}
    ],
    {}",
            headers, body, footer
        )
    }

    pub fn get_name(&self) -> String {
        self.prefix.clone()
    }
}
