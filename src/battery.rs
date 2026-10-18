use std::{process::Command, time::Duration};

use cosmic::iced::{
    futures::{SinkExt, Stream},
    stream,
};

use crate::app::{AppMsg, BatteryStatus};

const POLL_INTERVAL: Duration = Duration::from_secs(5);

pub fn subscription() -> impl Stream<Item = AppMsg> {
    stream::channel(
        1,
        |mut output: cosmic::iced::futures::channel::mpsc::Sender<AppMsg>| async move {
            loop {
                let result = tokio::task::spawn_blocking(read_battery).await;
                let status = match result {
                    Ok(Ok(Some(percentage))) => BatteryStatus::Connected(percentage),
                    Ok(Ok(None)) => BatteryStatus::Disconnected,
                    Ok(Err(error)) => {
                        tracing::warn!("{error}");
                        BatteryStatus::Unavailable
                    }
                    Err(error) => {
                        tracing::error!("Battery query task failed: {error}");
                        BatteryStatus::Unavailable
                    }
                };

                if output.send(AppMsg::BatteryUpdated(status)).await.is_err() {
                    break;
                }

                tokio::time::sleep(POLL_INTERVAL).await;
            }
        },
    )
}

fn read_battery() -> Result<Option<u8>, String> {
    let devices = Command::new("upower")
        .arg("-e")
        .output()
        .map_err(|error| format!("Could not run upower -e: {error}"))?;

    if !devices.status.success() {
        return Err(format!(
            "upower -e failed: {}",
            String::from_utf8_lossy(&devices.stderr).trim()
        ));
    }

    let devices = String::from_utf8_lossy(&devices.stdout);
    let paths: Vec<_> = devices
        .lines()
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .map(str::to_owned)
        .collect();

    let mut failed_queries = 0;
    for path in paths {
        let info = match Command::new("upower").args(["-i", &path]).output() {
            Ok(info) if info.status.success() => info,
            Ok(info) => {
                failed_queries += 1;
                tracing::warn!(
                    "upower -i {path} failed: {}",
                    String::from_utf8_lossy(&info.stderr).trim()
                );
                continue;
            }
            Err(error) => {
                return Err(format!("Could not run upower -i {path}: {error}"));
            }
        };

        let details = String::from_utf8_lossy(&info.stdout);
        if is_dualsense(&path, &details) {
            return parse_percentage(&details)
                .map(Some)
                .map_err(|error| format!("Invalid DualSense battery information: {error}"));
        }
    }

    if failed_queries > 0 {
        return Err(format!(
            "Could not inspect {failed_queries} UPower device(s); controller status is unknown"
        ));
    }

    Ok(None)
}

fn is_dualsense(device_path: &str, details: &str) -> bool {
    let path = device_path.to_ascii_lowercase();
    if path.contains("ps_controller_battery") {
        return true;
    }

    let mut vendor = "";
    let mut model = "";
    let mut native_path = "";

    for line in details.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        match key.trim() {
            "vendor" => vendor = value.trim(),
            "model" => model = value.trim(),
            "native-path" => native_path = value.trim(),
            _ => {}
        }
    }

    let vendor = vendor.to_ascii_lowercase();
    let model = model.to_ascii_lowercase();
    let native_path = native_path.to_ascii_lowercase();

    model.contains("dualsense")
        || (vendor.contains("sony") && model.contains("controller"))
        || native_path.contains("ps_controller_battery")
}

fn parse_percentage(details: &str) -> Result<u8, String> {
    for line in details.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if key.trim() != "percentage" {
            continue;
        }

        let percentage = value
            .trim()
            .trim_end_matches('%')
            .trim()
            .parse::<f32>()
            .map_err(|error| format!("invalid percentage value: {error}"))?;
        if !percentage.is_finite() || !(0.0..=100.0).contains(&percentage) {
            return Err(format!("percentage outside 0-100 range: {percentage}"));
        }

        return Ok(percentage.round() as u8);
    }

    Err("percentage field is missing".to_owned())
}

#[cfg(test)]
mod tests {
    use super::{is_dualsense, parse_percentage};

    const DEVICE_INFO: &str = "\
  native-path:          sony_controller_battery_1234
  vendor:               Sony Interactive Entertainment
  model:                Wireless Controller
  percentage:           67%
";

    #[test]
    fn recognizes_upower_dualsense_device() {
        assert!(is_dualsense(
            "/org/freedesktop/UPower/devices/battery_ps_controller_battery_1234",
            ""
        ));
        assert!(is_dualsense("/some/device", DEVICE_INFO));
        assert!(!is_dualsense(
            "/org/freedesktop/UPower/devices/battery_mouse",
            "  vendor: Logitech\n  model: Wireless Mouse\n"
        ));
    }

    #[test]
    fn parses_percentage() {
        assert_eq!(parse_percentage(DEVICE_INFO).unwrap(), 67);
        assert_eq!(parse_percentage("  percentage: 42.6%\n").unwrap(), 43);
    }

    #[test]
    fn rejects_invalid_percentages() {
        assert!(parse_percentage("  percentage: 101%\n").is_err());
        assert!(parse_percentage("  percentage: unknown\n").is_err());
        assert!(parse_percentage("  state: discharging\n").is_err());
    }
}
