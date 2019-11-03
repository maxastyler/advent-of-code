(require '[clojure.string :as str])

(defn remove-dashes [s]
  (apply str (str/split s #"-")))

(defn group-by-frequency [s]
  (let [f-dict (frequencies s)]
    (group-by #(get f-dict %) (keys f-dict))))

(defn shift-char [c s]
  (char (+
         (mod (+
               (- (int c)
                  (int \a))
               s) 26)
         (int \a))))

(defn shift-name [n s]
  (defn shift-or-replace [c]
    (case c
      \- \space
      (shift-char c s)
      ))
  (apply str (map shift-or-replace n))
  )

(defn freq-dict-to-vec [f]
  (->> (keys f)
       (sort >)
       (map #(->> (get f %)
                  (apply str)
                  (sort)
              ))
       flatten
       (take 5)
       (apply str)))

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map #(re-find #"(\S+\-)(\d+)\[(\S+)\]" %))
       (map (fn [[_ name id csum]]
              {:name (remove-dashes name)
               :id (Integer/parseInt id)
               :csum csum}))))

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map #(re-find #"(\S+\-)(\d+)\[(\S+)\]" %))
       (map (fn [[_ name id csum]]
              {:name (remove-dashes name)
               :id (Integer/parseInt id)
               :csum csum}))))

(def part2-input
  (->> (str/split (slurp "./input") #"\n")
       (map #(re-find #"(\S+\-)(\d+)\[(\S+)\]" %))
       (map (fn [[_ name id csum]]
              {:name (shift-name name (Integer/parseInt id))
               :id (Integer/parseInt id)
               :csum csum}))))

(def valid-inputs
  (filter
   #(= (:csum %)
       (freq-dict-to-vec
        (group-by-frequency (:name %)))) input))

(def valid-inputs-sum
  (reduce + 0 (map :id valid-inputs)))
