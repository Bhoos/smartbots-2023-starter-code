use serde::Deserialize;
// ********************** For /bid *************************************** \\

#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BidState {
    pub defender_id: String,   // player_id of defender
    pub challenger_id: String, // player_id of challenger
    pub defender_bid: u32,     // current bid by defender
    pub challenger_bid: u32,   // current bid by challenger
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
    pub player_id: String,               // My player_id
    pub player_ids: [String; 4],         // All 4 player_ids
    pub cards: Vec<String>,              // All 4 cards string
    pub time_remaining: f64,             // time remaining float value
    pub bid_history: Vec<(String, u32)>, // bid history, vector made of tuples (player_id, bid_by_plyer_id)
    pub bid_state: BidState,             // current bid state
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
    pub player_id: String,               // My player_id
    pub player_ids: [String; 4],         // All 4 player_ids
    pub cards: Vec<String>,              // All 4 cards string
    pub time_remaining: f64,             // time remaining float value
    pub bid_history: Vec<(String, u32)>, // bid history, vector made of tuples (player_id, bid_by_plyer_id)
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

// ********************** For /play *********************************** \\
#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub players: [String; 2], // two players in a team
    pub bid: u32,             // maximum bid among both teams (bid of won team)
    pub won: u32,             // total points won by that team
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
// this shows if trumpsuit is known to me(being bidder, or trump already revealed) Option<Suit>, i.e Some(Suit) or None
pub struct TrumpSuit(pub Option<String>);
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
    // Information of trump_revealer
    pub hand: usize,       // The hand he revealed trump on
    pub player_id: String, // player_id of trump_revealer
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
// This shows if trump is Revealed,
// if Yes, it contains the (Some(TrumpRevealer (hand, player_id)),) and
// if No,  it contains the (None,) variant inside the Struct wrapped Option.
pub struct TrumpRevealed(pub Option<TrumpRevealer>); // option version
impl From<TrumpSuitRevealedEnum> for TrumpRevealed {
    fn from(t: TrumpSuitRevealedEnum) -> Self {
        match t {
            TrumpSuitRevealedEnum::NotRevealed(_) => TrumpRevealed(None),
            TrumpSuitRevealedEnum::Revealed(trump_revealer) => TrumpRevealed(Some(trump_revealer)),
        }
    }
}

// ******************************** main PlayPayLoad Struct here *******************************
#[derive(Clone, PartialEq, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayPayload {
    pub player_id: String,                                 // My player_id
    pub player_ids: [String; 4],                           // All 4 player_ids
    pub time_remaining: f64,                               // time remaining float value
    pub teams: [Team; 2],                                  // Two teams
    pub cards: Vec<String>,                                // All remaining cards string
    pub bid_history: Vec<(String, u32)>, // bid history, vector made of tuples (player_id, bid_by_plyer_id)
    pub played: Vec<String>,             // cards played in this hand, mover needs inference
    pub hands_history: Vec<(String, [String; 4], String)>, // player and winner of previous hands (mover, Vec<cardstring>, winner)
    pub trump_suit: TrumpSuit, // Trump Suit struct that wraps Option<Suit>, = either Some(Suit) or none
    pub trump_revealed: TrumpRevealed, // TrumpRevealed struct that wraps Option<TrumpRevealer>, = either Some(TrumpRevealer) or none
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
