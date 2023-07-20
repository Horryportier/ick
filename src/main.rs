use std::{fs, os::unix::net::UnixListener, path::Path};

use debug::debug::Debug;
use gtk::{ScaleButton, Label, Button};
use gtk::traits::{GtkWindowExt, ButtonExt};
use rand::{distributions::Alphanumeric, Rng};

use gtk::prelude::ApplicationExtManual;
use gtk::{prelude::ApplicationExt, traits::WidgetExt, Application};

use crate::wrapper::window_wrapper::DefWindow;

mod debug;
mod error;
mod wrapper;

const PATH: &str = "/tmp/test_socket";

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut debuger = Debug::new(Vec::new(), Vec::new());
    let socket = Path::new(PATH);
    if socket.exists() {
        fs::remove_file(PATH)?;
    }

    let listener = UnixListener::bind(PATH)?;
    loop {
        let mut response = "".into();
        match listener.accept() {
            Ok((mut socket, addr)) => {
                println!("Got a client: {:?} - {:?}", socket, addr);
                //socket.write_all(b"hello world")?;
                socket.read_to_string(&mut response)?;
                //println!("Response: {}", response);
            }
            Err(e) => println!("accept function failed: {:?}", e),
        }

        let def = DefWindow::default();
        debuger.add_window(def.clone());
        // let res = def.deserialize(&response);
        // debuger.from_result(Ok(res));

        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();

        let app = Application::builder().application_id(name).build();

        app.connect_activate(move |app| {
            println!("{:?}", def);
            let window = def.clone().genrate_window(app);

            let button = Button::with_label("Test");
            button.connect_clicked(|_| {
                eprintln!("Clicked!");
            });
            let laber = Label::builder()
                .name("laber")
                .label("# test")
                .use_markup(true)
                .build();
            laber.set_xalign(150.);
            let scroll = ScaleButton::builder()
                .orientation(gtk::Orientation::Vertical)
                .build();
             window.set_child(Some(&button));
            window.set_child(Some(&laber));
            window.set_child(Some(&scroll));
            window.show();
        });

        debuger.output_to_file();
        println!("Exit Code {:?}", app.run());
    }
}
