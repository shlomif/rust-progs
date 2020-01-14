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
        columns[i & 7].push(deck[i]);
    };

    return columns.iter().map(|col| {
        return format!("{}\n", col.iter().map(|card| {
            let suit = card & 3;
            let rank = card >> 2;
            return format!("{}{}",rank_strings[rank as usize], suit_strings[suit as usize])
        }).collect::<Vec<String>>().join(" "))
    }).collect::<Vec<String>>().join("")

}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut dir = String::from("");
    let mut suffix = String::from("");

    let mut argidx: usize = 1;
    let mut still_loop: bool = true;
    while (argidx < args.len()) && still_loop {
        let arg = &args[argidx];
        let s = arg;
        let first_char = s.chars().next().unwrap();
        if first_char != '-'
        {
            still_loop = false;
        }
        else {
            match s.as_ref() {
                "--dir" => {
                    argidx +=1;
                    let p = &args[argidx];
                    dir = (&p).to_string();
                },
                "--suffix" => {
                    argidx +=1;
                    let p = &args[argidx];
                    suffix = (&p).to_string();
                },
                _ => panic!("Unknown flag {}", s),
            }
            argidx+= 1;
        }
    }
    while argidx < args.len() {
        let arg = &args[argidx];
        let s = arg;
        match s.as_ref() {
            "seq" => {
                argidx +=1;
                let p = &args[argidx];
                match p.to_string().parse::<u32>() {
                    Ok(y) => {
                        argidx +=1;
                        let pend = &args[argidx];
                        match pend.to_string().parse::<u32>() {
                            Ok(yend) => {
                                for i in (y as i32) .. ((yend+1) as i32) {
                                    let mut f = File::create(format!("{}/{}{}", dir, i, suffix))?;
                                    f.write(deal_ms_fc_board(i).as_bytes())?;
                                }
                            },
                            Err(_e) => panic!("I need a real number"),
                        };
                    },
                    Err(_e) => panic!("I need a real number"),
                };
            },
            _ => {
                panic!("I need a seq");
            },
        }
        argidx += 1;
    }
    return Ok(());
}
