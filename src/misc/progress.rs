use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;


lazy_static!{
    static ref PROGRESS_STYLE: ProgressStyle =
        ProgressStyle::with_template("[{elapsed_precise}] {spinner} {bar:64} {pos:>4}/{len:4} {eta} {msg}")
            .expect("Progress style is invalid")
            .progress_chars("#<-")
            .tick_chars(r"-\|/.");
}

pub fn pretty_progress_bar(length: u64) -> ProgressBar {
    let bar = ProgressBar::new(length);
    bar.set_style(PROGRESS_STYLE.clone());
    bar.enable_steady_tick(Duration::from_millis(150));

    bar
}
