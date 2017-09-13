use std::env;
/*
 * Microsoft C Run-time-Library-compatible Random Number Generator
 * Copyright by Shlomi Fish, 2011.
 * Released under the MIT/X11 License
 * ( http://en.wikipedia.org/wiki/MIT_License ).
 * */

struct MSVC_Rand_Gen {
    seed: i32
}

impl MSVC_Rand_Gen {
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
    let mut randomizer = MSVC_Rand_Gen { seed: seed, };
    let num_cols = 8;

    let mut columns = vec![vec![]; num_cols];
    let mut deck = (0..4*13).into_iter().collect::<Vec<u32>>();

    let rank_strings: Vec<char> = "A23456789TJQK".chars().collect();
    let suit_strings : Vec<char> = "CDHS".chars().collect();

    randomizer.shuffle(&mut deck);

    deck.reverse();

    for i in 0 .. 52 {
        columns[i % num_cols].push(deck[i]);
    };

    return columns.iter().map(|col| {
        return format!(": {}\n", col.iter().map(|card| {
            let suit = card % 4;
            let rank = card / 4;
            return format!("{}{}",rank_strings[rank as usize], suit_strings[suit as usize])
        }).collect::<Vec<String>>().join(" "))
    }).collect::<Vec<String>>().join("")

}

fn main() {
    let mut args = env::args();

    match args.nth(1) {
        Some(x) => match x.to_string().parse::<u32>() {
            Ok(y) => print!("{}", deal_ms_fc_board(y as i32)),
            Err(E) => println!("I need a real number"),
        },
        None => println!("I need a real number"),
    }

    if false {
        let mut r = MSVC_Rand_Gen { seed: 1,};

        println!("Result={}",r.rand());
        println!("Result={}",r.rand());
        println!("Result={}",r.rand());

        /*
        let mut array: [int, ..10] = [0,1,2,3,4,5,6,7,8,9];

        let mut shuffler = MSVC_Rand_Gen { seed : 24,};

        shuffler.shuffle(array);

        for array.each |i| {
            println!("A={}", *i);
        }
        */
    }
}
