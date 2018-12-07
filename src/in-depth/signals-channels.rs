use std::time::Duration;
use crossbeam_channel::{bounded, tick, Receiver, select};

fn ctrl_channel() -> Result<Receiver<()>, ctrlc::Error> {
    let (sender, receiver) = bounded(100);
    ctrlc::set_handler(move || {
        let _ = sender.send(());
    })?;

    Ok(receiver)
}

fn main() -> Result<(), exitfailure::ExitFailure> {
    let ctrls = ctrl_channel()?;
    let ticks = tick(Duration::from_secs(1));

    loop {
        select! {
            recv(ticks) -> _ => {
                println!("working!");
            }
            recv(ctrls) -> _ => {
                println!();
                println!("Goodbye!");
                break;
            }
        }
    }

    Ok(())
}
