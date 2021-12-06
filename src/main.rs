use anyhow::Result;
use fanotify::poll;
use std::io::Write;

use structopt::StructOpt;

use crate::fanotify::Event;

mod fanotify;
mod fanotify_header;
mod store;
mod dfa;

#[derive(Debug, StructOpt)]
#[structopt(name = "hunt")]
struct Opt {
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn print<T: std::fmt::Display>(path: T) -> std::io::Result<()> {
    println!("{}", path);
    std::io::stdout().flush()
}

fn handle_event(ev: &Event) -> std::io::Result<()> {
    match ev {
        c @ Event::Create(_) => print(c),
        d @ Event::Delete(_) => print(d),
        Event::Move(from, to) => print(format!("{} -> {}", from, to)),
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let fanotify_fd = unsafe { fanotify::prepare_fd(opt.path)? };

    let (tx, rx) = std::sync::mpsc::channel::<Event>();
    let send = std::thread::spawn(move || unsafe { poll(fanotify_fd, &tx) });
    let recv = std::thread::spawn(move || loop {
        let _ = match rx.recv() {
            Ok(ref e) => handle_event(e),
            Err(err) => print(err),
        };
    });
    send.join();
    recv.join();
    Ok(())
}

fn main() -> Result<()> {
    store::foo("home")?;
    Ok(())
}
