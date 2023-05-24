use std::fmt;

enum LineType{
    TextFile,
    Directory,
    ErrorMessage,
    ArchiveFile,
    SearchQuery,
    TelnetSession,
    BinaryFile,
    GIFImage,
    HTMLFile,
    InfoText,
    GenericImageFile,
    DocumentFile,
    SoundFile,
    VideoFile,
    CalendarFile,
    MIMEFile,
    Comment,
    RawText,
    EOF
}

enum BrowserMode{
    View,
    Search
}
enum SearchEditing{
    Host,
    Path
}

struct BrowserState<T>{
    page_content: T,
    page_type: LineType
    host: String,
    path: String,
    mode: BrowserMode

}

struct SearchState{
    host: String,
    path: String,
    editing: SearchEditing


}
struct GopherLine{
    l_type: LineType,
    label: String, // Displayed Text
    path: Option<String>,
    hostname: Option<String>, // URL/Path location
    port: Option<String> // target port number
}

fn map_types(line: &str) -> LineType{
    return match line.chars().next().unwrap() {
	'0' => LineType::TextFile,
	'1' => LineType::Directory,
	'3' => LineType::ErrorMessage,
	'5' => LineType::ArchiveFile,
	'7' => LineType::SearchQuery,
	'8' => LineType::TelnetSession,
	'9' => LineType::BinaryFile,
	'g' => LineType::GIFImage,
	'h' => LineType::HTMLFile,
	'i' => LineType::InfoText,
	'I' => LineType::GenericImageFile,
	'd' => LineType::DocumentFile,
	's' => LineType::SoundFile,
	';' => LineType::VideoFile,
	'c' => LineType::CalendarFile,
	'M' => LineType::MIMEFile,
	'#' => LineType::Comment,
	'.' => LineType::EOF,
	_ => LineType::RawText,
    }
}

impl fmt::Display for LineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
	match self {
	    LineType::TextFile => write!(f, "Textfile"),
	    LineType::Directory => write!(f, "Dir"),
	    LineType::ErrorMessage => write!(f, "Error"),
	    LineType::ArchiveFile => write!(f, "Archive"),
	    LineType::SearchQuery => write!(f, "Search"),
	    LineType::TelnetSession => write!(f, "Telnet"),
	    LineType::BinaryFile => write!(f, "Binary"),
	    LineType::GIFImage => write!(f, "GIF"),
	    LineType::HTMLFile => write!(f, "HTML"),
	    LineType::InfoText => write!(f, "Info"),
	    LineType::GenericImageFile => write!(f, "Img"),
	    LineType::DocumentFile => write!(f, "Doc"),
	    LineType::SoundFile => write!(f, "Audio"),
	    LineType::VideoFile => write!(f, "Video"),
	    LineType::CalendarFile => write!(f, "Calendar"),
	    LineType::MIMEFile => write!(f, "MIME"),
	    LineType::Comment => write!(f, "Comment"),
	    LineType::EOF => write!(f, "EOF"),
	    LineType::RawText => write!(f, "RAW"),
	    _ => write!(f, "Nil"),
	}
    }
}
