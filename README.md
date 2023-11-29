# letpl

letpl is a toy programming langauge based on example langauges in the book _Essentials of Programming Languages_ by Daniel P. Friedman and Mitchell Wand.

# Language

letpl's grammar is:

    Program    ::= Expression

    Expression ::= Number
               ::= "zero?" "(" Expression ")"
               ::= "-" "(" Expression "," Expression ")"
               ::= "-" "(" Expression ")"
               ::= "if" Expression "then" Expression "else" Expression
               ::= Identifier
               ::= "let" Identifier "=" Expression "in" Expression
               ::= "proc" "(" Param ")" Expression
               ::= "(" Expression Expression ")"
               ::= "letrec" Type Identifier "(" Param ")" Expression "in" Expression
               ::= "assert" Expression "then" Expression

    Param      ::= Identifier ":" Type

    Number     ::= "0" .. "9"
    
    Identifier ::= Letter ( Letter | Digit | "_" | "?" )*

    Letter     ::= ( "A" .. "Z" ) | ( "a" .. "z" )

    Digit      ::= "0" .. "9"

    Type       ::= "int"
               ::= "bool"
               ::= "(" Type "->" Type ")"

Comments start with `#` and end at a line feed.

Numbers are integer values of type `int`.  There is a runtime error if a number cannot fit into a 64 bit signed integer.

`zero?(x)` evaluates to `true` if `x` is `0`, otherwise it evaluates to `false`.  The expression `x` must evaulate to an `int`.

`-(x, y)` evaluates to `x - y`.  Both expressions `x` and `y` must evaluate to an `int`.

`-(x)` evaluates to `-x`. `x` must evaluate to an `int`.

`if guard then consequent else alternative` evaluates to `consequent` if `guard` is `true`, otherwise it evaluates to `alternative`. The expression `guard` must evaulate to a `bool`.  The expressions `consequent` and `alternative` must have the same type.

An identifier evaluates to the value bound to it.  All identifiers are lexically scoped.

`let id = initializer in body` binds an identifier `id` to the value of the expression `initializer`.  Then the expression `body` is evaluated which becomes the value of the `let` expression as a whole.  The identier `id` is in scope in the expression `body`.  An identfier may be a letter followed by letters, digits, underscores, and/or question marks.

`proc` creates a procedure object of one variable which may be bound to an identifer with `let`. `letrec` creates and binds to an identifer a procedure which can recursively call itself. All procedures are closures. `(f x)` calls the procedure `f` with the argument `x`.  `f` must be an expression that evaulates to a procedure type.  `x` must be an expression that evaulates to the parameter type of `f`.

`assert guard then body` evaluates to the expression `body` if the expression `guard` is `true`.  If `guard` is false there is a runtime error.  The expression `guard` must be of type `bool`.
