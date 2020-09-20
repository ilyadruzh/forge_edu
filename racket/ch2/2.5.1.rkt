#lang racket

(define (apply-generic op . args)
  (let ((type-tags (map type-tag args)))
    (let ((proc (get op type-tags)))
      (if proc
          (apply proc (map contentsargs))
          (error
           "нет метода дляэтих типов -- APPLY-GENERIC"
           (list op type-tags))))))

(define (add x y) (apply-generic `add x y))
(define (sub x y) (apply-generic `sub x y))
(define (mul x y) (apply-generic `mul x y))
(define (div x y) (apply-generic `div x y))

; (define (install-racket-number-package)
;  (define (tag x)
 ;   (attach-tag `racket-number x))
  ;(put `add `(racket-number rucket-number)
   ;    (lambda (x y) (tag (+ x y))))

