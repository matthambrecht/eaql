# eaql - Syntax

The core aim of eaql's syntax structure is to convert the core goals achieved by other database query languages like SQL into a more english form.


## Query Formats
This is how specific queries are supposed to be formatted to be properly understood and parsed by the EAQL Parser. To get a better understanding of why they are this way and how queries are processed from tokens into a usable format see [parsing](#parsing).

### Table Accessing Queries
Table accessing queries require 4 parts: target table, associated columns, filters (limiting queries based on table values), and post-processors (i.e. limiting number of results).

#### Choosing a Table Name
- **Format**: [[Get Keyword](#get-keywords)] ([Column Selection](#choosing-target-columns)) [[From Keyword](#from-keywords)] {Table Name}
- **Example**: `get column_name from table_name`

#### Choosing Target Columns
- **Format**: Column(s) can be listed in standard english listing format or as a comma-separated list.
- **Example**: `column_1, column_2 and column_3`

#### Filters
- **Format**: This is just listed as a mathematical boolean expression. Keep in mind, order of operations is determined by parentheses and ordring at this point in time, this *will* not be the end format and is a more simplified placeholder for furture versions. This means that `a and b or c and c` is treated as `(a and (b or (c and (c))))`. If you don't understand this concept please see this first: [logical expressions](https://runestone.academy/ns/books/published/thinkcspy/Selection/Logicaloperators.html).
- **Example**: `cost < 15 and (expiration_year > 2026 or best_by_date_exists = False)`

- [Logical Keywords](#logical-keywords)
- [Logical Operators](#logical-operators)

#### Post-Processor Documentation
![](../images/utils/under_construction.png)

## Technical Documentation
### BNF (Brackus-Naur Form)
See [Syntax BNF](./SYNTAX.ebnf)


### Tokenization
Sample Query:
- `Get everything from drinks whenever the price is 5 and the category is "coffee" then sort it in ascending order.`
Query Tokenized: 
```json
Lexer { 
    Tokens: [
        Token {
            token_type: Get,
            literal: "get",
            lexeme: "get",
        },
        Token {
            token_type: WildcardKeyword,
            literal: "everything",
            lexeme: "everything",
        },
        Token {
            token_type: From,
            literal: "from",
            lexeme: "from",
        },
        Token {
            token_type: Identifier,
            literal: "drinks",
            lexeme: "drinks",
        },
        Token {
            token_type: FilterKeyword,
            literal: "where",
            lexeme: "where",
        },
        Token {
            token_type: Identifier,
            literal: "price",
            lexeme: "price",
        },
        Token {
            token_type: Equal,
            literal: "is",
            lexeme: "is",
        },
        Token {
            token_type: NumberLiteral,
            literal: "5",
            lexeme: "5",
        },
        Token {
            token_type: And,
            literal: "and",
            lexeme: "and",
        },
        Token {
            token_type: Identifier,
            literal: "category",
            lexeme: "category",
        },
        Token {
            token_type: Equal,
            literal: "is",
            lexeme: "is",
        },
        Token {
            token_type: StringLiteral,
            literal: "coffee",
            lexeme: "\"coffee\"",
        },
        Token {
            token_type: PostProcessorEntrance,
            literal: "then",
            lexeme: "then",
        },
        Token {
            token_type: Sort,
            literal: "sort",
            lexeme: "sort",
        },
        Token {
            token_type: SortHelper,
            literal: "in",
            lexeme: "in",
        },
        Token {
            token_type: SortType,
            literal: "ascending",
            lexeme: "ascending",
        },
        Token {
            token_type: Order,
            literal: "order",
            lexeme: "order",
        },
        Token {
            token_type: EoqToken,
            literal: "",
            lexeme: ".",
        },
    ]
}
```

## Parsing
See [Parser Documentation](./PARSER.md)

## Keyword Glossary
### Get Keywords
Indicates that the current query is for data retrieval from a table.

- Get
- Retrieve
- Find

### From Keywords
Indicates during retrieval which table we are getting data from.

- From
- In

### Logical Keywords
- And
- Or

### Logical Operators
- `> (Greater Than)`
- `>= (Greater Than or Equal to)`
- `< (Less Than)`
- `<= (Less Than or Eaql to)`
- `= (Equal to)`