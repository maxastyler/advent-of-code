(ns day_07 (:require [clojure.test :as t]))

(def prog (as-> (slurp "./input") i
             (clojure.string/trim-newline i)
             (clojure.string/split i #",")
             (map read-string i)
             (vec i)))

(defn intcode
  "takes in a dictionary of {:pointer pointer :tape tape
                             :input input :output output} and returns
  a new state of the computer"
  [{:keys [pointer tape input output] :as state}]
  (if (reduced? state)
    state
    (let [instruction (tape pointer)
          opcode (mod instruction 100)
          exp (fn [x n] (reduce * (repeat n x)))
          p-mode #(mod (quot instruction (exp 10 (+ % 2))) 10)
          n #(case (p-mode %)
               0 (tape (tape (+ pointer % 1)))
               1 (tape (+ pointer % 1)))
          w #(tape (+ pointer % 1))]
      (case opcode 
        1 (assoc state
                 :pointer (+ 4 pointer)
                 :tape (assoc tape (w 2) (+ (n 0) (n 1))))
        2 (assoc state
                 :pointer (+ 4 pointer)
                 :tape (assoc tape (w 2) (* (n 0) (n 1))))
        3 (if-let [inp (peek input)]
            (assoc state
                   :pointer (+ 2 pointer)
                   :input (pop input)
                   :tape (assoc tape (w 0) inp)
                   :waiting nil)
            (assoc state :waiting true))
        4 (assoc state
                 :pointer (+ 2 pointer)
                 :output (conj output (n 0)))
        5 (assoc state
                 :pointer (if-not (zero? (n 0))
                            (n 1)
                            (+ 3 pointer)))
        6 (assoc state
                 :pointer (if (zero? (n 0))
                            (n 1)
                            (+ 3 pointer)))
        7 (assoc state
                 :pointer (+ 4 pointer)
                 :tape (assoc tape (w 2) (if (< (n 0) (n 1))
                                           1
                                           0)))
        8 (assoc state
                 :pointer (+ 4 pointer)
                 :tape (assoc tape (w 2) (if (= (n 0) (n 1))
                                           1
                                           0)))
        99 (reduced state)))))

(defn run-until-reduced [tape input] (->> (iterate intcode {:pointer 0 :tape tape :input input :output (clojure.lang.PersistentQueue/EMPTY)})
                                     (take-while #(not (reduced? %)))
                                     (last)))

(defn run-until-waiting [pointer tape input]
  (loop [p pointer i input o (clojure.lang.PersistentQueue/EMPTY)]
    (let [state (intcode {:pointer p :tape tape :input i :output o})]
      (if (or (reduced? state)
              (:waiting state))
        state
        (recur (:pointer state) (:input state) (:output state))))))

(t/with-test
  (defn max-thruster-signal [prog]
    (->> (for [a (range 5)
               b (range 5)
               c (range 5)
               d (range 5)
               e (range 5)
               :when (= (count (set [a b c d e])) 5)]
           [a b c d e])
         (map
          #(reduce (fn [input phase]
                     (peek (:output (run-until-reduced prog [input phase])))) 0 %))
         (apply max)))
  (t/is (= 43210 (max-thruster-signal
                  [3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0])))
  (t/is (= 54321 (max-thruster-signal
                  [3,23,3,24,1002,24,10,24,1002,23,-1,23,
                   101,5,23,23,1,24,23,23,4,23,99,0,0])))
  (t/is (= 65210 (max-thruster-signal
                  [3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                   1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]))))

(defn rot-vec [v] (apply conj [(peek v)] (subvec v 0 (dec (count v)))))

(defn get-outputs [s] (if (reduced? s) (:output @s) (:output s)))

(defn recursive-max-thruster-signal [prog phases]
  (loop [i (update
            (mapv #(conj (clojure.lang.PersistentQueue/EMPTY) %) phases)
            0 #(conj % 0))
         t (vec (repeat 5 prog))
         p (mapv (constantly 0) phases)]
    (let [r0 (run-until-waiting (p 0) (t 0) (i 0))
          r1 (run-until-waiting (p 1) (t 1) (apply conj (i 1) (get-outputs r0)))
          r2 (run-until-waiting (p 2) (t 2) (apply conj (i 2) (get-outputs r1)))
          r3 (run-until-waiting (p 3) (t 3) (apply conj (i 3) (get-outputs r2)))
          r4 (run-until-waiting (p 4) (t 4) (apply conj (i 4) (get-outputs r3)))
          ]
      (if (reduced? r4)
        (get-outputs r4)
      ))))
  
(def part-1 (max-thruster-signal prog))
(t/run-tests 'day_07)

(comment "Need a set of computers all connected together with fifos instead of ")
