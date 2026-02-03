//! Test runner infrastructure for Blargg test ROMs.

use std::sync::{Arc, Mutex};

use crate::cpu::CPUController;
use crate::{GameBoy, Output};

/// Result of running a Blargg test ROM.
pub struct TestResult {
    /// Serial output captured from the test as a string.
    pub output: String,
    /// Number of CPU cycles executed.
    pub cycles: u64,
    /// Final status of the test.
    pub status: TestStatus,
    /// Final PC value (for debugging).
    pub final_pc: u16,
}

/// Status of a test execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestStatus {
    /// Test is still running (should not be returned).
    Running,
    /// Test passed successfully.
    Passed,
    /// Test failed.
    Failed,
    /// Test timed out (hit cycle limit).
    Timeout,
    /// Test hit an unimplemented feature.
    Unimplemented(String),
}

/// Runner for Blargg test ROMs.
pub struct BlarggTestRunner;

impl BlarggTestRunner {
    /// Run a test ROM for up to max_cycles.
    pub fn run_test(rom: Vec<u8>, max_cycles: u64) -> TestResult {
        let output_buffer = Arc::new(Mutex::new(Output::new()));
        let mut gameboy = GameBoy::new(rom, output_buffer);

        let mut cycles: u64 = 0;
        let mut last_output_len = 0;
        let mut cycles_since_output = 0;
        const IDLE_CYCLES_THRESHOLD: u64 = 1_000_000;

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            while cycles < max_cycles {
                let opex = gameboy.tick();
                let tick_cycles = opex.t_1 - opex.t_0;
                cycles += tick_cycles;

                // Check for new serial output
                let current_output_len = gameboy.serial_output().len();
                if current_output_len > last_output_len {
                    last_output_len = current_output_len;
                    cycles_since_output = 0;
                } else {
                    cycles_since_output += tick_cycles;
                }

                // If no output for a while and we have some output, test might be done
                if cycles_since_output > IDLE_CYCLES_THRESHOLD && last_output_len > 0 {
                    break;
                }
            }
            gameboy
        }));

        match result {
            Ok(gameboy) => {
                let output = String::from_utf8_lossy(gameboy.serial_output()).to_string();
                let final_pc = gameboy.pc();
                let status = if cycles >= max_cycles {
                    TestStatus::Timeout
                } else {
                    parse_test_status(&output)
                };
                TestResult {
                    output,
                    cycles,
                    status,
                    final_pc,
                }
            }
            Err(panic_info) => {
                let panic_msg = if let Some(s) = panic_info.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = panic_info.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic".to_string()
                };
                TestResult {
                    output: String::new(),
                    cycles,
                    status: TestStatus::Unimplemented(panic_msg),
                    final_pc: 0,
                }
            }
        }
    }
}

/// Parse the serial output to determine test status.
fn parse_test_status(output: &str) -> TestStatus {
    let output_lower = output.to_lowercase();
    if output_lower.contains("passed") {
        TestStatus::Passed
    } else if output_lower.contains("failed") {
        TestStatus::Failed
    } else {
        TestStatus::Running
    }
}
