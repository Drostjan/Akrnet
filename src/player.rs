#![allow(non_snake_case)]
#![allow(unused_assignments)]
pub mod ship;
pub mod board;

use board::Board;
use ship::Ship;

use std::io;
use std::str::FromStr;

#[derive(Clone)] 
pub struct Player {
    name: String,
    tab: Board,
    tabA: Board,
    ships: Vec<Ship>,
}

impl Player {
    pub fn start(name: &str,tab:Board,tabA: Board,ships: Vec<Ship>) -> Player {
        Player {
            name: name.to_owned(),
            tab: tab,
            tabA: tabA,
            ships: ships,
        }
    }

    pub fn new() -> Player {
        Player {
            name: String::new(),
            tab: Board::new(0),
            tabA: Board::new(0),
            ships: Vec::new(),
        }
    }
    
    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_tab(&self) -> Board {
        return self.tab.clone();
    }

    pub fn set_tab(&mut self,tab: Board) {
        self.tab = tab;
    }

    pub fn getTabA(&self) -> Board {
        return self.tabA.clone();
    }

    pub fn setTabA(&mut self,tabA: Board) {
        self.tabA = tabA;
    }

    pub fn getShips(&self) -> Vec<Ship> {
        return self.ships.clone();
    }

    pub fn setShips(&mut self,lenShips: usize) {
        for i in 0..lenShips -2 {
            let ship = Ship::new(lenShips - 1 - i);
            self.ships.push(ship);
        }
    }

    pub fn prepareTab(&mut self){
        print!("Introduce tu nombre: \n");
        let mut name = String::new();
        io::stdin().read_line(&mut name).ok().expect("Error al leer de teclado");
        self.name = name;
        print!("Coloca tu tablero {}:   \n",self.name.replace('\n', ""));
        
        self.tab.draw_tab();
        println!();
        for i in 0..self.ships.len() {
            let mut push = false;
            while !push {
                let cas = self.tab.set_pos_ship(self.ships[i].get_ship().len());
                let dir = self.tab.set_dir_ship();
                let r = &dir;
                if self.tab.cabe(r.to_owned(), cas, self.ships[i].get_ship().len()) {
                    self.tab.insert_ship(self.ships[i].get_ship().len(),cas,r);
                    self.ships[i].set_ship(cas,dir);
                    push = true;
                    println!(" {}", self.ships.len());
                    print!("Coloca tu tablero {}:   \n",self.name.replace('\n', ""));
                    println!("");
                    self.tab.draw_tab();
                    println!("");
                }else{
                    println!("  No cabe");
                }
            }

        }

    }

    pub fn setPlay(&self) -> u64 {
        println!("\n Turno de: {}",self.get_name());
        println!("  ####################################");
        println!("  #           M  E  N  Ú             #");
        println!("  ####################################");
        println!("  #   1) Ver mi tablero              #");
        println!("  #   2) Ver tablero de ataque       #");
        println!("  #   3) Atacar                      #");
        println!("  ####################################");
        print!("Introduce la opción que deseas: \n");
        let mut option = String::new();
        io::stdin().read_line(&mut option).ok().expect("Error al leer de teclado");
        let opt: u64 = u32::from_str(&option.trim()).unwrap() as u64;
        return opt;
    }

    pub fn play(&self,player: Player){
        let mut exit = false;
        while !exit {
            match self.setPlay() {
                1=>{
                    println!("");
                    self.get_tab().draw_tab();
                    break;
                },
                2=>{
                    println!("");
                    self.getTabA().draw_tab();
                    break;
                },
                3=>{
                    self.atackShips(player);
                    exit = true;
                    break;
                }
                _=>{
                    println!("  Opcion no valida");
                    break;
                }
            }
        }
    }

    pub fn atackShips(&self, player: Player){
        println!("");
        self.getTabA().draw_tab();

        print!("Introduce la fila donde quieres atacar: \n");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let r: u64 = (u32::from_str(&input.trim()).unwrap() - 1)as u64;

        print!("Introduce la columna donde quieres atacar: \n");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).ok().expect("Error al leer de teclado");
        let c: u64 = (u32::from_str(&input2.trim()).unwrap() - 1) as u64;
        let cas = [r as usize, c as usize];

        match player.get_tab().get_cas(cas) {
            ' '=>{
                self.getTabA().set_cas(cas,'*');
                player.get_tab().set_cas(cas,'*');
                println!("");
                self.getTabA().draw_tab();
                println!("\n  AGUA!!");
            }
            '@'=>{
                self.getTabA().set_cas(cas, 'X');
                player.get_tab().set_cas(cas, 'X');
                println!("");
                self.getTabA().draw_tab();
                let mut i = 0;
                while player.getShips()[i].isTouch(cas){
                    i += 1;
                }
                println!("\n  TOCADO!!");
                player.getShips()[i].setUnder();
                if player.getShips()[i].is_under() {
                    println!("\n    BARCO HUNDIDO!!");
                }
            }
            _=>{
                println!("\n  ¡Que pena! Ya habías tirado ahí");
            }
        }
    }

    pub fn underShips(&self) -> bool{
        let mut win = false;
        let mut i = 0;
        while win && i < self.ships.len() {
            if self.ships[i].is_under() {
                win = true;
            }
            i += 1;
        }
        return win;
    }
}
