use signal_hook::{iterator::Signals, SIGINT};
use std::{error::Error, thread, time::Duration};

fn main() -> Result<(), Box<Error>> {
    let signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            println!("Received signal {:?}", sig);
        }
    });
    thread::sleep(Duration::from_secs(2));

    Ok(())
}
