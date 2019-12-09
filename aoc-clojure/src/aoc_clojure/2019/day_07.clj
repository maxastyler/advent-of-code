(ns aoc-clojure.2019.day-07
  (:require [aoc-clojure.2019.intcode :refer [run-until-paused]]))

(def prog (as-> (-> "2019/day_07/input"
                    clojure.java.io/resource
                    slurp) i
            (clojure.string/trim-newline i)
            (clojure.string/split i #",")
            (mapv read-string i)))

(def initial-states
  (update-in (vec
              (repeat 4 {:pointer 0 :tape prog
                         :input (conj (clojure.lang.PersistentQueue/EMPTY) 8)}))
             [0 :input] #(conj % 0)))
(defmethod print-method clojure.lang.PersistentQueue [q, w] ; Overload the printer for queues so they look like fish
  (print-method '<- w)
  (print-method (seq q) w)
  (print-method '-< w))

(defn one-round
  "run one round of computations"
  [states]
  (loop [i 0 s states]
    (let [ns (run-until-paused (s i))]
      (if (>= i (dec (count states)))
        (assoc-in s [i] ns)
        (recur (inc i)
               (-> s
                   (assoc-in [i] ns)
                   (update-in [(inc i) :input] #(apply conj % (:output ns)))
                   (assoc-in [i :output] (clojure.lang.PersistentQueue/EMPTY))))))))

(defn recursive-machine [phases]
  (let [s (update-in (mapv #(hash-map :pointer 0 :tape prog
                           :input (conj (clojure.lang.PersistentQueue/EMPTY) %)
                           :output (clojure.lang.PersistentQueue/EMPTY)) phases)
                     [0 :input] #(conj % 0))]
    (loop [ns (one-round s)]
      (if (:finished (ns (dec (count phases))))
        ns
        (recur
         (one-round
          (-> ns
              (update-in [0 :input] #(apply conj % (:output (peek ns))))
              (assoc-in [(dec (count ns)) :output]
                        (clojure.lang.PersistentQueue/EMPTY)))))))))

(def part-2 (->> (for [a (range 5 10)
                       b (range 5 10)
                       c (range 5 10)
                       d (range 5 10)
                       e (range 5 10)
                       :when (= (count (set [a b c d e])) 5)]
                   [a b c d e])
                 (map #(peek (:output ((recursive-machine %) 4))))
                 (apply max)))
