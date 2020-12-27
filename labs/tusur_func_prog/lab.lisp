;; Задание 1
(defun summa_digits (n) 
        (cond 
            ((< n 10) 1) 
            (t (1+ (summa_digits (truncate n 10)))))
)

;; Проверка задания 1
(summa_digits 123)

;; Задание 2 
(defun f (s)
        
        (labels ((flatten (lst) 
                (apply 'append (mapcar (lambda (x) (if (atom x) (list x) (flatten x))) lst))))

        (labels ((neg-list (lst) 
                (cond ((null lst) nil)
                        ((numberp (car lst)) (cons (- (car lst)) (neg-list (cdr lst))))
                        (t (cons (neg-list (car lst)) (neg-list (cdr lst)))))))
         
         (sort (neg-list (flatten s)) #'<)))
)

;; Проверка задания 2
(f '(4 -8 6 (4 -8 6 -9 -7) -9 -2 4 5))

;; Задание 3
(defun f (s)
  (cond ((null s) nil)
        ((numberp (car s)) (cons (- (car s)) (f (cdr s))))
        (t (cons (f (car s)) (f (cdr s)))))
)

;; Проверка задания 3
(f '(4 -8 6 (4 -8 6 -9 -7) -9 -2 4 5))
