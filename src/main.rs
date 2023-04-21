use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{thread, time};

const DB_PATH: &str = "./temperture.db";
const INTERVAL: u64 = 5000;
const TEMPERATURE_FILE: &str = "/sys/class/thermal/thermal_zone0/temp";

fn init() {
    let query = "
    CREATE TABLE monitor (temperature REAL, timestamp INTEGER);
";

    let path = Path::new(DB_PATH);
    if !path.is_file() {
        let connection = sqlite::open(DB_PATH).unwrap();

        connection.execute(query).unwrap();
    }
}

fn load() {
    let connection = sqlite::open(DB_PATH).unwrap();

    macro_rules! query {
        () => {
            "INSERT INTO monitor VALUES ({temperature}, {timestamp});"
        };
    }

    let mut f = File::open(TEMPERATURE_FILE).unwrap();
    let mut c = String::new();
    f.read_to_string(&mut c).unwrap();

    // 45678 -> 45.678
    let temperature = format!("{}.{}", &c[0..2], &c[2..5]);

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let timestamp = since_the_epoch.as_secs();

    connection
        .execute(format!(
            query!(),
            temperature = temperature,
            timestamp = timestamp
        ))
        .unwrap()
}

fn main() {
    init();
    let ten_millis = time::Duration::from_millis(INTERVAL);
    loop {
        load();
        thread::sleep(ten_millis);
    }
}
