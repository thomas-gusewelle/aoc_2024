fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part1: {:?}", solution(&input));
}

fn solution(input: &str) -> usize {
    let times: Vec<usize> = input.lines().take(1).flat_map(|x| x.split(":").skip(1).take(1).flat_map(|x| x.split_whitespace().map(|x| x.parse().unwrap()))).collect();
    let distances: Vec<usize> = input.lines().skip(1).take(1).flat_map(|x| x.split(":").skip(1).take(1).flat_map(|x| x.split_whitespace().map(|x| x.parse().unwrap()))).collect();
    let mut anwers: Vec<usize> = vec![];

    for (index, time) in times.into_iter().enumerate() {
        let mut times = 0;
        for ms in 0..=time {

            let seconds_left = time - ms; 
            let speed = ms;
            if speed * seconds_left > distances[index] {
                times += 1;
            }
        }
        if times > 0 {
            anwers.push(times);
        }
    }


    anwers.iter().product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"Time:      7  15   30
Distance:  9  40  200"#;

        assert_eq!(288, solution(input));
    }
}
