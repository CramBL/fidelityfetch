use crate::icon::FileTypeCategory;
use std::fs::FileType;

/// A directory entry with metadata
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
        let fname = if self.ftype.is_dir() {
            format!("{}/", self.name)
        } else {
            self.name.to_owned()
        };

        let file_item = if self.ftype.is_dir() {
            "directory"
        } else {
            "file"
        };

        format!(
            r#"
            <li class="file-item {file_item}">
                <div class="file-icon">{icon}</div>
                <div class="file-details">
                    <a href="{fname}" class="file-name">{fname}</a>
                    <div class="file-info">
                        <span class="file-size">{size}</span>
                        <span class="file-date">{modified_date}</span>
                    </div>
                </div>
            </li>
            "#,
            icon = self.category().icon(),
            size = self.size,
            modified_date = self.modified
        )
    }
}
