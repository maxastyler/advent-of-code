(ns aoc-clojure-2020.day-02
  (:require [aoc-clojure-2020.helpers :refer [get-input]]))

(def input (->> (get-input 2)
                (re-seq #"(\d+)-(\d+)\ ([a-z]):\ ([a-z]+)")
                (map (fn [[_ mi ma ch pass]] [(Integer/parseInt mi)
                                              (Integer/parseInt ma)
                                              (first (char-array ch))
                                              pass]))))

(defn pass-valid-1? [[mi, ma, letter, password]]
  (let [letter-count (->> password
                          (filter #(= %1 letter))
                          count)]
    (and (>= letter-count mi)
         (<= letter-count ma))))

(defn pass-valid-2? [[mi, ma, letter, password]]
  (let [s1 (= (get password (dec mi)) letter)
        s2 (= (get password (dec ma)) letter)]
    (not= s1 s2)))

(def part-1 (count (filter pass-valid-1? input)))
(def part-2 (count (filter pass-valid-2? input)))
