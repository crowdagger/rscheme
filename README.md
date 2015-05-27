# rscheme

A toy scheme interpreter written in Rust.

Dependencies
------------
You'll need the Rust compiler and
Cargo. See there : {http://www.rust-lang.org/install.html.

Usage
-----
`$ cargo run`

should build and run rscheme. Running the program just launches a
pseudo-scheme REPL, then you can enter (pseudo-)scheme code.

The file `data/init.scm` is loaded when `rscheme` is launched. If you
run it from another directory, it won't work as well and you'll miss
some features.

Features
--------

### Numbers ###

Integers and Floats only (corresponding to `i64` and `f64`
respectively). 

`_+`, `_-`, `_*`, `_/`, `_=` are builtin; `init.scm` also provides
wrappers `+`, `-`, `*`, `/`, `=`. This is so these functions can be
used as first class functions (which isn't possible for primitives).

### List ###

Building a list is possible either with  `cons` or `'`.

`car` and `cdr` are also available.

### Lambdas ###

`(lambda (args) body)`

Body can only be one expression.

### def ###

Def is a builtin primitive allowing to map variables to values: 

`(def x 42)`

`(def f (lambda (x) (* 2 x)))`

`(f x) ; returns 84`

There is also the more standard `(define (name arg1 ... argn) expr1
... exprn)`, but it is implemented via a macro.

### Macros ###

There is support for macros. E.g., `if` is a builtin, but `cond`
isn't. So let's implement it:

```scheme
(defmacro cond (preds)
  `(if ,(car (car preds))
       ,(cadr (car preds))
       ,(if (nil? (cdr preds))
            ()
            `(cond ,(cdr preds)))))
```

### Let ###

Another example of macro usage is the definition of `let`, which isn't a
primitive but is built as a macro. So

```scheme
(let ((x 2)
      (y 3))
     (+ x y))
```

will be expanded to:

```scheme
((lambda (x y) (+ x y)) 2 3)
```

Variadic arguments
-------------------
It is possible to define functions taking an arbitrary number of
arguments. Syntax is a bit different than classic Scheme, though it is
roughly the same principle:

```scheme
(define (f x1 x2 &xs)
        (println x1)
        (println x2)
        (println xs))
```

`(f 1 2 3 4 5)` will print `1`, `2`, and `(3 4 5)` (on different
lines).

It's the same syntax for declaring as first parameter in lambdas:

`(lambda (& args) (println args)`

Input/output
------------
There is one print primitive, called `_print`. The function `println`
uses it, as can other functions do. It simply prints a string to
standard output.

There is not yet support of input.


Not implemented (yet?)
----------------------

* input
* including another file

Well, it's only a toy project, to learn Rust, right ? You want some
real Scheme/Lisp interpreter, there's plenty to find :)

License
--------
GNU General Public License, 2 or later. [See license.](LICENSE)

ChangeLog
---------
Current version is 1.0.0. [See ChangeLog.](ChangeLog.md)
