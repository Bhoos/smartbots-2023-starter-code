package core

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"

	. "github.com/Bhoos/smartbots-2023-starter-code/go/models"
)

func HandleHi(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Headers", "*")

	positive_resp := Greeting{"hello"}
	w.Header().Set("Content-Type", "application/json")

	err := json.NewEncoder(w).Encode(positive_resp)

	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
	}

	w.WriteHeader(http.StatusOK)
}

func HandleBid(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Headers", "*")
	if r.Method == "OPTIONS" { // for OPTIONS preflight response
		w.WriteHeader(http.StatusOK)
		return
	}

	var bid_request BidRequest

	err := json.NewDecoder(r.Body).Decode(&bid_request)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
		return
	}

	bid_value := getBid(bid_request)

	w.Header().Set("Content-Type", "application/json")

	err = json.NewEncoder(w).Encode(BidValue{bid_value})
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
	}

	w.WriteHeader(http.StatusOK)
}

func HandleTrump(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Headers", "*")
	if r.Method == "OPTIONS" { // for OPTIONS preflight response
		w.WriteHeader(http.StatusOK)
		return
	}

	var trump_request TrumpRequest

	err := json.NewDecoder(r.Body).Decode(&trump_request)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
		return
	}

	trump_suit := selectTrumpSuit(trump_request)

	w.Header().Set("Content-Type", "application/json")

	err = json.NewEncoder(w).Encode(TrumpSuit{trump_suit})
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
	}

	w.WriteHeader(http.StatusOK)
}

func HandlePlay(w http.ResponseWriter, r *http.Request) {
	w.Header().Set("Access-Control-Allow-Origin", "*")
	w.Header().Set("Access-Control-Allow-Headers", "*")
	w.Header().Set("Access-Control-Allow-Methods", "*")
	if r.Method == "OPTIONS" { // for OPTIONS preflight response
		w.WriteHeader(http.StatusOK)
		return
	}

	var play_request PlayRequest

	err := json.NewDecoder(r.Body).Decode(&play_request)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
		return
	}

	fmt.Printf("\nState:\n%v\n", play_request)

	card_to_play, reveal_trump, play_trump, err := play(play_request)

	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
		return
	}
	if reveal_trump && play_trump {
		w.Header().Set("Content-Type", "application/json")

		err = json.NewEncoder(w).Encode(RevealTrumpCard{true, card_to_play})
		if err != nil {
			w.WriteHeader(http.StatusBadRequest)
			fmt.Fprintf(w, err.Error())
		}

		w.WriteHeader(http.StatusOK)
		return
	} else if reveal_trump {
		w.Header().Set("Content-Type", "application/json")

		err = json.NewEncoder(w).Encode(RevealTrumpNoCard{true})
		if err != nil {
			w.WriteHeader(http.StatusBadRequest)
			fmt.Fprintf(w, err.Error())
		}

		w.WriteHeader(http.StatusOK)
		return
	}

	w.Header().Set("Content-Type", "application/json")

	err = json.NewEncoder(w).Encode(PlayCard{card_to_play})
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		fmt.Fprintf(w, err.Error())
	}
	w.WriteHeader(http.StatusOK)
}

func EntryPoint(port_addr int) {
	http.HandleFunc("/hi", HandleHi)

	http.HandleFunc("/bid", HandleBid)

	http.HandleFunc("/chooseTrump", HandleTrump)

	http.HandleFunc("/play", HandlePlay)

	port_str := ":" + fmt.Sprint(port_addr)

	log.Fatal(http.ListenAndServe(port_str, nil))
}
