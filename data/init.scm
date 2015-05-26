(defmacro defn (name args body)
  `(def ,name   (lambda ,name ,args
                        ,body)))




(defn car (xs)
  (_car xs))

(defn cdr (xs)
  (_cdr xs))

(defn cadr (xs)
  (car (cdr xs)))

(defn cons (x xs)
  (_cons x xs))

(defn apply (f args)
  (eval (cons f args)))

(defn = (x y)
  (_= x y))

(defn nil? (xs)
  (if (= () xs)
      't
      ()))

(defn + (x & args)
  (if (nil? args)
      x
      (_+ x (apply + args))))

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
  (cond (((< x y) "less than")
         ((> x y) "greater than")
         ('else "must be equal"))))

(defn factorial (x)
  (if (<= x 1)
      1
      (* x (factorial (- x 1)))))


(defn fibo (fibox)
  (if (< fibox 2)
      1
      (+ (fibo (- fibox 1)) (fibo (- fibox 2)))))

(defn g (x & args)
  (cdr args))



(defn f (x & args)
  (if (= 0 (count args))
      x
      (+ x (apply f args))))
