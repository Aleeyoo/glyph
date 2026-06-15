//! Benchmark suite: startup time, keystroke latency, file load.
//!
//! Run with: cargo bench
//! These are simple throughput tests, not microbenchmarks.

use glyph::buffer::text::GapBuffer;
use glyph::util::killring::KillRing;

fn main() {
    println!("glyph benchmarks");
    println!("================");

    // GapBuffer insert throughput
    let start = std::time::Instant::now();
    let mut gb = GapBuffer::new();
    let chunk = "a".repeat(1000);
    for _ in 0..1000 {
        gb.insert_at(gb.len(), chunk.as_bytes());
    }
    let elapsed = start.elapsed();
    println!("GapBuffer 1M chars insert: {:?} ({} MB/s)",
        elapsed,
        (1000000u64) / elapsed.as_micros().max(1) as u64);

    // KillRing throughput
    let start = std::time::Instant::now();
    let mut kr = KillRing::new();
    for i in 0..10000 {
        kr.push(&format!("entry-{}", i), false);
    }
    let elapsed = start.elapsed();
    println!("KillRing 10k push: {:?}", elapsed);

    // Startup simulation (Editor creation)
    use glyph::editor::Editor;
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let _ed = Editor::new(24, 80);
    }
    let elapsed = start.elapsed();
    println!("Editor creation (x100): {:?}", elapsed);
}
