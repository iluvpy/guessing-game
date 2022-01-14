
mod game;
use std::vec;

use game::player;
use game::input;
use rand::Rng;


fn main() {
    let min_number = input::get_number("Numero minimo");
    let max_number = input::get_number("Numero massimo");
    let start_life = input::get_number("Scegli la vita iniziale");

    let mut players: Vec<player::Player> = vec![];
    let players_num = input::get_number("Scegli il numero di giocatori");


    for i in 0..players_num {
        let name = input::get_string(format!("Scegli il nome del giocatore {}", i+1).as_str());
        players.push(player::Player::new(start_life, name));
    }

    loop {
        let random_number = rand::thread_rng().gen_range(min_number..max_number);
        println!("Ho fatto la mia scelta, adesso scegliete voi");
        for player_ in players.iter_mut() {
            player_.guess();
        }   

        for player in players.iter_mut() {
            player.calculate_guess(random_number);
        }

        let mut i = 0;
        let mut killed = vec![];
        let len = players.len();
        for player in players.iter_mut() {
            if player.get_life() < 0 && i != len-1 {
                println!("{} sei morto!", player.get_name());
                killed.push(player::Died {
                    name: player.get_name(),
                    i: i
                });
            }
            i += 1;
        }

        println!("La mia scelta è stata {}! Adesso guardo se avete perso", random_number);
        println!("\nGiocatori vivi: ");
        for player in &players {
            println!("{} ha {} vita", player.get_name(), player.get_life());
        }

        if killed.len() > 0 {
            println!("Giocatori morti: \n");
        }
        for died in killed {
            println!("{} died!", died.name);
            players.remove(died.i);
        }

    }
}
