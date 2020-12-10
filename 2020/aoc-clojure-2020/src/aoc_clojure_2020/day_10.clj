(ns aoc-clojure-2020.day-10
  (:require [aoc-clojure-2020.helpers :refer [get-input]]))

(def input (->> (get-input 10)
                clojure.string/split-lines
                (map read-string)
                sort
                (#(concat [0] % [(+ 3 (last %))]))
                (into [])))

(def part-1 (->> input
                 (partition 2 1)
                 (map #(apply - (reverse %)))
                 (frequencies)
                 ((fn [{ones 1 threes 3}] (* ones threes)))))

(def inp (mapv vector (range) input))

(def count-paths (memoize
                  (fn [[i v]]
                    (if (>= i (dec (count inp)))
                      1
                      (->> (drop (inc i) inp)
                           (take-while #(<= (- (second %) v) 3))
                           (map count-paths)
                           (apply +))))))

(def part-2 (count-paths [0 0]))
