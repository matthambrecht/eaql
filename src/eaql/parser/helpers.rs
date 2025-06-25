use crate::eaql::tokens::{Token};

const TAB_SIZE: u16 = 2;

pub fn validate_length(
    tokens: &Vec<Token>,
    idx: &usize,
    prior: &str,
    required: bool
) -> Result<(), String>{
    if *idx >= tokens.len() && required {
        return Err(format!(
            "Query was missing expected data after \"{prior}\"!"
        ));
    }

    Ok(())
}

pub fn get_tab(
    depth: u16
) -> String {
    " ".repeat(TAB_SIZE as usize * depth as usize)
}

pub fn valid_until_warning(
    tokens: &Vec<Token>,
    idx: &usize
) -> String {
    if *idx >= tokens.len() {
        return "N/A".to_string();
    }

    format!("Query valid until after \"{}\"!",
        tokens[..*idx].into_iter().map(
        |x| x.lexeme.as_str()
        ).collect::<Vec<&str>>().join(" ")).to_string()
}