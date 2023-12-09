import ArgumentParser
import Foundation

enum RTError : Error {
    case runtimeError(String)
}

enum Mode {
    case seed
    case seed2Soil
    case soil2Fert
    case fert2Water
    case water2Light
    case light2Temp
    case temp2Hum
    case hum2Loc
}

struct Mapping {
    var left: Int
    var right: Int
    var len: Int
}

@main
struct Puzzle: ParsableCommand {
    @Argument(help: "the puzzle input file")
    var file: String

    mutating func run() async throws {
        let input = try! String(contentsOfFile: file)
        let lines = input.components(separatedBy: .newlines)

        var seeds: [(Int, Int)] = []
        var mappings: [Mode: [Mapping]] = [:]
        var current: Mode = .seed
        for line in lines {
            //print(line)
            if line == "" {
                continue
            }  else if line.starts(with: "seeds:") {
                var seedlist: [String] = []
                if #available(macOS 13, *) {
                    seedlist = Array(line.split(separator: ": ")[1].split(separator: " ").map {x in String(x)})
                } else {
                    throw RTError.runtimeError("oh noes")
                }

                let seedlist1 = seedlist.map {x in Int(x)!}
                for i in 0 ..< (seedlist1.count - 1) {
                    if i % 2 != 0 { continue }
                    seeds.append((seedlist1[i], seedlist1[i+1]))
                }
            } else if line.starts(with: "seed-to-soil")  {
                current = .seed2Soil;
            } else if line.starts(with: "soil-to-fertilizer") {
                current = .soil2Fert;
            } else if line.starts(with: "fertilizer-to-water") {
                current = .fert2Water;
            } else if line.starts(with: "water-to-light") {
                current = .water2Light;
            } else if line.starts(with: "light-to-temperature") {
                current = .light2Temp;
            } else if line.starts(with: "temperature-to-humidity") {
                current = .temp2Hum;
            } else if line.starts(with: "humidity-to-location") {
                current = .hum2Loc;
            } else {
                // all other lines should be map numbers.
                let map = line.split(separator: " ").map {x in Int(x)!}

                // ideally I would rename these src and dest at the appropriate places
                let left = map[1]
                let right = map[0]
                let len = map[2]

                if var map_vec = mappings[current] {
                    //since we don't insert ordered here, we need to sort later
                    map_vec.append(Mapping( left: left, right: right, len: len ))
                    mappings[current] = map_vec
                } else {
                    mappings[current] = [Mapping( left: left, right: right, len: len )]
                }
            }
        }
        //sort here
        for (key, var map_vec) in mappings {
            map_vec.sort(by: { $0.left < $1.left })
            mappings[key] = map_vec
        }

        //ok, do the work
        let modes: [Mode] = [
          .seed2Soil,
          .soil2Fert,
          .fert2Water,
          .water2Light,
          .light2Temp,
          .temp2Hum,
          .hum2Loc,
        ]
        let mode_map: [(Mode, [Mapping])] = modes.map {x in (x, mappings[x]!) }

        print("seeds", seeds);
        @Sendable
        func search_seed(start: Int, end: Int) async -> Int {
            var low_loc = Int.max

            print(start);
            for seed in start..<(start + end) {
                var input = seed;
                //println!("seed {}", seed);
                for (mode, mv) in mode_map {
                    for mapping in mv {
                        let span = (mapping.left + mapping.len) - 1;
                        if input > span {
                            // too low
                            //println!("cont");
                            continue;
                        } else if input >= mapping.left && input <= span {
                            // in range
                            //let prior = input;
                            input = mapping.right + (input - mapping.left);
                            //println!("found {:?} {} {} mapping {:?}", mode, prior, input, mapping);
                            break;
                        } else {
                            // missing, leave input as is
                            //println!("miss");
                            break;
                        }
                    }
                    if mode == .hum2Loc {
                        //println!("loc {}", input);
                        if input <= low_loc {
                            low_loc = input;
                        }
                    }
                }
            }
            return low_loc
        }
        if #available(macOS 11, *) {

            let locs: [Int] = await withTaskGroup(of: Int.self)
            { taskGroup in
                for (start, end) in seeds {
                    taskGroup.addTask { await search_seed(start: start, end: end) }
                }
                var l: [Int] = []
                for await result in taskGroup {
                    l.append(result)
                }
                return l
            }

            let lowest_loc: Int = locs.reduce(Int.max, min)
            print("lowest loc", lowest_loc)
        } else {
            print("why so low?")
        }
    }
}
