import ArgumentParser
import Foundation

enum RTError : Error {
    case runtimeError(String)
}

enum Card: Int {
    case ace = 13
    case king = 12
    case queen = 11
    case ten = 10
    case nine = 9
    case eight = 8
    case seven = 7
    case six = 6
    case five = 5
    case four = 4
    case three = 3
    case two = 2
    case joker = 1
}

func char_to_card(c: Character) throws -> Card {
    switch c {
    case "A":
        return .ace
    case "K":
        return .king
    case "Q":
        return .queen
    case "J":
        return .joker
    case "T":
        return .ten
    case "9":
        return .nine
    case "8":
        return .eight
    case "7":
        return .seven
    case "6":
        return .six
    case "5":
        return .five
    case "4":
        return .four
    case "3":
        return .three
    case "2":
        return .two
    case _:
        throw RTError.runtimeError("bad card")
    }
}

enum HandType: Int {
    case five = 7
    case four = 6
    case full = 5
    case three = 4
    case twoPair = 3
    case onePair = 2
    case high = 1
}

struct Hand: Comparable {
    var str_cards: String
    var cards: [Card]
    var typ: HandType
    var bid: Int

    static func < (lhs: Hand, rhs: Hand) -> Bool {
        if lhs.typ == rhs.typ && lhs.cards != rhs.cards {
            for i in 0..<lhs.cards.count {
                if lhs.cards[i] == rhs.cards[i] {
                    continue;
                } else {
                    return lhs.cards[i].rawValue < rhs.cards[i].rawValue
                }
            }
            // throw RTError.runtimeError("unreachable")
        } else {
            return lhs.typ.rawValue < rhs.typ.rawValue
        }
        return true // unreachable I think
    }

    static func == (lhs: Hand, rhs: Hand) -> Bool {
        return lhs.typ == rhs.typ && lhs.bid == rhs.bid
            && lhs.str_cards == rhs.str_cards
    }
}



// impl PartialOrd for Hand {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

func determine_type(hand: String) throws -> HandType {
    var map: [Character: Int] = [:]
    for c in hand {
        if let entry = map[c] {
            map[c] = entry + 1
        } else {
            map[c] = 1
        }
    }
    let jokers = map["J"] ?? 0
    map["J"] = nil
    var sort_count = Array(map.values)
    sort_count.sort()
    sort_count.reverse()

    switch jokers {
    case 5:
        return .five
    case 4:
        return .five // matches whatever else was there
    case 3:
        switch sort_count[0] {
        case 2:
            return .five
        case 1:
            return .four
        default:
            throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
        }
    case 2:
        switch sort_count[0] {
        case 3:
            return .five
        case 2:
            return .four
        case 1:
            return .three
        default:
            throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
        }
    case 1:
        switch sort_count[0] {
        case 4:
            return .five
        case 3:
            return .four
        case 2:
            switch sort_count[1] {
            case 2:
                return .full
            case 1:
                return .three
            default:
                throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
            }
        case 1:
            return .onePair
        default:
            throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
        }
    case 0:
        switch sort_count[0] {
        case 5:
            return .five
        case 4:
            return .four
        case 3:
            switch sort_count[1] {
            case 2:
                return .full
            case 1:
                return .three
            default:
                throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
            }
        case 2:
            switch sort_count[1] {
            case 2:
                return .twoPair
            case 1:
                return .onePair
            default:
                throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
            }
        case 1:
            return .high
        default:
            throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
        }
        default:
            throw RTError.runtimeError("bad hand \(sort_count) \(jokers)")
    }
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

        var hands: [Hand] = []
        for line in lines {
            if line == "" { continue }

            let bid_parts = line.split(separator: " ")
            let cards = String(bid_parts[0])
            let bid = Int(bid_parts[1])!
            hands.append(Hand(
                           str_cards: cards,
                           cards: try! cards.map(char_to_card),
                           typ: try! determine_type(hand: cards),
                           bid: bid
                         ));
        }

                                                  /*
                                                   print(
                                                   "hand {} rank {} * {} t {:?}",
                                                   hand.str_cards,
                                                   i + 1,
                                                   hand.bid,
                                                   hand.typ
                                                   ); */
        hands.sort();
        let ret = hands.enumerated().reduce(0)  { (acc, tuple) in acc + (tuple.1.bid * (tuple.0 + 1)) }
        print("total", ret)
    }
}
