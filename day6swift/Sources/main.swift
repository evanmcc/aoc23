let time = [59688274]
let distance = [543102016641022]

var margin = 1
for i in 0..<time.count {
    print("{}", i)
    let target = distance[i]
    let dur = time[i]
    var pos_wins = 0
    for n in 0..<dur {
        if n * (dur - n) > target {
            pos_wins += 1
        }
    }
    margin *= pos_wins
}

print("wins {}", margin)
