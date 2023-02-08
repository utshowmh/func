# Grammer of Func

```
program     ->      stmt*
stmt        ->      expr
expr        ->      unary | binary | group
unary       ->      "-" expr
binary      ->      expr op expr
group       ->      "(" expr ")"
op          ->      "+" | "-" | "*" | "/"
literal     ->      int
int         ->              [0-9]+
indet       ->              [_a-zA-Z]+ [0-9]*
```
