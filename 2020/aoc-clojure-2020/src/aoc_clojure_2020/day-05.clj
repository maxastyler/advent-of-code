(ns aoc-clojure-2020.day-05
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]))

(def input (->> (get-input 5)
                s/split-lines
                (map #(-> (s/replace % #"F|L" "0") (s/replace #"B|R" "1") (Integer/parseInt 2)))
                sort))

(def part-1 (last input))
(def part-2 (reduce #(if (> (- %2 %1) 1) (reduced (inc %1)) %2) input))
