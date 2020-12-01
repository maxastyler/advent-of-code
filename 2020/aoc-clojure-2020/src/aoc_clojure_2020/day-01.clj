(ns aoc-clojure-2020.day-01
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :refer [split-lines trim-newline]]))

(defn combinations
  "get all combinations of n elements from xs"
  [xs n]
  (if (= n 2)
    (if (> (count xs) 1)
      (lazy-cat (map #(vector (first xs) %) (rest xs))
                (combinations (rest xs) n))
      nil)
    (if (>= (count xs) n)
      (lazy-cat (map #(conj % (first xs)) (combinations (rest xs) (dec n)))
                (combinations (rest xs) n))
      (list))))

(defn solution "get the solution for n elements" [xs n] (->> (combinations xs n)
                                                             (filter #(= 2020 (apply + %)))
                                                             first
                                                             (apply *)))

(def input (->> (get-input 1)
                trim-newline
                split-lines
                (mapv #(Integer/parseInt %))))

(def part-1 (-> (for [i input
                      j input
                      :when (= (+ i j) 2020)]
                  (* i j))
                first))

(def part-2 (-> (for [i input
                      j input
                      k input
                      :when (= (+ i j k) 2020)]
                  (* i j k))
                first))

(def part-1-alt (solution input 2))
(def part-2-alt (solution input 3))
