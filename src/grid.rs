use std::io;
use std::io::Write;
use std::collections;
use crossterm;

use super::data;
use super::snake;



const NUMERIC : &'static str = "0123456789";



#[derive(Clone, Debug)]
pub struct Grid {
    slots     : Vec<Vec<Option<char>>>,
    snake     : snake::Snake,
    variables : collections::HashMap<char, usize>
}
impl Grid {

    pub fn from(script : String) -> Grid {
        let mut grid = Grid {
            slots     : Vec::new(),
            snake     : snake::Snake::default(),
            variables : collections::HashMap::new()
        };
        let mut snake_set : bool = false;
        for line in script.split('\n') {
            grid.slots.push(Vec::new());
            for ch in line.chars().collect::<Vec<char>>() {
                let len = grid.slots.len();
                let pos = (grid.slots[len - 1].len(), len - 1);
                if (ch == '$') {
                    if (snake_set) {
                        panic!("More than one start position defined.");
                    } else {
                        grid.snake = snake::Snake::new(pos.0, pos.1);
                        snake_set  = true;
                    };
                };
                grid.slots[len - 1].push(Some(ch));
            };
        };
        if (snake_set) {
            grid.snake.direction = grid.argument_direction(grid.snake.position.0 + 1, grid.snake.position.1);
        } else {
            panic!("No start position defined.");
        }
        return grid;
    }

    
    pub fn get(&mut self, x : usize, y : usize) -> Option<char> {
        if (self.slots.len() > y) {
            if (self.slots[y].len() > x) {
                return self.slots[y][x];
            };
        };
        return None;
    }


    pub fn check_snake_dead(&mut self) -> bool {
        if (matches!(self.get(self.snake.position.0, self.snake.position.1), None) || ! self.snake.alive) {
            return true;
        }
        for tail in &self.snake.tails {
            if (*tail == self.snake.position) {
                return true;
            }
        }
        return false;
    }


    pub fn update_snake(&mut self) -> () {
        self.snake.shift();
        let command = self.get_command(self.snake.position.0, self.snake.position.1);
        match (command) {
            data::Command::None                    => (),
            data::Command::IncreaseLength(amount)  => {
                for _i in 0..amount {
                    self.snake.add_tail();
                };
            },
            data::Command::DecreaseLength(amount) => {
                for _i in 0..amount {
                    self.snake.remove_tail();
                };
            },
            data::Command::SetLength(amount) => {
                self.snake.set_length(amount);
            },
            data::Command::SetDirection(direction) => {
                self.snake.set_direction(direction);
            },
            data::Command::Print(ch) => {
                print!("{}", ch);
                io::stdout().flush().unwrap();
            },
            data::Command::PrintLen => {
                print!("{}", self.snake.tails.len());
                io::stdout().flush().unwrap();
            }
            data::Command::WriteVariable(ch) => {
                self.variables.remove(&ch);
                self.variables.insert(ch, self.snake.tails.len());
            },
            data::Command::ReadVariable(ch) => {
                println!("{}", ch);
                if (self.variables.contains_key(&ch)) {
                    self.snake.set_length(self.variables[&ch]);
                } else {
                    self.snake.set_length(0);
                };
            },
            data::Command::NumberInput => {
                let mut stdout = io::stdout();
                let mut number = String::new();
                let mut sigint = false;
                crossterm::terminal::enable_raw_mode().unwrap();
                loop {
                    let ch = crossterm::event::read().unwrap());
                    println!("{}", ch);
                }
                /*for key in stdin.keys() {
                    match key.unwrap() {
                        Key::Ctrl('c')  => {
                            sigint = true;
                            break;
                        },
                        Key::Char('\n') => break,
                        Key::Char(ch)   => {
                            if (NUMERIC.contains(ch)) {
                                number += ch.to_string().as_str();
                            }
                        },
                        Key::Backspace => {
                            number = number[0..number.len() - 1].to_string();
                        },
                        _ => ()
                    };
                };*/
                crossterm::terminal::disable_raw_mode().unwrap();
                if (sigint) {
                    panic!("Sigint signal received.");
                }
                self.snake.set_length(number.parse::<usize>().unwrap());
            }
        }
    }
    

    pub fn get_command(&mut self, x : usize, y : usize) -> data::Command {
        let value = self.get(x, y);
        match (value) {
            Some(ch) => {
                match (ch) {
                    '+' => data::Command::IncreaseLength(self.argument_number(x + 1, y)),
                    '-' => data::Command::DecreaseLength(self.argument_number(x + 1, y)),
                    '=' => data::Command::SetLength(self.argument_number(x + 1, y)),
                    '^' => data::Command::SetDirection(data::Direction::Up),
                    '>' => data::Command::SetDirection(data::Direction::Right),
                    'v' => data::Command::SetDirection(data::Direction::Down),
                    '<' => data::Command::SetDirection(data::Direction::Left),
                    '#' => data::Command::Print(self.argument_character(x + 1, y, true)),
                    '*' => data::Command::PrintLen,
                    '@' => data::Command::WriteVariable(self.argument_character(x + 1, y, false)),
                    '%' => data::Command::ReadVariable(self.argument_character(x + 1, y, false)),
                    '?' => {
                        let name = self.argument_character(x + 1, y, false);
                        if (self.variables.contains_key(&name) && self.variables[&name] >= 1) {
                            return self.get_command(x + 2, y);
                        }
                        return data::Command::None;
                    },
                    '&' => data::Command::NumberInput,
                    _   => data::Command::None
                }
            },
            None => data::Command::None
        }
    }

    
    fn argument_direction(&mut self, x : usize, y : usize) -> data::Direction {
        let value = self.get(x, y);
        match (value) {
            Some(ch) => {
                match (ch) {
                    '^' => data::Direction::Up,
                    '>' => data::Direction::Right,
                    'v' => data::Direction::Down,
                    '<' => data::Direction::Left,
                    _   => panic!("Invalid direction, {}.", ch)
                }
            },
            None => panic!("No direction argument found.")
        }
    }

    
    fn argument_number(&mut self, x : usize, y : usize) -> usize {
        let start_value = self.get(x, y);
        match (start_value) {
            Some(start_ch) => {
                if (NUMERIC.contains(start_ch)) {
                    let mut num = String::from(start_ch);
                    let mut i   = 0;
                    loop {
                        i += 1;
                        let value = self.get(x + i, y);
                        match (value) {
                            Some(ch) => {
                                if (NUMERIC.contains(ch)) {
                                    num += ch.to_string().as_str();
                                    continue;
                                }
                            },
                            _ => ()
                        }
                        break;
                    }
                    return num.parse().unwrap();
                } else {
                    panic!("Invalid number, {}.", start_ch)
                }
            },
            None => panic!("No number argument found.")
        }
    }

    
    fn argument_character(&mut self, x : usize, y : usize, allow_escape : bool) -> char {
        let value = self.get(x, y);
        match (value) {
            Some(mut ch) => {
                if (ch == '\\' && allow_escape) {
                    let escape_value = self.get(x + 1, y);
                    match (escape_value) {
                        Some(escape_ch) => {
                            ch = match (escape_ch) {
                                'n'  => '\n',
                                't'  => '\t',
                                '\\' => '\\',
                                _    => ch
                            };
                        },
                        _ => ()
                    }
                }
                return ch
            },
            None     => panic!("No number argument found.")
        }
    }
    
}
