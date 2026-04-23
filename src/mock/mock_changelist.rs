use crate::{ChangeList, mock::mock_change};

#[allow(dead_code)]
pub fn mock_changelist() -> ChangeList {
    ChangeList::new(vec![
        mock_change("^feat(\n|.)*$", "Feature"),
        mock_change("^fix(\n|.)*$", "Fix"),
    ])
}
