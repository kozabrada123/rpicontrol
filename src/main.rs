mod fan;
mod sys;
use chrono::prelude::*;
use daemonize::Daemonize;
use log::{debug, info};
use rppal;
use simple_logger::SimpleLogger;
use std;
use std::fs::{self, File};
use std::time::Duration;

use crate::sys::{GREEN_LED, RED_LED};

fn main() {
    SimpleLogger::new().init().unwrap();

    debug!("Starting Daemon..");

    fs::create_dir("/tmp/rpictl/").unwrap();

    let stdout = File::create("/tmp/rpictl/rpictl.out").unwrap();
    let stderr = File::create("/tmp/rpictl/rpictl.err").unwrap();

    // Files we save to
    let nightmode_path = "/tmp/rpictl/nightmode";
    let fanfile_path = "/tmp/rpictl/fan";
    let temp_path = "/tmp/rpictl/temp";

    File::create(nightmode_path).unwrap();
    File::create(fanfile_path).unwrap();
    File::create(temp_path).unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/rpictl.pid") // Every method except `new` and `start`
        .chown_pid_file(true) // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2) // or group id.
        .umask(0o777) // Set umask, `0o027` by default.
        .stdout(stdout) // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr) // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => info!("Started Daemon!"),
        Err(e) => eprintln!("Error, {}", e),
    }

    // Signal that daemon is enabled
    sys::change_led_perms();
    for _i in 1..4 {
        sys::led_off(GREEN_LED);
        sys::led_off(RED_LED);

        std::thread::sleep(std::time::Duration::from_millis(200));

        sys::led_on(GREEN_LED);
        sys::led_on(RED_LED);

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    debug!("Creating gpio controller...");

    let gpioctl = rppal::gpio::Gpio::new().unwrap();

    debug!("Created!");

    debug!("Getting pins...");

    let pin_fanctl = gpioctl.get(23).unwrap().into_output();
    let mut night_mode = false;

    debug!("Successfully intialized pins..");

    debug!("Creating fan controller..");

    let mut fan = fan::FanCtl::new(pin_fanctl, gpioctl);

    debug!("Created, enabling fan..");

    fan.enable();

    debug!("Enabled!");

    debug!("Starting loop..");

    loop {
        let now: DateTime<Local> = Local::now();

        if now.hour() >= 21 && !night_mode {
            info!("Enabling night mode..");

            sys::led_off(GREEN_LED);
            sys::led_off(RED_LED);

            night_mode = true;
        }

        if now.hour() >= 7 && now.hour() < 21 && night_mode {
            info!("Disabling night mode..");

            sys::led_on(GREEN_LED);
            sys::led_on(RED_LED);

            night_mode = false;
        }

        fs::write(nightmode_path, night_mode.to_string()).unwrap();
        fs::write(fanfile_path, fan.enabled.to_string()).unwrap();
        fs::write(temp_path, sys::get_temp().to_string()).unwrap();

        // Don't spend too many cpu cycles on this..
        std::thread::sleep(Duration::from_millis(100));
    }
}

