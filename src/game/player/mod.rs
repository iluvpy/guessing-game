use super::input;

#[derive(Debug, Clone)]
pub struct Player {
    guess: i32,
    life: i32,
    name: String
}

pub struct Died {
    pub name: String,
    pub i: usize
}

impl Player {

    pub fn guess(&mut self) {
        self.guess = input::get_number(format!("{} scegli un numero!", self.name).as_str());
    }

    pub fn calculate_guess(&mut self, random_num: i32) {
        self.life -= random_num - self.guess;
    }

    pub fn get_life(&self) -> i32 {
        return self.life;
    }

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn new(life: i32, name: String) -> Player {
        return Player { guess: 0, life: life, name: name };
    }
}