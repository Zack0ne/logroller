/// This is simulatin if there are lost of data during compression.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "xz")]
    {
        let start = Instant::now();
        use {
            logroller::{LogRollerBuilder, Rotation, RotationSize},
            std::{io::Write, time::Instant},
        };
        let mut logger = LogRollerBuilder::new("./logs", "sized.log")
            .rotation(Rotation::SizeBased(RotationSize::KB(1)))
            .max_keep_files(5) // Keep only last 5 files
            // Highest rate for making the compression takes longer
            .compression(logroller::Compression::XZ(9))
            .build()?;

        // Simulate writing logs that will trigger size-based rotation
        for i in 1..=10_000 {
            writeln!(
                logger,
                "Log entry #{i}: This is a sample log message that will contribute to file size"
            )?;
        }
        logger.flush()?;
        println!("Done logging: {:?}", start.elapsed());
    }

    #[cfg(not(feature = "xz"))]
    {
        println!("XZ compression example skipped. Enable 'xz' feature to run XZ compression example.");
        println!("Run with: cargo run --example rapid_compression --features xz");
    }

    Ok(())
}
