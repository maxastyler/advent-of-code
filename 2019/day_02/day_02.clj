(def input (as-> (slurp "./input") i
;; (def input (as-> "1,0,0,0,99" i
             (clojure.string/trim-newline i)
             (clojure.string/split i #",")
             (map read-string i)
             (vec i)))

(defn step [[p, xs]]
  (case (xs p)
    99 (reduced [p, xs])
    [(+ p 4)
     (assoc xs (xs (+ p 3))
            ((case (xs p)
               1 +
               2 *)
             (xs (xs (+ 1 p)))
             (xs (xs (+ 2 p)))))]))

(defn find-output [noun verb input]
  (as-> [0 (assoc input 1 noun 2 verb)] i
    (iterate step i)
    (take-while #(not (reduced? %)) i)
    (last i)
    (get-in i [1 0])))

(def part-1 (find-output 12 2 input))

(def part-2 (->> (for [noun (range 100) verb (range 100)] [noun verb])
                 (filter (fn [[n v]] (= (find-output n v input) 19690720)))
                 (first)
                 ((fn [[n v]] (+ v (* n 100))))))
