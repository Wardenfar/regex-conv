use std::{
    collections::HashSet,
    fmt::{Display, Write},
};

pub type State = u32;

#[derive(Debug, Clone)]
pub struct Automata<T> {
    pub initial_states: HashSet<State>,
    pub accept_states: HashSet<State>,
    pub links: Vec<Link<T>>,
}

#[derive(Debug, Clone)]
pub struct Link<T> {
    pub from: State,
    pub symbol: T,
    pub to: State,
}

pub type Nfa<T> = Automata<MaybeSymbol<T>>;
pub type Dfa<T> = Automata<T>;

pub enum MaybeSymbol<T> {
    Symbol(T),
    Epsilon,
}

impl<T: Display> Display for MaybeSymbol<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MaybeSymbol::Symbol(it) => Display::fmt(it, f),
            MaybeSymbol::Epsilon => f.write_char('ε'),
        }
    }
}

impl<T> Dfa<T> {
    pub fn to_nfa(&self) -> Nfa<T>
    where
        T: Clone,
    {
        Nfa {
            initial_states: self.initial_states.clone(),
            accept_states: self.accept_states.clone(),
            links: self
                .links
                .iter()
                .map(|link| Link {
                    from: link.from,
                    to: link.to,
                    symbol: MaybeSymbol::Symbol(link.symbol.clone()),
                })
                .collect(),
        }
    }
}

impl<T> Automata<T> {
    pub fn new() -> Self {
        Self {
            initial_states: Default::default(),
            accept_states: Default::default(),
            links: Default::default(),
        }
    }

    pub fn invert(&mut self) {
        std::mem::swap(&mut self.accept_states, &mut self.initial_states);
        for link in self.links.iter_mut() {
            std::mem::swap(&mut link.from, &mut link.to);
        }
    }

    pub fn links_from(&self, from: State) -> impl Iterator<Item = &Link<T>> {
        self.links.iter().filter(move |link| link.from == from)
    }

    pub fn links_to(&self, to: State) -> impl Iterator<Item = &Link<T>> {
        self.links.iter().filter(move |link| link.to == to)
    }

    pub fn links_from_to(&self, from: State, to: State) -> impl Iterator<Item = &Link<T>> {
        self.links
            .iter()
            .filter(move |link| link.from == from && link.to == to)
    }

    pub fn remove_links(&mut self, from: State, to: State) {
        self.links
            .retain(|link| !(link.from == from && link.to == to))
    }

    pub fn remove_links_any(&mut self, from_ot_to: State) {
        self.links
            .retain(|link| !(link.from == from_ot_to || link.to == from_ot_to))
    }

    pub fn max_state(&self) -> State {
        self.all_states_iter().max().unwrap_or_default()
    }

    pub fn all_states(&self) -> HashSet<State> {
        self.all_states_iter().collect()
    }

    fn all_states_iter(&self) -> impl Iterator<Item = State> + '_ {
        self.initial_states
            .iter()
            .chain(self.accept_states.iter())
            .chain(self.links.iter().map(|l| &l.from))
            .chain(self.links.iter().map(|l| &l.to))
            .copied()
    }

    pub fn link(&mut self, from: State, to: State, symbol: T) {
        self.links.push(Link {
            from,
            symbol: symbol,
            to,
        })
    }
}
