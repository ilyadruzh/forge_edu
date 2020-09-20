#lang racket

(define (square x) (* x x))

(define (sum-of-squares x y)
  (+ (square x) (square y)))

(define (f a) (sum-of-squares (+ a 1) (* a 2)))

(define (abs x)
  (cond ((> x 0) x)
        ((= x 0) 0)
        ((< x 0) (- x))))

(define (func1-3 x y z) (+ (* x x) (* y y) (* z z)))


(define (average x y)
  (/ (+ x y) 2))

(define (new-good-enough? guess guess-prev)
  (< (abs (/ (- guess guess-prev) guess-prev) 0.001)))

(define (new-if predicate then-clause else-clause)
  (cond (predicate then-clause)
        (else else-clause)))

(define (sqrt-iter-two guess x)

    (define (improve guess x)
    (average guess (/ x guess)))
  
  (new-if (new-good-enough? guess x)
          guess
          (sqrt-iter-two (improve guess x)
                     x)))

(define (sqrt-iter-three guess guess-prev x)
    (define (improve guess x)
    (average guess (/ x guess)))
  (if (new-good-enough? guess guess-prev)
          guess
          (sqrt-iter-three (improve guess x)
                         guess
                         x)))

(define (sqrt-one x)
  (define (good-enough? guess x)
    (< (abs (- (square guess) x)) 0.001))

  (define (improve guess x)
    (average guess (/ x guess)))

  (define (sqrt-iter guess x)
    (if (good-enough? guess x)
      guess
      (sqrt-iter (improve guess x)
                 x)))
  (sqrt-iter 1.0 x))

(define (sqrt x)
    (sqrt-iter-three 1.0 x))

(define (cubert-iter-one guess x)
  (if (cube-good-enough? guess x)
      guess
      (cubert-iter-one (cubert-improve guess x)
                       x)))

(define (cube-good-enough? guess x)
  (< (abs (- (* guess guess guess) x)) 0.001))

(define (cubert-improve guess x)
   (/ (+ (/ x (* guess guess)) (* 2 guess)) 3))

(define (cubrt x)
  (cubert-iter-one 1.0 x))

(define (factorial n)
  (fact-iter 1 1 n))

(define (fact-iter product counter max-count)
  (if (> counter max-count)
      product
      (fact-iter (* counter product)
                (+ counter 1)
                max-count)))

(define (A x y)
  (cond ((= y 0) 0)
         ((= x 0) (* 2 y))
         ((= y 1) 2)
         (else (A (- x 1)
                  (A x (- y 1))))))

(define (fib n)
  (cond ((= n 0) 0)
        ((= n 1) 1)
        (else (+ (fib (- n 1))
                 (fib (- n 2))))))

(define (count-change amount)
  (cc amount 5))

(define (cc amount kinds-of-coins)
  (cond ((= amount 0) 1)
        ((or (< amount 0) (= kinds-of-coins 0)) 0)
        (else (+ (cc amount
                     (- kinds-of-coins 1))
                 (cc (- amount (first-denomination kinds-of-coins))
                     kinds-of-coins)))))

(define (first-denomination kinds-of-coins)
  (cond ((= kinds-of-coins 1) 1)
        ((= kinds-of-coins 2) 5)
        ((= kinds-of-coins 3) 10)
        ((= kinds-of-coins 4) 25)
        ((= kinds-of-coins 5) 50)))

(define (func111req n)
  (cond ((< n 3) n)
        (else (+ (func111req (- n 1)) (func111req (- n 2)) (func111req (- n 3))))))

; wrong
(define (func111iter n nmin)
  (if (< n nmin)
      n
      (+ (func111iter (- n 1) nmin) (func111iter (- n 2) nmin) (func111iter (- n 3) nmin))))

(define (pascal row element)
(cond ((or (= element 1) (= element row)) 1)
       ((and (> element 1) (< element row)) (+ (pascal (- row 1) (- element 1))
                                               (pascal (- row 1) element)))))