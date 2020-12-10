(ns aoc-clojure-2020.day-03
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]))

(def input (-> (get-input 3)
               s/split-lines))

(defn gradient-tree-count [tree-map x-step y-step]
  (let [width (count (first tree-map))
        x-positions (iterate #(mod (+ % x-step) width) 0)]
    (->> (take-nth y-step tree-map)
         (map #(get %2 %1) x-positions)
         (filter #(= % \#))
         count)))

(def part-1 (gradient-tree-count input 3 1))
(def part-2 (->> (map #(apply gradient-tree-count input %)
                      [[1 1] [3 1] [5 1] [7 1] [1 2]])
                 (apply *)))
