use std::{fs, path::Path, process::Command};
use substring::Substring;

pub const GREEN_LED: &str = "0";
pub const RED_LED: &str = "1";

//leds; see https://forums.raspberrypi.com/viewtopic.php?t=12530
pub fn set_led_driver(iled: &str, driver: &str) {
    let led = match iled.to_lowercase().as_str() {
        "red" => RED_LED,
        "green" => GREEN_LED,
        _ => iled,
    };

    let ledpath = format!("/sys/class/leds/led/{led}/trigger");

    let path = Path::new(&ledpath);

    fs::write(path, driver).unwrap();
}

pub fn led_on(iled: &str) {
    let led = match iled.to_lowercase().as_str() {
        "red" => RED_LED,
        "green" => GREEN_LED,
        _ => iled,
    };

    let driver = match led {
        "0" => "mmc0",
        "1" => "default-on",
        _ => "mmc0",
    };

    set_led_driver(led, driver);

    // Write 1 to the brightness file
    let ledpath = format!("/sys/class/leds/led/{led}/brightness");
    let path = Path::new(&ledpath);

    fs::write(path, "1").unwrap();
}

pub fn led_off(iled: &str) {
    set_led_driver(iled, "none");
}

pub fn change_led_perms() {
    Command::new("sudo")
        .arg("chmod")
        .arg("777")
        .arg(r#"/sys/class/leds/led0/trigger"#)
        .output()
        .expect("Failed to execute command");

    Command::new("sudo")
        .arg("chmod")
        .arg("777")
        .arg(r#"/sys/class/leds/led1/trigger"#)
        .output()
        .expect("Failed to execute command");

    Command::new("sudo")
        .arg("chmod")
        .arg("777")
        .arg(r#"/sys/class/leds/led0/brightness"#)
        .output()
        .expect("Failed to execute command");

    Command::new("sudo")
        .arg("chmod")
        .arg("777")
        .arg(r#"/sys/class/leds/led1/brightness"#)
        .output()
        .expect("Failed to execute command");
}

// -------- Get commands --------

pub fn get_temp() -> f64 {
    // Get temp w/ vcgencmd

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg("/usr/bin/vcgencmd measure_temp")
        .output()
        .expect("Failed to execute command");

    // Get the float celcius temperature from the output
    let temp_str = String::from_utf8_lossy(&output.stdout).to_string();

    // Get a substring of just the number
    let temp = temp_str.substring(
        temp_str.rfind("=").unwrap() + 1,
        temp_str.rfind("'").unwrap(),
    );

    //println!("Temp: {}", temp);

    return temp.parse::<f64>().unwrap();
}

pub fn get_sysinfo() {
    // Get system info dump

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg("cat /proc/cpuinfo")
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn get_osinfo() {
    // Get info about the operating system from os-release

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg("cat /etc/os-release")
        .output()
        .expect("Failed to execute command");

    println!(
        "
{}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn get_osarch() {
    // Get info about os architecture w/ uname

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg("uname -m")
        .output()
        .expect("Failed to execute command");

    println!(
        "
Your cpu architecutre: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn get_mem() {
    // Get memory w/ free -h

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg("free -h")
        .output()
        .expect("Failed to execute command");

    println!(
        "
{}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn get_voltages() {
    // Get internal voltages with sh
    // Stolen from https://www.maketecheasier.com/finding-raspberry-pi-system-information/

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg(
            r#"for id in core sdram_c sdram_i sdram_p ; do \
    echo -e "$id:\t$(vcgencmd measure_volts $id)" ; \
    done"#,
        )
        .output()
        .expect("Failed to execute command");

    println!(
        "
{}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}

pub fn get_disk_info() {
    // Get internal voltages with sh
    // Stolen from https://www.maketecheasier.com/finding-raspberry-pi-system-information/

    let output = Command::new("sudo")
        .arg("bash")
        .arg("-c")
        .arg(r#"df -h"#)
        .output()
        .expect("Failed to execute command");

    println!(
        "
{}",
        String::from_utf8_lossy(&output.stdout)
    );
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}

