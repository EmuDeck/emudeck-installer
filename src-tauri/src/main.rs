// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::process::Command;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: String) -> Result<String, String> {
	let output = Command::new("sh").arg("-c").arg(&name).output();

	match output {
		Ok(result) => {
			if result.status.success() {
				Ok(String::from_utf8_lossy(&result.stdout).to_string())
			} else {
				Err(String::from_utf8_lossy(&result.stderr).to_string())
			}
		},
		Err(err) => Err(err.to_string()),
	}
}


fn main() {
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![greet])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
