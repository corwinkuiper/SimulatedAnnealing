use anneal::Anneal;
use rand::prelude::*;
use std::cell::RefCell;

#[derive(Debug, Clone)]
struct State {
    order: Vec<Position>,
}

#[derive(Debug, Copy, Clone)]
struct Position {
    x: f64,
    y: f64,
    tag: i64,
}

struct TravellingSalesman {
    rng: RefCell<rand::rngs::ThreadRng>,
}

impl Anneal<State> for TravellingSalesman {
    fn neighbour(&self, state: &State) -> State {
        let mut rng = self.rng.borrow_mut();
        let n1 = rng.gen_range(0, state.order.len());
        let n2 = rng.gen_range(0, state.order.len());

        let mut new_state = state.clone();
        new_state.order.swap(n1, n2);

        new_state
    }

    fn random(&self) -> f64 {
        self.rng.borrow_mut().gen_range(0.0, 1.0)
    }

    fn temperature(&self, iteration: f64, energy: f64) -> f64 {
        100.0 * std::f64::consts::E.powf(-12.0 * iteration) * energy
    }

    fn energy(&self, state: &State) -> f64 {
        let mut energy = 0.0;
        for i in 0..(state.order.len()) {
            let index = i;
            let index_next = (i + 1) % state.order.len();

            energy += (state.order[index].x - state.order[index_next].x).powi(2)
                + (state.order[index].y - state.order[index_next].y).powi(2);
        }

        energy.sqrt()
    }
}

const NUM_ITERATIONS: i64 = 1_000;
const NUM_POINTS: i64 = 10;

fn main() {
    // generate a circle of initial positions
    let radius = 100.0;
    let mut positions: Vec<Position> = vec![];

    for i in 0..NUM_POINTS {
        let x = radius * (std::f64::consts::PI * 2.0 * (i as f64) / (NUM_POINTS as f64)).cos();
        let y = radius * (std::f64::consts::PI * 2.0 * (i as f64) / (NUM_POINTS as f64)).sin();
        positions.push(Position { x, y, tag: i });
    }

    positions.shuffle(&mut thread_rng());

    let initial_state = State { order: positions };
    let travelling_salesman = TravellingSalesman {
        rng: RefCell::new(thread_rng()),
    };

    let final_state = travelling_salesman.optimise(NUM_ITERATIONS, initial_state);

    // will now check if optimal solution was found, as optimal solution in this case is accending / decending order of the tag due to being a circle
    let mut success = true;
    for i in 0..(final_state.order.len()) {
        let index = i;
        let index_next = (i + 1) % final_state.order.len();

        let n1 = final_state.order[index].tag;
        let n2 = final_state.order[index_next].tag;

        if !((n1 + 1) % NUM_POINTS == n2 || (n1 - 1).rem_euclid(NUM_POINTS) == n2) {
            println!("{}:{}", n1, n2);
            success = false;
            break;
        }
    }
    if success {
        println!("Optimal solution was found!");
    } else {
        println!("Optimal solution was not found :(");
    }
}
