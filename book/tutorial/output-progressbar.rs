fn main() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        do_hard_work();
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

fn do_hard_work() {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_millis(250));
}
