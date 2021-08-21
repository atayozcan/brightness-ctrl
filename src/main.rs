use std::{collections::HashMap, error::Error};
use zbus::{dbus_proxy, Connection};
use zvariant::Value;

#[dbus_proxy]
trait Notifications {
    fn notify(
        &self,
        app_name: &str,
        replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        actions: &[&str],
        hints: HashMap<&str, &Value>,
        expire_timeout: i32,
    ) -> zbus::Result<u32>;

    fn close_notification(&self, id: u32) -> zbus::Result<u32>;
}

fn main() -> Result<(), Box<dyn Error>> {
    for _i in 0..1000 {
        NotificationsProxy::new(&Connection::new_session()?)?.notify(
            "",
            1,
            "",
            &*format!(
                "Brightness: %{}",
                std::fs::read_to_string("/sys/class/backlight/nvidia_0/actual_brightness")?
            ),
            "",
            &[],
            HashMap::new(),
            0,
        )?;
    }

    NotificationsProxy::new(&Connection::new_session()?)?.close_notification(1)?;
    Ok(())
}
