fn main() {
    // let input = std::fs::read_to_string("input.txt").unwrap();
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    println!("part 1: {:?}", part1_solution(input));
}

fn part1_solution(input: &str) -> u32 {
    //total games = 100
    // 12 red
    // 13 green
    // 14 blue
    let mut unplayable_games: u32 = 0;
    for line in input.lines() {
        let game_split: Vec<&str> = line.split(": ").collect();
        let games: Vec<&str> = game_split[1].split("; ").collect();
        println!("{:?}", games);

        for game in games.into_iter() {
            let ind: Vec<&str> = game.split(", ").collect();

            ind.into_iter().map(|c| c.as_bytes()).map(|c| match (c[0], c[2]) {
                
            })
        }

    }
8
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
        assert_eq!(8, part1_solution(input));
    }

}
