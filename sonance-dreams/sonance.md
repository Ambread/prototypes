# Sonance Programming Language

Inspiration:
- [Rust](https://www.rust-lang.org/)
- [Kotlin](https://kotlinlang.org/)
- [Swift](https://developer.apple.com/swift/)
- [Smalltalk](https://en.wikipedia.org/wiki/Smalltalk)


## Features 

## - Chain Operator

Unifies function call syntax and method call syntax from other languages.

`foo(arg1, arg2, arg3)` is interchangeable with `arg1.foo(arg2, arg3)`.

When applied repeatably, this allows you to *chain* function calls. 
```
foo().add(12).bar().join().baz().negate()
```
The calls are evaluated left-to-right on the result of the previous call.

Without this feature, the expression would be written in a mix of function calls, regular operators, and chains. 
```
!baz(bar(foo() + 12).join())
```
This makes it more difficult to understand what is being executed and when.


The accepted solution is to introduce intermediate variables. 
```
let rawThing = foo() + 12;
let thing = bar(rawThing).join();
let finalThing = !baz(thing);
```
This preferable to the previous example, but has downsides of its own. The programmer is now required to create unique names for each split, and also remember which is which. Additionally, the values now have the potential of being used more than once or mutated. This isn't bad for things that should be variables, but they add extra overhead for intermediate values.


## Examples

```
import {
    std { compare.Ordering, io.stdin }, 
    random { Random, thread_rng },
};

func main() {
    print_line("Guess a number 0 to 100");

    let correct = range(from: 0, to: 100).random(using: thread_rng());
    let mut guess = String.new();

    loop {
        guess.clear()
        stdin().read_line(into: &mut guess).expect();

        let guess = guess.trim().parse().else {
            print_line("Try again");
            return@loop;
        };

        correct.compare_to(&guess).match {
            Ordering.Greater -> print_line("Greater"),
            Ordering.Less -> print_line("Less"),
            Ordering.Equal -> block {
                print_line("Correct");
                return@main;
            },
        };
    };
};
```

## Syntax

EBNF-ish
- `UPPERCASE` : terminals 
- `lowercase` : non-terminals 
- `< >` : parameters
- `"text"` : literal terminals
- `|` : or
- `?` : zero or one
- `*` : zero or more
- `+` : one or more
- `( )` : grouping

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
