use rand::seq::SliceRandom;
use std::io::{self, Write};

struct Card {
    value: u32,
    revealed: bool,
}

struct Game(Vec<Card>);

impl Card {
    fn new(n: u32) -> Card {
        Card {
            value: n,
            revealed: false,
        }
    }
}

impl Game {
    fn new(n: usize) -> Game {
        let mut data = Vec::with_capacity(2 * n);
        data.extend((1..=n).flat_map(|i| vec![Card::new(i as u32), Card::new(i as u32)]));
        data.shuffle(&mut rand::thread_rng());
        Game(data)
    }

    fn display(&self) {
        print!("\n[ ");
        for (key, x) in self.0.iter().enumerate() {
            if x.revealed {
                print!("( {} ) ", x.value);
            } else {
                print!("..{}.. ", key + 1);
            }
        }
        print!("]\n\n");
    }

    fn guess(&mut self, idx: usize) -> io::Result<()> {
        if idx <= self.0.len() && idx > 0 {
            let card = &mut self.0[idx - 1];
            if !card.revealed {
                card.revealed = true;
                println!("\nFOUND {} AT {}", card.value, idx);
                self.display();
                print!("GUESS AGAIN: ");
                io::stdout().flush()?;
                let mut g2 = String::new();
                io::stdin().read_line(&mut g2)?;
                match g2.trim().parse::<usize>() {
                    Ok(value) => {
                        if value <= self.0.len() && value > 0 {
                            self.check_matching_cards(value - 1, idx - 1);
                        } else {
                            self.hide_guess(vec![value]);
                            println!("INVALID CARD POSITION");
                        }
                    }
                    Err(_) => {
                        self.hide_guess(vec![idx]);
                        println!("ONLY INTEGERS FROM 1-{}", self.0.len());
                    }
                }
            } else {
                println!("CARD AT POSITION {} IS ALREADY REVEALED.", idx);
            }
        } else {
            self.hide_guess(vec![idx]);
            println!("INVALID CARD POSITION.");
        }
        Ok(())
    }

    fn check_matching_cards(&mut self, v1: usize, v2: usize) {
        let c1 = self.0.get(v1).unwrap();
        let c2 = self.0.get(v2).unwrap();
        if c1.value == c2.value {
            self.reveal_new_guess(vec![v1, v2]);
            println!("MATCH! REVEALED NEW GUESS: {}", v1);
        } else {
            self.hide_guess(vec![v1, v2]);
            println!("NO MATCH. FIRST GUESS HIDDEN.");
        }
    }

    fn reveal_new_guess(&mut self, revealed_cards: Vec<usize>) {
        for card_idx in revealed_cards {
            self.0[card_idx].revealed = true;
        }
    }

    fn hide_guess(&mut self, revealed_cards: Vec<usize>) {
        for card_idx in revealed_cards {
            self.0[card_idx].revealed = false;
        }
    }

    fn win(&self) -> bool {
        self.0.iter().all(|card| card.revealed)
    }
}

fn main() -> io::Result<()> {
    let mut cards = Game::new(4);
    println!("\nCARDS INITIATED ^-^");
    cards.display();
    loop {
        print!("GUESS ANY CARD: ");
        io::stdout().flush()?;
        let mut g1 = String::new();
        io::stdin().read_line(&mut g1)?;
        match g1.trim().parse::<usize>() {
            Ok(value) => {
                cards.guess(value)?;
                cards.display();
                if cards.win() {
                    println!("\nYOU WON ^-^\n");
                    break;
                }
            }
            Err(_) => {
                println!("ONLY INTEGERS FROM 1-{}", cards.0.len());
            }
        }
    }
    Ok(())
}
