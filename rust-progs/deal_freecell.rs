/*
 * Microsoft C Run-time-Library-compatible Random Number Generator
 * Copyright by Shlomi Fish, 2011.
 * Released under the MIT/X11 License
 * ( http://en.wikipedia.org/wiki/MIT_License ).
 * */

struct MSRand {
    seed: i32
}

impl MSRand {
    fn rand(&mut self) -> i32 {
        self.seed = ((self.seed * 214013i32 + 2531011i32) & 0x7FFFFFFFi32);
        return ((self.seed >> 16i32) & 0x7FFFi32);
    }
    fn max_rand(&mut self, mymax: i32) -> i32 {
        return (self.rand() % mymax);
    }
    fn shuffle<T>(&mut self, deck: &mut [T]) {
        if (deck.len() > 0) {
            let mut i = (deck.len() as i32) - 1;
            while (i > 0) {
                let j = self.max_rand(i+1i32);
                vec::swap(deck, i as uint, j as uint);
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

fn deal_ms_fc_board(seed: i32) -> ~str {
    let mut randomizer = MSRand { seed: seed, };
    let num_cols = 8;

    let _perl_range = |start: uint, end: uint| {
        let ret = do vec::build |push| {
            for uint::range(start, end+1) |i| { push(i); }
        };
        ret
    };

    let mut columns = _perl_range(0, num_cols-1).map(|i| { ~[] });
    let mut deck = _perl_range(0, 4*13-1);

    randomizer.shuffle(deck);

    vec::reverse(deck);

    for uint::range(0, 52) |i| {
        columns[i % num_cols].push(deck[i]);
    };

    let render_card = |card_i: uint| {
        let card = card_i;
        let suit = card % 4;
        let rank = card / 4;

        // println(fmt!("card=%i,suit=%i,rank=%i", card as int, suit as int, rank as int));
        fmt!("%c%c", "A23456789TJQK"[rank] as char, "CDHS"[suit] as char)
    };

    let render_column = |col: &~[uint]| {
        fmt!(": %s\n", str::connect((col.map(|i| { render_card( *i ) })), " "))
    };

    return str::connect(columns.map(render_column), &"");
}

fn main() {
    print(deal_ms_fc_board(24));

    if (false) {
        let mut r = MSRand { seed: 1,};

        println(fmt!("Result=%i",r.rand() as int));
        println(fmt!("Result=%i",r.rand() as int));
        println(fmt!("Result=%i",r.rand() as int));

        let mut array: [int, ..10] = [0,1,2,3,4,5,6,7,8,9];

        let mut shuffler = MSRand { seed : 24,};

        shuffler.shuffle(array);

        for array.each |i| {
            println(fmt!("A=%i", *i));
        }
    }
}
