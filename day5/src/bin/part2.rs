use indicatif::{ParallelProgressIterator, ProgressIterator};
// use rayon::iter::IntoParallelRefMutIterator;
use rayon::prelude::*;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    //     let input = r#"seeds: 79 14 55 13
    //
    // seed-to-soil map:
    // 50 98 2
    // 52 50 48
    //
    // soil-to-fertilizer map:
    // 0 15 37
    // 37 52 2
    // 39 0 15
    //
    // fertilizer-to-water map:
    // 49 53 8
    // 0 11 42
    // 42 0 7
    // 57 7 4
    //
    // water-to-light map:
    // 88 18 7
    // 18 25 70
    //
    // light-to-temperature map:
    // 45 77 23
    // 81 45 19
    // 68 64 13
    //
    // temperature-to-humidity map:
    // 0 69 1
    // 1 0 69
    //
    // humidity-to-location map:
    // 60 56 37
    // 56 93 4"#;
    println!("Part 1: {:?}", solution(&input));
}

#[derive(Debug)]
struct Seed {
    start: usize,
    range: usize,
}

impl Seed {
    fn new(input: &[usize]) -> Seed {
        Seed {
            start: input[0],
            range: input[1],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Map {
    destination: usize,
    source: usize,
    range: usize,
}

impl Map {
    fn new(input: &str) -> Map {
        let nums: Vec<usize> = input
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        Map {
            destination: nums[0],
            source: nums[1],
            range: nums[2],
        }
    }

    fn find_location(&self, seed: usize) -> Option<usize> {
        if seed >= self.source && seed <= self.source + self.range - 1 {
            let length_between = seed - self.source;

            return Some(self.destination + length_between);
        }
        return None;
    }
}


#[derive(Debug, Clone )]
struct SeedMaps {
    maps: Vec<Map>
}

impl SeedMaps {
    fn new(maps: Vec<Map>) -> SeedMaps {
SeedMaps { maps }
    }
    fn translate(&self, seed: usize) -> usize {
        let conversions: Vec<Option<usize>> = self.maps.iter().map(|x| x.find_location(seed.clone())).collect();
        if let Some(matched) = conversions.into_iter().find(|x| match x {
            Some(_) => return true,
            None => return false
        }) {
            return matched.unwrap();
        }
        return seed;

    }
}
fn solution(input: &str) -> usize {
    let maps: Vec<&str> = input
        .split("\n\n")
        .flat_map(|x| x.split(":").skip(1).map(|x| x.trim_start()))
        .collect();

    let seeds_vec: Vec<usize> = maps[0]
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let mut locations: Vec<usize> = seeds_vec
        .windows(2)
        .step_by(2)
        .progress()
        .flat_map(|x| x[0]..(x[0] + x[1]))
        .collect();

    let mut all_maps: Vec<SeedMaps> = vec![];
    let ss_map: Vec<Map> = maps[1].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(ss_map));
    let sf_map: Vec<Map> = maps[2].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(sf_map));
    let fw_map: Vec<Map> = maps[3].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(fw_map));
    let wl_map: Vec<Map> = maps[4].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(wl_map));
    let lt_map: Vec<Map> = maps[5].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(lt_map));
    let th_map: Vec<Map> = maps[6].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(th_map));
    let hl_map: Vec<Map> = maps[7].lines().map(|x| Map::new(x)).collect();
    all_maps.push(SeedMaps::new(hl_map));


    let locations: Vec<usize> = locations.par_iter().progress().map(|seed| {
        all_maps
            .iter()
            .fold(*seed, |seed, maps| {
                maps.translate(seed)
            })
    }).collect();

    // for map in all_maps.into_iter().progress() {
    //     locations.par_iter_mut().progress().for_each(|seed| {
    //         let conversions: Vec<Option<usize>> = map
    //             .into_iter()
    //             .map(|x| x.find_location(seed.clone()))
    //             .collect();
    //         if let Some(matched) = conversions.into_iter().find(|x| match x {
    //             Some(_) => return true,
    //             _ => return false,
    //         }) {
    //             *seed = matched.unwrap();
    //         }
    //     })
    // }

    *locations.par_iter().progress().min().unwrap()
}

#[cfg(test)]
mod test {
    use crate::solution;

    use super::*;

    #[test]
    fn part1() {
        let input = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

        assert_eq!(46, solution(input));
    }
}
