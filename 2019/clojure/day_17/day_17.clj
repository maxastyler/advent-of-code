(def input (as-> (slurp "input") i
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

(defn neighbours [[x y]]
  [[(inc x) y]
   [(dec x) y]
   [x (inc y)]
   [x (dec y)]])


(def part-1 (as-> (run-until-paused
                   {:pointer 0
                    :tape input
                    :input []
                    :output []
                    :base 0}) i
              (:output i)
              (let [width (count (take-while #(not= 10 %) i))
                    no-newline (filter #(not= 10 %) i)
                    height (/ (count no-newline) width)
                    scaffolds (->> no-newline
                                   (map-indexed vector)
                                   (filter #(= (second %) 35))
                                   (map (fn [[x _]]
                                          [(mod x width) (quot x width)]))
                                   (into #{}))
                    intersections (->> scaffolds
                                       (filter
                                        #(>= (count (filter scaffolds
                                                            (neighbours %)))
                                             4)))
                    alignment-params (map #(apply * %) intersections)]
                (apply + alignment-params))))
