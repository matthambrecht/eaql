# eaql - Syntax

The core aim of eaql's syntax structure is to convert the core goals achieved by other database query languages like sql into a more english form.

## BNF
See [Syntax BNF](./SYNTAX.ebnf)


## Tokenization
Sample Query:
- `get everything from drinks where price is 5 and category is "coffee" then sort in ascending order.`

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