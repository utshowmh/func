# Grammer of Func

```
program     ->      stmt* comment*

comment     ->      "#" ... "\n"

stmt        ->      expr | let

let         ->      "let" ident "=" expr
expr        ->      unary | binary | group | literal

unary       ->      "-" expr
binary      ->      expr op expr
group       ->      "(" expr ")"
literal     ->      int | float | ident

int         ->      [0-9]+
float       ->      int"."int
indet       ->      [a-zA-Z]+int*

op          ->      "+" | "-" | "*" | "/" | "%"
```
