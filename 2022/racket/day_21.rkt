#lang racket

(define (process-input input)
  (let* ([split-input (string-split input "\n")]
         [convert-fun (λ (x) (let ((op-map (hash "*" * "+" + "/" / "-" -)))
                               (match (regexp-match #px"([[:alpha:]]+) ([+*-/]) ([[:alpha:]]+)" x)
                                 ((list _ a1 op a2) (list (hash-ref op-map op) a1 a2))
                                 (#f (string->number x)))))]
         [reg-fun (λ (x) (match-let (((list _ n r) (regexp-match #px"([[:alpha:]]+): ([^\n]*)" x)))
                           (list n (convert-fun r))))])
    (apply hash (apply append (map reg-fun split-input)))))


(define (part-1 input)
  (letrec ([p1 (λ (key results)
                 (match (hash-ref results key)
                   [(list op a1 a2) (let*-values ([(a1res hash1) (p1 a1 results)]
                                                  [(a2res hash2) (p1 a2 hash1)])
                                      (values (op a1res a2res) hash2))]
                   [constant (values constant results)]))])
    (let-values ([(ans _) (p1 "root" input)]) ans)))

(define inp (process-input (call-with-input-file "./day_21_input" port->string)))

(letrec ([parent (λ (node) (first (filter (λ (x) (match (hash-ref inp x)
                                                   [(list op a b) (or (equal? node a) (equal? node b))]
                                                   [_ #f])) (hash-keys inp))))]
         [inversion ()])
  (parent "humn"))

(part-1 inp)
