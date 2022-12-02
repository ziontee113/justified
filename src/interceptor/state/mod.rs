#[cfg(test)]
pub mod test;

use crate::units::KeyIdentifier;

use super::incoming_fragment::IncomingFragment;

#[derive(Debug, PartialEq, Eq)]
pub enum LatestUpDown {
    Down,
    Up,
}

pub struct State {
    fragments: Vec<IncomingFragment>,
    latest_up_down: LatestUpDown,
}

impl State {
    pub fn new() -> Self {
        Self {
            fragments: vec![],
            latest_up_down: LatestUpDown::Up,
        }
    }

    pub fn receive(&mut self, fragment: &IncomingFragment) {
        if fragment.value() == 0 {
            self.remove_fragment(fragment);
            self.latest_up_down = LatestUpDown::Up;
        }

        if fragment.value() == 1 {
            self.add_fragment(fragment.clone());
            self.latest_up_down = LatestUpDown::Down;
        }
    }

    pub fn fragments(&self) -> &[IncomingFragment] {
        self.fragments.as_ref()
    }

    pub fn latest_up_down(&self) -> &LatestUpDown {
        &self.latest_up_down
    }

    pub fn fragments_as_key_identifiers(&self) -> Vec<KeyIdentifier> {
        self.fragments().iter().map(|f| f.key().clone()).collect()
    }

    pub fn prefix(&self) -> Vec<KeyIdentifier> {
        if self.fragments.len() > 1 {
            return self.fragments[0..self.fragments.len() - 1]
                .iter()
                .map(|f| f.key().clone())
                .collect::<Vec<KeyIdentifier>>();
        }

        self.fragments_as_key_identifiers()
    }
}

impl State {
    fn add_fragment(&mut self, fragment: IncomingFragment) {
        self.fragments.push(fragment);
    }

    fn remove_fragment(&mut self, incoming_fragment: &IncomingFragment) {
        self.fragments
            .retain(|f| !f.has_same_key(incoming_fragment));
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.fragments
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
