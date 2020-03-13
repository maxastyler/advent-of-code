(ns user (:require [clojure.string :as s]))

(def recipes (->> (slurp "input")
                  (s/split-lines)
                  (map #(as-> (s/split % #" => ") i
                          (conj (s/split (nth i 0) #", ")
                                (nth i 1))))
                  (map (fn [xs] (map #(let [[_ n c]
                                            (re-find #"(\d+) ([A-Z]+)" %)]
                                        [(keyword c) (read-string n)]) xs)))
                  (map (fn [xs] [((last xs) 0)
                                 [((last xs) 1)
                                  (into {} (butlast xs))]]))
                  (into {})))  

(defn convert-reagent
  "convert a reagent r with a needed number n"
  [r n]
  (let [recipe-num (nth (r recipes) 0)
        needed (Math/ceil (/ n recipe-num))]
    (into {} (map (fn [[k v]] [k (* v needed)]) (nth (r recipes) 1)))))

(def priority
  (let [p 
        (fn priority [k]
          (if (= :ORE k)
            1
            (inc (apply + (map (fn [[k _]] (priority k))
                               (nth (k recipes) 1))))))]
    (into {} (map #(vector % (p %)) (conj (into #{} (keys recipes)) :ORE)))))

(defn fuel-to-ore [f]
  (loop [reagents {:FUEL f}]
    (if (= (into #{} (keys reagents)) #{:ORE})
      reagents
      (let [[k v] (apply max-key (fn [[k _]] (k priority)) reagents)]
        (recur (as-> (dissoc reagents k) i
                 (merge-with + i (convert-reagent k v))))))))

(defn binary-search [max-allowed left right]
  (let [mid (+ left (quot (- right left) 2))
        mid-ore (:ORE (fuel-to-ore mid))]
    (if (> mid-ore max-allowed)
      (if (= mid left)
        mid
        (binary-search max-allowed left mid))
      (if (= (inc mid) right)
        mid
        (max mid (binary-search max-allowed mid right))))))

(def part-1 (fuel-to-ore 1))
(def part-2 (binary-search 1000000000000 0 100000000))
