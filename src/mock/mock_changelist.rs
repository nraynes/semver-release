#[cfg(test)]
pub mod changelist {
    use crate::{ChangeList, tests::mock};

    pub fn create() -> ChangeList {
        ChangeList::new(vec![
            mock::change::create("^feat(\n|.)*$", "Feature"),
            mock::change::create("^fix(\n|.)*$", "Fix"),
        ])
    }
}
