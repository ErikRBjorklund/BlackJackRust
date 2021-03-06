use rand::Rng;
use std::io::*;
use std::io;

fn main() {
    println!("What is your name, Player?");
    let stdin = io::stdin();
    let name = stdin.lock().lines().next().unwrap().unwrap();

    println!("Welcome, {}!", name);

    loop {
        let mut player_hand: Vec<String> = Vec::new();
        let mut computer_hand: Vec<String> = Vec::new();

        player_hand.push(get_card(rand::thread_rng().gen_range(1..14)));
        player_hand.push(get_card(rand::thread_rng().gen_range(1..14)));
        computer_hand.push(get_card(rand::thread_rng().gen_range(1..14)));
        computer_hand.push(get_card(rand::thread_rng().gen_range(1..14)));
        if get_score(&player_hand) != 21 {
            loop {
                println!("{}'s Hand: {:?}", name, player_hand);
                println!("{}'s Score: {}", name, get_score(&player_hand));
                println!("Dealer's Showing Card: {}", computer_hand[0]);
                println!("Would you like to [hit/hold]?");
                let answer = stdin.lock().lines().next().unwrap().unwrap();
                if answer.eq("hit") {
                    player_hand.push(get_card(rand::thread_rng().gen_range(1..14)));
                    let score = get_score(&player_hand);
                    if score == 21 {
                        println!("BlackJack!! Congratulations {}, you won!", name);
                        break;
                    } else if score > 21 {
                        println!("Sorry {}, you lost with a score of {}! You're bound to win next time!", name, score);
                        break;
                    }
                } else {
                    while get_score(&computer_hand) < 17 {
                        computer_hand.push(get_card(rand::thread_rng().gen_range(1..14)));
                    }
                    if get_score(&computer_hand) > 21 {
                        println!("Congratulations {}! You WON!", name);
                    } else if get_score(&computer_hand) < get_score(&player_hand) {
                        println!("Congratulations {}! You WON!", name);
                    } else if get_score(&computer_hand) > get_score(&player_hand) {
                        println!("Sorry {}, you lost with a score of {}! You're bound to win next time!", name, get_score(&player_hand));
                    } else {
                        println!("You tied! Almost gottem!");
                    }
                    break;
                }
            }
        } else {
            println!("BlackJack!! Congratulations {}, you won!", name);
        }

        

        println!("{}'s Hand: {:?}", name, player_hand);
        println!("Dealer's Hand: {:?}", computer_hand);


        println!("Would you like to play again? [yes/no]");
        if stdin.lock().lines().next().unwrap().unwrap().eq("no") {
            break;
        }
    }
}

fn get_card(num: i8) -> String {
    if num <= 10 && num != 1 {
        return num.to_string();
    } 
    if num == 11 {
        return "J".to_string();
    }
    if num == 12 {
        return "Q".to_string();
    }
    if num == 13 {
        return "K".to_string();
    }
    return "A".to_string();
}

fn get_score(hand: &Vec<String>) -> i8 {
    let mut score: i8 = 0;
    let mut aces: i8 = 0;
    for card in hand.iter() {
        if card.eq("2") {
            score += 2;
        } else if card.eq("3") {
            score += 3;
        } else if card.eq("4") {
            score += 4;
        } else if card.eq("5") {
            score += 5;
        } else if card.eq("6") {
            score += 6;
        } else if card.eq("7") {
            score += 7;
        } else if card.eq("8") {
            score += 8;
        } else if card.eq("9") {
            score += 9;
        } else if card.eq("10") {
            score += 10;
        } else if card.eq("J") {
            score += 10;
        } else if card.eq("Q") {
            score += 10;
        } else if card.eq("K") {
            score += 10;
        } else if card.eq("A") {
            aces += 1;
        }
    }
    if aces == 1 {
        if 21 - score >= 11 {
            score += 11;
        } else {
            score += 1;
        }
    } else if aces > 1 {
        score += aces - 1;
        if 21 - score >= 11 {
            score += 11;
        } else {
            score += 1;
        }
    }
    return score;
}