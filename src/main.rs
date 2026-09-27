use std::collections::HashMap;

fn main() {
    let log = std::fs::read_to_string("logs/boot.log")
        .expect("could not read logs/boot.log");
    let mut events = HashMap::new();
    for line in log.lines() {
        let (timestamp, label) = line.split_once(',').unwrap();
        let timestamp = timestamp.parse::<u64>().expect("invalid timestamp");
        events.insert(label,timestamp);
    }
    if (events.contains_key("POWER_ON") && events.contains_key("SERVICE_READY")) {
        let elapsed_time = (events[(&"SERVICE_READY")] - events[(&"POWER_ON")]);
        if (elapsed_time < 500) {
            println!("Boot time passed and was {}", elapsed_time);
        }
        else {
            println!("Boot time failed and was {}", elapsed_time);
        }
    } else {
    println!("FAIL");
}
    
}
