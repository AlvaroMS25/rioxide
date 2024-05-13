(define (double x) (list x x))
(define double-lambda (lambda (x) (cons x x)))

(double 2)
(double-lambda 5)
