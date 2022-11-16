;;;; package.lisp

(uiop:define-package #:29points/utils
  (:use #:cl #:alexandria #:serapeum #:let-plus)
  (:reexport #:alexandria
             #:serapeum
             #:let-plus))


(defpackage #:29points
  (:use #:cl #:defstar)
  (:local-nicknames (#:u #:29points/utils)
                    (#:h #:hunchentoot))
  (:export
   #:main
   #:start))
