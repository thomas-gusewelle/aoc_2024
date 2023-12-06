fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part 1: {}", solution(&input));
}

fn solution(input: &str) -> u32 {
    let grid: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();

    let mut part_numbers: Vec<u32> = vec![];
    for (ri, row) in grid.iter().enumerate() {
        let mut row_iter = row.into_iter().enumerate();

        while let Some((i, byte)) = row_iter.next() {
            if is_gear(byte) {
               let gear_ration = check_for_part_numbers(ri, i, &grid); 
                match gear_ration {
                    Some(x) => part_numbers.push(x),
                    None => ()
                    
                }
            }
            // if is_digit(byte) {
            //     //check to see if it touches a symbol
            //     if check_for_symbol(ri, i, &grid) {
            //         // println!("i is: {}", i);
            //         let mut x = i;
            //         let left_bound;
            //         let right_bound;
            //         //check left-boudn
            //         while x > 0 && is_digit(&grid[ri][x - 1]) {
            //             x -= 1;
            //         }
            //         left_bound = x;
            //
            //         // check right bound
            //         x = i;
            //         while x < grid[ri].len() && is_digit(&grid[ri][x]) {
            //             x += 1;
            //         }
            //         right_bound = x;
            //
            //         let test: Vec<u8> = grid[ri]
            //             .into_iter()
            //             .skip(left_bound)
            //             .take(right_bound - left_bound)
            //             .map(|c| *c)
            //             .collect();
            //         let num: u32 = String::from_utf8(test).unwrap().parse().unwrap();
            //         part_numbers.push(num);
            //
            //         for _ in i..right_bound {
            //             row_iter.next();
            //         }
            //     }
            // }
        }
    }
    println!("{:?}", part_numbers);
    part_numbers.into_iter().sum()
}

fn check_for_part_numbers(row: usize, col: usize, grid: &Vec<&[u8]>) -> Option<u32> {
    let mut part_numbers: Vec<u32> = vec![];
    let moves: [(isize, isize); 8] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
    ];
    for dir in moves.into_iter() {
        let x: isize = row as isize + dir.0;
        let mut y = col as isize + dir.1;
        if x >= 0 && y >= 0 {
            if (y as usize) < grid[row as usize].len() && (x as usize) < grid.len() {
                if is_digit(&grid[x as usize][y as usize]) {
                    let left_bound: usize;
                    let right_bound: usize;
                    //check left-boudn
                    while y > 0 && is_digit(&grid[x as usize][(y as usize) - 1]) {
                        y -= 1;
                    }
                    left_bound = y as usize;

                    // check right bound
                    y = col as isize + dir.1;
                    while (y as usize) < grid[x as usize].len()
                        && is_digit(&grid[x as usize][y as usize])
                    {
                        y += 1;
                    }
                    right_bound = y as usize;

                    let test: Vec<u8> = grid[x as usize]
                        .into_iter()
                        .skip(left_bound)
                        .take(right_bound - left_bound)
                        .map(|c| *c)
                        .collect();
                    let num: u32 = String::from_utf8(test).unwrap().parse().unwrap();
                    part_numbers.push(num);
                }
            }
        }
    }
    println!("{:?}", part_numbers);
    part_numbers.sort();
    part_numbers.dedup();
    match part_numbers.len() {
        0 | 1 => None,
        2 => Some(part_numbers.iter().product()),
        _ => None
    }
}

fn is_symbol(input: &u8) -> bool {
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

fn is_gear(input: &u8) -> bool {
    match input {
        b'*' => true,
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
        assert_eq!(467835, solution(input));
    }

  
}
