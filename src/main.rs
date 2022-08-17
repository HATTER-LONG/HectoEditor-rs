mod editor;
mod terminal;
use editor::Editor;
use editor::Position;

pub use terminal::Terminal;
fn main() {
    Editor::default().run();
}
