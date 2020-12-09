(ns aoc-clojure-2020.day-09
  (:require [aoc-clojure-2020.day-01 :refer [combinations]]
            [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :refer [split-lines]]))

(def input (->> (get-input 9) split-lines (map read-string)))

(defn correct-numbers [xs] (some #(= (apply + %) (last xs))
                                 (combinations (butlast xs) 2)))

(def part-1 (->> (partition 26 1 input) (remove correct-numbers) first last))

(def part-2 (->> (mapcat #(partition % 1 input) (range 2 (count input)))
                 (filter #(= (apply + %) part-1))
                 first
                 (#(+ (apply min %) (apply max %)))))
