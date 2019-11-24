(require '[clojure.string :as str] '[clojure.core.reducers :as red])

(def input (str/split (slurp "./input") #"\n"))

(def input-freqs (map frequencies input))

(def val-counts (->> input
                     (map frequencies)
                     (map #(group-by val %))))

(defn contains-n [n] (count (filter #(contains? % n) val-counts)))

(defn diff-count [a b]
  (reduce + (map #(if (= %1 %2) 0 1) a b)))

(defn reduce-vec [v] (apply str (keep (fn [[a b]] (if (= a b) a)) v)))

(def part-1 (* (contains-n 2) (contains-n 3)))
(def part-2 (->> input
                 (mapcat (fn [y] (map (fn [x] [x y]) input)))
                 (filter (fn [[a b]] (= (diff-count a b) 1)))
                 (map (fn [[a b]] (map vector a b)))
                 (map reduce-vec)
                 first
                 ))
