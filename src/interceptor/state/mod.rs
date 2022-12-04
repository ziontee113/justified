#[cfg(test)]
pub mod test;

use crate::units::key_identifier::KeyIdentifier;

use super::incoming_fragment::{IncomingFragment, KeyState};

pub struct State {
    sequence: Vec<IncomingFragment>,

    latest_value: KeyState,
    latest_key: Option<KeyIdentifier>,

    identifiers_before_key_up_event: Vec<KeyIdentifier>,

    key_down_combo_count: u16,
    key_down_counter: u16,
}

impl State {
    pub fn new() -> Self {
        Self {
            sequence: vec![],

            latest_value: KeyState::Uninitiated,
            latest_key: None,

            identifiers_before_key_up_event: vec![],

            key_down_counter: 0,
            key_down_combo_count: 0,
        }
    }

    pub fn receive(&mut self, fragment: &IncomingFragment) {
        if fragment.value() == KeyState::Up {
            self.identifiers_before_key_up_event = self.sequence_identifiers();
            self.remove_fragment(fragment);

            self.key_down_counter -= 1;
        }

        if fragment.value() == KeyState::Down {
            self.add_fragment(fragment.clone());

            self.key_down_counter += 1;
            self.key_down_combo_count = self.key_down_counter;
        }

        self.latest_value = fragment.value();
        self.latest_key = Some(fragment.key().clone());
    }

    pub fn sequence(&self) -> &[IncomingFragment] {
        self.sequence.as_ref()
    }

    pub fn latest_value(&self) -> KeyState {
        self.latest_value
    }

    pub fn latest_key_identifier(&self) -> Option<KeyIdentifier> {
        self.latest_key.clone()
    }

    pub fn sequence_identifiers_before_key_up_event(&self) -> &[KeyIdentifier] {
        self.identifiers_before_key_up_event.as_ref()
    }

    pub fn key_down_combo_count(&self) -> u16 {
        self.key_down_combo_count
    }

    pub fn sequence_identifiers(&self) -> Vec<KeyIdentifier> {
        self.sequence().iter().map(|f| f.key().clone()).collect()
    }

    pub fn modifier_identifiers(&self) -> Vec<KeyIdentifier> {
        if self.sequence.len() > 1 {
            return self.sequence[0..self.sequence.len() - 1]
                .iter()
                .map(|f| f.key().clone())
                .collect::<Vec<KeyIdentifier>>();
        }

        self.sequence_identifiers()
    }
}

impl State {
    fn add_fragment(&mut self, fragment: IncomingFragment) {
        self.sequence.push(fragment);
    }

    fn remove_fragment(&mut self, incoming_fragment: &IncomingFragment) {
        self.sequence.retain(|f| !f.has_same_key(incoming_fragment));
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.sequence
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
