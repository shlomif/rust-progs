/*
 * String concatenation in Rust.
 * Copyright by Shlomi Fish, 2013.
 * Released under the MIT/X11 License
 * ( http://en.wikipedia.org/wiki/MIT_License ).
 * */

fn main() {
    let s = "hello";
    println!("s={}", s.to_owned() + " world");

    let s1 = s.to_owned() + " world";
    println!("s1={}", s1);

    let mut mutable_s = "hello".to_owned();
    mutable_s += " world";
    println!("mutable_s={}", mutable_s);
}
