(def input (as-> (slurp "./input") i
             (clojure.string/trim-newline i)
             (clojure.string/split i #",")
             (map-indexed #(vector %1 (read-string %2)) i)
             (into {} i)))

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

(defn new-pos
  [[x y] d]
  (case (mod d 4)
    0 [(inc x) y]
    1 [x (inc y)]
    2 [(dec x) y]
    3 [x (dec y)]))

(defn paint-surface [input hull]
  "Direction gives the direction the robot's facing in:
0 right
1 up
2 left
3 down"
  (loop [hull hull
         [x y :as pos] [0 0]
         dir 0
         brain {:input (clojure.lang.PersistentQueue/EMPTY)
                :output (clojure.lang.PersistentQueue/EMPTY)
                :pointer 0
                :base 0
                :tape input}]
    (let [new-brain (run-until-paused
                     (update brain :input #(conj % (get hull pos 0))))
          [col turn] (get new-brain :output)
          dir ((case turn 0 + 1 -) dir 1)]
      (if (:finished new-brain)
        hull
        (recur (assoc hull pos col)
               (new-pos pos dir)
               dir
               (update new-brain :output empty))))))

(def part-1 (count (paint-surface input {})))

(def part-2 (let [hull (paint-surface input {[0 0] 1})
                  lims (juxt #(apply min %) #(apply max %))
                  [min-x max-x] (lims (map first (keys hull)))
                  [min-y max-y] (lims (map second (keys hull)))
                  width (- max-x min-x)
                  height (- max-y min-y)
                  hull-repr (vec (repeat (inc width)
                                         (vec (repeat (inc height) \space))))]
              (->> (reduce (fn [hr [[x y] c]]
                             (assoc-in hr [(- width (- x min-x))
                                           (- height (- y min-y))]
                                       (case c 0 \space 1 \#)))
                           hull-repr hull)
                   (map #(apply str %))
                   (clojure.string/join "\n"))))
