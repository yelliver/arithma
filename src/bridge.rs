use crate::latex::{evaluate_latex, format_latex, process_latex, simplify_latex};

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// Single-parse pipeline: formatted canonical LaTeX, simplified form, optional decimal.
pub struct ProcessLatexResult {
    pub formatted: String,
    pub simplified: String,
    pub decimal: Option<String>,
}

#[flutter_rust_bridge::frb(sync)]
pub fn process_latex_bridge(latex: String, env_json: String) -> Result<ProcessLatexResult, String> {
    let (formatted, simplified, decimal) = process_latex(&latex, &env_json)?;
    Ok(ProcessLatexResult {
        formatted,
        simplified,
        decimal,
    })
}

#[flutter_rust_bridge::frb(sync)]
pub fn format_latex_bridge(latex: String) -> Result<String, String> {
    format_latex(&latex)
}

#[flutter_rust_bridge::frb(sync)]
pub fn simplify_latex_bridge(latex: String) -> Result<String, String> {
    simplify_latex(&latex)
}

#[flutter_rust_bridge::frb(sync)]
pub fn evaluate_latex_bridge(latex: String, env_json: String) -> Result<String, String> {
    evaluate_latex(&latex, &env_json)
}
