(defun get-file (filename)
  (with-open-file (stream filename)
    (loop for line = (read-line stream nil)
          while line
       collect (parse-integer line))))

(defun fuel-req (fuel)
  (- (floor fuel 3) 2))

(defun fuel-req-rec (fuel)
  (labels ((fuel-req-rec (fuel total)
           (let ((new-fuel (fuel-req fuel)))
             (if (> new-fuel 0)
                 (fuel-req-rec new-fuel (+ total fuel))
                 (+ total fuel)))))
    (fuel-req-rec fuel 0)))

(defvar *part-1* (apply #'+ (map 'list #'fuel-req-rec (get-file "input"))))
(defvar *part-2* (- (apply #'+ (map 'list #'fuel-req-rec (get-file "input")))
                    (apply #'+ (get-file "input"))))
