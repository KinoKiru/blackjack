use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut dealer: u8 = 0;
    let mut user: u8 = 0;
    let range = 1..11;
    loop {
        println!("dealer {}", dealer);
        match dealer.cmp(&17) {
            Ordering::Less => {
                dealer += rand::thread_rng().gen_range(range.clone());
            }
            Ordering::Greater => break,
            Ordering::Equal => break,
        }
    }

    //initial draw for the user
    user += rand::thread_rng().gen_range(range.clone());
    loop {
        //ask the user for a action based on there drawn value
        println!("you drew {} what will you do?", user);

        println!("stand or hit");
        let mut action = String::new();
        io::stdin()
            .read_line(&mut action)
            .expect("failed to read line");
        action = action.trim().to_string();
        if action == "hit" {
            user += rand::thread_rng().gen_range(range.clone());
            if user > 21 {
                println!("bust! \r\nDealer won!");
                break;
            }
            continue;
        } else {
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
}
