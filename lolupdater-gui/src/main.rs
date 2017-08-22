#![cfg_attr(feature="clippy", feature(plugin))]

#![cfg_attr(feature="clippy", plugin(clippy))]

#![feature(drop_types_in_const)]

extern crate nfd;
extern crate ui;
extern crate lolupdater_core;

use ui::{BoxControl, Button, Entry};
use ui::{Group, InitOptions, Label, RadioButtons};
use ui::{Separator, Window};

use std::thread;
use lolupdater_core::*;
use errors::*;

thread_local! {
    static LOLPATH_ENTRY: Entry = Entry::new();
    static INSTALLMODE_RADIO: RadioButtons = RadioButtons::new();
}

fn run() {
    let program_name = format!("LoLUpdater for macOS v{}", VERSION);

    let mainwin = Window::new(&program_name, 640, 240, true);
    mainwin.set_margined(true);
    mainwin.on_closing(Box::new(|_| {
        ui::quit();
        false
    }));


    let hbox = BoxControl::new_vertical();
    hbox.set_padded(true);
    mainwin.set_child(hbox.clone().into());

    let program_name_label = Label::new(&program_name);
    hbox.append(program_name_label.clone().into(), false);
    hbox.append(Separator::new_horizontal().into(), false);

    let options = Group::new("Options");
    options.set_margined(true);
    hbox.append(options.clone().into(), false);

    let inner = BoxControl::new_vertical();
    inner.set_padded(true);
    options.set_child(inner.clone().into());

    let rb_label = Label::new("Install mode");
    inner.append(rb_label.clone().into(), false);
    let rb = INSTALLMODE_RADIO.with(|ir| ir.clone());
    rb.append("Install");
    rb.append("Uninstall");
    rb.set_selected(0);
    inner.append(rb.clone().into(), false);
    inner.append(Separator::new_horizontal().into(), false);

    let install_path_label = Label::new("League Location");
    inner.append(install_path_label.clone().into(), false);
    let install_path_box = BoxControl::new_horizontal();
    install_path_box.set_padded(true);
    inner.append(install_path_box.clone().into(), false);
    let install_path_entry = LOLPATH_ENTRY.with(|lpe| lpe.clone());
    install_path_entry.set_text("/Applications/League of Legends.app");
    install_path_box.append(install_path_entry.clone().into(), true);
    let install_path_button = Button::new("Locate");
    install_path_button.on_clicked(Box::new(ask_for_loldir));
    install_path_box.append(install_path_button.clone().into(), false);

    let install_button = Button::new("Patch!");
    install_button.on_clicked(Box::new(install_clicked));
    hbox.append(install_button.clone().into(), false);

    mainwin.show();
    ui::main();
}

pub fn main() {
    ui::init(InitOptions).unwrap();
    run();
    ui::uninit();
}


fn ask_for_loldir(_: &Button) {
    let result = nfd::open_file_dialog(Some("app"), Some("/Applications")).unwrap_or_else(|e| {
        panic!(e);
    });

    if let nfd::Response::Okay(file_path) = result {
        print!("{}", file_path);
        LOLPATH_ENTRY.with(|lpe| { lpe.set_text(&file_path); });
    }
}

fn install_clicked(install_button: &Button) {
    install_button.disable();
    let mode = INSTALLMODE_RADIO.with(|ir| ir.selected());
    let target = LOLPATH_ENTRY.with(|lpe| lpe.text());

    let result = start_install(&*target, mode);
    // TODO: Error Handling
    if result.is_ok() {}

}

fn start_install(target: &str, mode: i32) -> Result<()> {
    // TODO: Start a new Thread here
    init_backups()?;
    if mode == 0 {
        install(target)
    } else {
        uninstall(target)
    }
}
