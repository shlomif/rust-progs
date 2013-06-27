/*
 * String concatenation in Rust.
 * Copyright by Shlomi Fish, 2013.
 * Released under the MIT/X11 License
 * ( http://en.wikipedia.org/wiki/MIT_License ).
 * */

fn main() {
    let s = ~"hello";
    println(fmt!("s=%s", s + " world"));

    let s1 = s + ~" world";
    println(fmt!("s1=%s", s1));

    let mut mutable_s = ~"hello";
    mutable_s += ~" world";
    println(fmt!("mutable_s=%s", mutable_s));
}
