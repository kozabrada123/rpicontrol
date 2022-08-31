mod fan;
mod sys;
use rppal;
use std;
use log::{debug, info};
use std::fs::{self, File};
use daemonize::Daemonize;
use chrono::prelude::*;
use simple_logger::SimpleLogger;

fn main() {

    // Init logger
    SimpleLogger::new().init().unwrap();

    debug!("Starting Daemon..");

    // Start ourselves a daemon
    // Create the daemon's directory
    fs::create_dir("/tmp/rpictl/").unwrap();

    let stdout = File::create("/tmp/rpictl/rpictl.out").unwrap();
    let stderr = File::create("/tmp/rpictl/rpictl.err").unwrap();

    // Files we save to
    let nightmode = "/tmp/rpictl/nightmode";
    let fanfile = "/tmp/rpictl/fan";
    let temp = "/tmp/rpictl/temp";

    // Create all needed files
    File::create("/tmp/rpictl/nightmode").unwrap();
    File::create("/tmp/rpictl/fan").unwrap();
    File::create("/tmp/rpictl/temp").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/rpictl.pid") // Every method except `new` and `start`
        .chown_pid_file(true)      // is optional, see `Daemonize` documentation
        .working_directory("/tmp") // for default behaviour.
        .user("nobody")
        .group("daemon") // Group name
        .group(2)        // or group id.
        .umask(0o777)    // Set umask, `0o027` by default.
        .stdout(stdout)  // Redirect stdout to `/tmp/daemon.out`.
        .stderr(stderr)  // Redirect stderr to `/tmp/daemon.err`.
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => info!("Started Daemon!"),
        Err(e) => eprintln!("Error, {}", e),
    }

    // Signal that daemon is enabled
    sys::change_led_perms();
    for i in 1..4 {
        sys::led_off("0"); // Green
        sys::led_off("1"); // Red

        std::thread::sleep(std::time::Duration::from_millis(200));

        sys::led_on("0"); // Green
        sys::led_on("1"); // Red

        std::thread::sleep(std::time::Duration::from_millis(200));
    }


    debug!("Creating gpio controller...");

    // Create the global gpio controller
    let gpioctl = rppal::gpio::Gpio::new().unwrap();

    debug!("Created!");

    debug!("Getting pins...");

    // Define our pins & vars
    let pin_fanctl = gpioctl.get(23).unwrap().into_output();
    let mut night = false;

    debug!("Successfully intialized pins..");

    debug!("Creating fan controller..");

    // Create the fan controller
    let mut fan = fan::FanCtl::new(pin_fanctl, gpioctl);

    debug!("Created, enabling fan..");

    // Enable!
    fan.enable();

    debug!("Enabled!");

    debug!("Starting loop..");

    // Run daemon shit
    loop {
        // Get the time
        let now: DateTime<Local> = Local::now();

        // If it's after 21 and we haven't enterned night more, enter it
        if now.hour() >= 21 && !night{

            info!("Enabling night mode..");

            // Turn off the leds
            sys::led_off("0"); // Green
            sys::led_off("1"); // Red

            // Set night mode to on, so we know we've already set it
            night = true;
        }

        // If it's between 7 and 21 and we're in night mode, turn it off
        else if now.hour() >= 7 && night {

            info!("Disabling night mode..");

            // Turn on the leds
            sys::led_on("0"); // Green
            sys::led_on("1"); // Red

            // Set night mode to off, so we know we've already set it
            night = false;
        }
        
        // Save down data to the right files
        fs::write(nightmode, format!("{:?}", night)).unwrap(); // Nightmode
        fs::write(fanfile, format!("{:?}", fan.enabled)).unwrap(); // Whether or not the fan is enabled
        fs::write(temp, format!("{:?}", sys::get_temp())).unwrap(); // The temprature
    }
}