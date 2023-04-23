use super::*;
use crate::ast_util::range;
use std::convert::Infallible;

use full_moon::{
    ast::{self, Ast},
    visitors::Visitor,
};

pub struct FunctionNameCasingLint;

fn lowercase_first_char(string: &str) -> String {
    let mut chars = string.chars();
    let first_char = chars.next().unwrap().to_lowercase().to_string();
    let rest = chars.as_str().to_string();
    first_char + &rest
}

impl Lint for FunctionNameCasingLint {
    type Config = ();
    type Error = Infallible;

    const SEVERITY: Severity = Severity::Warning;
    const LINT_TYPE: LintType = LintType::Style;

    fn new(_: Self::Config) -> Result<Self, Self::Error> {
        Ok(FunctionNameCasingLint)
    }

    fn pass(&self, ast: &Ast, _: &Context, _: &AstContext) -> Vec<Diagnostic> {
        let mut visitor = FunctionNameCasingVisitor::default();

        visitor.visit_ast(ast);

        let mut diagnostics = Vec::new();

        for (name, position) in visitor.uppercase_name {
            diagnostics.push(Diagnostic::new_complete(
                "function_name_casing",
                "function names must start with a lowercase letter".to_owned(),
                Label::new(position),
                vec![format!("try: {}(...) instead", lowercase_first_char(&name))],
                Vec::new(),
            ));
        }

        diagnostics
    }
}

#[derive(Default)]
struct FunctionNameCasingVisitor {
    uppercase_name: Vec<(String, (usize, usize))>,
}

impl Visitor for FunctionNameCasingVisitor {
    fn visit_function_declaration(&mut self, declaration: &ast::FunctionDeclaration) {
        let function_name = declaration.name();

        if let Some(name) = function_name.method_name() {
            if name.to_string().chars().next().unwrap().is_uppercase() {
                self.uppercase_name.push((name.to_string(), range(name)));
            }
        } else if let Some(name) = function_name.names().iter().last() {
            if name.to_string().chars().next().unwrap().is_uppercase() {
                self.uppercase_name.push((name.to_string(), range(name)));
            }
        }
    }

    fn visit_local_function(&mut self, function: &ast::LocalFunction) {
        let name_str = function.name().to_string();
        if name_str.chars().next().unwrap().is_uppercase() {
            self.uppercase_name.push((name_str, range(function.name())));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{super::test_util::test_lint, *};

    #[test]
    fn test_function_name_casing() {
        test_lint(
            FunctionNameCasingLint::new(()).unwrap(),
            "function_name_casing",
            "function_name_casing",
        );
    }
}
