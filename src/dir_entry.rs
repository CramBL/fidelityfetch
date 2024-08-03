use std::fs::FileType;

use crate::{icon::FileTypeCategory, util::generate_list_item};

pub struct FifeDirEntry {
    pub name: String,
    pub ftype: FileType,
    size: String,
    category: FileTypeCategory,
    modified: String,
}

impl FifeDirEntry {
    pub fn new(
        name: String,
        ftype: FileType,
        size: String,
        category: FileTypeCategory,
        modified: String,
    ) -> Self {
        Self {
            name,
            ftype,
            size,
            category,
            modified,
        }
    }

    pub fn category(&self) -> FileTypeCategory {
        self.category
    }

    pub fn to_html(self) -> String {
        generate_list_item(
            &self.name,
            self.ftype,
            &self.size,
            &self.modified,
            self.category,
        )
    }
}
