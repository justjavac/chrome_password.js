#[macro_use]
extern crate napi_derive;
use std::env;
use std::path::PathBuf;
use tabled::builder::Builder;
use tabled::settings::object::Columns;
use tabled::settings::{Style, Width};

#[napi]
pub fn get_password() -> Vec<Vec<String>> {
  let user_profile = env::var("LOCALAPPDATA").unwrap();
  let local_state_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Local State");
  let login_data_path = PathBuf::from(&user_profile).join("Google/Chrome/User Data/Default/Login Data");

  let master_key = chrome_password::get_master_key(&local_state_path);
  chrome_password::get_password(&login_data_path, &master_key)
}

#[napi]
pub fn get_password_table() -> String {
  let password = get_password();
  let mut builder = Builder::default();
  builder.push_record(["url", "username", "password"]);
  for p in password {
    builder.push_record(p);
  }
  builder
    .build()
    .with(Style::ascii())
    .modify(Columns::first(), Width::wrap(50).keep_words(true))
    .to_string()
}
