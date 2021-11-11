// https://github.com/torvalds/linux/blob/master/include/uapi/linux/fanotify.h

/* the following events that user-space can register for */
#[rustfmt::skip]
#[allow(dead_code)]
mod __fanotify {
pub const FAN_ACCESS:                   u32 = 0x00000001;  /* File was accessed */
pub const FAN_MODIFY:                   u32 = 0x00000002;  /* File was modified */
pub const FAN_ATTRIB:                   u32 = 0x00000004;  /* Metadata changed */
pub const FAN_CLOSE_WRITE:              u32 = 0x00000008;  /* Writtable file closed */
pub const FAN_CLOSE_NOWRITE:            u32 = 0x00000010;  /* Unwrittable file closed */
pub const FAN_OPEN:                     u32 = 0x00000020;  /* File was opened */
pub const FAN_MOVED_FROM:               u32 = 0x00000040;  /* File was moved from X */
pub const FAN_MOVED_TO:                 u32 = 0x00000080;  /* File was moved to Y */
pub const FAN_CREATE:                   u32 = 0x00000100;  /* Subfile was created */
pub const FAN_DELETE:                   u32 = 0x00000200;  /* Subfile was deleted */
pub const FAN_DELETE_SELF:              u32 = 0x00000400;  /* Self was deleted */
pub const FAN_MOVE_SELF:                u32 = 0x00000800;  /* Self was moved */
pub const FAN_OPEN_EXEC:                u32 = 0x00001000;  /* File was opened for exec */

pub const FAN_Q_OVERFLOW:               u32 = 0x00004000;  /* Event queued overflowed */
pub const FAN_FS_ERROR:                 u32 = 0x00008000;  /* Filesystem error */

pub const FAN_OPEN_PERM:                u64 = 0x00010000;  /* File open in perm check */
pub const FAN_ACCESS_PERM:              u64 = 0x00020000;  /* File accessed in perm check */
pub const FAN_OPEN_EXEC_PERM:           u64 = 0x00040000;  /* File open/exec in perm check */

pub const FAN_EVENT_ON_CHILD:           u64 = 0x08000000;  /* Interested in child events */

pub const FAN_ONDIR:                    u64 = 0x40000000;  /* Event occurred against dir */

/* helper events */
pub const FAN_CLOSE:                    u32 = FAN_CLOSE_WRITE | FAN_CLOSE_NOWRITE; /* close */
pub const FAN_MOVE:                     u32 = FAN_MOVED_FROM | FAN_MOVED_TO;  /* moves */

/* flags used for fanotify_init() */
pub const FAN_CLOEXEC:                  u32 = 0x00000001;
pub const FAN_NONBLOCK:                 u32 = 0x00000002;

/* These are NOT bitwise flags.  Both bits are used together.  */
pub const FAN_CLASS_NOTIF:              u32 = 0x00000000;
pub const FAN_CLASS_CONTENT:            u32 = 0x00000004;
pub const FAN_CLASS_PRE_CONTENT:        u32 = 0x00000008;

pub const FAN_UNLIMITED_QUEUE:          u32 = 0x00000010;
pub const FAN_UNLIMITED_MARKS:          u32 = 0x00000020;
pub const FAN_ENABLE_AUDIT:             u32 = 0x00000040;

/* Flags to determine fanotify event format */
pub const FAN_REPORT_PIDFD:             u32 = 0x00000080;  /* Report pidfd for event->pid */
pub const FAN_REPORT_TID:               u32 = 0x00000100;  /* event->pid is thread id */
pub const FAN_REPORT_FID:               u32 = 0x00000200;  /* Report unique file id */
pub const FAN_REPORT_DIR_FID:           u32 = 0x00000400;  /* Report unique directory id */
pub const FAN_REPORT_NAME:              u32 = 0x00000800;  /* Report events with name */

/* Convenience macro - FAN_REPORT_NAME requires FAN_REPORT_DIR_FID */
pub const FAN_REPORT_DFID_NAME:         u32 = FAN_REPORT_DIR_FID | FAN_REPORT_NAME;


/* flags used for fanotify_modify_mark() */
pub const FAN_MARK_ADD:                 u8 = 0x00000001;
pub const FAN_MARK_REMOVE:              u8 = 0x00000002;
pub const FAN_MARK_DONT_FOLLOW:         u8 = 0x00000004;
pub const FAN_MARK_ONLYDIR:             u8 = 0x00000008;
/* FAN_MARK_MOUNT is        0x00000010 */
pub const FAN_MARK_IGNORED_MASK:        u16 = 0x00000020;
pub const FAN_MARK_IGNORED_SURV_MODIFY: u16 = 0x00000040;
pub const FAN_MARK_FLUSH:               u16 = 0x00000080;
/* FAN_MARK_FILESYSTEM is   0x00000100 */     

/* These are NOT bitwise flags.  Both bits can be used togther.  */
pub const FAN_MARK_INODE:               u32 = 0x00000000;
pub const FAN_MARK_MOUNT:               u32 = 0x00000010;
pub const FAN_MARK_FILESYSTEM:          u32 = 0x00000100;


pub const FANOTIFY_METADATA_VERSION:    u8 = 3;

// XXX: don't use; already exposed by libc
#[allow(non_camel_case_types)]
#[repr(C)]
#[repr(align(8))]
struct fanotify_event_metadata {
    event_len:    libc::__u32,
    vers:         libc::__u8,
    reserved:     libc::__u8,
    metadata_len: libc::__u16,
    mask:         libc::__u64,
    fd:           libc::__s32,
    pid:          libc::__s32,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum InfoType {
    FAN_EVENT_INFO_TYPE_FID       = 1,
    FAN_EVENT_INFO_TYPE_DFID_NAME = 2,
    FAN_EVENT_INFO_TYPE_DFID      = 3,
    FAN_EVENT_INFO_TYPE_PIDFD     = 4,
    FAN_EVENT_INFO_TYPE_ERROR     = 5,
}

/* Variable length info record following event metadata */
#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct fanotify_event_info_header {
    pub info_type: InfoType,  // TODO
    pub pad:       libc::__u8,
    pub len:       libc::__u16,
}


#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct __kernel_fsid_t {
    pub val: [libc::c_int; 2],
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct file_handle {
    pub handle_bytes: libc::c_uint,
    pub handle_type:  libc::c_int,
    pub f_handle:     [libc::c_uchar; 0],
}

/*
 * Unique file identifier info record.
 * This structure is used for records of types FAN_EVENT_INFO_TYPE_FID,
 * FAN_EVENT_INFO_TYPE_DFID and FAN_EVENT_INFO_TYPE_DFID_NAME.
 * For FAN_EVENT_INFO_TYPE_DFID_NAME there is additionally a null terminated
 * name immediately after the file handle.
 */
#[derive(Debug)]
#[repr(C)]
#[allow(non_camel_case_types)]
pub struct fanotify_event_info_fid {
    pub hdr: fanotify_event_info_header,
    pub fsid: __kernel_fsid_t,
    /*
     * Following is an opaque struct file_handle that can be passed as
     * an argument to open_by_handle_at(2).
     */
    pub file_handle: [libc::c_uchar; 0],
}


/*
 * This structure is used for info records of type FAN_EVENT_INFO_TYPE_PIDFD.
 * It holds a pidfd for the pid that was responsible for generating an event.
 */
#[allow(non_camel_case_types)]
#[repr(C)]
struct fanotify_event_info_pidfd {
    hdr:   fanotify_event_info_header,
    pidfd: libc::__s32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct fanotify_event_info_error {
    hdr:         fanotify_event_info_header,
    error:       libc::__s32,
    error_count: libc::__u32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
struct fanotify_response {
    fd:       libc::__s32,
    response: libc::__u32,
}

/* Legit userspace responses to a _PERM event */
pub const FAN_ALLOW: u64 = 0x01;
pub const FAN_DENY:  u64 = 0x02;
pub const FAN_AUDIT: u64 = 0x10; /* Bit mask to create audit record for result */

/* No fd set in event */
pub const FAN_NOFD:    i8 = -1;
pub const FAN_NOPIDFD: i8 = FAN_NOFD;
pub const FAN_EPIDFD:  i8 = -2;


// #define FAN_EVENT_METADATA_LEN (sizeof(struct fanotify_event_metadata))
// #define FAN_EVENT_NEXT(meta, len) ((len) -= (meta)->event_len, \
//                    (struct fanotify_event_metadata*)(((char *)(meta)) + \
//                    (meta)->event_len))
// 
// #define FAN_EVENT_OK(meta, len)  ((long)(len) >= (long)FAN_EVENT_METADATA_LEN && \
// (long)(meta)->event_len >= (long)FAN_EVENT_METADATA_LEN && \
// (long)(meta)->event_len <= (long)(len))

#[inline(always)]
pub unsafe fn fan_event_ok(meta: *const libc::fanotify_event_metadata, len: usize) -> bool {
    true
    && len >= std::mem::size_of::<libc::fanotify_event_metadata>()
    && (*meta).event_len as usize >= std::mem::size_of::<libc::fanotify_event_metadata>()
    && (*meta).event_len as usize >= len
}

} // mod __fanotify

pub use __fanotify::*;
