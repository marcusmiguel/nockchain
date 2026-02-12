use std::sync::Arc;
use std::path::PathBuf;
use std::fs;
use chumsky::Parser;
use std::time::Duration;
use parser::{native_pile_parser, native_hoon_parser};
use parser::utils::{LineMap, diff_noun, print_noun, parse_file_and_diff, collect_inputs};
use parser::noun::{pile_to_noun, hoon_to_noun};
use nockvm::noun::{D, T, Noun};
use nockapp::noun::slab::{slab_mug, slab_noun_equality, NockJammer, NounSlab};
use nockvm_macros::tas;
use bytes::Bytes;

#[test]
fn test_all_hoon_files() {
    use std::time::{Instant, Duration};
    use std::io::Write;

    let dir_path = PathBuf::from("../../hoon");
    let hoon_files = collect_inputs(&dir_path);

    assert!(!hoon_files.is_empty(),
        "No .hoon files found in {}", dir_path.display());

    let total_files = hoon_files.len();
    let file_num_width = total_files.to_string().len();

    println!("\n  Testing {} .hoon files from {}\n",
        total_files,
        dir_path.display()
    );

    let mut passed = 0;
    let mut failed = Vec::new();
    let mut total_file_time = Duration::ZERO;
    let mut total_native_time = Duration::ZERO;
    let mut total_expected_time = Duration::ZERO;

    for (i, file_path) in hoon_files.iter().enumerate() {
        let file_name = file_path.file_name()
            .unwrap_or_default()
            .to_string_lossy();

        let idx = i + 1;
        eprint!("  {:>width$}/{}  {:<30}  ",
            idx,
            total_files,
            file_name,
            width = file_num_width
        );
        std::io::stderr().flush().unwrap();

        let start_time = Instant::now();

        match parse_file_and_diff(file_path) {
            Ok((_, native_time, expected_time)) => {
                let duration = start_time.elapsed();
                total_file_time += duration;
                total_native_time += native_time;
                total_expected_time += expected_time;

                println!("OK  native: {:>6}  hoonc: {:>6}",
                    format_duration(native_time),
                    format_duration(expected_time)
                );
                passed += 1;
            }
            Err(e) => {
                println!("FAIL  {}", e);
                failed.push((file_path.clone(), e.to_string()));
            }
        }
    }

    // Summary
    println!("\n  {}\n", "─".repeat(50));

    let pass_rate = (passed as f32 / total_files as f32) * 100.0;

    println!("  {:<15} {} / {} ({:.1}%)",
        "Result:",
        passed,
        total_files,
        pass_rate
    );

    if !failed.is_empty() {
        println!("  {:<15} {}", "Failed:", failed.len());
    }

    let pass_rate = (passed as f32 / total_files as f32) * 100.0;
    let ratio = if total_native_time.as_nanos() > 0 {
        total_expected_time.as_secs_f64() / total_native_time.as_secs_f64()
    } else {
        0.0
    };
    println!("  {:<15} {}", "Total Time:", format_duration(total_file_time));
    println!("  {:<15} native: {} hoonc: {}",
        "Parsers:",
        format_duration(total_native_time),
        format_duration(total_expected_time),
    );
    println!("  {:<15} {:.1}x faster",
        "Ratio:",
        ratio
    );


    if !failed.is_empty() {
        eprintln!("\n  Failed files:");
        for (file_path, error) in &failed {
            eprintln!("    - {}: {}", file_path.display(), error);
        }
        panic!("{} out of {} tests failed", failed.len(), total_files);
    }

    println!();
}

fn format_duration(d: Duration) -> String {
    if d.as_secs() >= 10 {
        format!("{:.0}s", d.as_secs_f64())
    } else if d.as_secs() > 0 {
        format!("{:.1}s", d.as_secs_f64())
    } else if d.as_millis() >= 100 {
        format!("{}ms", d.as_millis())
    } else if d.as_micros() >= 1000 {
        format!("{:.1}ms", d.as_micros() as f64 / 1000.0)
    } else if d.as_micros() > 0 {
        format!("{}µs", d.as_micros())
    } else {
        format!("{}ns", d.as_nanos())
    }
}