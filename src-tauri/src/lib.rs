mod tagger;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder};

use tagger::{inference_single_image, InferenceArgs, InferenceResult, ModelArgs, TaggerError};

#[tauri::command]
#[specta::specta] // < You must annotate your commands
fn hello_world(my_name: String) -> String {
    format!("Hello, {my_name}! You've been greeted from Rust!")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = Builder::<tauri::Wry>::new()
        // Then register them (separated by a comma)
        .commands(collect_commands![hello_world, inference_single_image])
        .typ::<ModelArgs>()
        .typ::<InferenceArgs>()
        .typ::<InferenceResult>()
        .typ::<TaggerError>();

    #[cfg(debug_assertions)] // <- Only export on non-release builds
    builder
        .export(Typescript::default(), "../src/bindings.ts")
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // and finally tell Tauri how to invoke them
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            // This is also required if you want to use events
            builder.mount_events(app);

            Ok(())
        })
        // on an actual app, remove the string argument
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
