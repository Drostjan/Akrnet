use std::str::FromStr;
use std::io;

#[derive(Clone)] 
pub struct Board {
    tab: Vec<Vec<char>>,
}

impl Board {
    pub fn new(lenTab: usize) -> Board {
        Board {
            tab: vec![vec![' ';lenTab];lenTab],
        }
    }

    pub fn init(&mut self){
        for v1 in 0..self.tab.len() {
            for v2 in 0..self.tab.len() {
                self.tab[v1][v2] = '~';
            }
        }     
    }

    pub fn get_cas(&self,cas:[usize;2]) -> char {
        return self.tab[cas[0]][cas[1]]
    }

    pub fn set_cas(&mut self,cas:[usize;2],c:char){
        self.tab[cas[0]][cas[1]] = c;
    }

    pub fn insert_ship(&mut self,lenShip:usize,cas:[usize;2],dir:&String){
        if dir.contains("H") {
            for i in 0..lenShip {
                self.tab[cas[0]][cas[1] + i] = '@';
            }
        } else {
            for i in 0..lenShip {
                self.tab[cas[0] + i][cas[1]] = '@';
            }
        }
    }

    pub fn set_len_tab(&self) -> u64 {
        print!("Introduce el tamaño del tablero: \n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let lenShip: u32 = u32::from_str(&input.trim()).unwrap();
        
        if lenShip < 5  {
            print!("El tablero debe ser de 5x5 como mínimo \nvuelve a introducir el tamaño: ");
        }
        return lenShip as u64;
    }

    pub fn set_pos_ship(&self,lenShip:usize) -> [usize;2]{
        print!("Introduce la fila donde colocar el barco de tamaño {} :  \n", lenShip );
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let row: u64 = (u32::from_str(&input.trim()).unwrap() - 1) as u64;

        print!("Introduce la columna donde colocar el barco de tamaño {} :  \n", lenShip );
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).ok().expect("Error al leer de teclado");
        let col: u64 = (u32::from_str(&input2.trim()).unwrap() - 1) as u64;
        
        let cas = [row as usize,col as usize];

        return cas;
    }

    pub fn set_dir_ship(&self) -> String {
        print!("Introduce la orientación del barco (H / V): \n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");

        return input;
    }

    pub fn draw_tab(&self){
        print!("    ");
        for i in 0..self.tab.len() {
            if i < 10{
                print!("  {} ",i+1);
            }else{
                print!("  {}",i+1);
            }
        }
        println!();
        print!("  ");
        for i in 0..self.tab.len() {
            print!("|---");
        }
        println!("|");
        for i in 0..self.tab.len() {
            if (i +1) < 10 {
                print!("{} ",(i+1));
            }else{
                print!("{}",(i + 1));
            }
            for j in 0..self.tab.len() {
                print!("| {} ",self.tab[i][j]);
            }
            println!("|");
            print!("  ");
            for j in 0..self.tab.len() {
                print!("|---");
            }
            println!("|");
        }
    }

    pub fn cabe(&self,dir:String,cas:[usize;2],lenShip:usize) -> bool {
        let mut cabe = true;
        let mut cont = 0;
        if self.tab[cas[0]][cas[1]] == ' ' {
            if dir.contains("H"){
                if (cas[1] + lenShip) <= self.tab.len() {
                    while cabe && cont < lenShip {
                        if self.tab[cas[0]][cas[1] + cont] != ' ' {
                            cabe = false;
                        }
                        cont += 1;
                    }
                }else{
                    cabe = false;
                }
            }else{
                if (cas[0] + lenShip) <= self.tab.len() {
                    while cabe && cont < lenShip {
                        if self.tab[cas[0] + cont][cas[1]] != ' ' {
                            cabe = false;
                        }
                        cont += 1;
                    }
                }else{
                    cabe = false;
                }
            }
        }else{
            cabe = false;
        }
        return cabe;
    }
}