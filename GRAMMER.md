# Grammer of Func

```
program         ->      stmt* comment*

comment         ->      "#" ... "\n"

stmt            ->      let_stmt | print_stmt | block_stmt

let_stmt        ->      "let" ident "=" expr
print_stmt      ->      "print" "(" expr ")"
block_stmt      ->      "{" stmt* "}"

expr            ->      unary_expr | binary_expr | group_expr | literal_expr | ident_expr

unary_expr      ->      "-" expr
binary_expr     ->      expr op expr
group_expr      ->      "(" expr ")"
ident _expr     ->      ident
literal _expr   ->      number | nil

nil             ->      "nil"
number          ->      [0-9]+ "." [0-9]+
indet           ->      [_a-zA-Z]+ [0-9]+*

op              ->      "+" | "-" | "*" | "/" | "%"
```
