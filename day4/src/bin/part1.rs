fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
//     let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
// Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
// Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
// Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
// Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
// Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    println!("Part 1: {:?}", solution(&input));
}

fn solution(input: &str) -> u32 {
    let mut points: Vec<u32> = vec![];

    for line in input.lines() {
        let game: Vec<&str> = line
            .split(": ")
            .skip(1)
            .take(1)
            .flat_map(|x| x.split("|"))
            .collect();
        println!("Games: {:?}", game);
        let answers: Vec<u32> = game[0]
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let plays: Vec<u32> = game[1]
            .trim()
            .split_whitespace()
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let mut points_earned: u32 = 0;
        for play in plays.iter() {
            if answers.contains(play) {
                if points_earned == 0 {
                    points_earned += 1;
                } else {
                    points_earned = points_earned * 2;
                }
            }
        }
        points.push(points_earned);
    }
    points.into_iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_() {
        let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

        assert_eq!(13, solution(input));
    }
}
