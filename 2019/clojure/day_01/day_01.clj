(def input (->> (slurp "input")
                (clojure.string/split-lines)
                (map read-string)))

(defn fuel-req [x] (- (quot x 3) 2))

(def part-1 (transduce (map fuel-req) + input))
(def part-2 (transduce (map #(reduce +
                                     (rest (take-while
                                            pos?
                                            (iterate fuel-req %)))))
                       + input))
