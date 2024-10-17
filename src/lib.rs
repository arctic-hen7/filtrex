use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "filtrex.pest"]
struct FiltrexParser;
