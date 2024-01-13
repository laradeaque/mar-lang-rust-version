# Mar Programming Lang(Rust-Implementation)

Mar is a modern and easy-to-learn programming language designed for various 
applications, including database softwares and automation.

*Consider checking the main repo - (Mar lang)[https://github.com/laradeaque/mar-lang]
for Python implementation which is simpler and easier to follow.*

This repo focuses on rebuilding the same project(Mar lang) using Rust as the earlier
version was done in Python. 

Generally, I have learnt alot, hope you can learn also something by collaborating in 
the project. 

So far, I have completed this features and functions:
   - Function and variables declaration.
   - Function execution
   - Data Types -> {Int, Float, Bool, Str, None, Vector}
   - Expression evaluation

I have introduced improvement some features:
1. Lazy Evaluation - Main new feature. Aimed to increase speed and reduce memory
   and compute time wastage.

##How does **lazy evaluation** work? 
Example program
```Mar
let message = "Hello"

# This will not be executed(or asked for)
let long_expression = 1000 + 1200 - 343 - 54.55 - 555 - 54 - 324 / 32; 

println(message)

# Thousands of lines.. 
```
 Although the variable `long_expression` is not super big and our program is not 
 that big think of a longer expression and a program extending hundreds of lines 
 if not thousands.

 So it would be useless to evaluate the content of variable `long_expression` 
 assuming it will never be asked for in the program. Therefore we could all at 
 once leave it unevaluated. unless we need.

 That where **Lazy evaluation** comes in. Expressions are said to be lazy because 
 we do not evaluate them upon encoutering them.
 
 Note how beneficial it is. We can save compute time by not evaluating anything 
 that is not to be used in the program.

 You should note that a variable in the program can contain large logs of embedded 
 expressions as the functions are lazy e.g.
```Mar
let a = 3;
let b = 4;

let c = b - a;              # You expect 1, but its not 1 i am explaining below
let d = 5;

let e = c + d; 
let f = e * a; 
``` 
In memory 
c = {b - a}
e = {{b - a} + d}
f = {{{b - a} + d} * a} 

Note as we continue using more and more variables the logs of each variable enlarges

##How do we prevent **lazy evaluation** from trying to access variables out of scope?
We prevent such occasions by changing forcing evaluation for variable going **moving** 
out of scope.
**moving** -> being retured either by a function. this is because once we are out of
 function, the local variables are dropped.
So far this feature has not raised  any issue/bug, but with more testing I hope it shall
be more reliable.

2. Differentiation of `print` and `println` - This is to add flexibility to the 
      output.

`print` - does not add a line feed at the end of printing;
`println` - adds a line feed at the end of printing;
Example
```Mar
print("Hello, ")
print("World!\n")

println("Hello, World!") 
```

#Things I hope to do in future versions
- Add more functionality, Classes, Loops, Match, If e.t.c
- Write language libraries, math, date, sys, net, files e.t.c
- Borrowing and Ownership system (For Mar)
- Restructure code.
- Modularize Interpreter.
- Optimize Interpreter

... - Have you in the dev team.    :)

Read other files e.g. Challenges, MarWiz
