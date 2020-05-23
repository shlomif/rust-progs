use std::io::prelude::*;
use std::fs::File;
use std::env;
// use bytes::Bytes;
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


fn deal_ms_fc_board(seed: i32) -> [u8;52*3] {
    let mut randomizer = MsvcRandGen { seed: seed, };
    let mut deck = (0..4*13).into_iter().collect::<Vec<u32>>();

    const RANK_STRINGS: &[u8] = "A23456789TJQK".as_bytes();
    const SUIT_STRINGS: &[u8] = "CDHS".as_bytes();

    randomizer.shuffle(&mut deck);

    deck.reverse();

    const OFFSET_BY_I: [usize;52] = [ 0,21,42,63,84,102,120,138,3,24,45,66,87,105,123,141,6,27,48,69,90,108,126,144,9,30,51,72,93,111,129,147,12,33,54,75,96,114,132,150,15,36,57,78,99,117,135,153,18,39,60,81 ];

    let mut ret: [u8; 52*3]= [0;52*3];
    // <[u8; 52*3]>::from("XX XX XX XX XX XX XX\n".to_owned() +
    ret.copy_from_slice(
    // String::from("XX XX XX XX XX XX XX\n".to_owned() +
    ("XX XX XX XX XX XX XX\n".to_owned() +
                "XX XX XX XX XX XX XX\n" +
                "XX XX XX XX XX XX XX\n" +
                "XX XX XX XX XX XX XX\n" +
                "XX XX XX XX XX XX\n" +
                "XX XX XX XX XX XX\n" +
                "XX XX XX XX XX XX\n" +
                "XX XX XX XX XX XX\n"
                ).as_bytes());
    //as [u8; 52*3];
      //          ); //.as_bytes() as [u8; 52*3];

    for i in 0 .. 52 {
        let offset =OFFSET_BY_I[i];
        let rank = deck[i] >> 2;
        let suit = deck[i] & 0b11;
        ret[offset]= RANK_STRINGS[rank as usize];
        ret[offset+1]= SUIT_STRINGS[suit as usize];
    };
    return ret;
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
                                    f.write(&deal_ms_fc_board(i));
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
