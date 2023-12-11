fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    //     let input = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    // Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    // Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    // Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    // Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;
    println!("Part 2: {:?}", solution(&input));
}

fn solution(input: &str) -> u32 {
    // let mut points = 1;
    let mut cards: Vec<_> = vec![1; input.lines().count()];

    for (index, line) in input.lines().enumerate() {
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

        let num_of_this_card = cards[index];

        let mut answers_there: u32 = 0;
        for _ in 0..num_of_this_card {
            for play in plays.iter() {
                if answers.contains(play) {
                    answers_there += 1;
                }
            }
        }
        let number_of_wins: usize = (answers_there / num_of_this_card).try_into().unwrap();

        for j in 1..=number_of_wins {
            let temp = cards[index + j];
            cards[index + j] = temp + num_of_this_card;
        }
        println!("answers there: {}", answers_there);
        // points += answers_there;
    }
    // points
    println!("This is the cards: {:?}", cards);
    cards.into_iter().sum()

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

        assert_eq!(30, solution(input));
    }
}
