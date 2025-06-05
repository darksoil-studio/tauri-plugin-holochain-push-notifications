use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::HolochainPushNotifications;
#[cfg(mobile)]
use mobile::HolochainPushNotifications;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the holochain-push-notifications APIs.
pub trait HolochainPushNotificationsExt<R: Runtime> {
  fn holochain_push_notifications(&self) -> &HolochainPushNotifications<R>;
}

impl<R: Runtime, T: Manager<R>> crate::HolochainPushNotificationsExt<R> for T {
  fn holochain_push_notifications(&self) -> &HolochainPushNotifications<R> {
    self.state::<HolochainPushNotifications<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("holochain-push-notifications")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let holochain_push_notifications = mobile::init(app, api)?;
      #[cfg(desktop)]
      let holochain_push_notifications = desktop::init(app, api)?;
      app.manage(holochain_push_notifications);
      Ok(())
    })
    .build()
}
