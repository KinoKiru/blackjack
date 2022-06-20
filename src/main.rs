//models aka object
mod model;
use model::card::Card;
use model::dealer::Dealer;
use model::deck::Deck;
use model::player::Player;

use std::io;

//menu crates
use crossterm::style::Color;
use terminal_menu::{button, label, menu, mut_menu, run};

fn main() {
    loop {
        let menu = menu(vec![
            label("----------------------"),
            label("blackjack game!"),
            label("use wasd or arrow keys to navigate"),
            label("enter to select"),
            label("game made in rust by kinokiru"),
            label("----------------------"),
            label("select dealer name"),
            button("Jerome").colorize(Color::DarkRed),
            button("Quandale Dingle").colorize(Color::Blue),
            button("CumQuad").colorize(Color::Green),
            button("Pelluh").colorize(Color::Magenta),
        ]);
        run(&menu);

        let menu = mut_menu(&menu);
        let dealer_name = menu.selected_item_name();
        let mut deck = Deck::new();
        let mut dealer = Dealer::new(dealer_name.to_string(), &mut deck);
        let mut players: Vec<Player> = vec![];

        //hoeveel users?
        let mut amount_players = String::new();

        println!("How many players will be playing?");
        io::stdin()
            .read_line(&mut amount_players)
            .expect("failed to read line");

        for index in 0..amount_players.trim().parse().unwrap() {
            let mut name = String::new();
            println!("name of player {}", index + 1);
            io::stdin()
                .read_line(&mut name)
                .expect("failed to read line");

            players.push(Player::new(name.trim().to_string(), &mut deck));
        }

        println!("{} has {}", dealer_name, to_string(&dealer.hand));
        for mut player in players.clone() {
            println!("{}'s turn", &player.name);
            println!("you drew {} what will you do?", to_string(&player.hand));

            loop {
                println!("stand or hit");
                let mut action = String::new();
                io::stdin()
                    .read_line(&mut action)
                    .expect("failed to read line");
                action = action.trim().to_lowercase().to_string();

                if action.starts_with('h') {
                    player.hit(&mut deck);
                    println!("you drew {}", to_string(&player.hand));
                    if calculate_hand_size(&player.hand) > 21 {
                        println!("{} bust!", &player.name);
                        break;
                    }
                    println!("what will you do?");
                } else {
                    break;
                }
            }
        }

        while calculate_hand_size(&dealer.hand) < 17 {
            dealer.hit(&mut deck);
            println!(
                "{} drew a card,\r\n{}'s score is now: {}",
                dealer_name,
                dealer_name,
                to_string(&dealer.hand)
            );
        }
        let dealer_bust: bool = calculate_hand_size(&dealer.hand) > 21;
        if dealer_bust {
            println!("{} bust", dealer_name);
        }

        players.sort_by(|p1, p2| calculate_hand_size(&p2.hand).cmp(&calculate_hand_size(&p1.hand)));
        players = players
            .into_iter()
            .filter(|p| calculate_hand_size(&p.hand) <= 21)
            .collect();

        if !dealer_bust && calculate_hand_size(&players[0].hand) < calculate_hand_size(&dealer.hand)
        {
            println!("Dealer wins");
        } else {
            let mut index_draw: usize = 0;
            for index in 0..players.len() - 1 {
                let p1 = calculate_hand_size(&players[index].hand);
                let p2 = calculate_hand_size(&players[index + 1].hand);
                if p1 == p2 {
                    index_draw = index;
                } else {
                    break;
                }
            }

            if index_draw == 0 {
                println!("{} won!", &players[index_draw].name);
            } else {
                println!(
                    "draw between: {}",
                    players[0..index_draw + 1]
                        .iter()
                        .map(|p| p.name.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                );
            }
        }

        let mut replay = String::new();
        println!("Play again? Y[es] N[o]");
        io::stdin()
            .read_line(&mut replay)
            .expect("failed to read line");

        replay = replay.trim().to_lowercase().to_string();

        if replay.starts_with("n") {
            break;
        }
    }
}

/*
 * Function to calculate handsize based on hand<Vec<Card>>
 * calculates the value of the aces based on your hand either 1-11
 */
fn calculate_hand_size(hand: &Vec<Card>) -> u8 {
    let mut total_value: u8 = 0;
    let mut amount_aces: u8 = 0;
    for card in hand {
        if card.value == 1 {
            amount_aces += 1;
            continue;
        }
        total_value += card.value;
    }

    if amount_aces > 0 {
        for _ in 0..amount_aces {
            if total_value < 10 {
                total_value += 11;
                continue;
            }
            total_value += 1;
        }
    }
    return total_value;
}

/*
* Returns vector in a readable state for the user
*/
fn to_string(hand: &Vec<Card>) -> String {
    hand.iter()
        .map(|card| card.short_name.to_owned())
        .collect::<Vec<String>>()
        .join(", ")
        + " ["
        + calculate_hand_size(hand).to_string().as_str()
        + "]"
}
