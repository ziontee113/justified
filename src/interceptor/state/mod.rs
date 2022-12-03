#[cfg(test)]
pub mod test;

use crate::{ki, units::KeyIdentifier};

use super::incoming_fragment::IncomingFragment;

pub struct State {
    fragments: Vec<IncomingFragment>,

    latest_value: i32,
    latest_key: KeyIdentifier,

    identifiers_before_key_up_event: Vec<KeyIdentifier>,
    key_up_combo_count: u16,
}

impl State {
    pub fn new() -> Self {
        Self {
            fragments: vec![],

            latest_value: -1,
            latest_key: ki!(__DEV_CLEAN 0),

            identifiers_before_key_up_event: vec![],
            key_up_combo_count: 0,
        }
    }

    pub fn receive(&mut self, fragment: &IncomingFragment) {
        if fragment.value() == 0 {
            self.identifiers_before_key_up_event = self.fragment_identifiers();
            self.remove_fragment(fragment);
            self.key_up_combo_count += 1;
        }

        if fragment.value() == 1 {
            self.add_fragment(fragment.clone());
            self.key_up_combo_count = 0;
        }

        self.latest_value = fragment.value();
        self.latest_key = fragment.key().clone();
    }

    pub fn fragments(&self) -> &[IncomingFragment] {
        self.fragments.as_ref()
    }

    pub fn latest_value(&self) -> i32 {
        self.latest_value
    }

    pub fn latest_key_identifier(&self) -> KeyIdentifier {
        self.latest_key.clone()
    }

    pub fn identifiers_before_up_event(&self) -> &[KeyIdentifier] {
        self.identifiers_before_key_up_event.as_ref()
    }

    pub fn key_up_combo_count(&self) -> u16 {
        self.key_up_combo_count
    }

    pub fn fragment_identifiers(&self) -> Vec<KeyIdentifier> {
        self.fragments().iter().map(|f| f.key().clone()).collect()
    }

    pub fn prefix_identifiers(&self) -> Vec<KeyIdentifier> {
        if self.fragments.len() > 1 {
            return self.fragments[0..self.fragments.len() - 1]
                .iter()
                .map(|f| f.key().clone())
                .collect::<Vec<KeyIdentifier>>();
        }

        self.fragment_identifiers()
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
