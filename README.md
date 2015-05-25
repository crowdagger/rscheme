# rscheme

A toy scheme interpreter written in Rust.

Dependencies
------------
You'll need the Rust compiler and
Cargo. [See there]{http://www.rust-lang.org/install.html}.

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

Yeah it should be `define` but it's `def`.

`(def x 42)`

`(def f (lambda (x) (* 2 x)))`

`(f x) ; returns 84`

### Macros ###

There is some for macros. E.g., if you want to combine def and lambda:

`(defmacro defn (name args body) ``(def ,name (lambda ,args ,body)))`

### Let ###

One example of macro usage is the definition of `let`, which isn't a
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

Not implemented (yet?)
----------------------

* variadic args
* multiple definition of same-name func but different args number
* input/output
* including another file

Well, it's only a toy project, to learn Rust, right ? You want some real Scheme/Lisp interpreter, there's plenty to find.
