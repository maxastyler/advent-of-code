(ns aoc-clojure-2020.helpers
  (:require [clojure.java.io :as io]))

(defn get-input [day]
  (some-> (format "day-%02d" day)
          io/resource
          slurp))
