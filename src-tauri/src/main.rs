// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;

struct Note {
    id: i32,
    title: String,
    content: String,
}

impl Note {
    fn summary(&self) -> String {
        format!("{}: {} which is {}", self.id, self.title, self.content)
    }
}

#[tauri::command]
fn note_summary() -> String {
    let note = Note {
        id: 1,
        title: "Hello".into(),
        content: "World".into(),
    };
    note.summary()
}

fn main() {
    // app_lib::run();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![note_summary])
        .run(tauri::generate_context!())
        .expect("error while running Tauri app");
}
