#[cfg(test)]
pub mod changelist {
    use crate::{ChangeList, tests::mock};

    pub fn major() -> ChangeList {
        ChangeList::new(vec![mock::change::create(
            "^(.|\n)*BREAKING CHANGE(.|\n)*$",
            "BREAKING CHANGES",
        )])
    }

    pub fn minor() -> ChangeList {
        ChangeList::new(vec![mock::change::create("^feat(\n|.)*$", "Feature")])
    }

    pub fn patch() -> ChangeList {
        ChangeList::new(vec![mock::change::create("^fix(\n|.)*$", "Fix")])
    }

    pub fn other() -> ChangeList {
        ChangeList::new(vec![
            mock::change::create("^chore(\n|.)*$", "Maintenance Items"),
            mock::change::create("^docs(\n|.)*$", "Documentation"),
        ])
    }

    pub fn create() -> ChangeList {
        ChangeList::new(vec![
            mock::change::create("^feat(\n|.)*$", "Feature"),
            mock::change::create("^fix(\n|.)*$", "Fix"),
        ])
    }
}
