# letpl

letpl is a toy programming langauge based on the example langauges in the book _Essentials of Programming Languages_ by Daniel P. Friedman and Mitchell Wand.

# Language

letpl's grammar is:

    Program    ::= Expression

    Expression ::= Number
               ::= "-" "(" Expression "," Expression ")"
               ::= "zero?" "(" Expression ")"
               ::= "if" Expression "then" Expression "else" Expression
               ::= Identifier
               ::= "let" Identifier "=" Expression "in" Expression
               ::= "proc" "(" Identifier ":" Type ")" Expression
               ::= "(" Expression Expression ")"
               ::= "letrec" Type Identifier "(" Identifier ":" Type ")" Expression "in" Expression

    Type       ::= "int"
               ::= "bool"
               ::= "(" Type "->" Type ")"

Comments start with `#` and end at a line feed.

Numbers are 64 bit floating point values.

`-(x, y)`, where `x` and `y` are numbers, evaluates to `x - y`.

`zero?(x)`, where `x` is a number, evaluates to `true` if `x` is `0`, otherwise it evaluates to `false`.

`if` evaluates to the consequent expression if its guard expression is `true`, otherwise it evaluates to the alternative expression.

An identifier evaluates to the value bound to it. All identifiers are lexically scoped.

`let` binds an identifier to a value of the expression after `=`, and then evaluates the expression after `in`. An identfier may be a letter followed by letters and/or digits.

`proc` creates a procedure object of one variable. `letrec` creates and binds to a name a procedure which can recursively call itself. All procedures are closures. `(f x)` calls the procedure `f` with the argument `x`.
