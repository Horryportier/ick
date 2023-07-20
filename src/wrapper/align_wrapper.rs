use serde::{Serialize, Deserialize};

use gtk::Align as  gtkAlign;
use smart_default::SmartDefault;

#[derive(SmartDefault,Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
#[serde(tag = "Align", content = "content")]
pub enum Align {
    #[default]
    Fill,

    Start,
    End,
    Center,
    Baseline,

    __Unknown(i32),
}

impl From<gtkAlign> for Align {
    fn from(align: gtkAlign) -> Self{
        match align {
            gtkAlign::Fill =>  Align::Fill,
            gtkAlign::Start =>  Align::Start,
            gtkAlign::End =>  Align::End,
            gtkAlign::Center =>  Align::Center,
            gtkAlign::Baseline =>  Align::Baseline,
            gtkAlign::__Unknown(i) => Align::__Unknown(i),
            _ => Align::__Unknown(0),
        }
    }
} 


impl From<Align> for gtkAlign {
    fn from(align: Align) -> Self{
        match align {
            Align::Fill =>  gtkAlign::Fill,
            Align::Start =>  gtkAlign::Start,
            Align::End =>  gtkAlign::End,
            Align::Center=>  gtkAlign::Center,
            Align::Baseline =>  gtkAlign::Baseline,
            Align::__Unknown(i) => gtkAlign::__Unknown(i)
        }
    }
} 

