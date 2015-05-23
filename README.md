# rscheme

A toy scheme interpreter written in Rust

Usage
-----

Running the program just launches a pseudo-scheme REPL.

Features
--------

### Numbers ###

Integers and Floats only.

`+`, `-`, `*`, `/`, `=` are builtin.

### List ###

Building a list is possible either with  `cons` or `'`.

`car` and `cdr` are also bultins.

### Lambdas ###

`(lambda (args) body)`

Body can only be one expression.

### def ###

Yeah it should be `define` but it's `def`.

`(def x 42)`

`(def f (lambda (x) (* 2 x)))`

`(f x) ; returns 84`


Not implemented (yet?)
----------------------

* comments
* macros
* variadic args
* multiple definition of same-name func but different args number
* input/output
* including another file

Well, it's only a toy project, to learn Rust, right ? You want some real Scheme/Lisp interpreter, there's plenty to find.