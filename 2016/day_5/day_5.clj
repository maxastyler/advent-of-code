(import 'java.security.MessageDigest
        'java.math.BigInteger)

(defn md5
  [^String s]
  (->> s
       .getBytes
       (.digest (MessageDigest/getInstance "MD5"))
       (BigInteger. 1)
       (format "%032x")))

(def input "reyedfim")

(def hashes
  (map #(md5 (str input %)) (range)))

(def valid-hashes-p1
  (filter #(= '(\0 \0 \0 \0 \0) (take 5 %)) hashes))

(def valid-hashes-p2
  (filter #(and (>= (int (nth % 5)) 48) (< (int (nth % 5)) 56))
          valid-hashes-p1))

(def password-p1
  (->> (take 8 valid-hashes-p1)
       (map #(nth % 5))
       (apply str)))

(def password-p2 (reduce
                  (fn [acc v]
                    (let [key (int (nth v 5))
                          val (int (nth v 6))
                          new-pass (replace-maybe acc key val)]
                      (if (not-filled? new-pass)
                        new-pass
                        (reduced new-pass))))
                  (vec (repeat 8 nil))
                  valid-hashes-p2))

;; (def password-p2 (take 8 valid-hashes-p2))
(defn replace-maybe [v key val]
  (if (nil? (nth v key)) (assoc v key val) v))

(defn not-filled? [v]
  (nil? (reduce #(and %1 %2) v)))
