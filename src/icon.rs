use std::fmt;

mod svgs;
use svgs::*;

#[derive(Debug, Clone, Copy)]
pub enum FileTypeCategory {
    Json,
    Toml,
    Yaml,
    Xml,
    Markdown,
    Csv,
    Rust,
    Python,
    JavaScript,
    Html,
    Css,
    Java,
    Cpp,
    Directory,
    DirectoryEmpty,
    Unknown,
    Image,
    Video,
    Archive,
    DiskImage,
    ShellScript,
    SymbolicLink,
    FileSystemImage,
    Pdf,
    Git,
    BitBake,
    Text,
    LockFile,
    Svg,
}

impl FileTypeCategory {
    pub fn from_extension(ext: &str) -> Self {
        match ext {
            "json" => FileTypeCategory::Json,
            "toml" => FileTypeCategory::Toml,
            "yaml" | "yml" => FileTypeCategory::Yaml,
            "xml" => FileTypeCategory::Xml,
            "md" => FileTypeCategory::Markdown,
            "csv" => FileTypeCategory::Csv,
            "rs" => FileTypeCategory::Rust,
            "py" => FileTypeCategory::Python,
            "js" => FileTypeCategory::JavaScript,
            "html" => FileTypeCategory::Html,
            "css" => FileTypeCategory::Css,
            "java" => FileTypeCategory::Java,
            "cpp" | "h" => FileTypeCategory::Cpp,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "ico" | "webp" => {
                FileTypeCategory::Image
            }
            "svg" => FileTypeCategory::Svg,
            "mp4" | "avi" | "mov" | "mkv" | "webm" => FileTypeCategory::Video,
            "bz2" | "zip" | "tar" | "gz" | "rar" | "7z" | "raucb" => FileTypeCategory::Archive,
            "pdf" => FileTypeCategory::Pdf,
            "sh" | "bash" | "zsh" | "fish" | "just" => FileTypeCategory::ShellScript,
            "iso" | "img" | "dmg" | "vhd" => FileTypeCategory::DiskImage,
            "wic" | "bmap" | "squashfs" => FileTypeCategory::FileSystemImage,
            "git" => FileTypeCategory::Git,
            "bb" | "bbappend" => FileTypeCategory::BitBake,
            "txt" => FileTypeCategory::Text,
            "lock" => FileTypeCategory::LockFile,
            _ => FileTypeCategory::Unknown,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            FileTypeCategory::Yaml => YAML_SVG,
            FileTypeCategory::Xml => XML_SVG,
            FileTypeCategory::Json => "{ }",
            FileTypeCategory::Toml => "⚙️",
            FileTypeCategory::Markdown => MARKDOWN_SVG,
            FileTypeCategory::Csv => "📊",
            FileTypeCategory::Rust => RUST_SVG,
            FileTypeCategory::Python => PYTHON_SVG,
            FileTypeCategory::JavaScript => "📜",
            FileTypeCategory::Html => "🌐",
            FileTypeCategory::Css => CSS_SVG,
            FileTypeCategory::Pdf => PDF_SVG,
            FileTypeCategory::Java => "☕",
            FileTypeCategory::Cpp => "📜",
            FileTypeCategory::Directory => DIR_SVG,
            FileTypeCategory::DirectoryEmpty => EMPTY_DIR_SVG,
            FileTypeCategory::Image => IMAGE_SVG,
            FileTypeCategory::Video => "🎥",
            FileTypeCategory::Archive => "📦",
            FileTypeCategory::Unknown => "❓",
            FileTypeCategory::DiskImage => "💿",
            FileTypeCategory::FileSystemImage => "🗃️",
            FileTypeCategory::ShellScript => SHELL_SCRIPT_SVG,
            FileTypeCategory::Git => GIT_SVG,
            FileTypeCategory::SymbolicLink => SYMBOLIC_LINK_SVG,
            FileTypeCategory::BitBake => MITTEN_SVG,
            FileTypeCategory::Text => TEXT_SVG,
            FileTypeCategory::LockFile => LOCK_FILE_SVG,
            FileTypeCategory::Svg => SVG_SVG,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            FileTypeCategory::Json => "JSON",
            FileTypeCategory::Toml => "TOML",
            FileTypeCategory::Yaml => "YAML",
            FileTypeCategory::Xml => "XML",
            FileTypeCategory::Markdown => "Markdown",
            FileTypeCategory::Csv => "CSV",
            FileTypeCategory::Rust => "Rust",
            FileTypeCategory::Python => "Python",
            FileTypeCategory::JavaScript => "JavaScript",
            FileTypeCategory::Html => "HTML",
            FileTypeCategory::Css => "CSS",
            FileTypeCategory::Java => "Java",
            FileTypeCategory::ShellScript => "Shell Script",
            FileTypeCategory::Cpp => "C++",
            FileTypeCategory::Directory => "Directory",
            FileTypeCategory::DirectoryEmpty => "Directory Empty",
            FileTypeCategory::DiskImage => "Disk Image",
            FileTypeCategory::Unknown => "Unknown",
            FileTypeCategory::Image => "Image",
            FileTypeCategory::Video => "Video",
            FileTypeCategory::FileSystemImage => "File System Image",
            FileTypeCategory::Pdf => "PDF",
            FileTypeCategory::Archive => "Archive",
            FileTypeCategory::Git => "Git",
            FileTypeCategory::SymbolicLink => "Symbolic Link",
            FileTypeCategory::BitBake => "BitBake",
            FileTypeCategory::Text => "Text",
            FileTypeCategory::LockFile => "Lock-file",
            FileTypeCategory::Svg => "Svg",
        }
    }
}

impl fmt::Display for FileTypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
