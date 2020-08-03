use crate::io;
use libc::{sceIoRead, sceIoWrite, sceKernelStderr, sceKernelStdin, sceKernelStdout};

pub struct Stdin;
pub struct Stdout;
pub struct Stderr;

impl Stdin {
    pub fn new() -> io::Result<Stdin> {
        Ok(Stdin)
    }
}

impl io::Read for Stdin {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unsafe {
            let fd = sceKernelStdin();
            let ret = sceIoRead(fd, buf.as_ptr() as _, buf.len() as _);
            Ok(ret as usize)
        }
    }
}

impl Stdout {
    pub fn new() -> io::Result<Stdout> {
        Ok(Stdout)
    }
}

impl io::Write for Stdout {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let fd = sceKernelStdout();
            let ret = sceIoWrite(fd, buf.as_ptr() as _, buf.len());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl Stderr {
    pub fn new() -> io::Result<Stderr> {
        Ok(Stderr)
    }
}

impl io::Write for Stderr {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unsafe {
            let fd = sceKernelStderr();
            let ret = sceIoWrite(fd, buf.as_ptr() as _, buf.len());
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

// TODO: What's an appropriate size?
pub const STDIN_BUF_SIZE: usize = crate::sys_common::io::DEFAULT_BUF_SIZE;

// TODO: Determine what to do for this!
pub fn is_ebadf(_err: &io::Error) -> bool {
    true
}

pub fn panic_output() -> Option<impl io::Write> {
    Stderr::new().ok()
}
