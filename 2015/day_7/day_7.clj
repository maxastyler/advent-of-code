(require '[clojure.string :as str])

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map #(str/split % #" ->"))
       (map #(map str/trim %))
       (map (fn [[a b]] [a (keyword b)]))
       ))
