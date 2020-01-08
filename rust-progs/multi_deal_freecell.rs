use std::io::prelude::*;
use std::fs::File;
use std::env;
/*
 * Microsoft C Run-time-Library-compatible Random Number Generator
 * Copyright by Shlomi Fish, 2011.
 * Released under the MIT/X11 License
 * ( http://en.wikipedia.org/wiki/MIT_License ).
 * */

struct MsvcRandGen {
    seed: i32
}

impl MsvcRandGen {
    fn rand(&mut self) -> i32 {
        self.seed = self.seed.wrapping_mul(214013).wrapping_add(2531011) & 0x7FFFFFFF;
        return (self.seed >> 16) & 0x7FFF;
    }
    fn max_rand(&mut self, mymax: i32) -> i32 {
        return self.rand() % mymax;
    }
    fn shuffle<T>(&mut self, deck: &mut [T]) {
        if deck.len() > 0 {
            let mut i = (deck.len() as i32) - 1;
            while i > 0 {
                let j = self.max_rand(i+1);
                deck.swap(i as usize, j as usize);
                i = i-1;
            }
        }
    }
}

/*
 * Microsoft Windows Freecell / Freecell Pro boards generation.
 *
 * See:
 *
 * - http://rosettacode.org/wiki/Deal_cards_for_FreeCell
 *
 * - http://www.solitairelaboratory.com/mshuffle.txt
 *
 * Under MIT/X11 Licence.
 *
 * */


fn deal_ms_fc_board(seed: i32) -> String {
    let mut randomizer = MsvcRandGen { seed: seed, };
    let num_cols = 8;

    let mut columns = vec![vec![]; num_cols];
    let mut deck = (0..4*13).into_iter().collect::<Vec<u32>>();

    let rank_strings: Vec<char> = "A23456789TJQK".chars().collect();
    let suit_strings: Vec<char> = "CDHS".chars().collect();

    randomizer.shuffle(&mut deck);

    deck.reverse();

    for i in 0 .. 52 {
        columns[i % num_cols].push(deck[i]);
    };

    return columns.iter().map(|col| {
        return format!("{}\n", col.iter().map(|card| {
            let suit = card % 4;
            let rank = card / 4;
            return format!("{}{}",rank_strings[rank as usize], suit_strings[suit as usize])
        }).collect::<Vec<String>>().join(" "))
    }).collect::<Vec<String>>().join("")

}

fn main() -> std::io::Result<()> {
    let mut args = env::args();
    let mut dir = String::from("");
    let mut suffix = String::from("");

    let mut argidx: usize = 1;
    while argidx < args.len() {
        let arg = args.nth(argidx);
        match arg {
            Some(x)=> {
                let s = x.to_string();
                let first_char = s.chars().next().unwrap();
                if first_char != '-'
                {
                    break;
                }
                match s.as_ref() {
                    "--dir" => {
                        argidx +=1;
                        let p = args.nth(argidx);
                        match p {
                            Some(y) => {
                                dir = y.to_string();
                            },
                            _ => {},
                        }
                    },
                    "--suffix" => {
                        argidx +=1;
                        let p = args.nth(argidx);
                        match p {
                            Some(y) => {
                                suffix = y.to_string();
                            },
                            _ => {},
                        }
                    },
                    _ => {},
                }
            },
            None => {},
        }
        argidx+= 1;
    }
    while argidx < args.len() {
        let arg = args.nth(argidx);
        match arg {
            Some(x)=> {
                let s = x.to_string();
                match s.as_ref() {
                    "seq" => {
                        argidx +=1;
                        let p = args.nth(argidx);
                        match p {
                            Some(x) => match x.to_string().parse::<u32>() {
                                Ok(y) => {
                                    argidx +=1;
                                    let pend = args.nth(argidx);
                                    match pend {
                                        Some(xend) => match xend.to_string().parse::<u32>() {
                                            Ok(yend) => {
                                                for i in (y as i32) .. (yend as i32) {
                                                    let mut f = File::create(format!("{}/{}{}", dir, i, suffix))?;
                                                    f.write(deal_ms_fc_board(i).as_bytes())?;
                                                }
                                            },
                                            Err(_e) => println!("I need a real number"),
                                        },
                                        None => {}
                                    };
                                },
                                Err(_e) => println!("I need a real number"),
                            },
                            None => {},
                        }
                        argidx+= 1;
                    },
                    _ => {
                        println!("I need a seq");
                    },
                }
            },
            None => println!("foo"),
        }
    }
    return Ok(());
}
