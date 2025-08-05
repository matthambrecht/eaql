use eaql::{
    transpiler,
    validator,
    utils,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    utils::help::display_logo();

    if args.len() < 2 {
        utils::help::display_help(None);
        panic!("Abort: Invalid number of params");
    }

    match args[args.len() - 1].as_str() {
        "transpile" => transpiler::transpiler::engine(),
        "query_test" => validator::validator::engine(),
        _ => utils::help::display_help(
           Some(format!("Invalid Testing CLI Argument -> {}, see usage!", args[2]).as_str()))
    }
}