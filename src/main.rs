use reader::TaskListReader;
use writer::TaskListWriter;
use crate::util::inject;
use crate::app::App;

pub mod app;
pub mod task;
pub mod reader;
pub mod writer;
pub mod util;



fn main() {
    let mut app = App::new(inject::<TaskListReader>(), inject::<TaskListWriter>());
    app.run();
}
