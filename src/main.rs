use crossterm::style::Color;
use rand::Rng;
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

        let range = 1..11;
        let mut dealer: u8 = if dealer_name == "Jerome" {
            rand::thread_rng().gen_range(19..22)
        } else {
            rand::thread_rng().gen_range(range.clone())
        };
        let mut user: u8 = 0;
        'poo: loop {
            println!("{} {}", dealer_name, dealer);

            //initial draw for the user
            user += rand::thread_rng().gen_range(range.clone());
            if user > 21 {
                println!("bust! \r\n{} won!", dealer_name);
                break;
            }
            //ask the user for a action based on there drawn value
            println!("you drew {} what will you do?", user);

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
                    match dealer.cmp(&17) {
                        Ordering::Less => {
                            dealer += rand::thread_rng().gen_range(range.clone());
                            if dealer > 21 {
                                println!("{} bust, You Win!", dealer_name);
                                break 'poo;
                            } else {
                                println!(
                                    "{} drew a card,\r\n{}'s score is now: {}",
                                    dealer_name, dealer_name, dealer
                                );
                                continue;
                            }
                        }
                        Ordering::Greater => break,
                        Ordering::Equal => break,
                    }
                }
                if user > dealer {
                    println!("You've won!");
                    break;
                } else if user == dealer {
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
