use super::*;
use crate::ast_util::range;
use std::convert::Infallible;

use full_moon::{
    ast::{self, Ast},
    visitors::Visitor,
};
use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
#[serde(default)]
pub struct FunctionNameCasingLintConfig {
    use_uppercase: bool,
    allow: Vec<String>,
}

pub struct FunctionNameCasingLint {
    config: FunctionNameCasingLintConfig,
}

fn correct_name_casing(string: &str, use_uppercase: bool) -> String {
    let mut chars = string.chars();
    let first_char = chars.next().unwrap();
    let rest_chars = chars.as_str().to_string();

    if use_uppercase {
        first_char.to_uppercase().to_string() + &rest_chars
    } else {
        first_char.to_lowercase().to_string() + &rest_chars
    }
}

fn is_wrong_casing(string: &str, use_uppercase: bool) -> bool {
    let first_char = string.chars().next().unwrap();

    if use_uppercase {
        first_char.is_lowercase()
    } else {
        first_char.is_uppercase()
    }
}

impl Lint for FunctionNameCasingLint {
    type Config = FunctionNameCasingLintConfig;
    type Error = Infallible;

    const SEVERITY: Severity = Severity::Warning;
    const LINT_TYPE: LintType = LintType::Style;

    fn new(config: Self::Config) -> Result<Self, Self::Error> {
        Ok(FunctionNameCasingLint { config })
    }

    fn pass(&self, ast: &Ast, _: &Context, _: &AstContext) -> Vec<Diagnostic> {
        let mut visitor = FunctionNameCasingVisitor {
            wrong_casing: Vec::new(),
            config: self.config.clone(),
        };

        visitor.visit_ast(ast);

        visitor
            .wrong_casing
            .into_iter()
            .filter(|(name, _)| {
                !self
                    .config
                    .allow
                    .iter()
                    .any(|allowed_name| allowed_name == name)
            })
            .map(|(name, position)| {
                Diagnostic::new_complete(
                    "function_name_casing",
                    format!(
                        "function names must start with {} letter",
                        if self.config.use_uppercase {
                            "an uppercase"
                        } else {
                            "a lowercase"
                        }
                    ),
                    Label::new(position),
                    vec![format!(
                        "try: {}(...) instead",
                        correct_name_casing(&name, self.config.use_uppercase)
                    )],
                    Vec::new(),
                )
            })
            .collect()
    }
}

struct FunctionNameCasingVisitor {
    wrong_casing: Vec<(String, (usize, usize))>,
    config: FunctionNameCasingLintConfig,
}

impl Visitor for FunctionNameCasingVisitor {
    fn visit_function_declaration(&mut self, declaration: &ast::FunctionDeclaration) {
        let function_name = declaration.name();

        if let Some(name) = function_name.method_name() {
            if is_wrong_casing(&name.to_string(), self.config.use_uppercase) {
                self.wrong_casing.push((name.to_string(), range(name)));
            }
        } else if let Some(name) = function_name.names().iter().last() {
            if is_wrong_casing(&name.to_string(), self.config.use_uppercase) {
                self.wrong_casing.push((name.to_string(), range(name)));
            }
        }
    }

    fn visit_local_function(&mut self, function: &ast::LocalFunction) {
        let name = function.name().to_string();
        if is_wrong_casing(&name, self.config.use_uppercase) {
            self.wrong_casing.push((name, range(function.name())));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test_util::test_lint, *};

    #[test]
    fn test_function_name_casing() {
        test_lint(
            FunctionNameCasingLint::new(FunctionNameCasingLintConfig::default()).unwrap(),
            "function_name_casing",
            "function_name_casing",
        );
    }

    #[test]
    fn test_use_upper() {
        test_lint(
            FunctionNameCasingLint::new(FunctionNameCasingLintConfig {
                use_uppercase: true,
                ..Default::default()
            })
            .unwrap(),
            "function_name_casing",
            "use_upper",
        );
    }

    #[test]
    fn test_allow() {
        test_lint(
            FunctionNameCasingLint::new(FunctionNameCasingLintConfig {
                allow: vec!["Foo".to_string(), "Baz".to_string()],
                ..Default::default()
            })
            .unwrap(),
            "function_name_casing",
            "allow",
        );
    }
}
