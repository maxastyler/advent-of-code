(ns aoc-clojure-2020.day-06
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]
            [clojure.set :refer [union intersection]]))

(def input (as-> (get-input 6) i
             (s/split i #"\n\n")
             (map #(->> (s/split-lines %) (map set)) i)))

(def part-1 (reduce #(+ %1 (count (apply union %2))) 0 input))
(def part-2 (reduce #(+ %1 (count (apply intersection %2))) 0 input))
