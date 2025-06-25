use std::fmt;
use crate::eaql::tokens::{Token};
use crate::eaql::parser::helpers::{get_tab, valid_until_warning, validate_length};

#[derive(Debug)]
pub struct GetNode {
    _table: TableNode,
    _columns: ColumnNode,
    _where: Option<WhereNode>,
    _filter: Option<FilterNode>,
    _postprocessor: Option<PostProcessorNode>,

    _depth: u16
}

#[derive(Debug)]
pub struct TableNode {}    

#[derive(Debug)]
pub struct ColumnNode {
    column_names: Vec<String>,
    is_wildcard: bool,

    _depth: u16 
}

#[derive(Debug)]
pub struct WhereNode {}
#[derive(Debug)]
pub struct FilterNode {}
#[derive(Debug)]
pub struct PostProcessorNode {}

impl GetNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &usize,
        prior: &str,
        depth: u16) -> Result<GetNode, String>{
        let start_idx: usize = *idx;

        validate_length(
            tokens,
            idx,
            prior,
            true)?;

        // Ok(GetNode {
        //     _table: TableNode {},
        //     _columns: ColumnNode { column_names: vec![], is_wildcard: true, _depth: 2 },
        //     _where: Some(WhereNode {}),
        //     _filter: Some(FilterNode {}),
        //     _postprocessor: Some(PostProcessorNode {}),
        //     _depth: depth
        // })
        Err(valid_until_warning(tokens, &start_idx))
    }
}

impl ColumnNode {
    pub fn recurse_build(

    ) -> () {
        
    }

    pub fn parse(
        tokens: &Vec<Token>,
        idx: usize,
        depth: &u16) -> Result<ColumnNode, String> {
        Err("".to_string())
    }
}

impl fmt::Display for GetNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(GetNode){}",
            get_tab(self._depth),
            self._columns,
        )
    }
}

impl fmt::Display for ColumnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(ColumnNode)
{}is_wildcard: {:#?}
{}column_names: {:#?}
",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.is_wildcard,
            get_tab(self._depth + 1),
            self.column_names
        )
    }
}