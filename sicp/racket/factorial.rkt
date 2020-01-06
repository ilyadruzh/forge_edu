#lang racket

// линейно рекурсивный процесс

(define (factorial-rec n)
    (if (= n 1)
    1
    (* n (factorial-rec (- n 1)))))


// линейно итеративный проц
(define (factorial n)
    (fact-iter 1 1 n))

(define (fact-iter product counter max-count)
    (if (> counter max-count)
        product
        (fact-iter (* counter product)
        (+ counter 1)
        max-count)))