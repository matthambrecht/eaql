use std::collections::HashSet;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref DATA_ACTION: HashSet<&'static str> = HashSet::from([
        "get", "change", "delete", "add"
    ]);

    pub static ref WILDCARD_KEYWORD: HashSet<&'static str> = HashSet::from([
        "any", "all"
    ]);

    pub static ref TARGETING_KEYWORD: HashSet<&'static str> = HashSet::from([
        "from", "to"
    ]);

    pub static ref DATA_TARGET: HashSet<&'static str> = HashSet::from([
        "database", "table", "data", "index"
    ]);

    pub static ref FILTER: HashSet<&'static str> = HashSet::from([
        "where", "whenever"
    ]);

    pub static ref POST_PROCESSOR_ENTRANCE: HashSet<&'static str> = HashSet::from([
        "then", "afterwords", "after"
    ]);

    pub static ref POST_PROCESSOR: HashSet<&'static str> = HashSet::from([
        "with", "in"
    ]);

    pub static ref RELATIONAL: HashSet<&'static str> = HashSet::from([
        "and", "or", "not"
    ]);

    pub static ref ASSIGNMENT: HashSet<&'static str> = HashSet::from([
        "="
    ]);

    pub static ref EOS_TOKEN: HashSet<&'static str> = HashSet::from([
        ";", ".", "!"
    ]);

    pub static ref TOKENS: Vec<(&'static HashSet<&'static str>, &'static str)> = vec![
        (&*DATA_ACTION, "DATA_ACTION"),
        (&*TARGETING_KEYWORD, "TARGETING_KEYWORD"),
        (&*DATA_TARGET, "DATA_TARGET"),
        (&*FILTER, "FILTER"),
        (&*POST_PROCESSOR_ENTRANCE, "POST_PROCESSOR_ENTRANCE"),
        (&*POST_PROCESSOR, "POST_PROCESSOR"),
        (&*RELATIONAL, "RELATIONAL"),
        (&*ASSIGNMENT, "ASSIGNMENT"),
        (&*EOS_TOKEN, "EOS_TOKEN")
    ];
}
