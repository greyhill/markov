use std::collections::HashMap;

struct Chain {
    order: usize,
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
}
