use std::{process::Command, fs, path::Path};
use substring::Substring;

// -------- Misc Commands --------


// -------- Set commands --------

//leds; see https://forums.raspberrypi.com/viewtopic.php?t=12530
pub fn set_led_driver(iled: &str, driver: &str) {
    // First match "red" and "green"
    let mut led = iled;
    
    match led.to_lowercase().as_str() {
        "red" => led = "1",
        "green" => led = "0",
        _ => led = iled,
    }


    // Write the correct driver to the trigger file
    
    let mut ledpath =r#"/sys/class/leds/led"#.to_owned();
    ledpath.push_str(led);
    ledpath.push_str(r#"/trigger"#);

    let path = Path::new(&ledpath);

    let contents = driver;

    fs::write(path, contents).unwrap();

}

pub fn led_on(iled: &str) {

    // First match "red" and "green"
    let mut led = iled;
    
    match led.to_lowercase().as_str() {
        "red" => led = "1",
        "green" => led = "0",
        _ => led = iled,
    }


    // Match driver
    let mut driver = "mmc0";

    match led {
        "0" => driver = "mmc0",
        "1" => driver = "default-on",
        _ => driver = driver,
    }

    set_led_driver(led, driver);


    // Write 1 to the brightness file
    let mut ledpath =r#"/sys/class/leds/led"#.to_owned();
    ledpath.push_str(led);
    ledpath.push_str(r#"/brightness"#);

    let path = Path::new(&ledpath);

    let contents = "1";

    fs::write(path, contents).unwrap();
}

pub fn led_off(iled: &str) {
    // First match "red" and "green"
    let mut led = iled;
    
    match led.to_lowercase().as_str() {
        "red" => led = "1",
        "green" => led = "0",
        _ => led = iled,
    }

    // No need to match driver here because it's always none

    set_led_driver(led, "none");
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

pub fn get_temp() {
    // Get temp w/ vcgencmd

    let output = Command::new("sudo")
    .arg("bash")
    .arg("-c")
    .arg("/usr/bin/vcgencmd measure_temp")
    .output()
    .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
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

    println!(" 
{}", String::from_utf8_lossy(&output.stdout));
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


    println!("
Your cpu architecutre: {}", String::from_utf8_lossy(&output.stdout));
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

    
    println!("
{}", String::from_utf8_lossy(&output.stdout));
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
    .arg(r#"for id in core sdram_c sdram_i sdram_p ; do \
    echo -e "$id:\t$(vcgencmd measure_volts $id)" ; \
    done"#)
    .output()
    .expect("Failed to execute command");

    
    println!("
{}", String::from_utf8_lossy(&output.stdout));
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

    println!("
{}", String::from_utf8_lossy(&output.stdout));
    println!("");
    println!("status: {}", output.status);
    println!("err: {}", String::from_utf8_lossy(&output.stderr));
}