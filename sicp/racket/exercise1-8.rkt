#lang racket

(define (cubrt x)

  (define (abs x) (if (< x 0) (- x) x))

  (define (square x) (* x x))

  (define (enough? guess prev-guess) (< (abs (/ (- guess prev-guess) prev-guess)) 0.001))

  (define (improve guess x) (/ (+ (/ x (* guess guess)) ( * guess 2)) 3))

  (define (cubrt-iter guess prev-guess x) 
    (if (enough? guess prev-guess)
      guess 
      (cubrt-iter (improve guess x) guess x)
    )
  )

  (cubrt-iter 1.0 0.5 x)
)