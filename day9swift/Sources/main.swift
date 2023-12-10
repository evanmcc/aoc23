import ArgumentParser
import Foundation

enum RTError : Error {
    case runtimeError(String)
}

func predict(nums: [Int]) -> Int {
    var diffs: [Int] = [];
    for i in 0..<(nums.count - 1) {
        let diff = nums[i + 1] - nums[i]
        diffs.append(diff)
    }
    // last to first, add to sub
    if diffs.allSatisfy({x in x == 0}) {
        return nums[0]
    } else {
        let pred = predict(nums: diffs)
        //println!("nums {:?} diffs {:?}, pred {}", nums, diffs, pred);
        return nums[0] - pred
    }
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

        var total = 0
        for line in lines {
            if line == "" {
                continue;
            } else {
                let nums = line.split(separator: " ")
                  .map { x in Int(x)! }
                let prediction = predict(nums: nums);
                //println!("pred {}", prediction);
                total += prediction;
            }
        }
        print("prediction", total);
    }
}
