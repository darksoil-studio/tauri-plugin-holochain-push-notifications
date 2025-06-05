use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<HolochainPushNotifications<R>> {
  Ok(HolochainPushNotifications(app.clone()))
}

/// Access to the holochain-push-notifications APIs.
pub struct HolochainPushNotifications<R: Runtime>(AppHandle<R>);

impl<R: Runtime> HolochainPushNotifications<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
