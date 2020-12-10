(ns aoc-clojure-2020.day-08
  (:require [aoc-clojure-2020.helpers :refer [get-input]]))

(def input (->> (get-input 8)
                (re-seq #"(acc|jmp|nop) ([+-]\d+)")
                (mapv (fn [[_ op n]] [(keyword op) (Integer/parseInt n)]))))

(defn step [{:keys [pointer acc]} tape]
  (if-let [[op n] (get tape pointer)]
    (case op
      :nop {:pointer (inc pointer) :acc acc}
      :acc {:pointer (inc pointer) :acc (+ acc n)}
      :jmp {:pointer (+ pointer n) :acc acc})
    {:pointer pointer :acc acc
     :exit (if (= pointer (count input))
             :graceful
             :crash)}))

(defn get-value-on-loop-or-exit [input]
  (reduce (fn [visited {:keys [pointer acc exit] :as state}]
            (if (or exit (visited pointer))
              (reduced state)
              (conj visited pointer)))
          #{}
          (iterate #(step % input) {:pointer 0 :acc 0})))

(def part-1 (time (get-value-on-loop-or-exit input)))
(def part-2 (time (->> (range (count input))
                       (filter #(#{:jmp :nop} (get-in input [% 0])))
                       (map (fn [i] (update-in input [i 0]
                                               #(case %
                                                  :nop :jmp
                                                  :jmp :nop))))
                       (map get-value-on-loop-or-exit)
                       (filter :exit))))
