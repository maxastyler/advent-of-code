(ns aoc-clojure-2020.day-07
  (:require [aoc-clojure-2020.helpers :refer [get-input]]
            [clojure.string :as s]))

(defn parse-line [l]
  (let [[_ colour contained] (re-matches #"([a-z\ ]+) bags contain (.+)" l)
        contained-cols (into {} (map (fn [[_ n c]] [c (Integer/parseInt n)]))
                             (re-seq #"(\d+) ([a-z\ ]+) bag" contained))]
    [colour contained-cols]))

(def input (->> (get-input 7)
                s/split-lines
                (into {} (map parse-line))))

(defn bag-contains [bag contained rules]
  (if (contains? (get rules bag) contained)
    true
    (some #(bag-contains (first %) contained rules) (get rules bag))))

(defn count-bag [bag rules]
  (reduce (fn [acc [col n]] (+ acc (* n (inc (count-bag col rules))))) 0 (get rules bag)))

(def part-1 (-> (filter #(bag-contains (first %) "shiny gold" input) input) count))
(def part-2 (count-bag "shiny gold" input))

;; iterative solution =================

(defn count-bags-containing-child [bag rules]
  (->> (loop [to-check (keys rules)
              checked {}]
         (if-let [c (first to-check)]
           (if (contains? checked c)
             (recur (rest to-check) checked)
             (if-let [children (keys (get rules c))]
               (if (some #(= % bag) children)
                 (recur (rest to-check) (conj checked [c true]))
                 (if-let [to-calculate (not-empty (filter #(nil? (checked %)) children))]
                   (recur (apply conj to-check to-calculate) checked)
                   (recur (rest to-check) (conj checked [c (boolean (some #(checked %) children))]))))
               (recur (rest to-check) (conj checked [c false]))))
           checked))
       (filter second)
       count))

(def part-1-iter (count-bags-containing-child "shiny gold" input))
