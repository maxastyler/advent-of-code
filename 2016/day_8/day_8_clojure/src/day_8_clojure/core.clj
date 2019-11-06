(ns day-8-clojure.core
  (:require [instaparse.core :as insta]))

(def input (slurp "/home/max/git/advent_of_code/2016/day_8/input"))

(def instruction-parse
  (insta/parser
   "
<multiassignment> = {statement <#'\\n'>*}
<statement> = rect | row | column
rect = <'rect '>* number <'x'> number
row = <'rotate row y='>* number <' by '> number
column = <'rotate column x='>* number <' by '> number
number = #'[0-9]+'"))

(def instructions
  (->> input
       (instruction-parse)
       (map trans-line)))

(defn trans-line [[k [_ n1] [_ n2]]]
  [k (Integer/parseInt n1) (Integer/parseInt n2)])

(defn transpose [a] (vec(apply map vector a)))

(defn rotate [v n]
  (let [cv (count v), n (mod n cv)]
    (vec (concat (subvec v n cv) (subvec v 0 n)))))

(defn rotate-row-n [a row shift]
  (let [rot-row (nth a row)]
    (assoc a row (rotate rot-row (- shift)))))

(defn rotate-column-n [a column shift]
  (transpose (rotate-row-n (transpose a) column shift)))

(defn create-rect [a [x y]]
  (reduce
   (fn [acc row] (assoc acc row
                        (fillrow (nth acc row) x)))
   a (range y)))

(defn fillrow [row n] (vec (concat (repeat n 1) (subvec row n))))

(defn remove-space [& s] (apply str (filter #(not= % \space) (apply concat s))))

(defn create-screen [x y] (vec (repeat y (vec (repeat x " ")))))

(defn create-image [insts x y]
  (reduce (fn [acc [ins a b]]
            (case ins
              :rect (create-rect acc [a b])
              :row (rotate-row-n acc a b)
              :column (rotate-column-n acc a b)))
          (create-screen x y) insts))

(def p1-on-pixels (reduce + (flatten (create-image instructions 50 6))))
