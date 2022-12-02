use crate::{interceptor::state::State, rule_library::ruleset::RuleSet};

#[cfg(test)]
mod test;

fn ruleset_output_to_execute(state: &State, ruleset: &RuleSet) -> Option<u16> {
    if !state.fragments().is_empty() {
        let key = state.fragments_as_key_identifiers();
        return ruleset.rules().get(&key).copied();
    }

    None
}
