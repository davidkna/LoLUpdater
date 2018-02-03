#![windows_subsystem = "windows"]
extern crate lolupdater_core;
extern crate tinyfiledialogs;
extern crate ui;

use std::cell::RefCell;
use std::sync::mpsc;
use std::time;
use std::thread;

use ui::{msg_box, msg_box_error, BoxControl, Button, Entry, Group, InitOptions, Label,
         MultilineEntry, RadioButtons, Separator, Window};

use lolupdater_core::*;
use errors::*;

thread_local! {
    static MAIN_WINDOW: Window = Window::new(&format!("LoLUpdater {}", VERSION), 640, 240, true);
    static LOLPATH_ENTRY: Entry = Entry::new();
    static INSTALLMODE_RADIO: RadioButtons = RadioButtons::new();
    static CHANNEL: (mpsc::Sender<Result<()>>, mpsc::Receiver<Result<()>>) = mpsc::channel();
    static NOW: RefCell<time::Instant> = RefCell::new(time::Instant::now());
}

fn run() {
    let program_name = "LoLUpdater";

    let mainwin = MAIN_WINDOW.with(|w| w.clone());
    mainwin.set_margined(true);
    #[cfg(not(target_os = "linux"))]
    mainwin.center();
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
    install_path_entry.set_text(DEFAULT_LOL_DIR);
    install_path_box.append(install_path_entry.clone().into(), true);
    let install_path_button = Button::new("Locate");
    install_path_button.on_clicked(Box::new(ask_for_loldir));
    install_path_box.append(install_path_button.clone().into(), false);

    let install_box = BoxControl::new_horizontal();
    let install_button = Button::new("Patch!");
    install_button.on_clicked(Box::new(install_clicked));
    install_box.append(install_button.clone().into(), true);
    install_button.disable();

    let licenses_button = Button::new("Licenses");
    licenses_button.on_clicked(Box::new(show_licenses));
    install_box.append(licenses_button.clone().into(), false);
    hbox.append(install_box.clone().into(), false);

    mainwin.show();
    let has_update = update_available();
    if let Ok(has_update) = has_update {
        if has_update {
            msg_box_error(
                &mainwin,
                "Update available",
                "A new update is available.\nPlease download it from https://github.com/MOBASuite/LoLUpdater/releases/latest to use LoLUpdater.",
            );
            ui::quit();
            ::std::process::exit(0);
        }
    } else if let Err(ref e) = has_update {
        show_error_message(e);
        ui::quit();
        ::std::process::exit(1);
    }

    install_button.enable();

    ui::main();
}

pub fn main() {
    ui::init(InitOptions).unwrap();
    run();
    ui::uninit();
}

fn ask_for_loldir(_: &Button) {
    let result = if cfg!(target_os = "macos") {
        tinyfiledialogs::open_file_dialog(
            "Find LoL",
            "/Applications/",
            Some((&["*.app"], "Applications")),
        )
    } else {
        tinyfiledialogs::select_folder_dialog("Find LoL", "C:/")
    };

    if let Some(file_path) = result {
        LOLPATH_ENTRY.with(|lpe| {
            lpe.set_text(&file_path);
        });
    }
}

fn install_clicked(install_button: &Button) {
    install_button.disable();
    install_button.set_text("Working…");
    let mode = INSTALLMODE_RADIO.with(|ir| ir.selected());
    let target = LOLPATH_ENTRY.with(|lpe| lpe.text());

    let lol_dir = std::string::String::from(&*target);

    let tx = CHANNEL.with(|ch| {
        let (ref tx, _) = *ch;
        tx.clone()
    });
    thread::spawn(move || {
        let result = {
            if mode == 0 {
                install(&lol_dir)
            } else {
                uninstall(&lol_dir)
            }
        };
        tx.send(result).unwrap();
    });

    ui::queue_main(Box::new(is_done_check));
}

fn is_done_check() {
    let should_check = NOW.with(|now| {
        let elapsed = now.borrow().elapsed();
        let max_elapsed = time::Duration::from_secs(1);
        let should_continue = elapsed > max_elapsed;
        if should_continue {
            *now.borrow_mut() = time::Instant::now()
        }

        should_continue
    });
    if !should_check {
        ui::queue_main(Box::new(is_done_check));
        return;
    }

    let rx_message = CHANNEL.with(|ch| {
        let (_, ref rx) = *ch;
        rx.try_recv()
    });
    if let Ok(res) = rx_message {
        install_done(res);
    } else {
        ui::queue_main(Box::new(is_done_check));
    }
}

fn install_done(result: Result<()>) {
    MAIN_WINDOW.with(|win| {
        if result.is_ok() {
            msg_box(
                win,
                "Updating successful!",
                "LoLUpdater needs to be rerun after every LoL update.",
            );
        } else if let Err(ref e) = result {
            show_error_message(e);
        }
    });
    ui::quit();
}

fn show_error_message(error: &Error) {
    MAIN_WINDOW.with(|win| {
        let mut error_msg = format!("Error: {}\n", error);
        for error in error.iter().skip(1) {
            let error_line = format!("Caused by: {}\n", error);
            error_msg.push_str(&error_line);
        }
        error_msg.push_str("\nPlease report this error on Github!");
        msg_box_error(win, "Error!", &error_msg);
    });
}

fn show_licenses(_button: &Button) {
    let win = Window::new("License Info", 600, 400, false);
    win.set_margined(true);
    let entry = MultilineEntry::new_non_wrapping();
    entry.set_text(&get_license_info());
    entry.set_read_only(true);

    win.set_child(entry.clone().into());
    win.on_closing(Box::new(|_| true));
    win.show();
}
