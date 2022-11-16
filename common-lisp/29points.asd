;;;; 29Points.asd

(asdf:defsystem #:29points
  :description "Bot for 29Points Card Game"
  :author "Bibek Panthi <bpanthi977@gmail.com>"
  :license  "MIT"
  :version "0.0.1"
  :serial t
  :depends-on (#:alexandria #:serapeum #:hunchentoot #:jonathan #:let-plus #:quux-hunchentoot #:defstar)
  :build-operation "program-op"
  :build-pathname "../build/29Points"
  :entry-point "29Points::main"
  :components ((:file "package")
               (:file "data-types")
               (:file "29points")))
