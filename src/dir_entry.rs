use crate::icon::FileTypeCategory;
use std::fs::FileType;

#[derive(Debug)]
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
        let orig_fname = &self.name;
        let mut fname = orig_fname.clone();
        if self.ftype.is_dir() {
            fname.push('/');
        }

        let file_item = if self.ftype.is_dir() {
            "directory"
        } else {
            "file"
        };

        let actions = if self.ftype.is_dir() {
            format!(
                r#"<div class="file-actions"><a href="{fname}?zip=true" class="zip-icon" title="Download '{orig_fname}' as ZIP">📥</a></div>"#
            )
        } else {
            String::new()
        };

        format!(
            r#"
            <li class="file-item {file_item}">
                <div class="file-icon">{icon}</div>
                <div class="file-details">
                    <a href="{fname}" class="file-name">{fname}</a>
                    <div class="file-info">
                        <span class="file-date">{modified_date}</span>
                        <span class="file-size">{size}</span>
                    </div>
                </div>
                {actions}
            </li>
            "#,
            icon = self.category().icon(),
            size = self.size,
            modified_date = self.modified
        )
    }
}
