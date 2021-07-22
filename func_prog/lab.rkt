#lang racket

(define (sum-of-digits x)
  (if (= x 0) 
      0
      (+ (sqr (modulo x 10))
         (sum-of-digits (quotient x 10)))))

(sum-of-digits 100012)