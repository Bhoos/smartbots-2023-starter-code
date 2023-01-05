use serde::Deserialize;

/// datatypes for  post request`/bid`

// ********************** For /bid *************************************** \\

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BidState {
    pub defender_id: String,
    pub challenger_id: String,
    pub defender_bid: u32,
    pub challenger_bid: u32,
}

impl std::fmt::Display for BidState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "        defenderId : {},", self.defender_id)?;
        writeln!(f, "        challengerId : {},", self.challenger_id)?;
        writeln!(f, "        defenderBid : {},", self.defender_bid)?;
        writeln!(f, "        challengerBid : {},", self.challenger_bid)?;
        write!(f, "    }}")
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BidPayload {
    pub player_id: String,
    pub player_ids: [String; 4],
    pub cards: Vec<String>,
    pub time_remaining: f64,
    pub bid_history: Vec<(String, u32)>,
    pub bid_state: BidState,
}

impl std::fmt::Display for BidPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "    playerId : {},", self.player_id)?;
        writeln!(f, "    playerIds : {:?},", self.player_ids)?;
        writeln!(f, "    cards : {:?},", self.cards)?;
        writeln!(f, "    timeRemaining : {},", self.time_remaining)?;
        writeln!(f, "    bidHistory : [")?;
        for i in &self.bid_history {
            writeln!(f, "        {:?},", i)?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "    bidState : {},", self.bid_state)?;
        write!(f, "}}")
    }
}

// ********************** For /chooseTrump ***************************** \\
#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChooseTrumpPayload {
    pub player_id: String,
    pub player_ids: [String; 4],
    pub cards: Vec<String>,
    pub time_remaining: f64,
    pub bid_history: Vec<(String, u32)>,
}

impl std::fmt::Display for ChooseTrumpPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "    playerId : {},", self.player_id)?;
        writeln!(f, "    playerIds : {:?},", self.player_ids)?;
        writeln!(f, "    cards : {:?},", self.cards)?;
        writeln!(f, "    timeRemaining : {},", self.time_remaining)?;
        writeln!(f, "    bidHistory : [")?;
        for i in &self.bid_history {
            writeln!(f, "        {:?},", i)?;
        }
        writeln!(f, "    ],")?;
        write!(f, "}}")
    }
}

// {
//     "playerId":"A2",
//     "playerIds":["A1","B1","A2","B2"],
//     "cards":["JS","TS","KH","9C"],
//     "timeRemaining":1000,
//     "bidHistory":[["A1",16],["B1",0]]
// }

// ********************** For /play *********************************** \\

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub players: Vec<String>,
    pub bid: u32,
    pub won: u32,
}
impl std::fmt::Display for Team {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        write!(f, "        players : {:?}", self.players)?;
        write!(f, "        bid : {:?}", self.bid)?;
        writeln!(f, "        won : {:?}", self.won)?;
        write!(f, "    }}")
    }
}

// **************** Json Deserialization Tricks here ********************

#[derive(PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TrumpSuitEnum {
    // enum version
    Suit(String),
    NotShown(bool),
}
#[derive(Clone, Deserialize, PartialEq, Debug)]
#[serde(from = "TrumpSuitEnum")]
pub struct TrumpSuit(pub Option<String>); // option version
impl From<TrumpSuitEnum> for TrumpSuit {
    fn from(t: TrumpSuitEnum) -> Self {
        match t {
            TrumpSuitEnum::NotShown(_) => TrumpSuit(None),
            TrumpSuitEnum::Suit(suit) => TrumpSuit(Some(suit)),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TrumpRevealer {
    pub hand: usize,
    pub player_id: String,
}
impl std::fmt::Display for TrumpRevealer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "        hand : {}", self.hand)?;
        writeln!(f, "        player : {}", self.player_id)?;
        write!(f, "    }}")
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TrumpSuitRevealedEnum {
    // enum version
    Revealed(TrumpRevealer),
    NotRevealed(bool),
}

#[derive(Clone, Deserialize, PartialEq, Debug)]
#[serde(from = "TrumpSuitRevealedEnum")]
pub struct TrumpRevealed(pub Option<TrumpRevealer>); // option version
impl From<TrumpSuitRevealedEnum> for TrumpRevealed {
    fn from(t: TrumpSuitRevealedEnum) -> Self {
        match t {
            TrumpSuitRevealedEnum::NotRevealed(_) => TrumpRevealed(None),
            TrumpSuitRevealedEnum::Revealed(trump_revealer) => TrumpRevealed(Some(trump_revealer)),
        }
    }
}
// #[derive(Deserialize, PartialEq)]
// #[serde(from = "TrumpSuitRevealedEnum")]
/*
type TrumpRevealed = Option::<TrumpRevealer>;
impl From<TrumpSuitRevealedEnum> for TrumpRevealed {
    fn from(t : TrumpSuitRevealedEnum) -> Self {
        match t {
            TrumpSuitRevealedEnum::NotRevealed(_) => None,
            TrumpSuitRevealedEnum::Revealed(trump_revealer) => Some(trump_revealer),
        }
    }
} */

// ******************************** main PlayPayLoad Struct here *******************************
#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayPayload {
    pub player_id: String,
    pub player_ids: [String; 4],
    pub time_remaining: f64,
    pub teams: [Team; 2],
    pub cards: Vec<String>,
    pub bid_history: Vec<(String, u32)>,

    pub played: Vec<String>, // cards played in this hand, mover needs inference
    pub hands_history: Vec<(String, Vec<String>, String)>, // (mover, cards, winner)
    // #[serde(from = "TrumpSuitEnum")]
    pub trump_suit: TrumpSuit,
    // #[serde(from = "TrumpSuitRevealedEnum")]
    pub trump_revealed: TrumpRevealed,
}

impl std::fmt::Display for PlayPayload {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{{")?;
        writeln!(f, "    playerId : {},", self.player_id)?;
        writeln!(f, "    playerIds : {:?},", self.player_ids)?;
        writeln!(f, "    timeRemaining : {},", self.time_remaining)?;
        writeln!(f, "    cards : {:?},", self.cards)?;
        writeln!(f, "    bidHistory : [")?;
        for i in &self.bid_history {
            writeln!(f, "        {:?},", i)?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "    played : {:?}", self.played)?;
        writeln!(f, "    handsHistory : {{")?;
        for i in &self.hands_history {
            writeln!(f, "        {:?}", i)?;
        }
        writeln!(f, "    }}")?;
        writeln!(f, "    trumpSuit : {:?}", self.trump_suit.0)?;
        writeln!(f, "    trumpRevealed : {:?}", self.trump_revealed.0)?;
        write!(f, "}}")
    }
}

// {
//     "playerId":"A2",
//     "playerIds":["A1","B1","A2","B2"],
//     "timeRemaining":1500,
//     "teams":[
//       {"players":["A1","A2"],"bid":17,"won":0},
//       {"players":["B1","B2"],"bid":17,"won":4}
//     ],
//       "cards":["JS","TS","KH","9C","JD","7D","8D"],
//       "bidHistory":[["A1",16],["B1",17],["A1",17],["B1",0],["A2",0],["B2",0]],
//       "played":["9S","1S","8S"],
//       "handsHistory":[
//         ["A1",["7H","1H","8H","JH"],"B2"]
//       ],
//       "trumpSuit":false,
//       "trumpRevealed":false
// }
