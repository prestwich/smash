use lain::{prelude::*, rand::rngs::StdRng};

use crate::traits::TargetWithControl;

fn _run<F>(threads: usize, callback: F)
where
    F: Fn(&mut Mutator<StdRng>) -> Result<(), ()> + Send + Sync + Copy + 'static
{
    let mut driver = lain::driver::FuzzerDriver::<()>::new(threads);
    driver.set_seed(42);
    let driver = std::sync::Arc::from(driver);

    let ctrlc_driver = driver.clone();
    let stop_progress = std::sync::atomic::AtomicBool::new(false);
    let stop_progress = std::sync::Arc::from(stop_progress);
    let ctrlc_driver_stop_progress = stop_progress.clone();

    ctrlc::set_handler(move || {
        ctrlc_driver_stop_progress.store(true, std::sync::atomic::Ordering::Relaxed);
        ctrlc_driver.signal_exit();
    })
    .expect("couldn't set CTRL-C handler");


    lain::driver::start_fuzzer(driver.clone(), move|mutator, _ctx: &mut (), _| {
        callback(mutator)
    });

    let progress_driver = driver.clone();

    let progress_thread = std::thread::spawn(move || {
        use console::Term;
        use console::Style;

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

            let stop = stop_progress.load(std::sync::atomic::Ordering::Relaxed);
            if stop {
                break;
            }
        }
    });

    driver.join_threads();
    progress_thread.join().unwrap();

    println!("Finished in {} iterations, {} failed iterations", driver.num_iterations(), driver.num_failed_iterations());

}

pub fn run<T>(threads: usize)
where
    T: TargetWithControl<Rng = StdRng>,
{
    _run(threads, move |mutator| {
        let target = T::new();

        let input = target.generate_next(mutator);

        let res = target.run_experimental(&input);

        if res.is_err() {
            let message = format!("{:?} {:?}", input, res);
            println!("{}", &message);
            return Err(());
        }

        Ok(())
    });
}


pub fn run_against_control<T>(threads: usize)
where
    T: TargetWithControl<Rng = StdRng>,
{
    _run(threads, move |mutator| {
        let target = T::new();

        let input = target.generate_next(mutator);

        let res = target.compare(&input);

        if res.is_err() {
            let message = format!("{:?} {:?}", input, res);
            println!("{}", &message);
            return Err(());
        }

        Ok(())
    });
}
