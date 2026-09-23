use egui_code_editor::{Patch, Syntax};
use std::collections::BTreeSet;


pub const DEFAULT_QUOTES: [char; 3] = ['\'', '"', '`'];

pub fn rva() -> Syntax {
    Syntax {
        language: "RVA",
        case_sensitive: false,
        comment: ";",
        comment_multiline: ["/*", "*/"],
        quotes: DEFAULT_QUOTES.into(),
        word_start: BTreeSet::new(),
        hyperlinks: BTreeSet::from(["http:", "https:", "www.", "ftp:", "file:"]),
        keywords: BTreeSet::from([
            ""
        ]),
        types: BTreeSet::from([
            "db", "dw"
        ]),
        special: BTreeSet::from([
            "r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
        ]),
        patch: Patch::default(),
    }
}
