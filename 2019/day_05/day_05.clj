(def input (as-> (slurp "./input") i
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
        3 (assoc state
                 :pointer (+ 2 pointer)
                 :input (pop input)
                 :tape (assoc tape (w 0) (peek input)))
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
        (reduced state)))))

(defn run-until-reduced [tape input] (->> (iterate intcode {:pointer 0 :tape tape :input input :output []})
                                     (take-while #(not (reduced? %)))
                                     (last)))

(def part-1 (->> (run-until-reduced input [1])
                 (:output)
                 (peek)))

(def part-2 (->> (run-until-reduced input [5])
                 (:output)
                 (peek)))
