// This monitoring facility leveraging fanotify(7) is on hold as the API is
// limited for now. But this guy has been working on extending fanotify
// https://github.com/amir73il/linux
//
// Thi restriction is https://man7.org/linux/man-pages/man2/fanotify_init.2.html
// > The same rule that applies to record type
// > FAN_EVENT_INFO_TYPE_DFID also applies to record type
// > FAN_EVENT_INFO_TYPE_DFID_NAME: if a non-directory object
// > has no parent, either the event will not be reported or it
// > will be reported without the directory entry information.
// > Note that there is no guarantee that the filesystem object
// > will be found at the location described by the directory
// > entry information at the time the event is received.  See
// > fanotify(7) for additional details.

use std::ffi::CStr;
use std::sync::mpsc::Sender;
use std::{ffi::CString, os::unix::prelude::OsStrExt};

use crate::fanotify_header::*;
use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use libc;

// https://man7.org/linux/man-pages/man7/fanotify.7.html
pub unsafe fn prepare_fd<P: AsRef<std::path::Path>>(__path: P) -> Result<libc::c_int> {
    let path = CString::new(__path.as_ref().as_os_str().as_bytes()).with_context(|| "wtf")?;
    let mount_fd = libc::open(path.as_ptr(), libc::O_DIRECTORY | libc::O_RDONLY);
    assert_ne!(mount_fd, 1);

    let flags = 0
        | FAN_CLASS_NOTIF
        | FAN_UNLIMITED_QUEUE  // requires CAP_SYS_ADMIN
        | FAN_UNLIMITED_MARKS
        | FAN_CLOEXEC
        // | FAN_NONBLOCK
        | FAN_REPORT_FID
        | FAN_REPORT_DIR_FID
        | FAN_REPORT_NAME;
    let fanotify_fd = libc::fanotify_init(flags, libc::O_RDONLY as u32 | libc::O_LARGEFILE as u32);
    if fanotify_fd == -1 {
        bail!("{} <- fanotify_init(...)", nix::errno::Errno::last());
    }

    // let marks = 0 | FAN_MARK_ADD as u32 | FAN_MARK_ONLYDIR as u32 | FAN_MARK_FILESYSTEM;
    let marks = 0 | FAN_MARK_ADD as u32 | FAN_MARK_ONLYDIR as u32;
    let masks = 0
        | FAN_CREATE as u64
        | FAN_DELETE as u64
        | FAN_DELETE_SELF as u64
        | FAN_MOVED_FROM as u64
        | FAN_MOVED_TO as u64
        | FAN_MOVE_SELF as u64
        | FAN_ONDIR
        | FAN_EVENT_ON_CHILD;
    libc::fanotify_mark(
        fanotify_fd,
        marks as u32,
        masks,
        libc::AT_FDCWD,
        path.as_ptr(),
    );
    if fanotify_fd == -1 {
        bail!("{} <- fanotify_mark(...)", nix::errno::Errno::last());
    }

    Ok(fanotify_fd)
}

const BATCH: usize = 200;
type Metadata = libc::fanotify_event_metadata;
const FAN_EVENT_METADATA_LEN: usize = std::mem::size_of::<Metadata>();

