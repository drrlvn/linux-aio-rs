#![allow(non_camel_case_types)]

//! Thin unsafe wrapper for Linux AIO API.

use std::os::raw::{c_int, c_long, c_uint, c_ulong};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub unsafe fn io_setup(nr_events: c_uint, ctx_idp: *mut aio_context_t) -> c_int {
    libc::syscall(libc::SYS_io_setup, nr_events, ctx_idp) as c_int
}

pub unsafe fn io_destroy(ctx_id: aio_context_t) -> c_int {
    libc::syscall(libc::SYS_io_destroy, ctx_id) as c_int
}

pub unsafe fn io_submit(ctx_id: aio_context_t, nr: c_ulong, iocbpp: *mut *mut iocb) -> c_int {
    libc::syscall(libc::SYS_io_submit, ctx_id, nr, iocbpp) as c_int
}

pub unsafe fn io_cancel(ctx_id: aio_context_t, iocb: *mut iocb, result: *mut io_event) -> c_int {
    libc::syscall(libc::SYS_io_cancel, ctx_id, iocb, result) as c_int
}

pub unsafe fn io_getevents(
    ctx_id: aio_context_t,
    min_nr: c_long,
    nr: c_long,
    events: *mut io_event,
    timeout: *mut libc::timespec,
) -> c_int {
    libc::syscall(libc::SYS_io_getevents, ctx_id, min_nr, nr, events, timeout) as c_int
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io;
    use std::os::unix::io::AsRawFd;
    use std::ptr::null_mut;

    // Adapted from https://github.com/cloudflare/cloudflare-blog/blob/master/2019-01-io-submit/aio_passwd.c
    #[test]
    fn read_passwd() -> io::Result<()> {
        let file = File::open("/etc/passwd")?;

        let mut ctx: aio_context_t = 0;
        let mut r = unsafe { io_setup(128, &mut ctx) };
        assert_eq!(r, 0);

        let mut buf: [u8; 4096] = unsafe { std::mem::uninitialized() };
        let mut iocb = iocb {
            aio_fildes: file.as_raw_fd() as u32,
            aio_lio_opcode: IOCB_CMD_PREAD as u16,
            aio_buf: buf.as_mut_ptr() as u64,
            aio_nbytes: buf.len() as u64,
            ..Default::default()
        };
        r = unsafe { io_submit(ctx, 1, [&mut iocb as *mut iocb].as_mut_ptr()) };
        assert_eq!(r, 1);

        let mut events = [unsafe { std::mem::uninitialized::<io_event>() }];
        r = unsafe { io_getevents(ctx, 1, 1, events.as_mut_ptr(), null_mut()) };
        assert_eq!(r, 1);
        assert!(events[0].res > 0);

        r = unsafe { io_destroy(ctx) };
        assert_eq!(r, 0);

        Ok(())
    }
}
