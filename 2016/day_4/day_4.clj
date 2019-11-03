(require '[clojure.string :as str])

(defn convert-num-str [t]
  (->> (str/split t #" ")
       (filter #(not= "" %))
       (map #(Integer/parseInt %))))
(defn remove-dashes [s]
  (apply str (str/split s #"-")))

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map #(re-find #"(\S+\-)(\d+)\[(\S+)\]" %))
       (map (fn [[_ name id csum]]
              {:name (remove-dashes name)
               :id id
               :csum csum}))))
