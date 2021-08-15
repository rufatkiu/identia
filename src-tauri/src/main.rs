// Copyright 2019-2021 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use ipfs_api::IpfsClient;
use std::{thread, time::Duration};
use tauri::api::process::Command;

use ipfs_api::IpfsClient;

fn main() {
  tauri::Builder::default()
    .setup(|_app| {
      // tauri::async_runtime::spawn(async move {
      tauri::async_runtime::block_on(async move {
        match launch_ipfs_daemon().await {
          Ok(()) => (),
          Err(_err) => {
            // log::error!("There was an error launching ipfs: {:?}", err);
          }
        }
        // log::info!("Launch setup successful")
      });
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn launch_ipfs_daemon() -> Result<(), String> {
  // config::create_initial_config_if_necessary();
  Command::new_sidecar("ipfs")
    .or(Err(String::from("Can't find ipfs binary")))?
    .args(&[
      "daemon",
      // config::conductor_config_path()
      //   .into_os_string()
      //   .to_str()
      //   .unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Failed to execute ipfs: {:?}", err))?;

  let client = IpfsClient::default();
  match wait_for_ipfs_ready().await {
    Ok(ready) => eprintln!("ipfs ready: {:?}", ready),
    Err(e) => eprintln!("error waiting for ipfs: {}", e),
  }

  match client.id(None).await {
    Ok(id) => eprintln!("id: {:?}", id.id),
    Err(e) => eprintln!("error getting id: {}", e),
  }

  Ok(())
}

async fn wait_for_ipfs_ready() -> Result<bool, bool> {
  let client = IpfsClient::default();
  // A counter variable
  let mut ready = false;
  let mut retries = 1;
  // Loop while `n` is less than 101
  while !ready {
    match client.id(None).await {
      Ok(_id) => {
        ready = true;
      }
      Err(_err) => {
        if retries > 300 {
          // Err()
          break;
        }
        retries += 1;
        thread::sleep(Duration::from_millis(100));
      }
    }
  }

  Ok(ready)
}
