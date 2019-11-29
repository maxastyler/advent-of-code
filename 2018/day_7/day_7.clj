(def input
  (->> "./input"
       (slurp)
       (clojure.string/split-lines)
       (map #(->> %
                  (re-find
                   #"Step (\w+) must be finished before step (\w+) can begin.")
                  (drop 1)
                  (reverse)))))

(def all-steps (set (apply concat input)))

(def dependencies "the key is the step which depends on all the others"
  (->> input
       (group-by #(first %))
       (map (fn [[k v]] [k (mapv second v)]))
       (into {})))

(defn none-of [& all] (apply every-pred (map (fn [x] #(not= x %)) all)))

(def order-steps
  (loop [deps dependencies
         steps-left all-steps
         step-order []]
    (let [next-step (->> (keys deps)
                         (apply disj steps-left)
                         (sort)
                         (first))
          new-deps (->> deps
                        (map (fn [[k v]] [k (filter #(not= % next-step) v)]))
                        (filter (fn [[_ v]] (not (empty? v))))
                        (into {}))
          new-step-order (conj step-order next-step)
          new-steps-left (disj steps-left next-step)]
        (if (empty? new-steps-left)
          new-step-order
          (recur new-deps new-steps-left new-step-order)))))

(defn time-char [c] (+ (- (int c) (int \A)) 61))

(defn order-steps-workers [n-workers]
  (loop [deps dependencies
         steps-left all-steps
         step-order []
         workers (repeat n-workers {:char nil :time 62})]
    (let [next-step (->> (keys deps)
                         (apply disj steps-left)
                         (sort)
                         (first))
          new-deps (->> deps
                        (map (fn [[k v]] [k (filter #(not= % next-step) v)]))
                        (filter (fn [[_ v]] (not (empty? v))))
                        (into {}))
          new-step-order (conj step-order next-step)
          new-steps-left (disj steps-left next-step)]
        (if (empty? new-steps-left)
          new-step-order
          (recur new-deps new-steps-left new-step-order)))))

(def part-1 (apply str order-steps))
