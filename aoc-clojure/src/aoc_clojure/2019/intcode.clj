(ns aoc-clojure.2019.intcode
  (:require [clojure.core.async
             :as a
             :refer [>! <! >!! <!! go chan buffer close! thread
                     alts! alts!! timeout]]
            [clojure.test :as t]))

(defn intcode
  "takes in a dictionary of {:pointer pointer :tape tape
                             :input input :output output} and returns
  a new state of the computer"
  [pointer tape input]
  (let [instruction (tape pointer)
        opcode (mod instruction 100)
        exp (fn [x n] (reduce * (repeat n x)))
        p-mode #(mod (quot instruction (exp 10 (+ % 2))) 10)
        n #(case (p-mode %)
             0 (tape (tape (+ pointer % 1)))
             1 (tape (+ pointer % 1)))
        w #(tape (+ pointer % 1))
        state {:pointer pointer :tape tape :input input}]
    (case opcode 
      1 (assoc state
               :pointer (+ 4 pointer)
               :tape (assoc tape (w 2) (+ (n 0) (n 1))))
      2 (assoc state
               :pointer (+ 4 pointer)
               :tape (assoc tape (w 2) (* (n 0) (n 1))))
      3 (if-let [i input]
          (assoc state
                 :pointer (+ 2 pointer)
                 :input-taken true
                 :tape (assoc tape (w 0) i))
          (assoc state :waiting true))
      4 (assoc state
               :pointer (+ 2 pointer)
               :output (n 0))
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
      99 (assoc state :finished true))))

(t/with-test 
  (defn run-until-paused [pointer tape inputs]
    "Return a vector contatining the final state and a queue of outputs [s o]"
    (loop [p pointer
           t tape
           i inputs
           o (clojure.lang.PersistentQueue/EMPTY)]
      (let [{:keys [pointer tape input-taken finished waiting output] :as s}
            (intcode p t (peek i))]
        (if (or finished waiting)
          [s o]
          (recur pointer tape
                 (if input-taken
                   (pop inputs)
                   inputs)
                 (if output (conj o output) o))))))

  (t/is (= (peek ((run-until-paused 0 [3,9,8,9,10,9,4,9,99,-1,8] [8]) 1)) 1))
  (t/is (= (peek ((run-until-paused 0 [3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9] [8]) 1)) 1))
  (t/is (= (peek ((run-until-paused 0 [3,3,1105,-1,9,1101,0,0,12,4,12,99,1] [8]) 1)) 1))
  (t/is (= (peek ((run-until-paused 0 [3,9,7,9,10,9,4,9,99,-1,8] [5]) 1)) 1)))
