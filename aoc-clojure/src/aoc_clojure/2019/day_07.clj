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
  (let [s (mapv #(hash-map :pointer 0 :tape prog
                           :input (conj (clojure.lang.PersistentQueue/EMPTY) %)
                           :output (clojure.lang.PersistentQueue/EMPTY)) phases)]
    (loop [ns (one-round s)]
      (if (:finished (ns (dec (count phases))))
        ns
        (recur
         (one-round
          (-> ns
              (update-in [0 :input] #(apply conj % (:output (peek ns))))
              (assoc-in [(dec (count ns)) :output]
                        (clojure.lang.PersistentQueue/EMPTY)))))))))
