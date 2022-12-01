#[cfg(test)]
mod test;

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
