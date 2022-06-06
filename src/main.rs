use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        let range = 1..11;
        let mut dealer: u8 = rand::thread_rng().gen_range(range.clone());
        let mut user: u8 = 0;
        loop {
            println!("dealer {}", dealer);

            //initial draw for the user
            user += rand::thread_rng().gen_range(range.clone());
            if user > 21 {
                println!("bust! \r\nDealer won!");
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
                            println!("Dealer drew a card,\r\ndealer's score is now: {}", dealer);
                            continue;
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
