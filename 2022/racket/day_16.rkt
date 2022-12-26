#lang racket
(require racket/match)
(require racket/hash)


(define (input-to-map input-path) (let*
                                      ([row-regex #px"Valve ([[:alpha:]]{2}) has flow rate=([[:digit:]]+); tunnels? leads? to valves? (.*)"]
                                       [input-lines (string-split (call-with-input-file input-path port->string) "\n")]
                                       [row-fun (λ (x)
                                                  (match-let* ([(list _ v r rest) (regexp-match row-regex x)]
                                                               [rest (regexp-match* #px"([[:alpha:]]{2})" rest)])
                                                    (list v (cons (string->number r) (list->set rest)))))]
                                       [lines (map row-fun input-lines)])
                                    (apply hash (apply append lines))))

(define (bfs graph start)
  (letrec ([inner (λ (queue distances)
                    (match queue
                      [(list* current tail)
                       (let* ([tent-d (+ 1 (hash-ref distances current))]
                              [neighbours
                               (filter (λ (e) (< tent-d (hash-ref distances e +inf.0)))
                                       (set->list (cdr (hash-ref graph current (set)))))]
                              [new-queue (append tail (filter (λ (e) (not (member e tail))) neighbours))]
                              [new-distances (foldl (λ (v acc) (hash-set acc v tent-d)) distances neighbours)])
                         (inner new-queue new-distances))]
                      [_ distances]))])
    (inner (list start) (hash start 0))))

(define (shorten-paths graph)
  (letrec ([important-nodes (cons "AA" (filter string?
                                               (hash-map graph (λ (k v) (if (> (car v) 0) k null)))))]
           [important-distances (λ (n) (let ([distances (hash-map/copy (bfs graph n) (λ (k v) (values k (+ v 1))))])
                                         (cons n (foldl (λ (v acc) (if (equal? n v) acc
                                                                       (hash-set acc v (hash-ref distances v))))
                                                        (hash) important-nodes))))])
    (make-immutable-hash (map important-distances important-nodes))))

(define (prod . ls)
  (match ls
    [(list f) (map list f)]
    [(list* f r) (let* ([inner (apply prod r)]
                        [unflattened (map (λ (x) (map (λ (y) (cons x y)) inner)) f)])
                   (apply append unflattened))]))

(define (best-choice graph pressures time-left activated per-second total positions)
  (cond
    [(<= time-left 0) total]
    [(>= (length activated) (hash-count graph)) (total . + . (per-second . * . time-left))]
    [else (match-letrec ([time-jump (apply min (map cdr positions))]
                         [(cons ready waiting) (foldl (λ (v acc)
                                                        (match-let* ([(cons ready waiting) acc]
                                                                     [(cons p pt) v]
                                                                     [npt (- pt time-jump)])
                                                          (if (> npt 0)
                                                              (cons ready
                                                                    (cons (cons p npt) waiting))
                                                              (cons (cons p ready)
                                                                    waiting))))
                                                      '(() . ()) positions)]
                         [to-open (set-subtract (hash-keys graph) activated)]
                         [next-waiting (map (λ (x) (append x waiting))
                                            (apply prod (map (λ (n) (hash->list (hash-ref graph n))) ready)))]
                         [next-total (+ total (* per-second time-jump))]
                         [(cons next-activated next-per-second)
                          (foldl (λ (v acc) (if (member v (car acc))
                                                acc
                                                (cons (cons v (car acc))
                                                      (+ (cdr acc) (hash-ref pressures v)))))
                                 (cons activated per-second)
                                 ready)])
            (foldl (λ (v acc) (max acc (best-choice graph pressures
                                                    (- time-left time-jump)
                                                    next-activated
                                                    next-per-second
                                                    next-total
                                                    v))) 0 next-waiting)
            )]))

;; (let* ([inp (input-to-map "day_16_input")]
;;        [pressures (hash-map/copy inp (λ (k v) (values k (car v))))]
;;        [shortened (shorten-paths inp)])
;;   (best-choice shortened pressures 30 '("AA") 0 0 '(("AA" . 0))))
