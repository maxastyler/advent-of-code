(ns aoc-clojure-2020.day-12
  (:require [aoc-clojure-2020.helpers :refer [get-input]]))

(def input (->> (get-input 12)
                (re-seq #"([NSEWLRF])(\d+)")
                (mapv (fn [[_ i d]] [(first (char-array i)) (Integer/parseInt d)]))))

(defn run-insts [init-pos instructions]
  (reduce (fn [[r x y] [i d]]
            (let [i (if (= i \F) (case (quot (mod r 360) 90) 0 \E 1 \N 2 \W 3 \S) i)]
              (case i
                \N [r x (+ y d)]
                \S [r x (- y d)]
                \E [r (+ x d) y]
                \W [r (- x d) y]
                \L [(+ r d) x y]
                \R [(- r d) x y]))) init-pos instructions))

(defn rotate [x y r]
  (case (mod r 360)
    0 [x y]
    (recur (- y) x (- r 90))))

(defn run-insts2 [init-pos instructions]
  (reduce (fn [[x y wx wy] [i d]]
            (case i
                \N [x (+ y d) wx wy]
                \S [x (- y d) wx wy]
                \E [(+ x d) y wx wy]
                \W [(- x d) y wx wy]
                \F [x y (+ wx (* x d)) (+ wy (* y d))]
                (let [[x' y'] (rotate x y (case i \L d \R (- d)))]
                  [x' y' wx wy]))) init-pos instructions))


(def part-1 (-> (run-insts [0 0 0] input)
                ((fn [[_ x y]] (+ (Math/abs x) (Math/abs y))))))

(def part-2 (-> (run-insts2 [10 1 0 0] input)
                ((fn [[_ _ x y]] (+ (Math/abs x) (Math/abs y))))))
