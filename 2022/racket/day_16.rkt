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
                              [neighbours                               (filter (λ (e) (< tent-d (hash-ref distances e +inf.0)))
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
                                         (cons n (cons (car (hash-ref graph n))
                                                       (foldl (λ (v acc) (if (equal? n v) acc
                                                                             (hash-set acc v (hash-ref distances v))))
                                                              (hash) important-nodes)))))])
    (make-immutable-hash (map important-distances important-nodes))))


(define (paths graph total-time)
  (letrec ([i-map (make-immutable-hash (map (λ (i v) (cons v i)) (range (hash-count graph)) (hash-keys graph)))]
           [list-to-int (λ (l) (foldl (λ (v a) (bitwise-ior
                                                (arithmetic-shift 1 (hash-ref i-map v))
                                                a)) 0 l))]
           [inner (λ (pos to-go time-left rate released)
                    (let* ([times (cdr (hash-ref graph pos))])
                      (for*/stream ([target to-go]
                                    #:when (< (hash-ref times target) time-left)
                                    [path (let ([t (hash-ref times target)]
                                                [r (car (hash-ref graph target))])
                                            (stream-cons (cons (list target) (+ released
                                                                                (* rate t)
                                                                                (* (+ rate r) (- time-left t))))
                                                         (inner target
                                                                (set-remove to-go target)
                                                                (- time-left t)
                                                                (+ rate (car (hash-ref graph target)))
                                                                (+ released (* t rate)))))])
                        (cons (cons pos (car path)) (cdr path)))))])
    (stream-map (λ (x) (cons (list-to-int (cdar x)) (cdr x)))
                (inner "AA" (set-remove (apply set (hash-keys graph)) "AA") total-time 0 0))))

(define (part-1 file-name)
  (let ([i (shorten-paths (input-to-map file-name))])
    (cdr (stream-fold (λ (v a) (if (> (cdr a) (cdr v)) a v)) '(() . 0) (paths i 30)))))

(define (part-2 file-name)
  (stream-fold (λ (v a) (if (> v a) v a)) 0 (let* ([i (shorten-paths (input-to-map file-name))]
                                                   [memo (stream->list (paths i 26))])
                                              (for*/stream ([a memo]
                                                            [b memo]
                                                            #:when (= (bitwise-and (car a) (car b)) 0))
                                                (+ (cdr a) (cdr b))))))

(part-1 "day_16_input")
(part-2 "day_16_input")
