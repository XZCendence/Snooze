use egui_code_editor::Syntax;
use std::collections::BTreeSet;

pub fn json_syntax() -> Syntax {
    Syntax {
        language: "json",
        case_sensitive: true,
        // json doesn't support comments
        comment: "",
        comment_multiline: ["", ""],
        hyperlinks: BTreeSet::new(),
        // only json literals
        keywords: BTreeSet::from(["true", "false", "null"]),
        types: BTreeSet::new(),
        special: BTreeSet::new(),
    }
}
