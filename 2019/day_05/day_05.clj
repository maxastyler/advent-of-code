(def input (as-> (slurp "./input") i
             (clojure.string/trim-newline i)
             (clojure.string/split i #",")
             (map read-string i)
             (vec i)))

(defn exp [x n]
  (reduce * (repeat n x)))

(defn intcode
  "takes in a dictionary of {:pointer pointer :tape tape
                             :input input :output output} and returns
  a new state of the computer"
  [{:keys [pointer tape input output] :as state}]
  (if (reduced? state)
    state
    (let [instruction (tape pointer)
          opcode (mod instruction 100)
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
        (reduced state)))))

(def part-1 (->> {:pointer 0 :tape input :input [1] :output []}
                 (iterate intcode)
                 (take-while #(not (reduced? %)))
                 (last)
                 (:output)
                 (peek)))
