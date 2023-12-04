fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
//         let input = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
// Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
// Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
// Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
// Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    println!("part 1: {:?}", part1_solution(input.as_str()));
}

fn part1_solution(input: &str) -> u32 {
    //total games = 100
    // 12 red
    // 13 green
    // 14 blue
    let mut games_list: Vec<u32> = (1..=input.lines().count() as u32).collect();
    for (index, line) in input.lines().enumerate() {
        let game_split: Vec<&str> = line.split(": ").collect();
        let games: Vec<&str> = game_split[1].split("; ").collect();
        println!("{:?}", games);
        for game in games.into_iter() {
            let ind: Vec<&str> = game.split(", ").collect();

            let _ = ind.into_iter().map(|c| c.split(" ").collect::<Vec<&str>>()).inspect(|x| println!("each one being passed to for: {:?}", x)).for_each(|v| match (v[0].parse::<u32>().unwrap(), v[1]){
                (x, "blue") => {
                    if x > 14 {
                        println!("blue x is: {}", x);
                        games_list[index] = 0;
                    } 
                }
                (x, "red") => {
                    println!("this is the red x: {:?}", x);
                    if x > 12 {
                        games_list[index] = 0;
                    }
                },
                (x, "green") => {
                    if x > 13 {
                        games_list[index] = 0;
                    }
                },
                (x, y) => {
                    println!("x: {} y: {}", x, y);
                }

            });
            println!("Gameslist: {:?}", games_list);
        }

    }
    println!("{:?}", games_list);
    games_list.into_iter().sum()
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
