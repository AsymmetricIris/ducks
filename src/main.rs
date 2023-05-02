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

fn fly_flap_flap() {
    println!("Flap flap flap");
}

fn fly_blast_off() {
    println!("Blast off");
}

fn fly_cant_fly() {
    println!("Can't fly");
}

struct Duck {
    fly_behavior: fn(&dyn Flying),
    colour:Colour,
}

impl Flying for Duck {
    fn fly(&self) {
        (self.fly_behavior)(&self);
    }
}

impl Duck {
    fn new(fly_behavior: fn(&dyn Flying)) -> Self {
        Duck { fly_behavior, colour: Colour::Red }
    }

    fn quack(&self) -> ()
    {
        println!("Quack");
    }

    fn tell_colour(&self) -> ()
    {
        println!("This duck is {}", self.colour);
    }
}

fn main() {
    let duck1 = Duck::new(fly_flap_flap);
    let duck2 = Duck::new(fly_blast_off);
    let duck3 = Duck::new(fly_flap_flap);
    let duck4 = Duck::new(fly_blast_off);
    let rubber_ducky = Duck::new(fly_cant_fly);

    let ducks: Vec<&Duck> = vec![&duck1, &duck2, &duck3, &duck4, &rubber_ducky];

    for duck in ducks {
        duck.fly();
    }

    duck2.quack();
    duck1.tell_colour();
}