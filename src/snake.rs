use super::data;



#[derive(Clone, Debug)]
pub struct Snake {
    pub position  : (usize, usize),
    pub direction : data::Direction,
    pub tails     : Vec<(usize, usize)>,
    pub needs_add : usize,
    pub alive     : bool
}
impl Snake {

    pub fn default() -> Snake {
        return Snake::new(0, 0);
    }
    
    pub fn new(x : usize, y : usize) -> Snake {
        return Snake {
            position  : (x, y),
            direction : data::Direction::Up,
            tails     : Vec::new(),
            needs_add : 0,
            alive     : true
        }
    }

    pub fn set_direction(&mut self, direction : data::Direction) -> () {
        self.direction = direction;
    }

    pub fn shift(&mut self) -> () {
        for _i in 0..self.needs_add {
            self.tails.push(self.position);
        }
        self.needs_add = 0;
        if (self.tails.len() >= 1) {
            for i in self.tails.len()..1 {
                self.tails[i] = self.tails[i - 1];
            }
            self.tails[0] = self.position;
        }
        match (self.direction) {
            data::Direction::Up => {
                if ((self.position.1 as isize) - 1 < 0) {
                    self.alive = false;
                } else {
                    self.position.1 -= 1
                }
            },
            data::Direction::Right => self.position.0 += 1,
            data::Direction::Down  => self.position.1 += 1,
            data::Direction::Left  => {
                if ((self.position.0 as isize) - 1 < 0) {
                    self.alive = false;
                } else {
                    self.position.0 -= 1
                };
            }
        };
    }

    pub fn add_tail(&mut self) -> () {
        if (self.tails.len() >= 1) {
            self.tails.push(self.tails[self.tails.len() - 1]);
        } else {
            self.needs_add += 1;
        };
    }

    pub fn remove_tail(&mut self) -> () {
        if (self.tails.len() >= 1) {
            self.tails.remove(self.tails.len() - 1);
        };
    }

    pub fn set_length(&mut self, amount : usize) -> () {
        while (self.tails.len() > amount) {
            self.remove_tail();
        };
        if (self.tails.len() < amount) {
            for _i in 0..(amount - self.tails.len()) {
                self.add_tail();
            };
        };
    }
    
}
