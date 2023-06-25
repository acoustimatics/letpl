# letpl
An implementation of the LET language from Chapter 3 of "Essentials of Programming Languages"

# Language

LET's grammar is:

    Program    ::= Expression
    Expression ::= Number
               ::= "-" "(" Expression "," Expression ")"
               ::= "zero?" "(" Expression ")"
               ::= "if" Expression "then" Expression "else" Expression
               ::= Identifier
               ::= "let" Identifier "=" Expression "in" Expression

Numbers are 64 bit floating point values.

`-(x, y)` evaluates to `x` - `y`.

`zero?(x)` evaluates to `true` if `x` is `0`, otherwise it evaluates to `false`.

`if` evaluates to the consequent expression if its guard expression is `true`, otherwise it evaluates to the alternative expression.

`let` binds an identifier to a value of the expression after `=`, and then evaluates the expression after `in`. An identfier may be a letter followed by letters and/or digits.

An identifier evaluates to the value bound to it.

# Implementation

The implementation uses a recursive descent parser and an AST walking interpreter.
