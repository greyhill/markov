extern crate rand;

use std::collections::HashMap;
use rand::random;

struct Chain {
    order: usize,
    // the key of transitions is history, i.e., the conditional state of the markov chain
    // the value of transitions is a tuple:
    //      - the first element of the tuple is a hashmap.  the first entry is destination
    //          states; the second entry is the relative probability of that transition (i.e.,
    //          a count of the number of times that transition has occurred.)
    //      - the second element is a normalizing factor: the total number of transitions from
    //          this conditional state.
    transitions: HashMap<Vec<usize>, (HashMap<usize, usize>, usize)>,
    history: Vec<usize>,
}

impl Chain {
    pub fn new(order: usize) -> Chain {
        if order == 0 {
            panic!("Chain order must be positive")
        }
        let mut chain = Chain{
            order: order,
            transitions: HashMap::<Vec<usize>, (HashMap<usize,usize>, usize)>::new(),
            history: Vec::<usize>::new(),
        };
        chain.end_sentence();
        chain
    }

    pub fn add_atom(self: &mut Self, atom: usize) -> () {
        // add transition
        if !self.transitions.contains_key(&self.history) {
            self.transitions.insert(self.history.clone(), (HashMap::<usize, usize>::new(), 0usize));
        }
        let (from_history, counts) = {
            if let Some(x) = self.transitions.get_mut(&self.history) {
                (&mut x.0, &mut x.1)
            } else {
                panic!("deep logic error");
            }
        };

        let old_count = match from_history.get(&atom) {
            Some(x) => *x,
            None => 0usize
        };
        from_history.insert(atom, old_count + 1);
        *counts = *counts + 1;

        // update history
        self.history.pop();
        self.history.insert(0, atom);
    }

    pub fn end_sentence(self: &mut Self) -> () {
        self.history.clear();
        for _ in 0 .. self.order {
            self.history.push(0usize);
        }
    }

    pub fn sample(self: &Self, state: &Vec<usize>) -> Option<usize> {
        if state.len() != self.order {
            panic!("length of state must be same as chain order");
        }
        match self.transitions.get(state) {
            Some(hcounts) => {
                let history = &hcounts.0;
                let counts = &hcounts.1;

                // sample from multinomial distribution
                let index = random::<usize>() % counts;
                let mut left = 0usize;
                for (x_dest, x_counts) in history.iter() {
                    let right = left + x_counts;
                    if (index >= left) & (index < right) {
                        return Some(*x_dest);
                    }
                    left = right;
                }
                panic!("internal state error");
            },
            None => {
                None
            }
        }
    }
}
