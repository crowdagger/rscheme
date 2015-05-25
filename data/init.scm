(defmacro defn (name args body)
  `(def ,name
        (lambda ,name ,args
                ,body)))

(defn car (xs)
  (_car xs))

(defn cdr (xs)
  (_cdr xs))

(defn cadr (xs)
  (car (cdr xs)))

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

(defn < (x y)
  (_< x y))

(defn > (x y)
  (_> x y))

(defmacro or (p1 p2)
  `(if ,p1
       't
       ,p2))

(defmacro and (p1 p2)
  `(if ,p1
       ,p2
       ()))

(defn <= (x y)
  (or (< x y)
      (= x y)))

(defn >= (x y)
  (or (> x y)
      (= x y)))

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
    
(defn factorial (x)
  (if (= x 0)
      1
      (* x (factorial (- x 1)))))

(defmacro cond (preds)
  `(if ,(car (car preds))
       ,(cadr (car preds))
       ,(if (nil? (cdr preds))
            ()
            `(cond ,(cdr preds)))))

(defn compare (x y)
  (cond (((< x y) "less_than")
         ((> x y) "greater_than")
         ('else "must_be_equal"))))

