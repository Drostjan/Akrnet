#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

pub mod player;

use player::Player;
use crate::player::board::Board;

use std::str::FromStr;
use std::io;

pub fn main() {
    let p1 = Player::new();
    let p2= Player::new();
    let mut players = Vec::new();
    players.push(p1);
    players.push(p2); 

    let mut exit = false;
    let mut ini = false;
    let mut pre1 = false;
    let mut pre2 = false;

    while !exit {
        match setOpt() {
            1=>{
                let tab = Board::new(0);
                let lenTab = tab.setLenTab();
                println!("");
                players[0].setTab(Board::new(lenTab as usize));
                players[0].setTabA(Board::new(lenTab as usize));
                players[0].setShips(lenTab as usize);

                players[1].setTab(Board::new(lenTab as usize));
                players[1].setTabA(Board::new(lenTab as usize));
                players[1].setShips(lenTab as usize);

                ini = true;
                pre1 = false;
                pre2 = false;
                continue;
            },
            2=>{
                if ini && !pre1 {
                    players[0].prepareTab();
                    //promptEnterKey();
                    pre1 = true;
                } else if pre1 {
                    println!("\n  Ya has preparado el tablero");
                } else {
                    println!("\n  Primero debes inicializar la partida");
                }
                continue;
            },
            3=>{
                if ini && !pre2 {
                    players[1].prepareTab();
                    pre2 = true;
                } else if pre2 {
                    println!("\n  Ya has preparado el tablero");
                } else {
                    println!("\n  Primero debes inicializar la partida");
                }
                continue;
            },
            4=>{
                if ini && pre1 && pre2 {
                    let mut fin = false;
                    let mut attacker:Player;
                    let mut defender:Player;

                    let mut turn = 0;
                    println!("Empieza la partida...");

                    while !fin {
                        attacker = players[turn%2].clone();
                        defender = players[(turn + 1)%2].clone();
                        println!("\n  Turno de {}...",attacker.getName());
                        let d = defender.clone();
                        attacker.play(defender);
                        
                        if d.underShips() {
                            fin = true;
                            println!("\n{} ????HA GANADO!!",attacker.getName());
                        }
                        turn += 1;
                    }

                    ini = false;
                    pre1 = false;
                    pre2 = false;
                    players[0].setTab(Board::new(0));
                    players[0].setTabA(Board::new(0));
                    players[0].setShips(0);

                    players[1].setTab(Board::new(0));
                    players[1].setTabA(Board::new(0));
                    players[1].setShips(0);
                }else{
                    println!("\n  No puedes empezar sin inicializar la partida ni los tableros de cada jugador");
                }
                continue;
            },
            5=>{
                exit = true;
                break;
            }
            _=>{
                println!("  Opci??n no v??lida");
            }
        }
    }

pub fn setOpt() -> u64{
    println!("####################################");
    println!("#          Undir la Flota          #");
    println!("####################################");
    println!("# 1 - Inicializar partida          #");
    println!("# 2 - Preparar tablero Jugador 1   #");
    println!("# 3 - Preparar tablero Jugador 2   #");
    println!("# 4 - Empezar partida              #");
    println!("# 5 - Salir                        #");
    println!("####################################");
    print!("Introduce la opci??n que deseas: \n");
    let mut option = String::new();
    io::stdin().read_line(&mut option).ok().expect("Error al leer de teclado");
    let opt: u64 = u32::from_str(&option.trim()).unwrap() as u64;

    return opt;
}

}