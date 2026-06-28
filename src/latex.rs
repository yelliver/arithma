use crate::environment::Environment;
use crate::evaluator::Evaluator;
use crate::node::Node;
use crate::parser::build_expression_tree;
use crate::simplify::Simplifiable;
use crate::tokenizer::Tokenizer;

fn parse_latex_expr(latex: &str) -> Result<Node, String> {
    let mut tokenizer = Tokenizer::new(latex);
    let tokens = tokenizer.tokenize();
    if let Some(err) = tokenizer.errors.into_iter().next() {
        return Err(err);
    }
    build_expression_tree(tokens).map_err(|e| format!("Error parsing LaTeX: {}", e))
}

/// Parse once, then format, simplify, and optionally evaluate.
pub(crate) fn process_latex(
    latex: &str,
    env_json: &str,
) -> Result<(String, String, Option<String>), String> {
    let env: Environment = serde_json::from_str(env_json)
        .map_err(|e| format!("Failed to parse environment: {}", e))?;

    let expr = parse_latex_expr(latex)?;
    let formatted = format!("{}", expr);
    let simplified = expr
        .simplify(&env)
        .map_err(|e| format!("Error simplifying: {}", e))?;
    let simplified_str = format!("{}", simplified);
    let decimal = Evaluator::evaluate(&simplified, &env)
        .ok()
        .map(|v| v.to_string());

    Ok((formatted, simplified_str, decimal))
}

pub(crate) fn format_latex(latex: &str) -> Result<String, String> {
    Ok(format!("{}", parse_latex_expr(latex)?))
}

pub(crate) fn simplify_latex(latex: &str) -> Result<String, String> {
    process_latex(latex, r#"{"vars":{}}"#).map(|(_, simplified, _)| simplified)
}

pub(crate) fn evaluate_latex(latex: &str, env_json: &str) -> Result<String, String> {
    let (_, simplified, decimal) = process_latex(latex, env_json)?;
    Ok(decimal.unwrap_or(simplified))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_latex_once() {
        let (formatted, simplified, decimal) = process_latex("1 + 1", r#"{"vars":{}}"#).unwrap();
        assert_eq!(formatted, "1 + 1");
        assert_eq!(simplified, "2");
        assert_eq!(decimal.as_deref(), Some("2"));
    }

    #[test]
    fn format_implicit_mul() {
        assert_eq!(format_latex("3x").unwrap(), "3x");
    }

    #[test]
    fn format_does_not_simplify() {
        assert_eq!(format_latex("1 + 1").unwrap(), "1 + 1");
    }

    #[test]
    fn simplify_addition() {
        assert_eq!(simplify_latex("1 + 1").unwrap(), "2");
    }

    #[test]
    fn simplify_sqrt_12() {
        assert_eq!(simplify_latex(r"\sqrt{12}").unwrap(), r"2\sqrt{3}");
    }

    #[test]
    fn evaluate_numeric() {
        assert_eq!(evaluate_latex("1 + 1", r#"{"vars":{}}"#).unwrap(), "2");
    }
}
