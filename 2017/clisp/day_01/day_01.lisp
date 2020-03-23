(ql:quickload "cl-ppcre")

(defun get-file (filename)
  (with-open-file (stream filename)
    (loop for char = (read-char stream nil)
       while char
       collect (digit-char-p char))))
