use std::fmt;

#[derive(Debug)]
enum Colour {
    Red,
    Blue,
    Green,
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!();
        // write!(f, "({})", 1)
    }
}

trait Flying {
    fn fly(&self);
}

fn fly_flap_flap(this_duck: &Duck) {
    println!("Flap flap flap");
}

fn fly_blast_off(this_duck: &Duck) {
    println!("Blast off");
}

fn fly_cant_fly(this_duck: &Duck) {
    println!("Can't fly");
}

trait Quacking {
    fn quack(&self);
}

fn quack_vanilla(this_duck: &Duck) {
    println!("Quack!");
}

fn quack_squeak(this_duck: &Duck) {
    println!("Squeak!");
}

struct Duck {
    fly_behavior: fn(&Duck),
    quack_behavior: fn(&Duck),
    colour:Colour,
}

impl Flying for Duck {
    fn fly(&self) {
        (self.fly_behavior)(self);
    }
}

impl Quacking for Duck {
    fn quack(&self) {
        (self.quack_behavior)(self);
    }
}

impl Duck {
    fn new(fly_behavior: fn(&Duck), quack_behavior: fn(&Duck)) -> Self {
        Duck { fly_behavior, quack_behavior, colour: Colour::Red }
    }

    fn tell_colour(&self) -> ()
    {
        println!("This duck is {}", self.colour);
    }
}

fn main() {  
    let duck1: Duck = Duck::new(fly_flap_flap, quack_vanilla);
    let duck2: Duck = Duck::new(fly_blast_off, quack_vanilla);
    let duck3: Duck = Duck::new(fly_flap_flap, quack_vanilla);
    let duck4: Duck = Duck::new(fly_blast_off, quack_vanilla);
    let rubber_ducky: Duck = Duck::new(fly_cant_fly, quack_squeak);
    
    let ducks: Vec<&Duck> = vec![&duck1, &duck2, &duck3, &duck4, &rubber_ducky];

    for duck in ducks {
        duck.fly();
        duck.quack();
    }

    duck2.quack();
    duck1.tell_colour();
}