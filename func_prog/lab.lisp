(defun summa-digits (n) (
    (if (= 0 n)
    0
    (+ (* (mod n 10) (mod n 10))
        (summa-digits (floor n 10))))
))

(summa-digits 123)