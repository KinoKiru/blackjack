//module deck &  card
mod card;
mod deck;

use card::Card;
use deck::Deck;

use crossterm::style::Color;
use std::cmp::Ordering;
use std::io;
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
        let mut dealer: Vec<Card> = vec![deck.pick_random()];
        let mut player: Vec<Card> = vec![deck.pick_random()];
        'poo: loop {
            println!("{} has {}", dealer_name, to_string(&dealer));

            //initial draw for the user
            player.push(deck.pick_random());
            if calculate_hand_size(&player) > 21 {
                println!("bust! \r\n{} won!", dealer_name);
                break;
            }
            //ask the user for a action based on there drawn value
            println!("you drew {} what will you do?", to_string(&player));

            println!("stand or hit");
            let mut action = String::new();
            io::stdin()
                .read_line(&mut action)
                .expect("failed to read line");
            action = action.trim().to_string();
            if action == "hit" {
                continue;
            } else {
                loop {
                    match calculate_hand_size(&dealer).cmp(&17) {
                        Ordering::Less => {
                            dealer.push(deck.pick_random());
                            if calculate_hand_size(&dealer) > 21 {
                                println!("{} bust, You Win!", dealer_name);
                                break 'poo;
                            } else {
                                println!(
                                    "{} drew a card,\r\n{}'s score is now: {}",
                                    dealer_name,
                                    dealer_name,
                                    to_string(&dealer)
                                );
                                continue;
                            }
                        }
                        Ordering::Greater => break,
                        Ordering::Equal => break,
                    }
                }
                if calculate_hand_size(&player) > calculate_hand_size(&dealer) {
                    println!("You've won!");
                    break;
                } else if calculate_hand_size(&player) == calculate_hand_size(&dealer) {
                    println!("Draw!");
                    break;
                } else {
                    println!("You've lost!");
                    break;
                }
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

//TODO make a total value function which calculated hand total
// for each card in the vector return that total value
fn calculate_hand_size(hand: &Vec<Card>) -> u8 {
    let mut total_value: u8 = 0;
    for card in hand {
        total_value += card.value;
    }
    return total_value;
}

fn to_string(hand: &Vec<Card>) -> String {
    // for card in hand {
    //     totalstring += card.short_name.as_str();
    // }
    hand.iter()
        .map(|card| card.short_name.to_owned())
        .collect::<Vec<String>>()
        .join(", ")
        + " ["
        + calculate_hand_size(hand).to_string().as_str()
        + "]"
}
