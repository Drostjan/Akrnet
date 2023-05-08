#[derive(Clone)]
pub struct Ship {
    ship: Vec<Vec<usize>>,
    status: bool
} 

impl Ship {
    pub fn new(len_ship:usize) -> Ship {
        Ship {
            ship: vec![vec![0;3];len_ship],
            status: false
        }
    }

    pub fn get_ship(&self) -> Vec<Vec<usize>>{
        return self.ship.clone();
    }

    pub fn set_ship(&mut self,cas:[usize;2],dir:String){
        if dir.contains("H") {
            for i in 0..self.ship.len() {
                self.ship[i][0] = cas[0];
                self.ship[i][1] = cas[1]+1;
                self.ship[i][2] = 0;
            }
        }else{
            for i in 0..self.ship.len() {
                self.ship[i][0] = cas[0]+1;
                self.ship[i][1] = cas[1];
                self.ship[i][2] = 0;
            }
        }
    }

    pub fn is_under(&self) -> bool{
        return self.status
    }

    pub fn set_under(&mut self){
        let mut i = 0;
        let mut under = true;

        while under && i < self.ship.len() {
            if self.ship[i][2] == 0 {
                under = false;
            }
            i += 1;
        }
        self.status = under;
    }

    pub fn is_touch(&mut self,cas:[usize;2]) -> bool{
        let mut touch = false;
        let mut i = 0;

        while touch && i < self.ship.len() {
            if self.ship[i][0] == cas[0] && self.ship[i][1] == cas[1] {
                touch = true;
                self.ship[i][2] = 1;
            }
            i += 1;
        }
        return touch;
    }
}