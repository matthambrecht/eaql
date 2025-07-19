# eaql - Parser
## Basic Concept
All queries start as a query node which can be branched off into 3 different actions based on the first keyword (table searching, table mutation, database modification).

## Table Searching
Table searches are parsed into 4 subnodes. The table to be searched, the columns in that table, a filter which uses optional [conditional parsing](#conditional-parsing) to narrow down queries, and a post processor to do things like limit the number of results retrieved. The table and column parsing are pretty self explanatory.

### Conditional Parsing
- Rules:
- Start with an "OR" node at the beginning always
- When you get a literal token evaluate it to its base expression and that evaluated expression will go on the left side of an "AND" node. Where that "AND" node goes is your next location as determined by future rules.
- If you run across an "and" keyword, go to the next available location and add an "AND" node:
```
       (AND)                      (AND)
    /        \           ->    /        \
  (EXPR)   Unvisited        (EXPR)     (AND)
```
- If you run across an "or" keyword, close out until an ancestor `"OR" node, and add an "OR" node as the child.
```
            (OR)                        (OR)
          /                          /         \
        (AND)                      (AND)        (OR)
      /       \           ->        / \        /
   (AND)    Unvisited           (AND)  True   Unvisited
  /    \                        /   \
(EXPR) Unvisited             (EXPR)  True
```
- If you run across opening parentheses add an "OR" node
```
          (OR)                       (OR)
        /     \                     /    \
    (AND)     Unvisited   ->     (AND)   Unvisited
    /   \                        /    \
(EXPR)  Unvisited             (EXPR)  (OR)
                                    /      \
                              Unvisited   Unvisited
```
- If you run across a closing parentheses, close out ancestor "OR" node. (If next keyword is "and", add an "AND node to your LS and set its LS to your LS and start from that child's RS, same thing happens with OR you just add an "OR" node).
```
            (OR)                              (OR)    
           /    \                            /   \
        (AND)   Unvisited                 (AND)   Unvisited
       /    \                            /    \
    (EXPR)  (OR)              ->      (EXPR)  (OR)
          /      \                            /   \
      (AND)   Unvisited                    (AND)    False
    /      \                              /     \
(EXPR)    Unvisited                    (EXPR)    True
```
## Table Mutation
![](../images/utils/under_construction.png)

## Database Modification
![](../images/utils/under_construction.png)