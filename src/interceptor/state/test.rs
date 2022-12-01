#[cfg(test)]
mod state_test {
    use std::time::SystemTime;

    use crate::{
        interceptor::{incoming_fragment::IncomingFragment, state::State},
        utils::mipoch,
    };

    fn fragment_has_contents(
        fragment: &IncomingFragment,
        device_alias: &str,
        code: u16,
        value: i32,
        timestamp: SystemTime,
    ) {
        assert_eq!(fragment.device_alias(), device_alias);
        assert_eq!(fragment.code(), code);
        assert_eq!(fragment.value(), value);
        assert_eq!(fragment.timestamp(), timestamp);
    }

    #[test]
    fn can_add_fragment_to_state() {
        let mut state = State::new();
        let fragment = IncomingFragment::new("L1", 32, 1, mipoch(0));

        state.add_fragment(fragment);
        fragment_has_contents(state.fragments().get(0).unwrap(), "L1", 32, 1, mipoch(0));
    }

    #[test]
    fn can_remove_fragment_base_on_alias_and_code() {
        let mut state = State::new();

        let l1_32_down = IncomingFragment::new("L1", 32, 1, mipoch(0));
        let l1_33_down = IncomingFragment::new("L1", 33, 1, mipoch(10));
        let l1_34_down = IncomingFragment::new("L1", 34, 1, mipoch(15));
        state.add_fragment(l1_32_down);
        state.add_fragment(l1_33_down);
        state.add_fragment(l1_34_down);

        let l1_32_up = IncomingFragment::new("L1", 32, 0, mipoch(40));
        state.remove_fragment(&l1_32_up);

        assert_eq!(state.fragments().len(), 2);
        fragment_has_contents(state.fragments().get(0).unwrap(), "L1", 33, 1, mipoch(10));
        fragment_has_contents(state.fragments().get(1).unwrap(), "L1", 34, 1, mipoch(15));

        let l1_33_up = IncomingFragment::new("L1", 33, 0, mipoch(50));
        state.remove_fragment(&l1_33_up);

        assert_eq!(state.fragments().len(), 1);
        fragment_has_contents(state.fragments().get(0).unwrap(), "L1", 34, 1, mipoch(15));

        let l1_34_up = IncomingFragment::new("L1", 34, 0, mipoch(60));
        state.remove_fragment(&l1_34_up);

        assert_eq!(state.fragments().len(), 0);
    }

    #[test]
    fn can_add_or_remove_fragment_base_on_incoming_fragment_value() {
        let mut state = State::new();
        let l1_32_down = IncomingFragment::new("L1", 32, 1, mipoch(0));

        state.receive(&l1_32_down);
    }
}
