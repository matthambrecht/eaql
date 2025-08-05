# Transpiling
## Preface
In Computer Science, "transpiling" involves taking one computer language and converting it to another. This is different from "compiling" where we take one computer language, and convert it either to an intermediate representation that can be converted to a machine instructions (see [LLVM](https://llvm.org/)), or directly to machine instrcutions.

Since EAQL is meant to be a learning tool and I didn't just want to go ahead and only make an interpreter which didn't feel like much of a challenge, I decided to also include a transpiler from EAQL to SQL. This increases the utility of EAQL as it'll inveitably allow users to interact with any SQL database using natural language, and can also be used to actually learn SQL.

SQL on it's own isn't a hard language to learn and is already pretty close to natural language (as far as a computer language goes) but it felt like a really cool concept to see if we could stretch that to the limit and make a query language as english-like as possible.

## How It Works
Once parsing was achieved transpiling was actually really easy to figure out. All we had to do to achieve this was give each node a set of transpiling instructions. We'll once again visit our sample query:

`Get me everything from drinks.`

and our AST

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

Rather than using a whole new tree for transpiling we just define what each node does with its children
in order to transpile.

```
                                "{GetNode};"
                                     |
 "SELECT {ColumnNode} {TableNode} {FilterNode | ''} {PostProcessorNode | ''}"
                            /                 \
                 "FROM {name}"  "{(wildcard) ? * : columns}"
                      |                       |
                 name: drinks           wildcard: True
                                          columns: []
```

To make it a bit more user friendly we also add some ANSI color codes, but thats about it, you can see how simple it ends up being.