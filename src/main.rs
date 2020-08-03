use std::env;
use rand::prelude::*;

fn weep(rng: &mut ThreadRng) {
    let rweep = [";w;", ";", ";;", ";-;", ":(", "T-T"];

    println!("{}", rweep.choose(rng).unwrap());
}

fn funeral(person: &str) {
    let mut rng = thread_rng();
    let cause_of_death = [
        "car accident",
        "nuclear bomb that North Korea set off",
        "fatal computer explosion",
        " ... wait, how did they die? oh well...",
    ];

    println!("{} was a good person", person);
    println!("Why did they have to die in that {}", cause_of_death.choose(&mut rng).unwrap());
    println!("RIP {}", person);
    weep(&mut rng);
}

fn main() {
    let name = env::args().nth(1);

    match name {
        Some(name) => funeral(&name),
        None => println!("No name given")
    }
}
