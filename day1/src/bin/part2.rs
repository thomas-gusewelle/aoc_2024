fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
//        let input = r#"two1nine
// eightwothree
// abcone2threexyz
// xtwone3four
// 4nineeightseven2
// zoneight234
// 7pqrstsixteen"#; 
    let sol = solution(input.as_str());
    println!("{:?}", sol);
}


fn solution(input: &str) -> usize {
    let mut ans_vec: Vec<usize> = vec![];
    for line in input.lines() {
    // let test: Vec<&str> = line.split_inclusive(char::is_numeric).collect();
    //     println!("{:?}", test);
    let test = line.to_string().replace("one", "one1one").replace("two", "two2two").replace("three", "three3three").replace("four", "four4four").replace("five", "five5five").replace("six", "six6six").replace("seven", "seven7seven").replace("eight", "eight8eight").replace("nine", "nine9nine");
    let chars: Vec<char> = test.chars().collect();

let num: Vec<usize> = chars.into_iter().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap() as usize).collect();

        ans_vec.push(num.first().unwrap() * 10 + num.last().unwrap());


 

}
    ans_vec.into_iter().sum()
}


#[cfg(test)]
mod tests {

    #[test]
    fn part2() {
       let input = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#; 

        let solution = super::solution(input);
        assert_eq!(281 as usize, solution);
    }
}
