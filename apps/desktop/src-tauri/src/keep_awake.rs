//! "Keep PC awake while sharing".
//!
//! Stops the computer from going to *sleep* while at least one window is
//! sharing with the option turned on. The display is still allowed to turn
//! off and the screen can still lock — only system sleep is blocked, which is
//! all a transfer needs. It does not touch the mouse or keyboard.
//!
//! - Windows: `SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED)`
//!   held on a dedicated thread (the request belongs to the calling thread).
//! - macOS:   `caffeinate -i -w <our pid>` (exits by itself if the app quits).
//! - Linux:   `systemd-inhibit --what=sleep` around a loop that ends with the app.

use std::collections::HashSet;
use std::sync::Mutex;

pub struct KeepAwake {
    inner: Mutex<Inner>,
}

struct Inner {
    /// Window labels currently asking to stay awake.
    holders: HashSet<String>,
    guard: Option<imp::Guard>,
}

impl KeepAwake {
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(Inner {
                holders: HashSet::new(),
                guard: None,
            }),
        }
    }

    /// Turns the request for `holder` (a window label) on or off. The PC stays
    /// awake while any holder is active and returns to normal once none are.
    pub fn set(&self, holder: &str, enabled: bool) -> Result<(), String> {
        let mut inner = self.inner.lock().unwrap();
        if enabled {
            inner.holders.insert(holder.to_string());
        } else {
            inner.holders.remove(holder);
        }

        if inner.holders.is_empty() {
            // Dropping the guard releases the sleep block.
            inner.guard = None;
        } else if inner.guard.is_none() {
            match imp::Guard::acquire() {
                Ok(guard) => inner.guard = Some(guard),
                Err(err) => {
                    inner.holders.remove(holder);
                    return Err(err);
                }
            }
        }
        Ok(())
    }
}

#[cfg(windows)]
mod imp {
    use std::sync::mpsc;
    use std::thread::JoinHandle;

    const ES_CONTINUOUS: u32 = 0x8000_0000;
    const ES_SYSTEM_REQUIRED: u32 = 0x0000_0001;

    #[link(name = "kernel32")]
    extern "system" {
        fn SetThreadExecutionState(flags: u32) -> u32;
    }

    pub struct Guard {
        stop: Option<mpsc::Sender<()>>,
        thread: Option<JoinHandle<()>>,
    }

    impl Guard {
        pub fn acquire() -> Result<Self, String> {
            let (stop_tx, stop_rx) = mpsc::channel::<()>();
            let (ready_tx, ready_rx) = mpsc::channel::<bool>();

            let thread = std::thread::Builder::new()
                .name("keep-awake".into())
                .spawn(move || {
                    let ok = unsafe { SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED) } != 0;
                    let _ = ready_tx.send(ok);
                    if !ok {
                        return;
                    }
                    // Wait until the guard is dropped (sender closed), then
                    // hand sleep control back to Windows.
                    let _ = stop_rx.recv();
                    unsafe {
                        SetThreadExecutionState(ES_CONTINUOUS);
                    }
                })
                .map_err(|e| format!("Couldn't keep this PC awake: {e}"))?;

            match ready_rx.recv() {
                Ok(true) => Ok(Guard {
                    stop: Some(stop_tx),
                    thread: Some(thread),
                }),
                _ => {
                    let _ = thread.join();
                    Err("Windows didn't allow the app to keep the PC awake.".into())
                }
            }
        }
    }

    impl Drop for Guard {
        fn drop(&mut self) {
            drop(self.stop.take());
            if let Some(thread) = self.thread.take() {
                let _ = thread.join();
            }
        }
    }
}

#[cfg(not(windows))]
mod imp {
    use std::process::{Child, Command, Stdio};

    pub struct Guard(Child);

    impl Guard {
        pub fn acquire() -> Result<Self, String> {
            let pid = std::process::id().to_string();

            #[cfg(target_os = "macos")]
            let mut cmd = {
                let mut c = Command::new("caffeinate");
                c.args(["-i", "-w", pid.as_str()]);
                c
            };

            #[cfg(not(target_os = "macos"))]
            let mut cmd = {
                let watch = format!("while kill -0 {pid} 2>/dev/null; do sleep 5; done");
                let mut c = Command::new("systemd-inhibit");
                c.args([
                    "--what=sleep",
                    "--who=PaulRussells File Share",
                    "--why=Sharing files",
                    "--mode=block",
                    "sh",
                    "-c",
                    watch.as_str(),
                ]);
                c
            };

            cmd.stdin(Stdio::null()).stdout(Stdio::null()).stderr(Stdio::null());
            cmd.spawn()
                .map(Guard)
                .map_err(|e| format!("Couldn't keep this computer awake: {e}"))
        }
    }

    impl Drop for Guard {
        fn drop(&mut self) {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}
