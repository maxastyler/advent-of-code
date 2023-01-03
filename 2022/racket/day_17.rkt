#lang racket

(require data/pvector)
(require data/collection)

(define (check-position shape position height state)
  (let inner ([h height]
              [i 0])
    (if (>= i (length shape))
        #t
        (let* ([shape-row (arithmetic-shift (ref shape i) (add1 (- position)))]
               [state-index (+ (length state) h)]
               [stencil (bitwise-ior #b100000001 (if (<= 0 state-index (sub1 (length state)))
                                                     (arithmetic-shift (nth state state-index) 1)
                                                     0))])
          (cond
            [(< state-index 0) #f]
            [(zero? (bitwise-and shape-row stencil)) (inner (add1 h) (add1 i))]
            [else #f])))))

(define (move-left? shape position height state) (check-position shape (sub1 position) height state))
(define (move-right? shape position height state) (check-position shape (add1 position) height state))
(define (move-down? shape position height state) (check-position shape position (sub1 height) state))

(define (add-to-state shape position height state)
  (let* ([num-rows-to-modify (min (max (- height) 0) (length shape))]
         [state-mod-indices (take num-rows-to-modify (range (+ (length state) height) (length state)))]
         [shape-rows (take num-rows-to-modify shape)]
         [mod-state (foldl (λ (a v)
                             (update-nth a (car v) (λ (x)
                                                     (bitwise-ior x (arithmetic-shift (cdr v) (- position))))))
                           state (map cons state-mod-indices shape-rows))]
         [new-state (map (λ (x) (arithmetic-shift x (- position))) (drop num-rows-to-modify shape))])
    (extend mod-state new-state)))


(define (lock commands shape position height state)
  (let ([new-pos
         (if (equal? (first commands) #\<)
             (if (move-left? shape position height state) (sub1 position) position)
             (if (move-right? shape position height state) (add1 position) position))])
    (if (move-down? shape new-pos height state)
        (lock (rest commands) shape new-pos (sub1 height) state)
        (values (rest commands)
                (add-to-state shape new-pos height state)))))

(define rocks (pvector (pvector #b0011110)
                       (pvector #b0001000 
                                #b0011100
                                #b0001000)
                       (pvector #b0011100
                                #b0000100
                                #b0000100)
                       (pvector #b0010000
                                #b0010000
                                #b0010000
                                #b0010000)
                       (pvector #b0011000
                                #b0011000)))

(define (run-game commands)
  (let inner ([commands (cycle commands)]
              [rocks (cycle rocks)]
              [state (pvector)])
    (let-values ([(new-commands new-state) (lock commands (first rocks) 0 3 state)])
      (stream-cons state (inner new-commands (rest rocks) new-state)))))


(define (display-tower state)
  (let ([display-row (λ (r) (string-join (stream->list (map (λ (i)
                                                              (if (bitwise-bit-set? r i) "#" "."))
                                                            (reverse (in-range 7)))) ""))])
    (string-join (stream->list (map display-row (reverse state))) "\n")))


(define (part-1 input-path)
  (let* ([commands (string->immutable-string (call-with-input-file input-path port->string))])
    (length (nth (run-game commands) 2022))))

(part-1 "day_17_input")
