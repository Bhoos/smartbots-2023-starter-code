;;;; 29points.lisp

(in-package #:29Points)
#+debug(defparameter *last-game* nil)
(defun card-point (card)
  (let ((number (card-number card)))
    (case number
      (#.(decode-card-number #\J) 3)
      (#.(decode-card-number #\9) 2)
      (#.(decode-card-number #\1) 1)
      (#.(decode-card-number #\T) 1)
      (t 0))))

(defun simple-bid (game)
  (let* ((my-card-points (reduce #'+ (player-cards (game-me game)) :key #'card-point))
         ;; A heuristic for how many points I expect
         (expected-bid (+ my-card-points (floor (/ (- 28 my-card-points) 3))))
         (bids (game-bid-state game))
         (defender? (eql (bid-state-defender bids)
                         (game-me game)))
         (competing-bid (if defender?
                            (bid-state-challenger-bid bids)
                            (bid-state-defender-bid bids))))
    (cond ((and (= (length (game-bid-history game)) 3)
                (every (lambda (bid)
                         (= (bid-history-bid bid) 0))
                       (game-bid-history game)))
           16) ;; If everyone has passed, I say 16.
          ((> competing-bid expected-bid)
           0) ;; pass
          ;; I expect higher or equal to competing-bid
          ((eql defender? t)
           (u:clamp competing-bid 16 28)) ;; match the competing bid
          ;; I am challenger
          ((> expected-bid competing-bid)
           (u:clamp (+ competing-bid 1) 16 28)) ;; raise the competing bid by one
          (t 0)))) ;; pass

(defun bid (game)
  #+debug(setf *last-game* game)
  (simple-bid game))

(defun choose-trump (game)
  (flet ((suit-count (suit)
           (count-if (lambda (card)
                       (= (card-suit card) suit))
                     (player-cards (game-me game)))))
    (u:extremum (u:iota 4) #'> :key #'suit-count)))

(defun random-valid-move (game)
  (let ((current-suit (when (> (length (game-played-cards game)) 0)
                        (card-suit (elt (game-played-cards game) 0))))
        (my-cards (player-cards (game-me game))))
    (flet ((trump-or-random ()
             (let ((trumps (u:keep (game-trump-suit game) my-cards :key #'card-suit)))
               (if (> (length trumps) 0)
                   (u:random-elt trumps)
                   (u:random-elt my-cards)))))
      (cond
        ;; I am first player, play anything
        ((null current-suit)
         (u:random-elt my-cards))
        ;; I have current-suit card, play one of them
        ((find current-suit my-cards :key #'card-suit)
         (u:random-elt (u:keep current-suit my-cards :key #'card-suit)))
        ;; I revealed the trump card just now, play trump suit
        ((and (game-trump-revealed game)
              (= (trump-reveal-round (game-trump-revealed game))
                 (1+ (length (game-history game))))
              (eql (trump-reveal-player (game-trump-revealed game))
                   (game-me game)))
         (trump-or-random))
        ;; maybe ask to reveal trump suit
        ((and (not (game-trump-revealed game))
              (= 0 (random 2)))
         (values (when (game-trump-suit game)
                   (trump-or-random)) ;; if I had set trump, send trump card (if possible)
                 t))
        ;; else throw any card
        (t (u:random-elt my-cards))))))

(defun play (game)
  #+debug(setf *last-game* game)
  (random-valid-move game))


;;;; API & Server
(defun parse-request ()
  (u:when-let* ((text (h:raw-post-data :force-text t))
                (game (handler-case (parse-json text)
                        (t (err)
                          (format t "~&[Error] While parsing input for ~a. Err: ~a~%"
                                  (h:request-uri*)
                                  err)
                          nil))))
    game))


(defun set-cors-headers ()
  (setf (h:header-out "Access-Control-Allow-Origin") "*")
  (setf (h:header-out "Access-Control-Allow-Methods") "POST,GET,OPTIONS,DELETE,PUT")
  (setf (h:header-out "Access-Control-Allow-Headers") "x-requested-with, Content-Encoding, Content-Type, origin, authorization, accept, client-security-token"))

(h:define-easy-handler (hi :uri "/hi") ()
  (set-cors-headers)
  "{ \"value\" : \"hello\" } ")

(h:define-easy-handler (bid-route :uri "/bid") ()
  (set-cors-headers)
  (when (eql (h:request-method*) :post)
    (u:when-let* ((game (parse-request)))
      (let ((bid (bid game)))
        (format nil "{\"bid\" : ~d}" bid)))))

(h:define-easy-handler (play-route :uri "/play")
    ()
  (set-cors-headers)
  (when (eql (h:request-method*) :post)
    (u:when-let ((game (parse-request)))
      (multiple-value-bind (card reveal-trump) (play game)
        (cond ((null reveal-trump)
               (format nil "{ \"card\": ~s }" (encode-card card)))
              ((null card)
               (format nil "{ \"revealTrump\": true }"))
              (t
               (format nil "{ \"card\": ~s,~% \"revealTrump\" : true }" (encode-card card))))))))

(h:define-easy-handler (choose-trump-route :uri "/chooseTrump")
    ()
  (set-cors-headers)
  (when (eql (h:request-method*) :post)
    (let ((game (parse-json (h:raw-post-data :force-text t))))
      (let ((result (choose-trump game)))
        (format nil " { \"suit\" : \"~a\" }" (encode-card-suit result))))))


(defvar *server* nil)
(defparameter *single-threadedp* nil)

(defun start (&optional (port 8001))
  (when *server*
    (stop))
  (unless *server*
    (setf *server*
          (if (not *single-threadedp*)
              (make-instance 'h:easy-acceptor :port port)
              (make-instance 'h:easy-acceptor
                             :port port
                             :taskmaster (make-instance 'h:single-threaded-taskmaster))))

    (setf (h:acceptor-access-log-destination *server*) nil))
  (handler-case (h:start *server*)
    (usocket:address-in-use-error (err)
      (setf *server* nil)
      (error err))
    (sb-sys:interactive-interrupt (err)
      (declare (ignore err))
      (stop))))


(defun stop ()
  (when *server*
    (h:stop *server*)
    (setf *server* nil))
  t)

(defun main ()
  (let* ((args (uiop:command-line-arguments))
         (port (or (and args
                        (stringp (first args))
                        (parse-integer (first args) :junk-allowed t))
                   8001)))
    (handler-bind ((usocket:address-in-use-error
                     (lambda (err)
                       (declare (ignorable err))
                       (format t "Port ~d already in use." port)))
                   (sb-sys:interactive-interrupt
                     (lambda (err)
                       (declare (ignorable err))
                       (format t "Interrupt Recieved. Quitting~%")
                       (sb-ext:quit))))
      (format t "Bot server starting at port ~d" port)
      (finish-output *standard-output*)
      (start port)
      (loop (sleep 1000)))))
