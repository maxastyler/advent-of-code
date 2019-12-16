(defn intcode
  "takes in a dictionary which should contain at least :pointer and :tape"
  [{:keys [pointer base tape input output] :as state}]
  (let [instruction (tape pointer)
        opcode (mod instruction 100)
        exp (fn [x n] (reduce * (repeat n x)))
        p-mode #(mod (quot instruction (exp 10 (+ % 2))) 10)
        n #(case (p-mode %)
             0 (get tape (get tape (+ pointer % 1) 0) 0)
             1 (get tape (+ pointer % 1) 0)
             2 (get tape (+ (get tape (+ pointer % 1) 0) base) 0))
        w #(case (p-mode %)
             0 (get tape (+ pointer % 1) 0)
             2 (+ (get tape (+ pointer % 1) 0) base))]
    (case opcode 
      1 (assoc state
               :pointer (+ 4 pointer)
               :tape (assoc tape (w 2) (+ (n 0) (n 1))))
      2 (assoc state
               :pointer (+ 4 pointer)
               :tape (assoc tape (w 2) (* (n 0) (n 1))))
      3 (if-let [i (peek input)]
          (assoc state
                 :pointer (+ 2 pointer)
                 :waiting nil
                 :input (pop input)
                 :tape (assoc tape (w 0) i))
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
      9 (assoc state
               :pointer (+ 2 pointer)
               :base (+ base (n 0)))
      99 (assoc state :finished true))))

(defn run-until-paused [s]
  "run the intcode computer on s until waiting or finished"
  (loop [ns (intcode s)]
    (if (or (:waiting ns)
            (:finished ns))
      ns
      (recur (intcode ns)))))

(defn parse-output-state [output]
  (as-> output i
    (partition 3 i)
    (let [min-x (apply min (map first i))
          max-x (inc (apply max (map first i)))
          min-y (apply min (map second i))
          max-y (inc (apply max (map second i)))
          pos (fn [[x y _]] [x y])
          type* (fn [[_ _ t]] t)
          to-map (fn [[x y t]] [[x y] (case t 1 :wall 2 :block)])]
      {:paddle (first (mapv pos (filter #(= (type* %) 3) i)))
       :ball (first (mapv pos (filter #(= (type* %) 4) i)))
       :tiles (into {} (map to-map (filter #(or (= (type* %) 1)
                                                (= (type* %) 2)) i)))
       :bounds {:x [min-x max-x] :y [min-y max-y]}})))

(def input (as-> (slurp "input") i
              (clojure.string/trim-newline i)
              (clojure.string/split i #",")
              (map-indexed #(vector %1 (read-string %2)) i)
              (into {} i)))

(def part-1 (as-> input i
              (run-until-paused {:pointer 0 :base 0 :tape i :input [] :output []})
              (:output i)
              (parse-output-state i)
              (:tiles i)
              (vals i)
              (filter #(= :block %) i)
              (count i)))
(def part-2 (as-> input i
              (assoc i 0 2)
              (loop [game (run-until-paused {:pointer 0 :base 0 :tape i :input [] :output []})]
                (if (:finished game)
                  game
                  (recur (run-until-paused (next-joystick-input game)))))
              (last (:output i))))

(defn next-joystick-input [intcode-state]
  (let [output (parse-output-state (:output intcode-state))
        ball (:ball output)
        paddle (:paddle output)
        ball-delta (- (ball 0) (get paddle 0 0))]
    (-> intcode-state
        (assoc :output [])
        (assoc :input [(cond
                         (neg? ball-delta) -1
                         (pos? ball-delta) 1
                         :else 0)]))))
