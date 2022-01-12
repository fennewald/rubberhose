/* sanity.rs
 * Sanity checks about the configs and stuff */
use config;

/// Calculates overhead of the internal data strucutres relative to tototal storage
fn overhead() -> f64 {
    return 0.0;
}

/// Address table size for a single partition
fn max_address_table_size () -> usize {
    config::MAX_PARTITION_SIZE;
}
