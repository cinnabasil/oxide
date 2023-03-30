# Definition of Oxide's Syntax

## EBNF Format

`{element} := {definition}`

x or y
`( x | y )` 

maybe x
`x?`

any amount (including 0) of x
`x*`

1 or more of x
`x+`

## EBNF

```ebnf
program := top_level_item*

top_level_item := function

function := function_attributes 'func' IDENT '(' function_parameters? ')' function_return_type ( block_expression | ';' )

function_attributes := 'impure'?
function_parameters := function_parameter (',' function_parameter)*
function_parameter := type IDENT
function_return_type := '~' return_type

return_type := type '!'?

type := compiler_type
compiler_type := numeric_type
numeric_type := 'i32'


block_expression := '{' statement* '}'
statement := expression ';'
expression := expression_without_block | expression_with_block

expression_without_block := literal_expression 
                            | function_call_expression 
                            | method_call_expression
                            | operator_expression

literal_expression := INTEGER_LIT | FLOAT_LIT | STRING_LIT | 'true' | 'false'

method_call_expression := expression '.' IDENT '(' call_params ')'
function_call_expression := IDENT '(' call_params ')'
call_params := expression (',' expression)*

operator_expression := comparison_expression 
                        | negation_expression 
                        | arithmetic_expression 
                        | boolean_expression

arithmetic_expression := expression '+' expression
                        | expression '-' expression
                        | expression '*' expression
                        | expression '/' expression
                        | expression '&' expression
                        | expression '|' expression

boolean_expression := expression '||' expression
                        | expression '&&' expression

negation_expression := '-' expression | '!' expression

comparison_expression := expression '==' expression
                        | expression '!=' expression
                        | expression '>' expression
                        | expression '<' expression
                        | expression '>=' expression
                        | expression '<=' expression

expression_with_block := if_expression

if_expression := 'if' expression block_expression ('else' (block_expression | if_expression) )?
```
