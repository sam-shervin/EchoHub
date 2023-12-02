use std::{
    env::var,
    sync::atomic::{AtomicU64, Ordering::Relaxed},
};

use chrono::Local;
use log::warn;

/// The epoch used for generating snowflake IDs.
const EPOCH: u64 = 1_672_511_400;

/// The last part of the snowflake ID.
static GENERATED_IDS: AtomicU64 = AtomicU64::new(0);

/// Generates a unique snowflake ID.
pub fn generate_snowflake() -> String {
    let process_id = match var("PROCESS_ID")
        .unwrap_or(String::from("0"))
        .parse::<u64>()
    {
        Ok(process_id) => process_id,
        Err(e) => {
            warn!("PROCESS_ID must be a valid integer, defaulting to 0: {}", e);
            0
        }
    };

    let worker_id = match var("PROCESS_ID")
        .unwrap_or(String::from("0"))
        .parse::<u64>()
    {
        Ok(worker_id) => worker_id,
        Err(e) => {
            warn!("WORKER_ID must be a valid integer, defaulting to 0: {}", e);
            0
        }
    };

    let snowflake_base = (process_id << 17) + (worker_id << 12);
    let snowflake = snowflake_base
        + ((Local::now().timestamp_millis() as u64 - EPOCH) << 22)
        + GENERATED_IDS.fetch_add(1, Relaxed);

    snowflake.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_snowflake() {
        let snowflake = generate_snowflake();
        assert_eq!(snowflake.len(), 19);
    }

    #[test]
    fn test_generate_snowflake_multiple() {
        let mut snowflakes = Vec::new();
        for _ in 0..10 {
            let snowflake = generate_snowflake();
            assert!(!snowflakes.contains(&snowflake));
            snowflakes.push(snowflake);
        }
    }
}
