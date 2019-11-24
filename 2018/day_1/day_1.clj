(require '[clojure.string :as str] '[clojure.core.reducers :as red])

(def input
    (map #(Integer/parseInt %)
        (str/split (slurp "./input") #"\n")))

(def part_1 (reduce + input))

(defn first-duplicate [xs]
  (let [result (reduce (fn [seen x]
                         (if (seen x)
                           (reduced x)
                           (conj seen x)))
                 #{} xs)]
    (if (set? result)
      nil
      result)))

(defn part_2 []
  (let [freqs (reductions + 0 (cycle input))]
    (first-duplicate freqs)))
