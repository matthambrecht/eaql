mod eaql;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    utils::help::display_logo();

    if args.len() <= 2 {
        panic!("Abort: Invalid number of params");
    }

    match args[2].as_str() {
        "eaql_repl" => eaql::repl::run(),
        _ => utils::help::display_help(
           Some(format!("Invalid Testing CLI Argument -> {}, see usage!", args[2]).as_str()))
    }
}
