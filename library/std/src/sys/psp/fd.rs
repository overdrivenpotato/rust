#![unstable(reason = "not public", issue = "none", feature = "fd")]

use crate::cmp;
use crate::io::{self, Initializer, IoSlice, IoSliceMut, Read};
use crate::mem;
use crate::sys::decode_error_kind;
use crate::sys_common::AsInner;

use libc::{
    self, c_void, sceIoClose, sceIoLseek, sceIoRead, sceIoWrite, ssize_t, IoWhence, SceUid,
};

fn i_to_usize_or_errkind(i: i32) -> io::Result<usize> {
    if i < 0 {
        Err(io::Error::from(decode_error_kind(i)))
    } else {
        Ok(i as usize)
    }
}

//#[derive(Debug)]
pub struct FileDesc {
    fd: SceUid,
}

// TODO: Determine appropriate value for PSP
const READ_LIMIT: u32 = ssize_t::MAX as u32;

impl FileDesc {
    pub fn new(fd: SceUid) -> FileDesc {
        FileDesc { fd: fd }
    }

    pub fn raw(&self) -> SceUid {
        self.fd
    }

    /// Extracts the actual file descriptor without closing it.
    pub fn into_raw(self) -> SceUid {
        let fd = self.fd;
        mem::forget(self);
        fd
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let ret = unsafe { sceIoRead(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len() as u32) };
        Ok(ret as usize)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        crate::io::default_read_vectored(|buf| self.read(buf), bufs)
    }

    #[inline]
    fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn read_to_end(&self, buf: &mut Vec<u8>) -> io::Result<usize> {
        let mut me = self;
        (&mut me).read_to_end(buf)
    }

    pub fn read_at(&self, buf: &mut [u8], offset: u64) -> io::Result<usize> {
        unsafe {
            let seekret = sceIoLseek(self.fd, offset as i64, IoWhence::Set);
            if seekret < 0 {
                Err(io::Error::from(decode_error_kind(seekret as i32)))
            } else {
                i_to_usize_or_errkind(sceIoRead(
                    self.fd,
                    buf.as_mut_ptr() as *mut c_void,
                    cmp::min(buf.len() as u32, READ_LIMIT),
                ))
            }
        }
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let ret = unsafe { sceIoWrite(self.fd, buf.as_ptr() as *mut c_void, buf.len()) };
        Ok(ret as usize)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        crate::io::default_write_vectored(|buf| self.write(buf), bufs)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn write_at(&self, buf: &[u8], offset: u64) -> io::Result<usize> {
        unsafe {
            let seekret = sceIoLseek(self.fd, offset as i64, IoWhence::Set);
            if seekret < 0 {
                Err(io::Error::from(decode_error_kind(seekret as i32)))
            } else {
                i_to_usize_or_errkind(sceIoRead(
                    self.fd,
                    buf.as_ptr() as *mut c_void,
                    cmp::min(buf.len() as u32, READ_LIMIT),
                ))
            }
        }
    }

    //    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
    //        unsafe {
    //            let v = nonblocking as c_int;
    //            cvt(libc::ioctl(self.fd, libc::FIONBIO, &v))?;
    //            Ok(())
    //        }
    //    }
    //
    //    // refer to pxPipeDrv library documentation.
    //    // VxWorks uses fcntl to set O_NONBLOCK to the pipes
    //    pub fn set_nonblocking_pipe(&self, nonblocking: bool) -> io::Result<()> {
    //        unsafe {
    //            let mut flags = cvt(libc::fcntl(self.fd, libc::F_GETFL, 0))?;
    //            flags = if nonblocking {
    //                flags | libc::O_NONBLOCK
    //            } else {
    //                flags & !libc::O_NONBLOCK
    //            };
    //            cvt(libc::fcntl(self.fd, libc::F_SETFL, flags))?;
    //            Ok(())
    //        }
    //    }

    //    pub fn duplicate(&self) -> io::Result<FileDesc> {
    //        let fd = self.raw();
    //        match cvt(unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) }) {
    //            Ok(newfd) => Ok(FileDesc::new(newfd)),
    //            Err(e) => return Err(e),
    //        }
    //    }
}

impl<'a> Read for &'a FileDesc {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        (**self).read(buf)
    }

    #[inline]
    unsafe fn initializer(&self) -> Initializer {
        Initializer::nop()
    }
}

impl AsInner<SceUid> for FileDesc {
    fn as_inner(&self) -> &SceUid {
        &self.fd
    }
}

impl Drop for FileDesc {
    fn drop(&mut self) {
        // Note that errors are ignored when closing a file descriptor. The
        // reason for this is that if an error occurs we don't actually know if
        // the file descriptor was closed or not, and if we retried (for
        // something like EINTR), we might close another valid file descriptor
        // (opened after we closed ours.
        let _ = unsafe { sceIoClose(self.fd) };
    }
}
