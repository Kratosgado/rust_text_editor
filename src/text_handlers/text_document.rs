use piston_window::Text;

pub struct TextDocument {
    length: i64,
    document_has_changed: bool,
    line_buffer: Vec<i64>,
}

trait Actions {
    fn init(filename: &str) -> bool;
    fn save_file(filename: &str) -> bool;
    fn has_changed() -> bool;

    fn get_line(line_number: i64) -> Text;
}
