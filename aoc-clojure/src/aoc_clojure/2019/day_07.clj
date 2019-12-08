(ns aoc-clojure.2019.day-07
  (:require [aoc-clojure.2019.intcode :refer [run-until-paused]]))

(def prog (as-> (-> "2019/day_07/input"
                    clojure.java.io/resource
                    slurp) i
            (clojure.string/trim-newline i)
            (clojure.string/split i #",")
            (mapv read-string i)))
