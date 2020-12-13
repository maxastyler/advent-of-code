(ns aoc-clojure-2020.day-13
  (:require [aoc-clojure-2020.helpers :refer [get-input]]))

(defn extended-gcd
  "The extended Euclidean algorithm
  Returns a list containing the GCD and the Bézout coefficients
  corresponding to the inputs. "
  [a b]
  (cond (zero? a) [(Math/abs b) 0 1]
        (zero? b) [(Math/abs a) 1 0]
        :else (loop [s 0
                     s0 1
                     t 1
                     t0 0
                     r (Math/abs b)
                     r0 (Math/abs a)]
                (if (zero? r)
                  [r0 s0 t0]
                  (let [q (quot r0 r)]
                    (recur (- s0 (* q s)) s
                           (- t0 (* q t)) t
                           (- r0 (* q r)) r))))))
 
(defn chinese_remainder
  " Main routine to return the chinese remainder "
  [n a]
  (let [prod (apply * n)
        reducer (fn [sum [n_i a_i]]
                  (let [p (quot prod n_i)          
                        egcd (extended-gcd p n_i)  
                        inv_p (second egcd)]       
                    (+ sum (* a_i inv_p p))))
        sum-prod (reduce reducer 0 (map vector n a))]
    (mod sum-prod prod)))

(defn parse-input [x] (let [[time-str buses-str] (clojure.string/split-lines x)
                            buses (->> (clojure.string/split buses-str #",")
                                       (map-indexed #(vector %2 %1))
                                       (filter #(re-matches #"\d+" (first %)))
                                       (map (fn [[id offset]] [(Integer/parseInt id)
                                                               (mod (- offset) (Integer/parseInt id))]))
                                       (into []))]
                        [(Integer/parseInt time-str) buses]))

(def part-1 (let [[time-str buses-str] (clojure.string/split-lines (get-input 13))
                  time (Integer/parseInt time-str)
                  buses (->> buses-str (re-seq #"\d+") (mapv #(Integer/parseInt %)))]
              (->> (range)
                   (map #(+ time %))
                   (map (fn [t] (some #(if (zero? (mod t %))
                                         (* % (- t time))
                                         false) buses)))
                   (remove nil?)
                   first)))

(def part-2 (let [[t buses] (parse-input (get-input 13))]
              (chinese_remainder (map first buses) (map second buses))))
