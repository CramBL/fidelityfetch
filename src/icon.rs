use std::fmt;

mod svgs;
#[allow(clippy::wildcard_imports)]
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
    Audio,
    Binary,
    MiscConfigFile,
    Compressed,
    SystemVerilog,
    Exe,
}

impl FileTypeCategory {
    pub fn from_extension_lower(ext: &str) -> Self {
        match ext {
            "bin" => Self::Binary,
            "json" => Self::Json,
            "toml" => Self::Toml,
            "yaml" | "yml" => Self::Yaml,
            "xml" => Self::Xml,
            "md" => Self::Markdown,
            "csv" => Self::Csv,
            "rs" => Self::Rust,
            "py" => Self::Python,
            "js" => Self::JavaScript,
            "html" | "htm" => Self::Html,
            "css" => Self::Css,
            "java" => Self::Java,
            "cpp" | "hpp" | "hh" | "cc" | "c++" | "h++" => Self::Cpp,
            "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "ico" | "webp" => Self::Image,
            "svg" => Self::Svg,
            "mp4" | "avi" | "mov" | "mkv" | "webm" => Self::Video,
            "zip" | "tar" | "rar" | "7z" | "raucb" => Self::Archive,
            "pdf" => Self::Pdf,
            "sh" | "bash" | "zsh" | "fish" | "just" => Self::ShellScript,
            "iso" | "img" | "dmg" | "vhd" => Self::DiskImage,
            "wic" | "bmap" | "squashfs" => Self::FileSystemImage,
            "git" => Self::Git,
            "bb" | "bbappend" => Self::BitBake,
            "txt" => Self::Text,
            "lock" => Self::LockFile,
            "mp3" | "wav" | "flac" | "aac" | "ogg" => Self::Audio,
            "ini" => Self::MiscConfigFile,
            "bz2" | "xz" | "gz" | "lz4" => Self::Compressed,
            "sv" => Self::SystemVerilog,
            "exe" => Self::Exe,
            _ => Self::Unknown,
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            Self::Yaml => YAML_SVG,
            Self::Xml => XML_SVG,
            Self::Json => "{ }",
            Self::Toml | Self::MiscConfigFile => "⚙️",
            Self::Markdown => MARKDOWN_SVG,
            Self::Csv => "📊",
            Self::Rust => RUST_SVG,
            Self::Python => PYTHON_SVG,
            Self::JavaScript | Self::Cpp => "📜",
            Self::Html => "🌐",
            Self::Css => CSS_SVG,
            Self::Pdf => PDF_SVG,
            Self::Java => "☕",
            Self::Directory => DIR_SVG,
            Self::DirectoryEmpty => EMPTY_DIR_SVG,
            Self::Image => IMAGE_SVG,
            Self::Video => "🎥",
            Self::Archive => "📦",
            Self::Unknown => "❓",
            Self::DiskImage => "💿",
            Self::FileSystemImage => "🗃️",
            Self::ShellScript => SHELL_SCRIPT_SVG,
            Self::Git => GIT_SVG,
            Self::SymbolicLink => SYMBOLIC_LINK_SVG,
            Self::BitBake => MITTEN_SVG,
            Self::Text => TEXT_SVG,
            Self::LockFile => LOCK_FILE_SVG,
            Self::Svg => SVG_SVG,
            Self::Audio => "♫",
            Self::Binary => BINARY_FILE_SVG,
            Self::Compressed => COMPRESSED_FILE_SVG,
            Self::SystemVerilog => SYSTEMVERILOG_SVG,
            Self::Exe => EXE_SVG,
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::Json => "JSON",
            Self::Toml => "TOML",
            Self::Yaml => "YAML",
            Self::Xml => "XML",
            Self::Markdown => "Markdown",
            Self::Csv => "CSV",
            Self::Rust => "Rust",
            Self::Python => "Python",
            Self::JavaScript => "JavaScript",
            Self::Html => "HTML",
            Self::Css => "CSS",
            Self::Java => "Java",
            Self::ShellScript => "Shell Script",
            Self::Cpp => "C++",
            Self::Directory => "Directory",
            Self::DirectoryEmpty => "Directory Empty",
            Self::DiskImage => "Disk Image",
            Self::Unknown => "Unknown",
            Self::Image => "Image",
            Self::Video => "Video",
            Self::FileSystemImage => "File System Image",
            Self::Pdf => "PDF",
            Self::Archive => "Archive",
            Self::Git => "Git",
            Self::SymbolicLink => "Symbolic Link",
            Self::BitBake => "BitBake",
            Self::Text => "Text",
            Self::LockFile => "Lock-file",
            Self::Svg => "Svg",
            Self::Audio => "Audio",
            Self::Binary => "Binary",
            Self::MiscConfigFile => "Misc. Config File",
            Self::Compressed => "Compressed",
            Self::SystemVerilog => "SystemVerilog",
            Self::Exe => "Executable",
        }
    }
}

impl fmt::Display for FileTypeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
