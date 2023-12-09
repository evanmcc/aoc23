import ArgumentParser
import Foundation

enum RTError : Error {
    case runtimeError(String)
}

struct Node {
    var left: String
    var right: String
}

func lcm<C: Collection>(nums: C) -> Int
  where C.Element == Int, C.Index == Int {
    if nums.count == 1 {
        return nums.first! // not sure why this is required, some generics thing?
    } else {
        let l = nums.first! // also this
        let r = lcm(nums: nums.dropFirst()) // also this
        return l * r / gcd(l: l, r: r)
    }
}

func gcd(l: Int, r: Int) -> Int {
    if r == 0 {
        return l;
    }
    return gcd(l: r, r: l % r)
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

          var instruction_list = ""
          var nodes: [String: Node] = [:]
          var starts: [String] = []

          for line in lines {
              if instruction_list == "" {
                  instruction_list = line;
              } else if line == "" {
                  continue;
              } else {
                  let start = line.startIndex
                  let name_end = line.index(start, offsetBy: 3)
                  let name = line[start..<name_end]

                  let left_start = line.index(start, offsetBy: 7)
                  let left_end = line.index(start, offsetBy: 10)
                  let left = line[left_start..<left_end]

                  let right_start = line.index(start, offsetBy: 12)
                  let right_end = line.index(start, offsetBy: 15)
                  let right = line[right_start..<right_end]
                  //print(name, right, left)

                  if name.suffix(1) == "A" {
                      starts.append(String(name));
                  }
                  nodes[String(name)] = Node(
                    left: String(left),
                    right: String(right)
                  )
              }
          }

          print("starts", starts);
          var currents: [Node] = [];
          for s in starts {
              currents.append(nodes[s]!)
          }

          var steps: [Int] = [];
          for var cur in currents {
              var step_count = 0;
              outer: while true {
                  for inst in instruction_list {
                      var next: String? = nil
                      switch inst {
                      case "R":
                          next = cur.right
                      case "L":
                          next = cur.left
                      default:
                          throw RTError.runtimeError("bad instruction")
                      };
                      cur = nodes[next!]!
                      step_count += 1;
                      if next!.suffix(1) == "Z" {
                          steps.append(step_count);
                          break outer;
                      }
                  }
              }
          }
          let res = lcm(nums: steps);
          print("steps", res);
    }
}
