#lang racket

(define (sqrt x) 

    (define (square x) (* x x))

    (define (abs x) (if (< x 0) (- x) x))

    (define (good-enough? guess x) (< (abs (- (square guess) x)) 0.000001))

    (define (sqrt-iter guess x) 
        (if (good-enough? guess x)
            guess 
            (sqrt-iter (improve guess x) x)))

    (define (improve guess x) (average guess (/ x guess)))

    (define (average x y) (/ (+ x y) 2))

    (sqrt-iter 1.0 x)
)

// TODO:  Who can explain it to me?
(define (better-sqrt x)  

    (define (square x) (* x x))

    (define (abs x) (if (< x 0) (- x) x))

    (define (better-good-enough? guess prev-guess) (< (abs (/ (- guess prev-guess) prev-guess)) 0.001))

    (define (better-sqrt-iter guess prev-guess x) 
        (if (better-good-enough? guess prev-guess) 
            guess 
            (better-sqrt-iter (improve guess x) guess x)))

    (define (improve guess x) (average guess (/ x guess)))

    (define (average x y) (/ (+ x y) 2))

    (better-sqrt-iter 1.0 0.5 x)

)
