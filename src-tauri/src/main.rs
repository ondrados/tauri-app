#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

extern crate redis;
use crate::redis::Commands;

use tauri::{
  command,
  api::process::{Command, CommandEvent},
  Manager,
};

#[command]
fn return_string(word: String) -> String{
    return word
}

fn set_i32(key: String, value: i32) -> redis::RedisResult<()> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  /* do something here */
  let _ : () = con.set(key, value)?;

  Ok(())
}

fn get_i32(key: String) -> redis::RedisResult<i32> {
  let client = redis::Client::open("redis://127.0.0.1/")?;
  let mut con = client.get_connection()?;

  /* do something here */
  let count : i32 = con.get(key)?;

  Ok(count)
}

#[command]
fn set_key(key: String, value: i32) {
  set_i32(key, value).unwrap();
}

#[command]
fn get_key(key: String) -> i32 {
  return match get_i32(key) {
    Ok(n) => n,
    Err(_) => 0
  }


}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let window = app.get_window("main").unwrap();
      tauri::async_runtime::spawn(async move {
        let (mut rx, mut child) = Command::new_sidecar("redis-server")
          .expect("failed to setup `redis-server` sidecar")
          .spawn()
          .expect("Failed to spawn packaged redis-server");

        let mut i = 0;
        while let Some(event) = rx.recv().await {
          if let CommandEvent::Stdout(line) = event {
            window
              .emit("message", Some(format!("'{}'", line)))
              .expect("failed to emit event");
            i += 1;
            if i == 4 {
              child.write("message from Rust\n".as_bytes()).unwrap();
              i = 0;
            }
          }
        }
      });

      Ok(())
    })
    .invoke_handler(tauri::generate_handler![return_string, set_key, get_key])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
