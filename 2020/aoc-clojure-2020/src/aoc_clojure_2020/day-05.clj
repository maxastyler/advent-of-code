(ns aoc-clojure-2020.day-05
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]))

(defn string-to-id [x]
  (-> (s/replace x #"F|B|L|R" {"F" "0" "B" "1" "L" "0" "R" "1"})
      (Integer/parseInt 2)))

(def input (->> (get-input 5)
                (s/split-lines)
                (map string-to-id)
                sort))

(def part-1 (last input))
(def part-2 (as-> (map vector (drop 1 input) (drop-last input)) i
              (filter #(= 2 (apply - %)) i)
              (get (first i) 1)
              (inc i)))
