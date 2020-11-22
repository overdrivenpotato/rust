use crate::mem;
use crate::io;

#[derive(Copy, Clone)]
pub struct IoSlice<'a>(&'a [u8]);

impl<'a> IoSlice<'a> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> IoSlice<'a> {
        IoSlice(buf)
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        self.0 = &self.0[n..]
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }
}

pub struct IoSliceMut<'a>(&'a mut [u8]);

impl<'a> IoSliceMut<'a> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> IoSliceMut<'a> {
        IoSliceMut(buf)
    }

    #[inline]
    pub fn advance(&mut self, n: usize) {
        let slice = mem::replace(&mut self.0, &mut []);
        let (_, remaining) = slice.split_at_mut(n);
        self.0 = remaining;
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0
    }
}

pub(crate) fn cvt_io_error(err: i32) -> io::Error {
    match err {
        0x80010001 => io::Error::new(io::ErrorKind::PermissionDenied, "Operation not permitted"),
        0x80010002 => io::Error::new(io::ErrorKind::NotFound, "File not found"),
        0x80010003 => io::Error::new(io::ErrorKind::Other, "File open error"),
        0x80010009 => io::Error::new(io::ErrorKind::InvalidInput, "Invalid file descriptor"),
        0x8001000B => io::Error::new(io::ErrorKind::Other, "Resource unavailable"),
        0x8001000C => io::Error::new(io::ErrorKind::Other, "No memory"),
        0x8001000D => io::Error::new(io::ErrorKind::PermissionDenied, "No permissions"),
        0x8001000E => io::Error::new(io::ErrorKind::InvalidInput, "Invalid address"),
        0x80010010 => io::Error::new(io::ErrorKind::Other, "Device busy"),
        0x80010011 => io::Error::new(io::ErrorKind::AlreadyExists, "File already exists"),
        0x80010012 => io::Error::new(io::ErrorKind::Other, "Cross device link"),
        0x80010013 => io::Error::new(io::ErrorKind::InvalidInput, "Device not found"),
        0x80010014 => io::Error::new(io::ErrorKind::Other, "Not a directory"),
        0x80010015 => io::Error::new(io::ErrorKind::Other, "Is a directory"),
        0x80010016 => io::Error::new(io::ErrorKind::InvalidInput, "Invalid argument"),
        0x80010018 => io::Error::new(io::ErrorKind::Other, "Too many system files open"),
        0x8001001B => io::Error::new(io::ErrorKind::Other, "File is too big"),
        0x8001001C => io::Error::new(io::ErrorKind::Other, "No free space on device"),
        0x8001001E => io::Error::new(io::ErrorKind::PermissionDenied, "Read only"),
        0x80010020 => io::Error::new(io::ErrorKind::InvalidInput, "File is closed"),
        0x8001005B => io::Error::new(io::ErrorKind::InvalidInput, "Name/Path too long"),
        0x8001005C => io::Error::new(io::ErrorKind::Other, "Too many levels of symbolic links"),
        0x80010068 => io::Error::new(io::ErrorKind::ConnectionReset, "Connection Reset"),
        0x80010069 => io::Error::new(io::ErrorKind::Other, "Out of buffer space"),
        0x8001006E => io::Error::new(io::ErrorKind::Other, "Socket was shutdown"),
        0x80010070 => io::Error::new(io::ErrorKind::AddrInUse, "Address in use"),
        0x80010071 => io::Error::new(io::ErrorKind::ConnectionAborted, "Connection aborted"),
        0x80010074 => io::Error::new(io::ErrorKind::TimedOut, "Timed out"),
        0x80010077 => io::Error::new(io::ErrorKind::AlreadyExists, "Already in progress"),
        // Already what? It doesn't seem to be documented anywhere
        0x80010078 => io::Error::new(io::ErrorKind::AlreadyExists, "Already"),
        0x8001007B => io::Error::new(io::ErrorKind::InvalidInput, "Protocol is not supported"),
        0x8001007C => io::Error::new(io::ErrorKind::InvalidInput, "Invalid socket type"),
        0x8001007D => io::Error::new(io::ErrorKind::AddrNotAvailable, "Address not available"),
        0x8001007F => io::Error::new(io::ErrorKind::AlreadyExists, "Already connected"),
        0x80010080 => io::Error::new(io::ErrorKind::NotConnected, "Not connected"),
        0x80010084 => io::Error::new(io::ErrorKind::Other, "File quota exceeded"),
        0x80010086 => io::Error::new(io::ErrorKind::Other, "Not supported"),
        0x80010087 => io::Error::new(io::ErrorKind::Other, "No medium was found"),
        0x80020064 => io::Error::new(io::ErrorKind::Other, "Called from interrupt handler/thread"),
        0x800200d1 => io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied"),
        0x800200d3 => io::Error::new(io::ErrorKind::InvalidInput, "Illegal address"),
        0x80020130 => io::Error::new(io::ErrorKind::Other, "Read error"),
        0x80020190 => io::Error::new(io::ErrorKind::Other, "No memory"),
        0x80020320 => io::Error::new(io::ErrorKind::Other, "Too many open files"),
        0x80020321 => io::Error::new(io::ErrorKind::InvalidInput, "No such device"),
        0x80020323 => io::Error::new(io::ErrorKind::InvalidInput, "Bad file descriptor"),
        0x80020324 => io::Error::new(io::ErrorKind::InvalidInput, "Invalid argument"),
        0x80020325 => io::Error::new(io::ErrorKind::InvalidInput, "Unsupported operation"),
        0x8002032c => io::Error::new(io::ErrorKind::InvalidInput, "No current working directory"),
        0x8002032d => io::Error::new(io::ErrorKind::InvalidInput, "File name too long"),
        _ => io::Error::new(io::ErrorKind::Other, "Unknown"),
    }
}
