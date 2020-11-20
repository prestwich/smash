use clap::Clap;

use crate::{fuzzer::Fuzzer, traits::*};

// TODO: express this with less boileplate

#[derive(Clap)]
#[clap(version = "1.0", author = "James Prestwich <prestwich@clabs.co>")]
pub struct Opts {
    /// Set to print errors as they occur. Note: this currently will report
    /// false positives during thread shutdown.
    #[clap(short, long)]
    pub verbose_errors: bool,

    /// The number of fuzzer threads to run.
    #[clap(short, long, default_value = "4")]
    pub threads: usize,

    /// Operation mode. 0 to run valid inputs. 1 to run valid inputs against a
    /// control. 2 to run mixed valid and invalid inputs. 3 to run invalid
    /// inputs only.
    #[clap(short, long, default_value = "0")]
    pub mode: usize,
}

fn mode_name(mode: usize) -> &'static str {
    match mode {
        0 => "0 (run valid inputs)",
        1 => "1 (run valid inputs against control)",
        2 => "2 (run with invalid inputs)",
        3 => "3 (run invalid only)",
        _ => "unknown mode",
    }
}

fn setup<T>() -> (Opts, Fuzzer<T>)
where
    T: Target,
{
    let opts = Opts::parse();

    let fuzzer = T::new_fuzzer()
        .set_verbose_errors(opts.verbose_errors)
        .set_threads(opts.threads);

    println!(
        "Running {} on mode {} with {} threads",
        T::name(),
        mode_name(opts.mode),
        opts.threads
    );

    (opts, fuzzer)
}

pub fn target<T>()
where
    T: Target<Rng = lain::rand::rngs::StdRng>,
{
    let (opts, fuzzer) = setup::<T>();

    match opts.mode {
        0 => fuzzer.run(),
        _ => println!(
            "Unsupported mode: {}. Supported for {} is 0",
            mode_name(opts.mode),
            T::name()
        ),
    }
}

pub fn target_with_control<T>()
where
    T: TargetWithControl + Target<Rng = lain::rand::rngs::StdRng>,
{
    let (opts, fuzzer) = setup::<T>();

    match opts.mode {
        0 => fuzzer.run(),
        1 => fuzzer.run_against_control(),
        _ => println!(
            "Unsupported mode: {}. Supported for {} are 0 & 1",
            mode_name(opts.mode),
            T::name()
        ),
    }
}

pub fn produce_invalid<T>()
where
    T: ProduceInvalid + Target<Rng = lain::rand::rngs::StdRng>,
{
    let (opts, fuzzer) = setup::<T>();

    match opts.mode {
        0 => fuzzer.run(),
        // 1 => fuzzer.run_against_control(),
        2 => fuzzer.run_mixed(),
        3 => fuzzer.run_invalid(),
        _ => println!(
            "Unsupported mode: {}. Supported for {} are 0, 2, & 3",
            mode_name(opts.mode),
            T::name()
        ),
    }
}

pub fn produce_invalid_with_control<T>()
where
    T: TargetWithControl + ProduceInvalid + Target<Rng = lain::rand::rngs::StdRng>,
{
    let (opts, fuzzer) = setup::<T>();

    match opts.mode {
        0 => fuzzer.run(),
        1 => fuzzer.run_against_control(),
        2 => fuzzer.run_mixed(),
        3 => fuzzer.run_invalid(),
        _ => println!(
            "Unsupported mode: {}. Supported for {} are 0, 1, 2, & 3",
            mode_name(opts.mode),
            T::name()
        ),
    }
}
