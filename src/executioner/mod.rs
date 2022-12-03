use crate::{interceptor::state::State, rule_library::ruleset::RuleSet};

#[cfg(test)]
mod test;

fn ruleset_output_to_execute(state: &State, ruleset: &RuleSet) -> Option<u16> {
    if state.latest_value() == 1
        && ruleset.prefixes().contains(&state.prefix_identifiers())
        && state.prefix_identifiers() == state.fragment_identifiers()
    {
        return None;
    }

    if state.latest_value() == 0 {
        if state.identifiers_before_up_event().len() >= 2 {
            return None;
        }

        if state.key_up_combo_count() >= 2 {
            return None;
        }

        if state.key_up_combo_count() == 1 {
            let key = vec![state.latest_key_identifier()];

            if ruleset.prefixes().contains(&key) {
                return ruleset.rules().get(&key).copied();
            }
        }
    }

    let key = state.fragment_identifiers();
    return ruleset.rules().get(&key).copied();
}
