//
// EPITECH PROJECT, 2023
// rustoban
// File description:
// game
//

extern crate ncurses;
use ncurses::*;

const KEY_Q: i32 = 113;
const KEY_R: i32 = 114;

#[allow(dead_code)]
pub mod rustoban {
    use super::*;
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    #[derive(Clone, Copy)]
    pub struct Position(i32, i32);
    impl Position {
        pub fn new(row: i32, col: i32) -> Position {
            Position(row, col)
        }

        pub fn neighbor(&self, dir: Direction) -> Position {
            match dir {
                Direction::Up => Position(self.0 - 1, self.1),
                Direction::Down => Position(self.0 + 1, self.1),
                Direction::Left => Position(self.0, self.1 - 1),
                Direction::Right => Position(self.0, self.1 + 1),
            }
        }

        pub fn row(&self) -> i32 {
            self.0
        }

        pub fn column(&self) -> i32 {
            self.1
        }

        pub fn clone(&self) -> Self {
            Position(self.0, self.1)
        }
    }

    // #[derive(Clone, Copy)]
    pub struct Goals(Position, bool);
    impl Goals {
        pub fn new(pos: Position, is_set: bool) -> Goals {
            Goals(pos, is_set)
        }

        pub fn position(&self) -> &Position {
            &self.0
        }

        // fn clone(&self) -> Self {
        //     Goals(self.0.clone(), self.1)
        // }
    }

    impl Clone for Goals {
        fn clone(&self) -> Goals {
            Goals(self.0.clone(), self.1)
        }
    }
    pub struct Game {
        map_data: Vec<Vec<char>>,
        all_goals: Vec<Goals>,
        nb_boxes: i32,
        nb_stuck_boxes: i32,
        player: Position,
        map_backup: Vec<Vec<char>>,
        player_backup: Position,
    }

    impl Game {
        pub fn new(map_data: &Vec<Vec<char>>,
                    all_goals: &Vec<Goals>,
                    nb_boxes: &i32,
                    player: &Position) -> Game {
        println!("Game is created");
        Game {
            map_data : map_data.clone(),
            all_goals: all_goals.clone(),
            player: player.clone(),
            map_backup: map_data.clone(),
            nb_boxes: nb_boxes.clone(),
            nb_stuck_boxes: 0,
            player_backup: player.clone(),
        }
    }

        pub fn debug(&self) {
            println!("Map data: {:?}", self.map_data);
            println!("Map backup: {:?}", self.map_backup);
            println!("Nb boxes: {}", self.nb_boxes);
            println!("Nb stuck boxes: {}", self.nb_stuck_boxes);
            println!("Player: {:?}, {:?}", self.player.0, self.player.1);
            for goal in self.all_goals.iter() {
                println!("Goal: {:?}, {:?}", goal.0.0, goal.0.1);
            }
        }

        fn init_ncurse() {
            initscr();
            raw();
            keypad(stdscr(), true);
            noecho();
            curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        }

        fn draw_map(&self) {
            clear();
            for (row, line) in self.map_data.iter().enumerate() {
                for (col, col_char) in line.iter().enumerate() {
                    mvaddch(row as i32, col as i32, *col_char as u32);
                }
            }
            refresh();
        }

        fn end_ncurse(&self) {
            Self::draw_map(self);
            endwin();
        }

        fn check_stuck_box(&mut self, position: Position) {
            let neighbors = [
                [position.neighbor(Direction::Up), position.neighbor(Direction::Right)],
                [position.neighbor(Direction::Right), position.neighbor(Direction::Down)],
                [position.neighbor(Direction::Down), position.neighbor(Direction::Left)],
                [position.neighbor(Direction::Left), position.neighbor(Direction::Up)],
            ];
            let mut stuck = false;

            for neighbor_pair in neighbors.iter() {
                let neighbor1 = &neighbor_pair[0];
                let neighbor2 = &neighbor_pair[1];
                let row1 = neighbor1.row();
                let col1 = neighbor1.column();
                let row2 = neighbor2.row();
                let col2 = neighbor2.column();
                if self.map_data[row1 as usize][col1 as usize] == '#' &&
                    self.map_data[row2 as usize][col2 as usize] == '#' {
                    stuck = true;
                    break;
                }
            }
            if stuck {
                self.nb_stuck_boxes += 1;
            }
        }

        fn movement(&mut self, position: &Position) {
            let row = position.row();
            let col = position.column();
            let player_row = self.player.row();
            let player_col = self.player.column();

            match self.map_data[row as usize][col as usize] {
                '#' => return,
                'X' => {
                    let new_box_row = row + (row - player_row);
                    let new_box_col = col + (col - player_col);
                    if self.map_data[new_box_row as usize][new_box_col as usize] == '#' {
                        return;
                    }
                    self.map_data[new_box_row as usize][new_box_col as usize] = 'X';
                    self.check_stuck_box(Position::new(new_box_row, new_box_col));
                }
                _ => {}
            }
            self.map_data[row as usize][col as usize] = 'P';
            self.map_data[player_row as usize][player_col as usize] =
                match self.map_backup[player_row as usize][player_col as usize] {
                    'O' => 'O',
                    _ => ' ',
                };
            self.player = Position::new(row, col);
        }

        fn check_win(&self) -> bool {
            for goal in &self.all_goals {
                if self.map_data[goal.0.row() as usize][goal.0.column() as usize] != 'X' {
                    return false;
                }
            }
                true
            }

        pub fn run(&mut self) -> i32 {
            println!("Game is running");
            Self::init_ncurse();
            loop {
                Self::draw_map(self);
                let ch = getch();
                match ch {
                    KEY_UP => {
                        Self::movement(self, &self.player.neighbor(Direction::Up));
                    },
                    KEY_DOWN => {
                        Self::movement(self, &self.player.neighbor(Direction::Down));
                    },
                    KEY_LEFT => {
                        Self::movement(self, &self.player.neighbor(Direction::Left));
                    },
                    KEY_RIGHT => {
                        Self::movement(self, &self.player.neighbor(Direction::Right));
                    },
                    KEY_R => {
                        self.map_data = self.map_backup.clone();
                        self.nb_stuck_boxes = 0;
                        self.player = self.player_backup.clone();
                    },
                    KEY_Q | KEY_EXIT | KEY_SEXIT => {
                        break;
                    },
                    _ => {
                        println!("Le bosquet a pris feu.");
                    }
                }
                if Self::check_win(self) {
                    break;
                }
                else if self.nb_stuck_boxes == self.nb_boxes {
                    Self::end_ncurse(self);
                    return 84;
                }
            }
            Self::end_ncurse(self);
            return 0;
        }
    }
}
