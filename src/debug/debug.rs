use std::fs;

use serde::{Serialize, Deserialize};

use crate::{error::error::Error, wrapper::window_wrapper::DefWindow};

#[allow(dead_code)]
const  DEBUG_FILE: &str = "test/debug.json";

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Debug{
    pub errors: Vec<Error>,
    pub windows: Vec<DefWindow>
}


#[allow(dead_code)]
impl  Debug {
    pub fn new(err: Vec<Error>, win: Vec<DefWindow>) -> Debug {
        Debug { errors: err  , windows: win }
    }
    pub fn add_err(&mut self, err: Error) {
       self.errors.push(err)
    }

    pub fn add_window(&mut self, win: DefWindow) {
       self.windows.push(win)
    }
    pub fn from_result(&mut self, res: Result<DefWindow, Error>) {
        match res {
            Ok(dw) => self.windows.push(dw),
            Err(err) => self.errors.push(err)
        }
    }

    pub fn output_to_file(&self) {
        let mut contents: String = Default::default(); 
        
        self.errors.iter().for_each(|e|{
                    contents = format!("{}\n{}",contents, e.to_string())
                               });

        self.windows.iter().for_each(|w|{
                    contents = format!("{}\n[Window Struct: ]{}",contents, match w.serialize() {
                            Ok(j) => j,
                            Err(e) => e.to_string(),
                    })
        });
        let _ = fs::write(DEBUG_FILE, contents);
    }
}
