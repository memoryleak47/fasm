# syntax
Program ::= Line | Line; Line
Line ::= FunctionName Pattern = Output
Pattern ::= Var | Atom | Tuple<Pattern>
Output ::= Var | Atom | Tuple<Output> | FunctionName ( Output )

Var ::= $string
FunctionName ::= string
Atom ::= :string

There exists a `main` function getting `argv[1]` as argument.
