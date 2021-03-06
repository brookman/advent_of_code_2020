extern crate nom;

use std::collections::HashSet;

use phf::phf_map;

use crate::common;
use crate::vectors::Vec3;

use self::nom::branch::alt;
use self::nom::bytes::complete::tag;
use self::nom::IResult;
use self::nom::multi::many0;

static DIRECTIONS: phf::Map<&'static str, Vec3<i32>> = phf_map! {
    "e"  =>  Vec3 { x:  1, y: -1, z:  0},
    "se" =>  Vec3 { x:  0, y: -1, z:  1},
    "sw" =>  Vec3 { x: -1, y:  0, z:  1},
    "w"  =>  Vec3 { x: -1, y:  1, z:  0},
    "nw" =>  Vec3 { x:  0, y:  1, z: -1},
    "ne" =>  Vec3 { x:  1, y:  0, z: -1},
};

struct Floor {
    tiles: HashSet<Vec3<i32>>
}

impl Floor {
    fn parse(filename: &str) -> Self {
        let mut tiles = HashSet::new();
        for line in common::read_strings(filename) {
            let parsed: IResult<&str, Vec<&str>> = many0(alt((tag("e"), tag("se"), tag("sw"), tag("w"), tag("nw"), tag("ne"))))(line.as_str());
            if let Ok((_, r)) = parsed {
                let sum = r.into_iter()
                    .map(|s| DIRECTIONS[s].clone())
                    .fold(Vec3::new(0, 0, 0), |s, c| s + c);
                if tiles.contains(&sum) {
                    tiles.remove(&sum);
                } else {
                    tiles.insert(sum);
                }
            }
        }
        Floor { tiles }
    }

    fn black(&self, tile: &Vec3<i32>) -> bool {
        self.tiles.contains(&tile)
    }

    fn neighbors(&self, tile: &Vec3<i32>) -> Vec<Vec3<i32>> {
        DIRECTIONS.values().map(|dir| tile.clone() + dir.clone()).collect()
    }

    fn count_neighbors(&self, tile: &Vec3<i32>) -> usize {
        self.neighbors(&tile).into_iter().filter(|n| self.black(n)).count()
    }

    fn iterate(&mut self) {
        self.tiles = self.get_affected_tiles().into_iter()
            .filter(|tile| {
                let count = self.count_neighbors(&tile);
                self.black(&tile) && count == 1 || count == 2
            }).collect();
    }

    fn get_affected_tiles(&self) -> HashSet<Vec3<i32>> {
        let mut result = HashSet::new();
        result.extend(self.tiles.clone());
        for tile in &self.tiles {
            result.extend(self.neighbors(tile));
        }
        result
    }
}

pub fn part_one() {
    println!("--- Part One ---");
    let floor = Floor::parse("./data/dec_24.txt");
    println!("Result: {}", floor.tiles.len());
}

pub fn part_two() {
    println!("--- Part Two ---");
    let mut floor = Floor::parse("./data/dec_24.txt");
    for _ in 0..100 {
        floor.iterate();
    }
    println!("Result: {}", floor.tiles.len());
}
