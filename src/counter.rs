use crate::automata::State;

pub struct Counter {
    curr: State,
}

impl Counter {
    pub fn new(start: State) -> Self {
        Self { curr: start }
    }

    pub fn next(&mut self) -> State {
        let curr = self.curr;
        self.curr = self.curr.checked_add(1).unwrap();
        curr
    }
}
