use crossbeam_channel::{Sender, bounded};
use std::io::stdin;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use std::thread::{self, JoinHandle};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub struct QuitChecker {
    quit_flag: Arc<AtomicBool>,
    stop_sender: Sender<()>,
    handle: Option<JoinHandle<()>>,
}

impl QuitChecker {
    pub fn new() -> Self {
        let quit_flag = Arc::new(AtomicBool::new(false));
        let flag_clone = Arc::clone(&quit_flag);

        let (stop_sender, stop_receiver) = bounded::<()>(1);

        let handle = thread::spawn(move || {
            let stdin = stdin();
            let stdin = stdin.lock();
            let _raw = termion::raw::RawTerminal::from(std::io::stdout().into_raw_mode().unwrap());

            let keys = stdin.keys();

            for key in keys {
                // Check for shutdown signal first
                if stop_receiver.try_recv().is_ok() {
                    break;
                }

                if let Ok(termion::event::Key::Char('q')) = key {
                    flag_clone.store(true, Ordering::SeqCst);
                    break;
                }
            }
        });

        QuitChecker {
            quit_flag,
            stop_sender,
            handle: Some(handle),
        }
    }

    pub fn should_quit(&self) -> bool {
        self.quit_flag.load(Ordering::SeqCst)
    }
}

impl Drop for QuitChecker {
    fn drop(&mut self) {
        let _ = self.stop_sender.send(());
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}
