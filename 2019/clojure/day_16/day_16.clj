(defn phases
  [n]
  (rest (cycle (mapcat #(repeat (inc n) %) [0 1 0 -1]))))

(def input (->> (slurp "input")
            ;; "80871224585914546619083218645595"
            (clojure.string/trim-newline)
            (map str)
            (mapv read-string)))

(defn fft [^longs input]
  (->> (range (count input))
       (map (comp
             (fn [^long x] (Math/abs x))
             #(rem % 10)
             #(apply + (map * (phases %) input))))))

(def part-1 (take 8 (nth (iterate fft input) 100)))
