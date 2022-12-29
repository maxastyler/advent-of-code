#lang racket
(require (for-syntax syntax/parse))

(define-syntax (as-> stx)
  (syntax-parse stx
    [(_ initial id:id exprs ...)
     #'(let* ([id initial]
              [id exprs] ...)
         id)]))

(define (load-input path)
  (as-> path x
        (call-with-input-file x port->string)
        (string-split x)
        (map (λ (x) (cons #f (string->number x))) x)
        (apply vector x)))

(define (modify-vec))

(let ([in (load-input "day_20_test")])
  (vector-set! in 0 (cons 2 3))
  in)

make-reader-graph
