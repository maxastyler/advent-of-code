(defn match-line [l]
  (condp re-matches l
    #"\[1518-\d+-\d+ \d\d:(\d\d)\] wakes up"
    :>> (fn [[_ t]] {:state :wake, :time (Integer/parseInt t)})
    #"\[1518-\d+-\d+ \d\d:(\d\d)\] falls asleep"
    :>> (fn [[_ t]] {:state :sleep, :time (Integer/parseInt t)})
    #"\[1518-\d+-\d+ \d\d:\d\d\] Guard #(\d+) begins shift"
    :>> (fn [[_ id]] {:state :shift, :id (Integer/parseInt id)})
    nil
    ))

(def input (as-> (slurp "./input") i
               (clojure.string/split i #"\n")
               (sort i)
               (map match-line i)
               ))

(defn add-sleeps [sleep-vec from to]
  (reduce #(update %1 %2 inc) sleep-vec (range from to)))

(def guard-sleeps
  (let [ids (disj (set (map :id input)) nil)
        init-guards (apply hash-map
                           (mapcat #(vector % (vec (repeat 60 0))) ids))]
    (nth (reduce (fn [[current-id sleep-at sleeps] {:keys [state time id]}]
                   (case state
                     :shift [id nil (if (nil? sleep-at)
                                      sleeps
                                      (update sleeps
                                              current-id
                                              add-sleeps
                                              sleep-at
                                              60))]
                     :wake [current-id nil (if (nil? sleep-at)
                                             sleeps
                                             (update sleeps
                                                     current-id
                                                     add-sleeps
                                                     sleep-at
                                                     time))]
                     :sleep [current-id time sleeps]))
                 [nil nil init-guards]
                 input)
         2)))

(defn max-index [v] (->> v
                         (map-indexed vector)
                         (apply max-key second)
                         (first)))

(def part-1 (let [[max-id max-minutes]
                      (apply max-key #(apply + (val %)) guard-sleeps)
                      max-minute (max-index max-minutes)]
                  (* max-id max-minute)))

(def part-2 (let [[id minutes] (apply max-key
                                      (fn [[_ v]] (apply max v))
                                      guard-sleeps)]
              (* (max-index minutes) id)))

