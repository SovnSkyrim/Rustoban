//
// EPITECH PROJECT, 2023
// rustoban
// File description:
// parser
//

use std::fs::File;
use std::io::{BufRead, BufReader, Error};
use crate::game::rustoban::Position;
use crate::game::rustoban::Goals;
use crate::game::rustoban::Direction;

#[allow(dead_code)]
pub mod rustobanparser {
    use super::*;
    pub struct Parser {
        map_data: Vec<Vec<char>>,
        all_goals: Vec<Goals>,
        nb_boxes: i32,
        player: Position,
    }
    impl Parser {
        pub fn new(path: String) -> Result<Parser, Error> {
            println!("Game is created");
            println!("Map path: {}", path);
            let map_data = Self::parse_map_file(&path)?;
            let goals = Self::parse_goals(&map_data)?;
            let player = Self::find_player(&map_data)?;
            let all_boxes = Self::parse_boxes(&map_data)?;
            if !Self::check_validity(&map_data) || Self::check_player_stuck(player, &map_data) {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "Invalid map"));
            }

            Ok(Parser {
                map_data,
                all_goals: goals,
                player,
                nb_boxes: all_boxes,
            })
        }

        fn parse_map_file(filename: &str) -> Result<Vec<Vec<char>>, Error> {
            let file = File::open(filename)?;
            let reader = BufReader::new(file);
            let mut map_data: Vec<Vec<char>> = Vec::new();
            for line in reader.lines() {
                let row = line?.chars().collect();
                map_data.push(row);
            }
            Ok(map_data)
        }

        pub fn debug(&self) {
            println!("Map data: {:?}", self.map_data);
            println!("Nb boxes: {}", self.nb_boxes);
            println!("Player: {:?}, {:?}", self.player.row(), self.player.column());
            for goal in self.all_goals.iter() {
                println!("Goal: {:?}, {:?}", goal.position().row(), goal.position().column());
                }
        }

        fn check_player_stuck(position: Position, map_data: &Vec<Vec<char>>) -> bool{
            let neighbors = [
                [position.neighbor(Direction::Up), position.neighbor(Direction::Right)],
                [position.neighbor(Direction::Right), position.neighbor(Direction::Down)],
                [position.neighbor(Direction::Down), position.neighbor(Direction::Left)],
                [position.neighbor(Direction::Left), position.neighbor(Direction::Up)],
            ];
            let mut count = 0;
            for neighbor_pair in neighbors.iter() {
                let neighbor1 = &neighbor_pair[0];
                let neighbor2 = &neighbor_pair[1];
                let row1 = neighbor1.row();
                let col1 = neighbor1.column();
                let row2 = neighbor2.row();
                let col2 = neighbor2.column();
                if map_data[row1 as usize][col1 as usize] == '#' &&
                    map_data[row2 as usize][col2 as usize] == '#' {
                        count += 1;
                    }
            }
            if count == 4 {
                return true;
            }
            false
        }

        fn parse_goals(map_data: &Vec<Vec<char>>) -> Result<Vec<Goals>, Error> {
            let mut goals: Vec<Goals> = Vec::new();
            for (row, line) in map_data.iter().enumerate() {
                for (col, col_char) in line.iter().enumerate() {
                    if *col_char == 'O' {
                        goals.push(Goals::new(Position::new(row as i32, col as i32), false));
                    }
                }
            }
            if goals.is_empty() {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "No goals found"));
            }
            Ok(goals)
        }

        fn parse_boxes(map_data: &Vec<Vec<char>>) -> Result<i32, Error> {
            let mut goals: i32 = 0;
            for (_row, line) in map_data.iter().enumerate() {
                for (_col, col_char) in line.iter().enumerate() {
                    if *col_char == 'X' {
                        goals += 1;
                    }
                }
            }
            if goals == 0 {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "No goals found"));
            }
            Ok(goals)
        }

        fn find_player(map_data: &Vec<Vec<char>>) -> Result<Position, Error> {
            let mut player: Position = Position::new(0, 0);
            for (row, line) in map_data.iter().enumerate() {
                for (col, col_char) in line.iter().enumerate() {
                    if *col_char == 'P' {
                      player = Position::new(row as i32, col as i32);
                    }
                }
            }
            if player.row() == 0 && player.column() == 0 {
                return Err(Error::new(std::io::ErrorKind::InvalidData, "No player found"));
            }
            Ok(player)
        }

        fn check_validity(map_data: &Vec<Vec<char>>) -> bool {
            let mut player: i32 = 0;
            let mut goals: i32 = 0;
            let mut boxes: i32 = 0;

            for (_row, line) in map_data.iter().enumerate() {
                for (_col, col_char) in line.iter().enumerate() {
                    if *col_char == 'P' {
                        player += 1;
                    }
                    if *col_char == 'O' {
                        goals += 1;
                    }
                    if *col_char == 'X' {
                        boxes += 1;
                    }
                }
            }
            if player != 1 || goals != boxes || goals == 0 {
                return false;
            }
            true
        }

        pub fn map_data(&self) -> &Vec<Vec<char>> {
            &self.map_data
        }

        pub fn all_goals(&self) -> &Vec<Goals> {
            &self.all_goals
        }

        pub fn nb_boxes(&self) -> &i32 {
            &self.nb_boxes
        }

        pub fn player(&self) -> &Position {
            &self.player
        }
    }
}