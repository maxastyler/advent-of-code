(def password-range [125730 579381])

(defn num->digit [x] (map #(- (int %) (int \0)) (str x)))

(defn digit->num [xs] (read-string (apply str xs)))

(def values (range (apply min password-range) (apply max password-range)))

(defn contains-repeating-numbers? [s]
  (not-empty (let [matcher (re-matcher #"(\d)\1+" s)]
               (loop [matches []]
                 (let [match (re-find matcher)]
                   (if (nil? match)
                     matches
                     (recur (conj matches match))))))))

(defn contains-pair? [s]
  (not-empty (let [matcher (re-matcher #"(\d)\1+" s)]
               (loop [matches []]
                 (let [match (re-find matcher)]
                   (if (nil? match)
                     matches
                     (if (not= (count (match 0)) 2)
                       (recur matches)
                       (recur (conj matches match)))))))))

(defn ascending? [l] (every? (fn [[a b]] (<= (int a) (int b))) l))

(def part-1 (transduce (comp
                        (map str)
                        (filter contains-repeating-numbers?)
                        (map #(partition 2 1 (str %)))
                        (filter ascending?)
                        (map (constantly 1))) + 0 values))

(def part-2 (transduce (comp
                        (map str)
                        (filter contains-pair?)
                        (map #(partition 2 1 (str %)))
                        (filter ascending?)
                        (map (constantly 1))) + 0 values))
