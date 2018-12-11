extern crate gtk;

use gtk::prelude::*;
use gtk::{Window, WindowType};
use gtk::{ListBox, ListBoxRow, Label, ScrolledWindow};
use gtk::{Paned, Orientation};
use gtk::{TextBuffer, TextView};

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

fn main() {

    let mut args = std::env::args();
    let root_path: String = args.nth(1)
            .unwrap_or(".".to_string());

    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let list_box = ListBox::new();

    let all_paths = Rc::new(RefCell::new(all_files(&root_path)));

    all_paths.borrow().iter().map(|path_buf| {
        let lstr: &str = &path_buf.display().to_string();
        let label = Label::new_with_mnemonic(Some(lstr));
        let list_box_row = ListBoxRow::new();
        list_box_row.add(&label);
        list_box_row
    })
    .for_each(|list_box_row| {
        list_box.add(&list_box_row);
    });

    let side_window = ScrolledWindow::new(None, None);
    side_window.add(&list_box);


    let text_buffer = TextBuffer::new(None);
    let text_view = TextView::new_with_buffer(&text_buffer);

    list_box.connect_row_activated(move |_, row| {

        let _all_paths = all_paths.borrow();
        let active_path = _all_paths.get(row.get_index() as usize).unwrap();

        let text = read_file(active_path).unwrap_or("".to_string());
        text_buffer.set_text(&text);

    });

    text_view.connect_key_press_event(move |_tv, key_event| {
        println!("key_event:{:?}", key_event);
        Inhibit(false)
    });

    let main_window = ScrolledWindow::new(None, None);
    main_window.add(&text_view);

    let paned = Paned::new(Orientation::Horizontal);
    paned.pack1(&side_window, true, true);
    paned.pack2(&main_window, true, true);

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Unified Object SearchCompute Engine");
    window.set_default_size(350, 70);

    window.add(&paned);
    window.show_all();

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    gtk::main();
}

use std::path::PathBuf;

//fn all_files(path: &str) -> impl Iterator<Item=PathBuf>
fn all_files(path: &str) -> Vec<PathBuf> {
    let entries = fs::read_dir(path).unwrap();
    entries.flat_map(|rentry| {
        let entry = rentry.unwrap();

        let path = entry.path();
        let path_iter = std::iter::once(path.clone());
        if path.clone().is_dir() {

            let path_str: &str = &path.display().to_string();
            path_iter.chain(all_files(path_str)).collect::<Vec<PathBuf>>()
        } else
        {
            path_iter.collect::<Vec<PathBuf>>()
        }
    }).collect()
}

use std::io::Result;
fn read_file(path: &PathBuf) -> Result<String> {
        let file = File::open(path)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let contents = contents;
        Ok(contents)
}
