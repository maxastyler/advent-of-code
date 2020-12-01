(ns aoc-clojure-2020.day-01
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :refer [split-lines trim-newline]]))

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
