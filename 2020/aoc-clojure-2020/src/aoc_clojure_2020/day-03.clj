(ns aoc-clojure-2020.day-03
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]))

(def input (-> (get-input 3)
               s/split-lines))

(defn gradient-tree-count [tree-map x-step y-step]
  (let [width (count (first tree-map))
        positions (iterate (fn [[x y]] [(mod (+ x x-step) width) (+ y y-step)]) [0 0])]
    (->> (map #(get-in tree-map (reverse %)) positions)
         (take-while some?)
         (filter #(= % \#))
         count)))

(defn gradient-tree-count-loop [tree-map x-step y-step]
  (let [width (count (first tree-map))
        height (count tree-map)
        tree-count-at-space (fn [x y] (if (= (get-in tree-map [y x]) \#) 1 0))]
    (loop [x 0 y 0 trees 0]
      (if (< y height)
        (recur (mod (+ x x-step) width) (+ y y-step) (+ trees (tree-count-at-space x y)))
        trees))))

(def part-1 (gradient-tree-count input 3 1))
(def part-2 (->> (map #(apply gradient-tree-count input %)
                      [[1 1] [3 1] [5 1] [7 1] [1 2]])
                 (apply *)))

(println "Part 1: " part-1)
(println "Part 2: " part-2)
