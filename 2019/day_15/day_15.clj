(def input (as-> (slurp "input") i
             (clojure.string/trim-newline i)
             (clojure.string/split i #",")
             (mapv read-string i)))

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

(defn move-dir
  "Calculate the direction command (1, 2, 3, 4) to move by the given vector
  Takes in [from, to]"
  [[x1 y1] [x2 y2]]
  (case [(- x2 x1) (- y2 y1)]
    [0 1] 1
    [0 -1] 2
    [-1 0] 3
    [1 0] 4))

(defn move-pos
  "Calculate the new position given a direction and an old position"
  [dir [x y]]
  (case dir
    1 [x (inc y)]
    2 [x (dec y)]
    3 [(dec x) y]
    4 [(inc x) y]))

(defn opp-dir
  "Get the opposite direction"
  [dir]
  (case dir
    1 2
    2 1
    3 4
    4 3))

(def part-1
  (loop [path (list [0 0]) maze {[0 0] {:dist 0 :dirs {1 :unk
                                                       2 :unk
                                                       3 :unk
                                                       4 :unk}}}
         bot {:pointer 0 :base 0 :input [] :output [] :tape input}]
    (if-let [[d _] (first (filter (fn [[_ v]] (= v :unk))
                                  (get-in maze [(peek path) :dirs])))]
      (let [cur-pos (peek path)
            new-pos (move-pos d (peek path))
            new-dist (inc (get-in maze [cur-pos :dist]))
            new-path (conj path new-pos)
            new-bot (run-until-paused (assoc bot :input [d]))]
        (case (peek (:output new-bot))
          0 (recur path (assoc-in maze [(peek path) :dirs d] :wall)
                   new-bot)
          1 (recur (conj path new-pos)
                   (as-> maze m
                     (assoc-in m [(peek path) :dirs d] :path)
                     (if (contains? m new-pos)
                       (if (< new-dist (get-in m [new-pos :dist]))
                         (assoc-in m [new-pos :dist] new-dist)
                         m)
                       (assoc m new-pos
                              {:dist new-dist
                               :dirs {1 :unk
                                      2 :unk
                                      3 :unk
                                      4 :unk}}))
                     (assoc-in m [new-pos :dirs (opp-dir d)] :path))
                   new-bot)
          2 {:distance new-dist :position new-pos}))
      (if (>= 1 (count path))
        path
        (recur (pop path) maze
               (run-until-paused (assoc bot :input
                                        [(move-dir (first path)
                                                   (second path))])))))))

(def part-2
  (let [pos (:position part-1)
        ground-maze
        (loop [path (list [0 0]) maze {[0 0] {:dist 0 :dirs {1 :unk
                                                             2 :unk
                                                             3 :unk
                                                             4 :unk}}}
               bot {:pointer 0 :base 0 :input [] :output [] :tape input}]
          (if-let [[d _] (first (filter (fn [[_ v]] (= v :unk))
                                        (get-in maze [(peek path) :dirs])))]
            (let [cur-pos (peek path)
                  new-pos (move-pos d (peek path))
                  new-dist (inc (get-in maze [cur-pos :dist]))
                  new-path (conj path new-pos)
                  new-bot (run-until-paused (assoc bot :input [d]))]
              (case (peek (:output new-bot))
                0 (recur path (assoc-in maze [(peek path) :dirs d] :wall)
                         new-bot)
                (1 2) (recur (conj path new-pos)
                             (as-> maze m
                               (assoc-in m [(peek path) :dirs d] :path)
                               (if (contains? m new-pos)
                                 (if (< new-dist (get-in m [new-pos :dist]))
                                   (assoc-in m [new-pos :dist] new-dist)
                                   m)
                                 (assoc m new-pos
                                        {:dist new-dist
                                         :dirs {1 :unk
                                                2 :unk
                                                3 :unk
                                                4 :unk}}))
                               (assoc-in m [new-pos :dirs (opp-dir d)] :path))
                             new-bot)))
            (if (>= 1 (count path))
              maze
              (recur (pop path) maze
                     (run-until-paused (assoc bot :input
                                              [(move-dir (first path)
                                                         (second path))]))))))]
    (loop [path (list pos) maze {pos {:dist 0 :dirs {1 :unk
                                                     2 :unk
                                                     3 :unk
                                                     4 :unk}}}]
      (if-let [[d _] (first (filter (fn [[_ v]] (= v :unk))
                                    (get-in maze [(peek path) :dirs])))]
        (let [cur-pos (peek path)
              new-pos (move-pos d (peek path))
              new-dist (inc (get-in maze [cur-pos :dist]))
              new-path (conj path new-pos)]
          (case (get-in ground-maze [cur-pos :dirs d])
            :wall (recur path (assoc-in maze [(peek path) :dirs d] :wall)
                 )
            :path (recur (conj path new-pos)
                   (as-> maze m
                     (assoc-in m [(peek path) :dirs d] :path)
                     (if (contains? m new-pos)
                       (if (< new-dist (get-in m [new-pos :dist]))
                         (assoc-in m [new-pos :dist] new-dist)
                         m)
                       (assoc m new-pos
                              {:dist new-dist
                               :dirs {1 :unk
                                      2 :unk
                                      3 :unk
                                      4 :unk}}))
                     (assoc-in m [new-pos :dirs (opp-dir d)] :path))
                  )))
        (if (>= 1 (count path))
          maze
          (recur (pop path) maze))))))
