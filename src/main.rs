use std::collections::HashMap;
use std::fs::read_to_string;
use zbus::Connection;
use zvariant::Value;

fn main() {
    let percent = format!(
        "Brightness: %{}",
        read_to_string("/sys/class/backlight/nvidia_0/actual_brightness").unwrap()
    );
    Connection::new_session()
        .unwrap()
        .call_method(
            Some("org.freedesktop.Notifications"),
            "/org/freedesktop/Notifications",
            Some("org.freedesktop.Notifications"),
            "Notify",
            &(
                "",
                0u32,
                "",
                percent,
                "",
                vec![""; 0],
                HashMap::<&str, &Value>::new(),
                5000,
            ),
        )
        .unwrap();
}
