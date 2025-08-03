# Scanning
## Preface
Any programming language you come across starts with "scanning". The process of scanning involves taking user provided input (whether it be a code or query), and ensuring it can actually be turned into a format that the interpreter can actually understand.

## How This Ties to EAQL
In the case of EAQL, a user may provide a query like:

`Get me everything from drinks.`

To a reader this is easily understood, but to a computer this is just a string of characters that are meaningless. To make this useful to the computer we need to perform lexical analysis/tokenizing to convert the user input into a more meaningful representation.

### Lexing

When a user provides that query to EAQL, this is what our lexer ends up spitting out.

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

You'll notice pretty much every "word" becomes a token and a select few are actually missing from the representation. This is intentional and actually ends up being very similar to how your brain reads this sentence. When you read the sentence you don't think too deeply but this is what is actually happening behind the scenes:

1. "**get**" tells me that I likely want something.
2. "**me**" tells me that it's for me. As long as I get to see it, it really doesn't matter who that thing is intended for.
3. "**everything**" means that I want all that can be found from wherever I'm looking.
4. "**from**" means that I am likely about to figure out where the aforementioned "everything" is coming from.
5. "**drinks**" indicates that I want "everything" that I'm looking for to come from whatever "drinks" is.
6. "**.**": A period tells me that we know all that we need to know from the sentence that I've just read, and now I'm done.

You can see every our lexer took all of the parts of that sentence that built up meaning and split them up. The understanding of how they are interlinked is a bit premature but this shows us why we chose to spit the sentence the way we did and removed parts that really provided no extra meaning (like "me").

You may notice that each token is comprised of 3 different parts, this is what they tell us.
- **Token Type:** This is where we get our understanding of what the token does and allows us to distinguish that words like "create" and "make" inherently mean the exact same thing to us.
- **Literal:** For most tokens this doesn't matter much, but for literal tokens (StringLiteral, NumberLiteral), this allows us to remember what the original value was since the intention of literal tokens is to reuse that value later on (i.e. if I see a "4" I want to always remember that it's a "4", and not just that I saw a number).
- **Lexeme:** All tokens have this and this is just the original value of the token. Because something like "!" or "." ends up becoming an EoqToken (End of Query token) we may need to remember at some point what that originally looked like.

### How We Did It

Lexing may seem super simple at first, particulary for English sentences, and you may think you can just split at the spaces and call it a day. The problem with this lies in the fact that something like "a>5" is equivalent to "a > 5". So we can't just do that. To combat this we can build somewhat of a trial-and-error process where we increase the length until we find something that is valid, then move forward and repeat until we have nothing left.

#### Single-Character Tokens
We start with single character tokens: ">", "=", "(", etc. If at any point we see these at the beginning of a new iteration of lexing we know we have a valid token. Some of these on their own may be a valid token, but what happens when we have ">=" and we just call it a day by making a "Greater Than" and an "Equal To" token? We end up modifying our lexical understanding, which is bad.

#### Double-Character Tokens
To avoid this we define a second category of tokens. We'll call these our "two character tokens." If we see any of those one character tokens we first need to check if it's a token that could be followed by another token, if not, we just call this token exactly what it is, and move forward to our next iteration. Otherwise, we want to look forward (peek) one character to confirm if it's a related token ahead of us. If it is we combine them into one token (\<Gt> + \<Eq> = \<Gte>), otherwise we keep it as is, make it into a single character token, and move forward to our next iteration.

#### Literals
What happens now if we haven't found a valid single token to start with? We look for a literal. This is any string or number. This is actually the reason you don't see programming lanaguages that allow you to make variable names start with numbers. For EAQL we look for a quote to indicate a StringLiteral, and any valid number character to start a NumberLiteral ('-', or any number). Number parsing is a large can of worms with trial-and-error, there's a lot of resources online for this so I'll leave this to someone else to explain, but string parsing is fairly simple as we can just look for an end quote and whatever is between the two quotes is our literal.

#### Identifiers and Keywords
At this point if we haven't found a matching start character or a token, we can assume we either have an identifier, or a keyword. To do this we just keep moving forward until our current match either matches a keyword in our keyword store, or we see a valid start token (anything from the prior sections). If it's a keyword in our keyword store it becomes the keyword token mapped to that particular lexeme, keep in mind there may be multiple lexemes for one keyword, otherwise it becomes an identifier (variable name, column name, function name, etc). This is actually why in a language like Python you can overwrite "print" but you can't overwrite "if" by setting it equal to something else. "print" is an identifier for a function name while "if" is an internal keyword.

### What Next?
To get a better understanding of our token typing you may want to take a look at our [Backus–Naur form](./EAQL.ebnf) definition of EAQL. This will define all tokens and the relations they have to eachother in a relatively easy to understand format. To learn more about and how to understand a grammar defined in "Backus-Naur form", visit [here](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form).

Now that we know how we can turn a string of characters into a strong lexical representation, we need to figure out how to make use of that to give our computer a structural representation of the things those tokens indicated need to be done. This is where [parsing](./PARSING.md) comes in.
