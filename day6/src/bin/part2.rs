fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 2: {:?}", solution(&input));
}

fn solution(input: &str) -> usize {
    let times: Vec<&str> = input
        .lines()
        .take(1)
        .flat_map(|x| {
            x.split(":")
                .skip(1)
                .take(1)
                .flat_map(|x| x.split_whitespace())
        })
        .collect();
    let time: String = times.iter().flat_map(|x| x.chars()).collect();
    let time_num: usize = time.parse().unwrap();
    let distances: Vec<&str> = input
        .lines()
        .skip(1)
        .take(1)
        .flat_map(|x| {
            x.split(":")
                .skip(1)
                .take(1)
                .flat_map(|x| x.split_whitespace())
        })
        .collect();
    let dis: String = distances.iter().flat_map(|x| x.chars()).collect();
    let dis_num: usize = dis.parse().unwrap();
    let mut anwers: Vec<usize> = vec![];

    for ms in 0..=time_num {
        let mut times: bool = false;
        let seconds_left = time_num - ms;
        let speed = ms;
        if speed * seconds_left > dis_num {
            times = true;
        }

        if times {
            anwers.push(1);
        }
    }

    anwers.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part2() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(71503, solution(input));
    }
}