pub unsafe fn handle_metadata(buf: *const Metadata, tx: &Sender<Event>) -> Result<()> {
    let md = &*buf;

    assert!(md.event_len as usize >= FAN_EVENT_METADATA_LEN);
    assert_eq!(md.vers, FANOTIFY_METADATA_VERSION);
    // use file_handle instead due to FAN_REPORT_FID or FAN_REPORT_DIR_FID
    assert_eq!(md.fd, FAN_NOFD as i32);

    // dbg!(md);
    dbg!(format!("md.mask = 0x{:x}", md.mask));
    /* dbg!(
        md.event_len - md.metadata_len as u32,
        std::mem::size_of::<fanotify_event_info_fid>(),
        std::mem::size_of::<fanotify_event_info_header>(),
        std::mem::size_of::<__kernel_fsid_t>(),
        std::mem::size_of::<file_handle>(),
    ); */

    match md.mask as u32 {
        FAN_CREATE => {
            let fid = &*(buf.offset(1) as *const _ as *const fanotify_event_info_fid);

            dbg!(&fid.hdr.info_type);
            if fid.hdr.info_type == InfoType::FAN_EVENT_INFO_TYPE_DFID_NAME {
                let file_handle = fid.file_handle.as_ptr() as *const _ as *const file_handle;
                assert!(!file_handle.is_null());
                let offset = (*file_handle).handle_bytes as isize;
                let ptr = (*file_handle).f_handle.as_ptr().offset(offset) as *const i8;
                let filename = CStr::from_ptr(ptr).to_owned();
                tx.send(Event::Create(filename.to_str()?.to_owned()))?;
            } else {
                tx.send(Event::Create("FAN_CREATE".into()))?;
            }
        }
        FAN_DELETE => {
            let fid = &*(buf.offset(1) as *const _ as *const fanotify_event_info_fid);

            dbg!(&fid.hdr.info_type);
            if fid.hdr.info_type == InfoType::FAN_EVENT_INFO_TYPE_DFID_NAME {
                let file_handle = fid.file_handle.as_ptr() as *const _ as *const file_handle;
                assert!(!file_handle.is_null());
                let offset = (*file_handle).handle_bytes as isize;
                let ptr = (*file_handle).f_handle.as_ptr().offset(offset) as *const i8;
                let filename = CStr::from_ptr(ptr).to_owned();
                tx.send(Event::Delete(filename.to_str()?.to_owned()))?;
            } else {
                tx.send(Event::Delete("FAN_DELETE".into()))?;
            }
        }
        FAN_DELETE_SELF => {
            println!("FAN_DELETE_SELF = 0x{:x}", FAN_DELETE_SELF);
            tx.send(Event::Delete("FAN_DELETE_SELF".into()))?;
        }
        FAN_MOVE => {
            println!("FAN_MOVED_FROM = 0x{:x}", FAN_MOVED_FROM);
            tx.send(Event::Create("FAN_MOVED_FROM".into()))?;
        }
        FAN_MOVE_SELF => {
            println!("FAN_MOVE_SELF = 0x{:x}", FAN_MOVE_SELF);
            tx.send(Event::Create("FAN_MOVED_SELF".into()))?;
        }
        // TODO: FAN_ONDIR in conjuction with type bits
        other => tx.send(Event::Create(format!("{:x}", other)))?,
    };

    match md.mask {
        _ if md.mask == FAN_ONDIR | FAN_CREATE as u64 => {}
        _ if md.mask == FAN_ONDIR | FAN_DELETE as u64 => {}
        _ if md.mask == FAN_ONDIR | FAN_DELETE_SELF as u64 => {}
        _ if md.mask == FAN_ONDIR | FAN_MOVE as u64 => {}
        _ if md.mask == FAN_ONDIR | FAN_MOVE_SELF as u64 => {}
        _ => {}
    };

    Ok(())
}

pub unsafe fn poll(fd: libc::c_int, tx: &Sender<Event>) -> Result<()> {
    let buf = std::alloc::alloc_zeroed(std::alloc::Layout::new::<[Metadata; BATCH]>());

    loop {
        let mut len = libc::read(fd, buf as *mut libc::c_void, FAN_EVENT_METADATA_LEN * BATCH);
        if len == -1 {
            bail!("{} <- read(...)", nix::errno::Errno::last());
        }

        let md = &*(buf as *const Metadata);

        while fan_event_ok(md, len as usize) {
            handle_metadata(md as *const Metadata, tx)?;
            len -= md.event_len as isize;
        }
    }
}

#[derive(Debug)]
pub enum Event {
    Create(String),
    Delete(String),
    Move(String, String),
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
