(defmacro defn (name args body)
  `(def ,name
        (lambda ,args
          ,body)))

(defn car (xs)
  (_car xs))

(defn cadr (xs)
  (car (cdr xs)))

(defn cdr (xs)
  (_cdr xs))

(defn cons (x xs)
  (_cons x xs))

(defn = (x y)
  (_= x y))

(defn + (x y)
  (_+ x y))

(defn - (x y)
  (_- x y))

(defn * (x y)
  (_* x y))

(defn / (x y)
  (_/ x y))

(defn nil? (xs)
  (if (= () xs)
      't
      ()))

(defn count (xs)
  (if (nil? xs)
      0
      (+ 1 (count (cdr xs)))))

(defn map (f xs)
  (if (nil? xs)
      ()
      (cons (f (car xs))
            (map f (cdr xs)))))

(defn inc (x)
  (+ x 1))

(defmacro let (args body)
  (cons `(lambda ,(map car args)
           ,body)
        (map cadr args)))
    
