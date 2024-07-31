use crate::states::MarioState;
use crate::states::MarioState::*;
use crate::transitions::MarioTransition;
use crate::transitions::MarioTransition::*;

mod states;
mod transitions;

struct Mario {
    state: MarioState,
}

impl Mario {
    fn new() -> Mario {
        Mario { state: Mario }
    }

    fn collect(&mut self, transition: MarioTransition) {
        match (&self.state, transition) {
            (Dead, _) => (),
            (Mario, Damage) => self.state = Dead,
            (SuperMario, Damage) => self.state = Mario,
            (FireMario | CapeMario, Damage) => self.state = SuperMario,
            (Mario, Mushroom) => self.state = SuperMario,
            (_, Flower) => self.state = FireMario,
            (_, Feather) => self.state = CapeMario,
            (_, Mushroom) => (),
        }
    }
}

fn main() {
    println!("Hello, world!");
    let mut mario: Mario = Mario::new();
    mario.collect(Mushroom);
    mario.collect(Flower);
    mario.collect(Feather);
    mario.collect(Damage);
    mario.collect(Damage);
    mario.collect(Flower);
    for _ in 0..3 {
        mario.collect(Damage);
    }
    mario.collect(Flower);
    assert!(mario.state == Dead);
}
