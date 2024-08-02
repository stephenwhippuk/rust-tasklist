use reader::TaskListReader;
use crate::util::inject;
use crate::app::App;

pub mod app;
pub mod task;
pub mod reader;
pub mod util;



fn main() {
    let app = App::new(inject::<TaskListReader>());
    app.run();
}
