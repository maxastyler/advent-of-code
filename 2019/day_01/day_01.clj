(def input (->> (slurp "input")
                (clojure.string/split-lines)
                (map #(Integer/parseInt %))))

(defn fuel-req [x] (- (quot x 3) 2))

(def part-1 (transduce (map fuel-req) + input))
(def part-2-func (transduce (map #(reduce + 0
                                          (rest
                                           (take-while pos?
                                                       (iterate fuel-req %)))))
                            + input))
