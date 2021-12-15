# Sonance Programming Language

Inspiration:
- [Rust](https://www.rust-lang.org/)
- [Kotlin](https://kotlinlang.org/)
- [Swift](https://developer.apple.com/swift/)
- [Smalltalk](https://en.wikipedia.org/wiki/Smalltalk)

## Syntax

EBNFish
- UPPERCASE : terminals 
- lowercase : non-terminals 
- < > : parameters
- "text": literal terminals
- | : or
- ? : zero or one
- * : zero or more
- + : one or more
- ( ) : grouping

```cs
list<T, sep = ","> = 
    | T sep list<T, sep>
    | T sep?

file = item*

item = function_item

function_item = "func" IDENTIFIER generic_parameter_list? parameter_list type block

generic_parameter_list = "<" list<generic_parameter> ">"

generic_parameter = IDENTIFIER type_bounds?

type_bounds = ":" type

parameter_list = "(" list<parameter> ")"

parameter = parameter_label? pattern type

parameter_label = IDENTIFIER ":"

pattern = "mut"? IDENTIFIER

type = IDENTIFIER type_parameter_list?

type_parameter_list = "<" list<type_parameter> ">"

type_parameter = parameter_label? type

block = "{" list<statement, sep = ";"> "}"

statement = let_statement | expression

let_statement = "let" pattern type? "=" expression

expression = "(" expression ")" | property_expression | chain_expression | call_expression | literal_expression

property_expression = expression "." IDENTIFIER 

chain_expression = expression "." call_expression

call_expression = IDENTIFIER type_parameter_list? arguments

arguments = argument_list | argument_list? block_argument+

argument_list = "(" list<argument> ")"

argument = parameter_label? expression

block_argument = parameter_label? block

match_expression = match_head "{" list<match_clause> "}"

match_head = 
    | "match" "(" expression ")"
    | expression "." "match"

match_clause = pattern "->" expression

literal_expression = STRING_LITERAL | INTEGER_LITERAL | FLOAT_LITERAL
```
