#pragma once
#include <cassert>
#include <iostream>
#include <variant>
#include <vector>

#if defined(__GNUC__)
#define UNREACHABLE() __builtin_unreachable()
#elif defined(_MSC_VER)
#define UNREACHABLE() __assume(false);
#endif

enum Rank
{
    SEVEN = 7,
    EIGHT = 8,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
    ACE
};

constexpr int32_t CardValue(Rank rank)
{
    constexpr int32_t card_value[] = { 0, 0, 2, 1, 3, 0, 0, 1 };
    return card_value[(int32_t)rank - (int32_t)Rank::SEVEN];
}

enum Suit
{
    CLUBS = 0,
    DIAMONDS,
    HEARTS,
    SPADES
};

constexpr const char *SuitToStr(Suit suit)
{
    switch (suit)
    {
    case SPADES:
        return "S";
    case DIAMONDS:
        return "D";
    case HEARTS:
        return "H";
    case CLUBS:
        return "C";
    default:
        UNREACHABLE();
    }
}
constexpr Suit StrToSuit(const char *suit)
{
    switch (*suit)
    {
    case 'S':
        return SPADES;
    case 'D':
        return DIAMONDS;
    case 'H':
        return HEARTS;
    case 'C':
        return CLUBS;
    default:
        UNREACHABLE();
    }
}

class Card
{

public:
    Rank rank = ACE;
    Suit suit = SPADES;

    Card()    = default;
    Card(Rank rank, Suit suit) : rank(rank), suit(suit)
    {
    }
    constexpr bool operator==(const Card &other) const
    {
        return rank == other.rank && suit == other.suit;
    }
    constexpr bool operator!=(const Card &other) const
    {
        return !(*this == other);
    }

    static std::string ToStr(Card const &card)
    {
        std::string        name(2, ' ');
        static const char *val = "789TJQK1";
        name[0]                = val[(uint32_t)card.rank - (uint32_t)Rank::SEVEN];

        switch (card.suit)
        {
        case SPADES:
            name[1] = 'S';
            break;
        case DIAMONDS:
            name[1] = 'D';
            break;
        case HEARTS:
            name[1] = 'H';
            break;
        case CLUBS:
            name[1] = 'C';
            break;
        default:
            UNREACHABLE();
        }
        return name;
    }

    static Card FromStr(const char *str)
    {
        Suit suit;
        Rank rank;

        switch (str[1])
        {
        case '7':
            rank = SEVEN;
            break;
        case '8':
            rank = EIGHT;
            break;
        case '9':
            rank = NINE;
            break;
        case 'T':
            rank = TEN;
            break;
        case 'J':
            rank = JACK;
            break;
        case 'Q':
            rank = QUEEN;
            break;
        case 'K':
            rank = KING;
            break;
        case '1':
            rank = ACE;
            break;
        default:
            UNREACHABLE();
        }

        switch (str[2])
        {
        case 'D':
            suit = DIAMONDS;
            break;
        case 'S':
            suit = SPADES;
            break;
        case 'H':
            suit = HEARTS;
            break;
        case 'C':
            suit = CLUBS;
            break;
        default:
            UNREACHABLE();
        }
        return Card(rank, suit);
    }
};

std::ostream &operator<<(std::ostream &os, Card const& card);

using PlayerID = std::string;

struct BidEntry
{
    int32_t  bid_value;
    PlayerID player_id;
};

struct BidState
{
    BidEntry challenger;
    BidEntry defender;
};

struct PlayPayload
{

    struct Teams
    {
        PlayerID players[2];
        int32_t  bid;
        int32_t  won;
    };

    struct HandHistoryEntry
    {
        PlayerID          initiator;
        PlayerID          winner;
        std::vector<Card> card; // card played in that round in chronological order, they say
    };

    struct RevealedObject
    {
        int32_t  hand;      // hand at which the trump was revealed
        PlayerID player_id; // Player who revealed the trump
    };

    // Or simply replace with union
    std::variant<bool, Suit>           trumpSuit      = false;
    std::variant<bool, RevealedObject> trumpRevealed  = false;

    int32_t                            remaining_time = 0;
    PlayerID                           player_id;

    Teams                              teams[2];

    std::vector<PlayerID>              player_ids;
    std::vector<Card>                  cards;
    std::vector<Card>                  played;
    std::vector<BidEntry>              bid_history;

    // hand history
    std::vector<HandHistoryEntry> hand_history;

    PlayPayload()                               = default;

    PlayPayload(PlayPayload const &)            = delete;
    PlayPayload(PlayPayload &&)                 = default;
    PlayPayload &operator=(PlayPayload const &) = delete;
    PlayPayload &operator=(PlayPayload &&)      = default;
};

std::ostream &operator<<(std::ostream &os, PlayPayload const &payload);

struct PlayAction
{
    enum Action
    {
        None          = 0,
        RevealTrump   = 1,
        PlayCard      = 2,
        RevealAndPlay = RevealTrump | PlayCard
    };

    Action action = PlayCard;
    Card   played_card;
};

struct GameState
{
    // Keep any game related extra metadata here
    // like std::vector<Card> seen_cards;

    // Time remaining is in milliseconds
    static Suit    ChooseTrump(PlayerID myid, std::vector<PlayerID> player_ids, std::vector<Card> mycards,
        int32_t time_remaining, std::vector<BidEntry> bid_history);
    static int32_t Bid(PlayerID myid, std::vector<PlayerID> player_ids, std::vector<Card> mycards,
                       int32_t time_remaining, std::vector<BidEntry> bid_history, BidState const &bid_state);

    // If two players are to play through same instance, it might be troublesome (lol, that shouldn't happen though ig)
    PlayAction Play(PlayPayload payload);
};

GameState &GetGameInstance();
void       InitGameInstance();