use crate::prettyprint::constants::*;
use std::collections::HashMap;
use string_template_plus::*;

pub struct PrettyPrinter {}

pub struct TemplateVars {
    verb: Option<String>,

    kind: Option<String>,
    path: Option<String>,
    error: Option<String>,
}

impl Default for TemplateVars {
    fn default() -> Self {
        Self::new(None, None, None, None)
    }
}

impl TemplateVars {
    pub fn new(
        verb: Option<String>,
        kind: Option<String>,
        path: Option<String>,
        error: Option<String>,
    ) -> Self {
        Self {
            verb: verb,
            kind: kind,
            path: path,
            error: error,
        }
    }
}

impl PrettyPrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_version(&self) {
        println!("{} version {}", PROJECT_NAME, VERSION);
        println!("Author: {}", AUTHOR);
    }

    pub fn print_prefix(&self) {
        print!("{}: ", PROJECT_NAME);
    }

    pub fn print_help_short(&self) {
        println!("Try {}", HELP);
    }

    pub fn print_help(&self) {
        println!("Usage: {} {}", PROJECT_NAME, USAGE);
        println!("{}", HELP_TEXT);
    }
    pub fn print_template(&self, template: &str, vars: TemplateVars) {
        let template = Template::parse_template(template).unwrap();
        let vars_map = HashMap::from([
            ("verb".to_string(), vars.verb.unwrap_or("".to_string())),
            ("kind".to_string(), vars.kind.unwrap_or("".to_string())),
            ("path".to_string(), vars.path.unwrap_or("".to_string())),
            ("error".to_string(), vars.error.unwrap_or("".to_string())),
        ]);
        let render = template
            .render(&RenderOptions {
                variables: (vars_map),
                ..Default::default()
            })
            .unwrap();
        self.print_prefix();
        println!("{}", render);
    }

    pub fn print_error(&self, error: &str) {
        self.print_template(
            ERROR_TEMPLATE,
            TemplateVars {
                error: Some(error.to_string()),
                ..Default::default()
            },
        );
    }
}
