use crate::{interceptor::state::State, rule_library::ruleset::RuleSet};

#[cfg(test)]
mod test;

fn ruleset_output_to_execute(state: &State, ruleset: &RuleSet) -> Option<u16> {
    let prefix = state.prefix();

    if state.latest_value() == 1 && ruleset.prefixes().contains(&prefix) {
        return None;
    }

    if state.latest_value() == 0 && state.fragments().is_empty() {
        let key = vec![state.latest_key()];
        if ruleset.prefixes().contains(&key) {
            return ruleset.rules().get(&key).copied();
        }
    }

    let key = state.fragments_as_key_identifiers();
    return ruleset.rules().get(&key).copied();
}
