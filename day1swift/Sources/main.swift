import ArgumentParser
import Foundation

func first_last(line: String) -> String {
    var first = Character("!")
    var last = Character("!")

    let replacements = [
      ("one", "o1e"),
      ("two", "t2o"),
      ("three", "t3e"),
      ("four", "f4r"),
      ("five", "f5e"),
      ("six", "s6x"),
      ("seven", "s7n"),
      ("eight", "e8t"),
      ("nine", "n9e"),
    ]
    var line1 = line
    for (from, to) in replacements {
        line1 = line1.replacingOccurrences(of: from, with: to)
    }

    for ch in line1 {
        if ch.isNumber {
            first = ch
            break
        }
    }
    let rev_line = line1.reversed()

    for ch in rev_line {
        if ch.isNumber {
            last = ch
            break
        }
    }

    return String([first, last])
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        guard let input = try? String(contentsOfFile: file) else {
            throw fatalError("Couldn't read from '\(file)'!")
        }
        let lines = input.components(separatedBy: .newlines)

        var total = 0
        for line in lines {
            if line != "" {
                let fl = first_last(line: line)
                print(line)
                let fl_int = Int(fl)!
                print("line \(line) fl \(fl) int \(fl_int)")
                total += fl_int
            }
        }

        print("sum total: ", total)
    }
}
