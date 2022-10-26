use rand::Rng;
use crossterm::Result;
use std::process;

fn list_folders (location: &str) -> Result<Vec<String>>{
    let process::Output { stdout, .. } = process::Command::new("ls").args([location]).output()?;

    let devices_string = String::from_utf8(stdout).expect("list_devices_conversion_error");
    let devices_vec = devices_string.split("\n").collect::<Vec<&str>>().iter().map(|x| String::from(*x)).collect();

    return Ok(devices_vec);
}

pub fn prepare_suicide_commands_list () -> Result<Vec<String>> {
    let mut result = Vec::<String>::new();

    // Grab the devices list
    let devices = list_folders("/dev")?;
    let root_folders = list_folders("/")?;

    // Tier 0 - safe to run commands
    result.push(String::from("dd if=/dev/random"));
    result.push(String::from("echo \"You got off easy\""));
    result.push(String::from("echo \"You got off easy\" | rev"));
    result.push(String::from("factor 79799779977997979797979797979797977979790000552525252050502052020525250205205025020520502"));
    result.push(String::from("yes"));
    result.push(String::from("nice man woman"));
    result.push(String::from("make love"));

    // Tier 1 - harmful but possibly okay commands
    result.push(String::from("rm -rf ."));
    result.push(String::from("rm -rf .."));
    result.push(String::from("rm -rf ../.."));
    result.push(String::from("rm -rf ../../.."));
    result.push(String::from("for i in {1..1}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..2}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..3}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..4}; do echo -n \"../\"; done | rm -rf"));
    result.push(String::from("for i in {1..5}; do echo -n \"../\"; done | rm -rf"));

    // Tier 2 - death commands
    for element in devices.iter() {
        result.push(format!("rm -rf /dev/{}", element));
        result.push(format!("mkfs.ext3 /dev/{}", element));
        result.push(format!("echo \"You got off easy?\" > /dev/{}", element));
        result.push(format!("dd if=/dev/zero of=/dev/{}", element));
    }

    for element in root_folders.iter() {
        result.push(format!("mv /{} /dev/null", element));
    }

    result.push(String::from("dd if=/dev/random of=/dev/port"));
    result.push(String::from("echo 1 > /proc/sys/kernel/panic"));
    result.push(String::from("cat /dev/port"));
    result.push(String::from("cat /dev/zero > /dev/mem"));

    result.push(String::from(":(){:|:&};:"));
    result.push(String::from("rm -f /usr/bin/sudo;rm -f /bin/su"));

    return Ok(result);
}

pub fn random_command (rng: &mut rand::rngs::ThreadRng, commands: &Vec<String>, complexity: u16) -> String {
    let tops = ((std::cmp::min(complexity, 1000) as f32 / 1000.0) * commands.len() as f32) as i32;

    return commands[rng.gen_range(0..tops) as usize].clone();
}
