(def str-input "047801")

(def num-input 47801)

(def vec-input [0 4 7 8 0 1])

(defn digitise [^Integer x] (->> (str x)
                        (map str)
                        (mapv #(Integer/parseInt ^String %))))

(defn lazy-recipes [elf-1 elf-2 recp-init]
  (lazy-cat recp-init
            ((fn rcp [e1 e2 rec]
               (let [new-recipes (digitise (+ (rec e1) (rec e2)))
                     y (+ (count new-recipes) (count rec))
                     new-e1 (mod (+ e1 (rec e1) 1) y)
                     new-e2 (mod (+ e2 (rec e2) 1) y)]
                 (lazy-cat new-recipes
                           (rcp new-e1 new-e2 (apply conj rec new-recipes)))))
             elf-1 elf-2 recp-init)))

(defn scan-seq
  "Scan over the sequence s with a slice length of n"
  [n s]
  (let [init (apply conj (clojure.lang.PersistentQueue/EMPTY) (take n s))]
    ((fn sc [q s]
       (let [new-q (pop (conj q (first s)))]
         (lazy-cat [q] (sc new-q (rest s)))))
     init (drop n s))))

(def part-1 (apply str (transduce (comp (drop num-input)
                                        (take 10))
                                  conj
                                  (lazy-recipes 0 1 [3 7]))))

(def part-2 (reduce (fn [i q] (if (= q vec-input)
                                (reduced i)
                                (+ i 1))) 0 (scan-seq 6 (lazy-recipes 0 1 [3 7]))))
