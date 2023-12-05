fn main() {
    // let input = std::fs::read_to_string("input.txt").unwrap();
    let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
    println!("Part 1: {}", solution(&input));
}

fn solution(input: &str) -> u32 {
    //break into vec<vec>
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    println!("{:?}", grid);
    println!("{:?}", 4 as u8);
    println!("{:?}", "4".as_bytes());
    // let mut part_grid: Vec<&[u8]> = grid.clone();
    let mut part_numbers: Vec<u32> = vec![];
    for (ri, row) in grid.iter().enumerate() {
        for (i, byte) in row.into_iter().enumerate() {
            if byte.is_ascii_digit() {
                //check to see if it touches a symbol
                if check_for_symbol(ri, i, &grid) {
                    println!("i is: {}", i);
                    let mut x = i;
                    let mut left_bound = i;
                    let mut right_bound = i;
                    //check left-boudn
                    while is_digit(&grid[ri][x]) && x > 0 {
                        x -= 1;
                    }
                    left_bound = x;

                    // check right bound
                    x = i;
                    while is_digit(&grid[ri][x]) && x < grid[ri].len() {
                        x += 1;
                    }
                    right_bound = x;

                    println!(
                        "Left: {} Right:{} btye: {}",
                        left_bound,
                        right_bound,
                        String::from_utf8(vec![*byte]).unwrap()
                    );
                    let test: Vec<u8> = grid[ri]
                        .into_iter()
                        .skip(left_bound + 1)
                        .take(right_bound - left_bound - 1)
                        .inspect(|c| println!("{:?}", c))
                        .map(|c| *c)
                        .collect();
                    println!("This is the take value: {:?}", test);
                    println!("String is: {:?}", String::from_utf8(test.clone()).unwrap());
                    let num: u32 = String::from_utf8(test).unwrap().parse().unwrap();
                    part_numbers.push(num);
                }
            }
        }
    }
    part_numbers.into_iter().sum()
}

fn check_for_symbol(row: usize, col: usize, grid: &Vec<&[u8]>) -> bool {
    println!("This is where I am checking {}, {}", row, col);
    let moves: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    for dir in moves.into_iter() {
        let x: isize = row as isize - dir.0;
        let y = col as isize - dir.1;
        if x >= 0 && y >= 0 {
            if (y as usize) < grid[row as usize].len() && (x as usize) < grid.len() {
                if is_symbol(&grid[x as usize][y as usize]) {
                    println!("return true with move {:?}", dir);
                    return true;
                }
            }
            // println!("x: {} Y: {}", x, y);
        }
    }

    println!("return false");
    return false;
}

fn is_symbol(input: &u8) -> bool {
    println!(
        "This is the byte {}",
        String::from_utf8(vec![*input]).unwrap()
    );
    match input {
        b'0'..=b'9' | b'.' => false,
        _ => true,
    }
}

fn is_digit(input: &u8) -> bool {
    match input {
        b'0'..=b'9' => true,
        _ => false,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(4361, solution(input));
    }
}
