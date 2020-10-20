use std::env;
use rand::prelude::*;

fn weep(rng: &mut ThreadRng) {
    let rweep = [
        ";w;",
        ";",
        ";;",
        ";-;",
        ":(",
        "T-T",
        ":'‑(",
        ":'(",
        ";(",
        ":{",
        ":c",
        "D:",
        "🦀"
    ];
    println!("{}", rweep.choose(rng).unwrap());
}

fn align(person: &str, rng: &mut ThreadRng) {
    let alignment = [
        "a good person",
        "a bad person",
        "very caring",
        "an asshole",
        "a wonderful person",
        "ok... i guess...",
        "tolerable",
        "honestly not that important",
        "a pretty bad person",
        "the literal worst",
        "the best",
        "my crush"
    ];
    println!("{} was {}", person, alignment.choose(rng).unwrap());
}

fn mourn(rng: &mut ThreadRng) {
    let cause = [
        "in that car accident",
        "in that nuclear bomb that North Korea set off",
        "in that fatal computer explosion",
        "from ... wait, how did they die? oh well...",
        "from COVID",
        "in a fire",
        "while petting a kitty"
    ];
    println!("Why did they have to die {}", cause.choose(rng).unwrap());
}

fn respect(person: &str, rng: &mut ThreadRng) {
    let f = [
        "RIP",
        "F",
        "Press f for",
        "rest in piss",
        "no one liked you anyway",
        "unfortunate,",
        "take me with you"
    ];
    println!("{} {}", f.choose(rng).unwrap(), person);
}

fn funeral(person: &str) {
    let mut rng = thread_rng();

    align(person, &mut rng);
    mourn(&mut rng);
    respect(person, &mut rng);
    weep(&mut rng);
}

fn main() {
    let name = env::args().nth(1);

    match name {
        Some(name) => funeral(&name),
        None => println!("No name given")
    }
}
