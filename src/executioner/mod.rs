use crate::{
    interceptor::{incoming_fragment::KeyState, state::State},
    rule_library::ruleset::RuleSet,
};

#[cfg(test)]
mod test;

fn ruleset_output_to_execute(state: &mut State, ruleset: &RuleSet) -> Option<u16> {
    if state.latest_value() == KeyState::Down {
        return handle_keystate_down(state, ruleset);
    }
    if state.latest_value() == KeyState::Up {
        return handle_keystate_up(state, ruleset);
    }
    None
}

fn handle_keystate_down(state: &mut State, ruleset: &RuleSet) -> Option<u16> {
    if state.latest_value() == KeyState::Down
        && ruleset.prefixes().contains(&state.fragment_identifiers())
    {
        return None;
    }

    if ruleset.prefixes().contains(&state.modifier_identifiers())
        && state.modifier_identifiers() == state.fragment_identifiers()
    {
        return None;
    }

    return ruleset.rules().get(&state.fragment_identifiers()).copied();
}

fn handle_keystate_up(state: &mut State, ruleset: &RuleSet) -> Option<u16> {
    let identifiers_before_up_event = state.identifiers_before_up_event();

    if identifiers_before_up_event.len() == state.key_down_combo_count().into()
        && ruleset.prefixes().contains(identifiers_before_up_event)
    {
        return ruleset.rules().get(identifiers_before_up_event).copied();
    }

    None
}
