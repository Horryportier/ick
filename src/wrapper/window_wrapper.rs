use gtk::{glib::StrV, Application, ApplicationWindow};
use smart_default::SmartDefault;

use serde::{Deserialize, Serialize};

use crate::error::error::Error;

use super::align_wrapper::Align;

#[derive(SmartDefault, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(tag = "DefWindow")]
pub struct DefWindow {
    #[default = "window"]
    #[serde(default)]
    name: String,

    #[default = 200]
    #[serde(default)]
    width: i32,

    #[default = 200]
    #[serde(default)]
    height: i32,

    #[default = true]
    #[serde(default)]
    decorated: bool,

    #[default = true]
    #[serde(default)]
    focusable: bool,

    #[default = true]
    #[serde(default)]
    resizable: bool,

    #[default = 1.]
    #[serde(default)]
    opacity: f64,

    #[default = "main"]
    #[serde(default)]
    css_name: String,

    #[default = ""]
    #[serde(default)]
    css_classes: String,

    #[serde(default)]
    halign: Align,
    #[serde(default)]
    valign: Align,
}

impl DefWindow {
    pub fn deserialize(mut self, json: &String) -> Result<DefWindow, Error> {
         match serde_json::from_str::<DefWindow>(json) {
            Ok(dw) => {
                self = dw.clone();
                Ok(dw)
            },
            Err(_) => Err(Error::JsonErr("deserialize failed".into()))
         }
    }

    pub fn serialize(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self).map_err(|_| Error::JsonErr("serialize failed".into()))?)
    }

    pub fn genrate_window(self, app: &Application) -> ApplicationWindow {
        let css_classes: StrV = StrV::from([self.css_classes.as_str()]);
        let win = ApplicationWindow::builder()
            .application(app)
            .name(self.name)
            .default_width(self.width)
            .default_height(self.height)
            .decorated(self.decorated)
            .focusable(self.focusable)
            .resizable(self.resizable)
            .opacity(self.opacity)
            .css_name(self.css_name)
            .css_classes(css_classes)
            .halign(self.halign.into())
            .valign(self.valign.into());
        win.build()
    }

}
