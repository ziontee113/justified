use super::incoming_fragment::IncomingFragment;

pub struct State {
    fragments: Vec<IncomingFragment>,
}

impl State {
    pub fn new() -> Self {
        Self { fragments: vec![] }
    }

    pub fn add_fragment(&mut self, fragment: IncomingFragment) {
        self.fragments.push(fragment);
    }

    pub fn remove_fragment(&mut self, fragment: &IncomingFragment) {
        self.fragments.retain(|f| {
            !(f.code() == fragment.code() && f.device_alias() == fragment.device_alias())
        });
    }

    pub fn fragments(&self) -> &[IncomingFragment] {
        self.fragments.as_ref()
    }
}
