# Conditional Parsing
## Preface
Since we use recursive descent for parsing it's not easy to assert precendence in conditionals and if we followed the structure we use most other places we'd end up having to evaluate expressions left-to-right which would end up being problematic for expressions like `a or b and c` which would be treated as `(a or b) and c` which is not correct. To combat this a lot of languages use the [shunting yard algorithm](https://en.wikipedia.org/wiki/Shunting_yard_algorithm) to first convert expressions into reverse polish notation, which makes left-to-right parsing feasible.

To challenge myself I decided to figure out my own algorithm to do the exact same thing but in one pass. This led to a ton of trial-and-error, and countless failed attempts, tons of drawing parse trees, and a lot of hours. In the end I was able to figure it out, I'm pretty happy with my final result which ended up with the following rules and resulted with an `O(n)` time and space algorithm that parses to a tree capable of postorder traversal evaluation.

## Rules
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

## Conditional Parsing Rules as Pseudocode
```python
# "and" keyword requires us to open an and node and continue parsing from
# there
def handle_and():
    return Op {
        _type: "AND",
        _ls: recurse_down(),
        _rs: recurse_down()
    }

# If we see an "or" we need to let recurse down know to close
# things out until we see an "open" "OR" node
def handle_or():
    set closing_or = True
    return handle_close()

# If we see an opening parentheses we open an "OR" node
def handle_open_paren():
    set ret = Op {
        _type: "OR",
        _ls: recurse_down(),
        _rs: recurse_down()
    }

    # After we are done parsing the open parentheses's tree we need
    # to check if add an "AND" if the next keyword is "and" and start parsing
    # on the right or "OR" if the next keyword is "or". If the next keyword
    # is another closing parentheses we can continue as normal.
    if tok == AND_TOK:
        set closing_paren = False
        *idx += 1

        return Op {
            _type: "AND",
            _ls: ret,
            _rs: recurse_down()
        }
    else if tok == OR_TOK:
        set closing_paren = False
        *idx += 1    

        return Op {
            _type: "OR",
            _ls: ret,
            _rs: recurse_down()
        }
    else if tok == CLOSE_PAREN:
        *idx += 1

        return ret
    else:
        return Err("Something went wrong parsing a nested conditional, expected a closing parentheses, 'and' or 'or', but got '{token}' instead.")

# Hanlde closing paren
def handle_close_paren():
    set closing_paren = True

    return handle_close()

# This is just a helper that defaults nodes for us to booleans
# when "closing" out
def handle_close():
    return parent_node == "AND" ? Bool(True) : Bool(False)

# When get a literal token we add an "AND" node and evaluate the base expression
# to the left of that node, while continuing our parsing on the right
def handle_literal():
    set ls = ExpressionNode::parse_child();
    *idx += 1;
    set rs = recurse_down();

    return Op {
        _type: "AND"
        _ls: ls,
        _rs: rs,
    }

def parse_child():
    if tok == AND_TOK:
        *idx += 1

        return handle_and()

    else if tok == OR_TOK:
        *idx += 1

        return handle_or()

    else if tok == OPEN_PAREN:
        *idx += 1

        return handle_open_paren()

    else if tok == CLOSE_PAREN:
        *idx += 1

        return handle_close_paren()

    else if tok == END_OF_CONDITIONAL:
        *idx += 1

        if closing_paren:
            return Err("Found end of conditional, but there was an unclosed parentheses!")

        set finished = True

        return handle_close()

    else if tok == LITERAL:
        return handle_literal()

    else:
        Err("Unexpected token while parsing conditional expresion -> {token}")

def recurse_down():
    # We aren't finished yet so we might need to parse
    # properly
    if finished:
        return handle_close()
    
    # If we have seen a closing or we need to close out
    # unless our parent node is an OR, then we need to add
    # and OR node and begin parsing from there
    if closing_paren:
        return handle_close()
    else if closing_or:
        if parent_node == "OR":
            set closing_or = False

            return Op {
                _type: "OR",
                _ls: recurse_down(),
                _rs: recurse_down()
            }
        else:
            return handle_close()

    return parse_child()

def parse_conditional():
    return Op {
        type: "OR",
        _ls: recurse_down(),
        _rs: recurse_down()
    }
```