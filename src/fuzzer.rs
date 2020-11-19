use lain::{prelude::*, rand::rngs::StdRng};
use std::{
    marker::PhantomData,
    sync::{self, atomic},
};

use crate::traits::{ProduceInvalid, Target, TargetWithControl};

#[derive(Copy, Clone, Default, Debug)]
pub struct Fuzzer<T>{
    verbose_errors: bool,
    _danny: PhantomData<T>,
}

impl<T> Fuzzer<T>
where
    T: Target,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn verbose_errors(&self) -> bool {
        self.verbose_errors
    }

    pub fn set_verbose_errors(&mut self, v: bool) {
        self.verbose_errors = v
    }

    pub fn run(&self, threads: usize)
    where
        T: Target<Rng = StdRng>,
    {
        let verbose_errors = self.verbose_errors;

        _run(threads, move |mutator, ctx| {
            let mut target = T::new();
            let input = target.generate_next(mutator);
            let res = target.run_experimental(ctx, &input);
            let errs = res.iter().filter(|r| r.is_err());

            let mut is_err = false;
            errs.for_each(|e| {
                if verbose_errors {
                    let message = format!(
                        "Error on input:\n\t{}\n{}",
                        hex::encode(&input),
                        e.as_ref().unwrap_err()
                    );
                    println!("{}", &message);
                }
                is_err = true;
            });

            if is_err {
                return Err(());
            }

            Ok(())
        });
    }
}

impl<T> Fuzzer<T>
where
    T: ProduceInvalid,
{
    // produce invalid inputs to try to get panics
    pub fn run_invalid(&self, threads: usize)
    where
        T: ProduceInvalid<Rng = StdRng>,
    {
        _run(threads, move |mutator, ctx| {
            let mut target = T::new();
            // okay as long as it doesn't panic
            let input = target.generate_next_invalid(mutator);
            target.run_experimental(ctx, &input);

            Ok(())
        });
    }

    pub fn run_mixed(&self, threads: usize)
    where
        T: ProduceInvalid<Rng = StdRng>,
    {
        let verbose_errors = self.verbose_errors;

        _run(threads, move |mutator, ctx| {
            let mut target = T::new();

            if mutator.gen_chance(0.1) {
                let input = target.generate_next_invalid(mutator);
                // okay as long as it doesn't panic
                target.run_experimental(ctx, &input);
                Ok(())
            } else {
                let input = target.generate_next(mutator);
                let res = target.run_experimental(ctx, &input);
                let errs = res.iter().filter(|r| r.is_err());

                let mut is_err = false;
                errs.for_each(|e| {
                    if verbose_errors {
                        let message = format!(
                            "Error on input:\n\t{}\n{}",
                            hex::encode(&input),
                            e.as_ref().unwrap_err()
                        );
                        println!("{}", &message);
                    }
                    is_err = true;
                });
                if is_err {
                    return Err(());
                }

                Ok(())
            }
        })
    }
}

impl<T> Fuzzer<T>
where
    T: TargetWithControl,
{
    pub fn run_against_control(&self, threads: usize)
    where
        T: TargetWithControl<Rng = StdRng>,
    {
        let verbose_errors = self.verbose_errors;

        _run(threads, move |mutator, ctx| {
            let mut target = T::new();

            let input = target.generate(mutator);
            let res = target.compare(ctx, &input);

            let errs = res.iter().filter(|r| r.is_err());

            let mut is_err = false;
            errs.for_each(|e| {
                if verbose_errors {
                    let mut buf = vec![];
                    input.binary_serialize::<_, lain::byteorder::BigEndian>(&mut buf);
                    let message = format!(
                        "Error on input:\n\t{}\n{}",
                        hex::encode(&buf),
                        e.as_ref().unwrap_err()
                    );
                    println!("{}", &message);
                }
                is_err = true;
            });

            if is_err {
                Err(())
            } else {
                Ok(())
            }
        });
    }
}

pub(crate) fn _run<C, F>(threads: usize, callback: F)
where
    C: Default + 'static,
    F: Fn(&mut Mutator<StdRng>, &mut C) -> Result<(), ()> + Send + Sync + Copy + 'static,
{
    let stop_progress = atomic::AtomicBool::new(false);
    let stop_progress = sync::RwLock::from(stop_progress);
    let stop_progress = sync::Arc::from(stop_progress);

    let mut driver = lain::driver::FuzzerDriver::<atomic::AtomicBool>::new(threads);
    driver.set_global_context(stop_progress.clone());
    driver.set_seed(42);
    let driver = sync::Arc::from(driver);
    // driver.set_to_reproduce_mode(31150, 31200);

    // set up ctrl+c handling
    let ctrlc_driver = driver.clone();
    let ctrlc_driver_stop_progress = stop_progress.clone();
    ctrlc::set_handler(move || {
        ctrlc_driver_stop_progress
            .write()
            .unwrap()
            .store(true, atomic::Ordering::Relaxed);
        ctrlc_driver.signal_exit();
    })
    .expect("couldn't set CTRL-C handler");

    lain::driver::start_fuzzer(driver.clone(), move |mutator, ctx, stop_progress| {
        let res = callback(mutator, ctx);
        if res.is_err()
            && stop_progress
                .unwrap()
                .read()
                .unwrap()
                .load(atomic::Ordering::Relaxed)
        {
            Ok(()) // silence errors during shutdown
        } else {
            res
        }
    });

    let progress_driver = driver.clone();

    let progress_thread = std::thread::spawn(move || {
        use console::Style;
        use console::Term;

        let green = Style::new().green();
        let red = Style::new().red();

        let term = Term::stdout();
        // let term = Term::buffered_stdout();
        loop {
            let msg = format!(
                "Done {} iterations, {} failed iterations",
                green.apply_to(format!("{}", progress_driver.num_iterations())),
                red.apply_to(format!("{}", progress_driver.num_failed_iterations()))
            );
            let _ = term.write_line(&msg);
            std::thread::sleep(std::time::Duration::from_millis(5000));
            // let _ = term.clear_line();

            let stop = stop_progress
                .read()
                .unwrap()
                .load(atomic::Ordering::Relaxed);
            if stop {
                break;
            }
        }
    });

    driver.join_threads();
    progress_thread.join().unwrap();

    println!(
        "Finished in {} iterations, {} failed iterations",
        driver.num_iterations(),
        driver.num_failed_iterations()
    );
}
