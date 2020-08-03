use std::env;
use rand::prelude::*;

fn weep(rng: &mut ThreadRng) {
    let rweep = [";w;", ";", ";;", ";-;", ":(", "T-T"];

    println!("{}", rweep.choose(rng).unwrap());
}

fn funeral(person: &str) {
    let mut rng = thread_rng();
    let cause_of_death = [
        "in that car accident",
        "in that nuclear bomb that North Korea set off",
        "in that fatal computer explosion",
        "from ... wait, how did they die? oh well...",
        "from COVID",
        "in a fire"
    ];

    println!("{} was a good person", person);
    println!("Why did they have to die {}", cause_of_death.choose(&mut rng).unwrap());
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
