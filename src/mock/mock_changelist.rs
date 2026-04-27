#[cfg(test)]
pub mod changelist {
    use crate::{ChangeList, tests::mock};

    pub fn major() -> ChangeList {
        ChangeList::new(vec![mock::change::create(
            "^(.|\n)*BREAKING CHANGE(.|\n)*$",
            "BREAKING CHANGES",
            1,
        )])
    }

    pub fn minor() -> ChangeList {
        ChangeList::new(vec![mock::change::create("^feat(\n|.)*$", "Feature", 2)])
    }

    pub fn patch() -> ChangeList {
        ChangeList::new(vec![mock::change::create("^fix(\n|.)*$", "Fix", 3)])
    }

    pub fn other() -> ChangeList {
        ChangeList::new(vec![
            mock::change::create("^chore(\n|.)*$", "Maintenance Items", 4),
            mock::change::create("^docs(\n|.)*$", "Documentation", 5),
        ])
    }

    pub fn create() -> ChangeList {
        ChangeList::new(vec![
            mock::change::create("^feat(\n|.)*$", "Feature", 2),
            mock::change::create("^fix(\n|.)*$", "Fix", 3),
        ])
    }
}
