use std::fs;

fn main(){
    let input = fs::read_to_string("input.txt").unwrap();
    let ans = solution(&input);
    println!("{:?}", ans);
}

fn solution(input: &str) ->  usize {
let mut ans_vec = vec![];
    for line in input.lines() {
        let chars: Vec<char> = line.chars().collect::<_>();
        let nums: Vec<usize> = chars.into_iter().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap() as usize).collect();

        let value: usize = nums.first().unwrap() *10 + nums.last().unwrap();
ans_vec.push(value);
    }
    ans_vec.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let ans = solution(input);
        assert_eq!(ans, 142);
    }
}
