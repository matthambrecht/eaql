# Parsing
## Preface
Given a lexical representation of our input we need some way to indicate to the computer what each token means and how they relate to eachother. This representation is known as an "Abstract Syntax Tree" or AST for short. You can essentially think this as your program/query in tree form.

 For the sake of EAQL, we use top-down parsing with [recursive-descent](https://en.wikipedia.org/wiki/Recursive_descent_parser). This means that all queries start as a query node which can be branched off into 3 different actions based on the first keyword (table searching, table mutation, database mutation). We continue to branch off from these based on the tokens that we see.

## How We Do it
Lets look at an example of the tree for our sample query:

`Get me everything from drinks.` 

becomes

```
                          (QueryNode)
                               |
                           (GetNode)
                        /           \
                 (TableNode)      (ColumnNode)
                      |                |
                 name: drinks     wildcard: True
                                  columns: []
```

A lot of parsing ends up being relatively simple and repetative at some times but there are some complex problems that we'll go over. Our AST ends up being defined by our grammar which can be found [here](./EAQL.ebnf). Essentially what we want when defining our grammar is to ensure that for every state, our language defines all possible following states we wish to support. This makes it such that when we arrive at any token, we can always assert exactly what we "expect" to see next, and if we don't see what we expect, we can easily tell the user exactly where or why something is wrong.

Lets revisit our example query tokenized:

```
Tokens: [                                        
    Token {                                              
        token_type: Get,                                 
        literal: "",                                     
        lexeme: "Get",                                   
    },                                                   
    Token {                                              
        token_type: WildcardKeyword,                     
        literal: "",                                     
        lexeme: "everything",                            
    },                                                   
    Token {                                              
        token_type: From,                                
        literal: "",                                     
        lexeme: "from",                                  
    },                                                   
    Token {                                              
        token_type: Identifier,                          
        literal: "drinks",                               
        lexeme: "drinks",                                
    },                                                   
    Token {                                              
        token_type: EoqToken,                            
        literal: "",                                     
        lexeme: "!",                                     
    },                                                   
]
```

What our parser will do is go through our entire token list in order. As previously mentioned, all queries start with a query node, this'll just allow us to chain queries together. We get to our first token, a "Get" token. This indicates that our QueryNode will need to point to a `GetNode` because out of the predefined states for a `QueryNode` (table accessing, table mutation, and database mutation), that is the one that matches our current state. We can then move forward a token. Our `GetNode` asserts that we only have 4 possible next states, two are required (choosing a table, and choosing a column or columns), and two are optional (filtering and post-processing). We also assert that the first thing that must be done is selecting a column, columns, or all data. This means that our current state must either be a WildcardKeyword, or an Identifier, otherwise it's an invalid state.

If we get a wildcard token we know our `ColumnNode` is done and can move on, otherwise we may want to check to see if the identifier is in a list form (\<Identifer> \<And> \<Identifier>) and extract all identifiers from that list. This is a relatively self-explanatory process and all you need to know is that by the end of the process our current token pointer must end after all of the content of the `ColumnNode` (\<Get> \<WildcardKeyword> (\<From>) \<Identifier> \<EoqToken>).

We can then move to our next state which we've defined must be the selection of a target table. We can ensure we're in this state by defining that we must see a "From" keyword and that our next token must be an "Identifier". The table name of our `TableNode` becomes the "literal" of our "Identifier".

By now you can hopefully understand what parsing entails. It ends up being a very tedious process and oftentimes repetative but once you've defined some of the logic you'll begin to see that you can reuse a lot of it elsewhere.

There are however some areas where parsing gets pretty complex, here are some areas and a deep dive into how I solved them:
- [Conditional Parsing](./CONDITIONAL.md)

## What Next?

Now that we have been able to provide a good representation of our language we can actually begin to use it to solve the problems we initally wanted to solve. Controlling the database, and transpiling. As of right now development on the database is halted until enough of the language has been defined for simple control, but once we have enough this document will be updated to account for how we manage to link the language to the database. Transpiling, however, is being done alongside language development, and is explained [here](./TRANSPILING.md)