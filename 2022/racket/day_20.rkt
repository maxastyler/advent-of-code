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
        (map (λ (x i) (cons i (string->number x))) x (range (length x)))
        (apply vector x)))

(define (mod-vec v position)
  (match-letrec ([(cons input-position current-value) (vector-ref v position)]
                 [l (vector-length v)]
                 [distance (modulo current-value (sub1 l))]
                 [inner (λ (p d)
                          (if (= d 0)
                              (vector-set! v p (cons input-position current-value))
                              (let ([next-pos (modulo (+ (sgn d) p) l)])
                                (vector-set! v p (vector-ref v next-pos))
                                (inner next-pos (- d (sgn d))))))])
    (inner position distance)))

(define (find-first-index v filter-fun)
  (let inner ([p 0])
    (if (filter-fun (vector-ref v p))
        p
        (inner (add1 p)))))

(define (decrypt-vec v)
  (for ([i (range (vector-length v))])
    (let ([p (find-first-index v (λ (x) (= (car x) i)))])
      (mod-vec v p))))

(define (coords v)
  (let* ([0-pos (find-first-index v (λ (x) (zero? (cdr x))))]
         [nth (λ (n) (cdr (vector-ref v (modulo (+ 0-pos n) (vector-length v)))))])
    (+ (nth 1000) (nth 2000) (nth 3000))))

(let* ([p1-v (load-input "day_20_input")]
       [p2-v (vector-map (λ (x) (cons (car x) (* 811589153 (cdr x)))) p1-v)])
  (decrypt-vec p1-v)
  (for ([_ (range 10)]) (decrypt-vec p2-v))
  (printf "Part 1: ~a~n" (coords p1-v))
  (printf "Part 2: ~a~n" (coords p2-v)))
