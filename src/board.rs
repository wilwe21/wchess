use gtk::prelude::*;

pub struct board {
    pub board: gtk::Box,
    pub wscore: i32,
    pub bscore: i32
}

impl board {
    pub fn new() -> Self {
        let gbox = gtk::Box::new(gtk::Orientation::Vertical, 1);
        let lab = gtk::Label::builder().label("sus").build();
        gbox.append(&lab);
        let wscore: i32 = 0;
        let bscore: i32 = 0;
        Self {
            board: gbox,
            wscore,
            bscore
        }
    } 
}
