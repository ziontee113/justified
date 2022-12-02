use crate::{interceptor::state::State, rule_library::ruleset::RuleSet};

#[cfg(test)]
mod test;

fn output_to_execute(state: &State, ruleset: &RuleSet) -> Option<u16> {
    if !state.fragments().is_empty() {
        let key = state.fragments_as_key_identifiers();
        if let Some(output) = ruleset.rules().get(&key) {
            return Some(*output);
        }
    }

    None
}
