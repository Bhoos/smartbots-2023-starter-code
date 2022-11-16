(in-package :29points)

;; Data types

(deftype suit ()
  `(integer 0 3))

(deftype card-number ()
  `(integer 0 7))

(defstruct card
  (suit 0 :type suit)
  (number 0 :type card-number))

(deftype player-id ()
  `(integer 0 4))

(defstruct team
  (bid 0 :type fixnum)
  (won 0 :type fixnum))

(defstruct player
  (id 0 :type player-id)
  (name "" :type string)
  (me? nil :type boolean)
  (team nil :type team)
  (cards #() :type (simple-array card 1)))

(defstruct history
  (first nil :type player)
  (winner nil :type player)
  (moves (make-array 4 :initial-element (make-card) :element-type 'card) :type (simple-array card (4))))

(defstruct bid-history
  (player nil :type player)
  (bid 0 :type (integer 0 29)))

(defstruct trump-reveal
  (player nil :type player)
  (round 1 :type u:positive-fixnum))

(defstruct bid-state
  (defender nil :type player)
  (challenger nil :type player)
  (defender-bid nil :type fixnum)
  (challenger-bid nil :type fixnum))

(defstruct game
  (players nil :type (simple-array player (4)))
  (me nil :type player)
  (my-team nil :type team)
  (opponent-team nil :type team)
  (played-cards nil :type (simple-array card 1))
  (history (make-array 0 :element-type 'history) :type (simple-array history 1))
  (time-remaining 0e0 :type single-float)
  (trump-suit nil :type (or null suit))
  (trump-revealed nil :type (or null trump-reveal))
  (bid-history nil :type (vector bid-history))
  (bid-state nil :type (or null bid-state)))

;;; PARSING

;;; encode/decode

(u:eval-always
  (defun decode-card-number (char)
    (case char
      (#\7 0)
      (#\8 1)
      (#\Q 2)
      (#\K 3)
      (#\T 4) ;; 10
      (#\1 5)
      (#\9 6)
      (#\J 7)
      (t (error "Invalid card number")))))

(defun decode-card-suit (char)
  (ecase char
    (#\H 0)
    (#\C 1)
    (#\D 2)
    (#\S 3)))

(defun encode-card-number (card-number)
  (ecase card-number
    (0 #\7)
    (1 #\8)
    (2 #\Q)
    (3 #\K)
    (4 #\T)
    (5 #\1)
    (6 #\9)
    (7 #\J)))

(defun encode-card-suit (suit)
  (case suit
    (0 #\H)
    (1 #\C)
    (2 #\D)
    (3 #\S)))

(defun encode-card (card)
  (let ((number (encode-card-number (card-number card)))
        (suit (encode-card-suit (card-suit card))))
    (format nil "~c~c" number suit)))

;; parse input

(defun parse-card (card)
  (make-card
   :number (decode-card-number (char card 0))
   :suit (decode-card-suit (char card 1))))

(defun parse-history (players history)
  (map '(simple-array history 1)
       (lambda (entry)
         (make-history :first (find (first entry) players :key #'player-name :test #'string-equal)
                       :winner (find (third entry) players :key #'player-name :test #'string-equal)
                       :moves (parse-cards (second entry))))
       history))

(defun parse-cards (cards)
  (map '(simple-array card 1) #'parse-card cards))

(defun parse-json (text)
  (let* ((json (jonathan:parse text :as :hash-table))
         (me (gethash "playerId" json))
         (my-id 0)
         (teams (list (make-team) (make-team)))
         (players (loop for name in (gethash "playerIds" json)
                        for me? = (string-equal name me)
                        for id from 0
                        for team-idx = 0 then (mod (1+ team-idx) 2)
                        when me?
                          do (setf my-id id)
                        collect (make-player :id id
                                             :name name
                                             :team (elt teams team-idx)
                                             :me? me?))))
    (flet ((find-player (player-name)
             (find player-name players :key #'player-name :test #'string-equal)))
      ;; teams
      (u:when-let ((teams (gethash "teams" json)))
        (loop for team-json in teams
              for players = (gethash "players" team-json)
              for team = (player-team (find-player (elt players 0))) do
                (setf (team-bid team) (or (gethash "bid" team-json) 0)
                      (team-won team) (or (gethash "won" team-json) 0))))
      ;; cards
      (setf (player-cards (elt players my-id)) (parse-cards (gethash "cards" json)))
      ;; bid-history & bid state if available
      (let (bid-history bid-state)
        ;; bidHistory
        (u:when-let ((history (gethash "bidHistory" json)))
          (setf bid-history
                (loop for (player-name bid) in history
                      collect (make-bid-history
                               :player (find-player player-name)
                               :bid (etypecase bid
                                      (string 0)
                                      (number bid))))))
        ;; bidState
        (u:when-let ((obj (gethash "bidState" json)))
          (setf bid-state (make-bid-state :defender (find-player (gethash "defenderId" obj))
                                          :challenger (find-player (gethash "challengerId" obj))
                                          :defender-bid (gethash "defenderBid" obj)
                                          :challenger-bid (gethash "challengerBid" obj))))

        ;; Finally the game object
        (make-game :players (map '(simple-array player 1) #'identity players)
                   :me (elt players my-id)
                   :my-team (player-team (elt players my-id))
                   :opponent-team (player-team (elt players (mod (1+ my-id) 4)))
                   :played-cards (parse-cards (gethash "played" json nil))
                   :history (parse-history players (gethash "handsHistory" json nil))
                   :trump-suit (u:when-let ((suit (gethash "trumpSuit" json)))
                                 (decode-card-suit (char suit 0)))
                   :trump-revealed (u:when-let ((move (gethash "trumpRevealed" json)))
                                     (make-trump-reveal :player (find-player (gethash "playerId" move))
                                                        :round (gethash "hand" move)))
                   :bid-history (map 'vector #'identity bid-history)
                   :bid-state bid-state
                   :time-remaining (coerce (gethash "timeRemaining" json 1500) 'single-float))))))
