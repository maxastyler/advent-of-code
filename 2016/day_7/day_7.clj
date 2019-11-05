(require '[clojure.string :as str])

(def input
  (->> (str/split (slurp "./input") #"\n")
       (map split-ip)))

(defn contains-abba [string]
  (some identity (mapv abba?
                       (drop 0 string)
                       (drop 1 string)
                       (drop 2 string)
                       (drop 3 string))))

(defn abba? [a b c d]
  (and (= a d) (= b c) (not= a b)))

(defn split-ip [ip]
  (let [fst (str/split ip #"\[")
        snd (str/split (last fst) #"\]")]
    [(first fst) (first snd) (last snd)]))

(defn valid-ip [[a b c]]
  (and (or (contains-abba a)
           (contains-abba c))
       (not (contains-abba b))))

(def tls-ip-count
  (count
   (filter identity
           (map valid-ip input))))

(def tls-ips
  (filter valid-ip input))
