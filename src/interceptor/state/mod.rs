#[cfg(test)]
pub mod test;

use crate::units::KeyIdentifier;

use super::incoming_fragment::IncomingFragment;

pub struct State {
    fragments: Vec<IncomingFragment>,
}

impl State {
    pub fn new() -> Self {
        Self { fragments: vec![] }
    }

    pub fn receive(&mut self, fragment: &IncomingFragment) {
        if fragment.value() == 0 {
            self.remove_fragment(fragment);
        }

        if fragment.value() == 1 {
            self.add_fragment(fragment.clone());
        }
    }

    pub fn fragments(&self) -> &[IncomingFragment] {
        self.fragments.as_ref()
    }

    pub fn fragments_as_key_identifiers(&self) -> Vec<KeyIdentifier> {
        self.fragments().iter().map(|f| f.key().clone()).collect()
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
