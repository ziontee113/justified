use crate::{
    interceptor::{incoming_fragment::KeyState, state::State},
    rule_library::ruleset::RuleSet,
};

#[cfg(test)]
mod test;

fn ruleset_output_to_execute(state: &mut State, ruleset: &RuleSet) -> Option<u16> {
    if state.latest_value() == KeyState::Down || state.latest_value() == KeyState::Hold {
        return handle_keystate_down_or_hold(state, ruleset);
    }
    if state.latest_value() == KeyState::Up {
        return handle_keystate_up(state, ruleset);
    }
    None
}

fn handle_keystate_down_or_hold(state: &mut State, ruleset: &RuleSet) -> Option<u16> {
    if state.sequence().len() == 1
        && !ruleset.prefixes().contains(&state.sequence_identifiers())
        && !ruleset.rules().contains_key(&state.sequence_identifiers())
    {
        return Some(state.sequence().get(0).unwrap().key().code());
    }

    if state.latest_value() == KeyState::Down
        && ruleset.prefixes().contains(&state.sequence_identifiers())
    {
        return None;
    }

    if ruleset.prefixes().contains(&state.modifier_identifiers())
        && state.modifier_identifiers() == state.sequence_identifiers()
    {
        return None;
    }

    return ruleset.rules().get(&state.sequence_identifiers()).copied();
}

fn handle_keystate_up(state: &mut State, ruleset: &RuleSet) -> Option<u16> {
    let before_up = state.sequence_identifiers_before_key_up_event();

    if before_up.len() == state.key_down_combo_count().into()
        && ruleset.prefixes().contains(before_up)
    {
        return ruleset.rules().get(before_up).copied();
    }

    None
}
