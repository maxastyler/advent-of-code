(require '[clojure.string :as str])

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map char-array)
       (apply mapv vector)
       ))

(def counted (map count-letters input))

(def max-letter
  (map #(key (apply max-key val %)) counted))

(def min-letter
  (map #(key (apply min-key val %)) counted))

(defn count-letters [a]
  (let [grp (group-by identity a)]
    (zipmap (keys grp) (map count (vals grp)))))
