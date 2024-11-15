const FIFE_HTML_DOC_START: &str = r#"<!DOCTYPE html><html lang="en"><head><meta charset="UTF-8"><meta name="viewport" content="width=device-width, initial-scale=1.0">"#;
const FIFE_BODY_STYLE: &str =
    "body{font-family:Arial,sans-serif;margin:20px;background-color:#f4f4f4;}";
const FIFE_UNORDERED_LIST_STYLE: &str = "ul{list-style-type:none;padding:0;}";
const FIFE_LIST_ITEM_STYLE: &str = "li{display:flex;align-items:center;padding:2px;background-color:#fff;border-radius:5px;margin-bottom:0px;box-shadow:0 2px 5px rgba(0, 0, 0, 0.1);}";
const FIFE_FILE_ICON_STYLE: &str = ".file-icon{font-size:24px;margin-right:10px;}";
const FIFE_FILE_DETAILS_STYLE: &str = ".file-details{flex:1;}";
const FIFE_FILE_NAME_STYLE: &str =
    ".file-name {font-size:16px;text-decoration:none;color:#333;font-weight:bold;}";
const FIFE_FILE_NAME_HOVER_STYLE: &str = ".file-name:hover{text-decoration:underline;}";
const FIFE_FILE_INFO_STYLE: &str = ".file-info{color: #888;font-size:14px;}";
const FIFE_FILE_DATE_STYLE: &str = ".file-size,.file-date{display:inline-block;margin-right:15px;}";
const FIFE_DIRECTORY_AND_FILE_STYLE: &str = ".directory{color:#0056b3;}.file{color:#333;}";

/// Build the HTML site that is sent as a response when a directory is clicked
pub(super) fn build_html_response(dir_path: &str, entries_html: &str) -> String {
    let mut response = FIFE_HTML_DOC_START.to_owned();
    response.push_str("<title>");
    response.push_str(dir_path);
    response.push_str("</title>");
    response.push_str("<style>");
    response.push_str(FIFE_BODY_STYLE);
    response.push_str(FIFE_UNORDERED_LIST_STYLE);
    response.push_str(FIFE_LIST_ITEM_STYLE);
    response.push_str(FIFE_FILE_ICON_STYLE);
    response.push_str(FIFE_FILE_DETAILS_STYLE);
    response.push_str(FIFE_FILE_NAME_STYLE);
    response.push_str(FIFE_FILE_NAME_HOVER_STYLE);
    response.push_str(FIFE_FILE_INFO_STYLE);
    response.push_str(FIFE_FILE_DATE_STYLE);
    response.push_str(FIFE_DIRECTORY_AND_FILE_STYLE);
    response.push_str("</style>");
    response.push_str("</head>");
    response.push_str("<body>");
    response.push_str("<h1>");
    response.push_str(dir_path);
    response.push_str("</h1>");
    response.push_str("<ul>");
    response.push_str(entries_html);
    response.push_str("</ul>");
    response.push_str("</body>");
    response.push_str("</html>");
    response
}
