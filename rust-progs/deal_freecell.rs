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

fn main() {
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
