macro_rules! svg {
    ($path:expr) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/svg/", $path))
    };
}

pub const EMPTY_DIR_SVG: &str = svg!("empty_dir.svg");
pub const DIR_SVG: &str = svg!("dir.svg");
pub const PDF_SVG: &str = svg!("pdf.svg");
pub const MARKDOWN_SVG: &str = svg!("markdown.svg");
pub const SHELL_SCRIPT_SVG: &str = svg!("shell_script.svg");
pub const YAML_SVG: &str = svg!("yaml.svg");
pub const XML_SVG: &str = svg!("xml.svg");
pub const IMAGE_SVG: &str = svg!("image.svg");
pub const GIT_SVG: &str = svg!("git.svg");
pub const SYMBOLIC_LINK_SVG: &str = svg!("symbolic_link.svg");
pub const MITTEN_SVG: &str = svg!("mitten.svg");
pub const CSS_SVG: &str = svg!("css.svg");
pub const TEXT_SVG: &str = svg!("text.svg");
pub const LOCK_FILE_SVG: &str = svg!("lock_file.svg");
pub const PYTHON_SVG: &str = svg!("python.svg");
pub const RUST_SVG: &str = svg!("rust.svg");
pub const SVG_SVG: &str = svg!("svg.svg");
pub const BINARY_FILE_SVG: &str = svg!("binary_file.svg");
pub const COMPRESSED_FILE_SVG: &str = svg!("compressed_file.svg");
pub const SYSTEMVERILOG_SVG: &str = svg!("systemverilog.svg");
pub const EXE_SVG: &str = svg!("exe.svg");
pub const ARCHIVE_SVG: &str = svg!("archive.svg");
pub const FILE_SYSTEM_IMAGE_SVG: &str = svg!("file_system_image.svg");
pub const DISK_IMAGE_SVG: &str = svg!("disk_image.svg");
pub const UNKNOWN_FILE_SVG: &str = svg!("unknown_file.svg");
pub const VIDEO_SVG: &str = svg!("video.svg");
pub const AUDIO_SVG: &str = svg!("audio.svg");
pub const TOML_SVG: &str = svg!("toml.svg");
pub const CONFIG_SVG: &str = svg!("config.svg");
pub const JAVASCRIPT_SVG: &str = svg!("javascript.svg");
pub const JSON_SVG: &str = svg!("json.svg");
pub const JAVA_SVG: &str = svg!("java.svg");
pub const CPP_SVG: &str = svg!("cpp.svg");
pub const HTML_SVG: &str = svg!("html.svg");
pub const CSV_SVG: &str = svg!("csv.svg");
pub const TYPESCRIPT_SVG: &str = svg!("typescript.svg");
pub const HDF5_SVG: &str = svg!("hdf5.svg");
