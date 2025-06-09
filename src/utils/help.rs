use crate::utils;

const LOGO_ASCII: &str = "
▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄▄
█                                                    █
█ \x1b[90m ▗▄▄▄▄▄▄▄▄▄▄ \x1b[94m  ▗▄▄▄▖ ▗▄▖ ▗▄▄▄▖ ▗▖      ▗▄▄▄  ▗▄▄▖\x1b[0m  █  
█ \x1b[90m ▜██████████▘\x1b[94m  ▐▌   ▐▌ ▐▌▐▌ ▐▌ ▐▌      ▐▌  █ ▐▌ ▐▌\x1b[0m █ 
█ \x1b[90m ▗▄▄▄▄▄▄▄▄▄▄ \x1b[94m  ▐▛▀▀▘▐▛▀▜▌▐▌ ▐▌ ▐▌   ▀▀▘▐▌  █ ▐▛▀▚▖\x1b[0m █ 
█ \x1b[90m ▜██████████▘\x1b[94m  ▐▙▄▄▖▐▌ ▐▌▐▙▄▟▙▖▐▙▄▄▖   ▐▙▄▄▀ ▐▙▄▞▘\x1b[0m █ 
█                                                    █ 
▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀▀
";

const HELP_MENU: &str = r#"
     EAQL DB - Test Environment Help Menu

Usage:
    cargo run main.rs <module>

Modules:
    eaql_repl: EAQL Repl for Testing Transpiling to SQL
══════════════════════════════════════════════════════
"#;

pub fn display_logo() -> () {
    print!("{LOGO_ASCII}");
}

pub fn display_help(msg: Option<&str>) -> () {
    print!("{HELP_MENU}");

    if let Some(s) = msg {
        utils::logger::error(s);
    }
}
