use crate::utils;

const LOGO_ASCII: &str = "
 \t\x1b[90m ▀▀▀▀▀▀▀▀▀▀  \x1b[94m  ▗▄▄▄▖ ▗▄▖ ▗▄▄▄▖ ▗▖   \x1b[0m       
 \t\x1b[90m ██████████  \x1b[94m  ▐▌   ▐▌ ▐▌▐▌ ▐▌ ▐▌   \x1b[0m     
 \t\x1b[90m ▄▄▄▄▄▄▄▄▄▄  \x1b[94m  ▐▛▀▀▘▐▛▀▜▌▐▌ ▐▌ ▐▌   \x1b[0m     
 \t\x1b[90m ▄▄▄▄▄▄▄▄▄▄  \x1b[94m  ▐▙▄▄▖▐▌ ▐▌▐▙▄▟▙▖▐▙▄▄ \x1b[0m    
 \t\x1b[90m ▀▀▀▀▀▀▀▀▀▀  \x1b[0m                                   
══════════════════════════════════════════════════
";

const HELP_MENU: &str = r#"
       EAQL - Test Environment Help Menu

Usage:
    cargo run <module>

Modules:
    transpile: EAQL -> SQL Language Transpiler
    validate: Query Validitor
    
═══════════════════════════════════════════════════
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
