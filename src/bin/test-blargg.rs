//! Binary for running Blargg test ROMs.

use std::env;

use zerodmg_codes::roms::blargg_tests;
use zerodmg_emulator::test_runner::{BlarggTestRunner, TestStatus};

const MAX_CYCLES: u64 = 100_000_000; // 100 million cycles

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let test_name = &args[1];

    if test_name == "--all" || test_name == "-a" {
        run_all_tests();
    } else if test_name == "--help" || test_name == "-h" {
        print_usage();
    } else {
        run_single_test(test_name);
    }
}

fn print_usage() {
    println!("Usage: test-blargg <test-name|--all>");
    println!();
    println!("Available tests:");
    println!("  cpu_instrs     - CPU instruction tests");
    println!("  halt_bug       - HALT bug test");
    println!("  instr_timing   - Instruction timing test");
    println!("  mem_timing     - Memory timing test");
    println!("  mem_timing_2   - Memory timing test 2");
    println!("  interrupt_time - Interrupt timing test");
    println!("  dmg_sound      - DMG sound test");
    println!("  oam_bug        - OAM bug test");
    println!("  cgb_sound      - CGB sound test");
    println!();
    println!("Options:");
    println!("  --all, -a      Run all tests");
    println!("  --help, -h     Show this help");
}

fn get_test_rom(name: &str) -> Option<Vec<u8>> {
    match name {
        "cpu_instrs" => Some(blargg_tests::cpu_instrs().to_bytes()),
        "halt_bug" => Some(blargg_tests::halt_bug().to_bytes()),
        "instr_timing" => Some(blargg_tests::instr_timing().to_bytes()),
        "mem_timing" => Some(blargg_tests::mem_timing().to_bytes()),
        "mem_timing_2" => Some(blargg_tests::mem_timing_2().to_bytes()),
        "interrupt_time" => Some(blargg_tests::interrupt_time().to_bytes()),
        "dmg_sound" => Some(blargg_tests::dmg_sound().to_bytes()),
        "oam_bug" => Some(blargg_tests::oam_bug().to_bytes()),
        "cgb_sound" => Some(blargg_tests::cgb_sound().to_bytes()),
        _ => None,
    }
}

fn run_single_test(name: &str) {
    let rom = match get_test_rom(name) {
        Some(r) => r,
        None => {
            eprintln!("Unknown test: {}", name);
            print_usage();
            return;
        }
    };

    println!("=== Running Blargg Test: {} ===", name);
    println!();

    let result = BlarggTestRunner::run_test(rom, MAX_CYCLES);

    println!("--- Serial Output ---");
    if result.output.is_empty() {
        println!("(no output)");
    } else {
        println!("{}", result.output);
    }
    println!();

    println!("--- Result ---");
    println!("Cycles: {}", result.cycles);
    println!("Final PC: 0x{:04X}", result.final_pc);
    print!("Status: ");
    match &result.status {
        TestStatus::Running => println!("RUNNING (unexpected)"),
        TestStatus::Passed => println!("✓ PASSED"),
        TestStatus::Failed => println!("✗ FAILED"),
        TestStatus::Timeout => println!("⏱ TIMEOUT (hit {} cycle limit)", MAX_CYCLES),
        TestStatus::Unimplemented(msg) => {
            println!("⚠ UNIMPLEMENTED");
            println!("  {}", msg);
        }
    }
}

fn run_all_tests() {
    let tests = [
        "cpu_instrs",
        "halt_bug",
        "instr_timing",
        "mem_timing",
        "mem_timing_2",
        "interrupt_time",
        "dmg_sound",
        "oam_bug",
        "cgb_sound",
    ];

    println!("=== Running All Blargg Tests ===");
    println!();

    let mut results = Vec::new();

    for test_name in tests {
        print!("{:20} ", test_name);

        let rom = get_test_rom(test_name).expect("known test");
        let result = BlarggTestRunner::run_test(rom, MAX_CYCLES);

        match &result.status {
            TestStatus::Running => print!("RUNNING"),
            TestStatus::Passed => print!("✓ PASSED"),
            TestStatus::Failed => print!("✗ FAILED"),
            TestStatus::Timeout => print!("⏱ TIMEOUT @ 0x{:04X}", result.final_pc),
            TestStatus::Unimplemented(msg) => {
                let short_msg = if msg.len() > 40 {
                    format!("{}...", &msg[..40])
                } else {
                    msg.clone()
                };
                print!("⚠ {}", short_msg);
            }
        }
        println!(" ({} cycles)", result.cycles);

        results.push((test_name, result));
    }

    println!();
    println!("=== Summary ===");
    let passed = results
        .iter()
        .filter(|(_, r)| r.status == TestStatus::Passed)
        .count();
    let failed = results
        .iter()
        .filter(|(_, r)| r.status == TestStatus::Failed)
        .count();
    let timeout = results
        .iter()
        .filter(|(_, r)| r.status == TestStatus::Timeout)
        .count();
    let unimpl = results
        .iter()
        .filter(|(_, r)| matches!(r.status, TestStatus::Unimplemented(_)))
        .count();

    println!("Passed:        {}", passed);
    println!("Failed:        {}", failed);
    println!("Timeout:       {}", timeout);
    println!("Unimplemented: {}", unimpl);
}
