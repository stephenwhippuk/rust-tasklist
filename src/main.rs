use reader::TaskListReader;
use writer::TaskListWriter;
use crate::util::inject;
use crate::app::App;
use crate::kbdin::ConsoleInput;
pub mod app;
pub mod task;
pub mod reader;
pub mod writer;
pub mod util;
pub mod kbdin;
pub mod cmdparse;
pub mod console;
fn main() {
    let mut app = App::new(inject::<TaskListReader>(), inject::<TaskListWriter>(), inject::<ConsoleInput>(), inject::<console::ConsoleOutput>());
    app.run();
}
