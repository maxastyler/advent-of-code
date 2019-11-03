(require '[clojure.string :as str])

(defn convert-num-str [t]
  (->> (str/split t #" ")
       (filter #(not= "" %))
       (map #(Integer/parseInt %))))

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map convert-num-str)))

(def trans-input
  (->> input
       transpose
       flatten
       (partition 3)))

(defn transpose [a]
  (apply mapv vector a))

(defn valid-triangle [[a b c]]
  (and
   (> (+ a b) c)
   (> (+ b c) a)
   (> (+ c a) b)))

(def valid-triangles (filter valid-triangle input))
(def valid-trans-triangles (filter valid-triangle trans-input))
