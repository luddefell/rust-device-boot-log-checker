# Rust device boot-log checker
- lets learn rust!

![Ferris, the Rust crab](assets/ferris.png)

Your program should:
1. Read boot.log with std::fs::read_to_string.
2. Split each line at the comma and parse the timestamp as u64.
3. Find POWER_ON and SERVICE_READY.
4. Calculate the elapsed time and compare it with a 500 ms limit.
5. Print FAIL if either event is missing.
