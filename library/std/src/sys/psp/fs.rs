use crate::ffi::{CString, OsString, c_void};
use crate::fmt;
use crate::hash::{Hash, Hasher};
use crate::io::{self, IoSlice, IoSliceMut, SeekFrom};
use crate::path::{Path, PathBuf};
use crate::sys::time::SystemTime;
use crate::sys::{unsupported, Void};
use core::time::Duration;

pub struct File(libc::SceUid);

#[derive(Copy, Clone)]
pub struct FileAttr(libc::SceIoStat);

pub struct ReadDir(Void);

pub struct DirEntry(libc::SceIoDirent);

#[derive(Clone, Debug)]
pub struct OpenOptions {
    flags: i32,
    perms: libc::IoPermissions,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct FilePermissions(libc::IoPermissions);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct FileType(_FileType);

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum _FileType {
    Symlink,
    Directory,
    File
}

#[derive(Debug)]
pub struct DirBuilder {}

impl FileAttr {
    pub fn size(&self) -> u64 {
        self.0.st_size as u64
    }

    pub fn perm(&self) -> FilePermissions {
        FilePermissions(self.0.st_mode & 0o777)
    }

    pub fn file_type(&self) -> FileType {
        if self.0.st_attr & libc::FIO_SO_IFLNK != 0 {
            return FileType(_FileType::Symlink)
        }
        if self.0.st_attr & libc::FIO_SO_IFDIR != 0 {
            return FileType(_FileType::Directory)
        }
        if self.0.st_attr & libc::FIO_SO_IFREG != 0 {
            return FileType(_FileType::File)
        }
        unreachable!()
    }

    pub fn modified(&self) -> io::Result<SystemTime> {
        SystemTime::try_from_psp_time(&self.0.st_mtime).map_err(|_|
            io::Error::new(io::ErrorKind::Other, "Invalid file modification date")
        )
    }

    pub fn accessed(&self) -> io::Result<SystemTime> {
        SystemTime::try_from_psp_time(&self.0.st_atime).map_err(|_|
        io::Error::new(io::ErrorKind::Other, "Invalid file access date")
        )
    }

    pub fn created(&self) -> io::Result<SystemTime> {
        SystemTime::try_from_psp_time(&self.0.st_ctime).map_err(|_|
            io::Error::new(io::ErrorKind::Other, "Invalid file creation date")
        )
    }
}

impl FilePermissions {
    pub fn readonly(&self) -> bool {
        unimplemented!()
    }

    pub fn set_readonly(&mut self, _readonly: bool) {
        unimplemented!()
    }
}

impl FileType {
    pub fn is_dir(&self) -> bool {
        match self.0 {
            _FileType::Directory => true,
            _ => false,
        }
    }

    pub fn is_file(&self) -> bool {
        match self.0 {
            _FileType::File => true,
            _ => false,
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self.0 {
            _FileType::Symlink => true,
            _ => false,
        }
    }
}

impl fmt::Debug for ReadDir {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {}
    }
}

impl Iterator for ReadDir {
    type Item = io::Result<DirEntry>;

    fn next(&mut self) -> Option<io::Result<DirEntry>> {
        match self.0 {}
    }
}

impl DirEntry {
    pub fn path(&self) -> PathBuf {
        unimplemented!()
    }

    pub fn file_name(&self) -> OsString {
        unimplemented!()
    }

    pub fn metadata(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn file_type(&self) -> io::Result<FileType> {
        unsupported()
    }
}

impl OpenOptions {
    pub fn new() -> OpenOptions {
        OpenOptions {
            flags: 0,
            perms: 0o666,
        }
    }

    pub fn read(&mut self, read: bool) {
        if read {
            self.flags |= libc::PSP_O_RD_ONLY;
        }
    }
    pub fn write(&mut self, write: bool) {
        if write {
            self.flags |= libc::PSP_O_WR_ONLY;
        }
    }
    pub fn append(&mut self, append: bool) {
        if append {
            self.flags != libc::PSP_O_APPEND;
        }
    }
    pub fn truncate(&mut self, truncate: bool) {
        if truncate {
            self.flags |= libc::PSP_O_TRUNC;
        }
    }
    pub fn create(&mut self, create: bool) {
        if create {
            self.flags |= libc::PSP_O_CREAT;
        }
    }
    pub fn create_new(&mut self, create_new: bool) {
        if create_new {
            self.flags |= libc::PSP_O_CREAT | libc::PSP_O_EXCL;
        }
    }
}

impl File {
    pub fn open(path: &Path, opts: &OpenOptions) -> io::Result<File> {
        let cstring = cstring(path)?;
        let open_result = unsafe {
            libc::sceIoOpen(cstring.as_c_str().as_ptr() as *const u8, opts.flags, opts.perms)
        };
        if open_result.0 < 0 {
            return Err(cvt_io_error(open_result.0));
        } else {
            Ok(File(open_result))
        }
    }

    pub fn file_attr(&self) -> io::Result<FileAttr> {
        unsupported()
    }

    pub fn fsync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn datasync(&self) -> io::Result<()> {
        unsupported()
    }

    pub fn truncate(&self, _size: u64) -> io::Result<()> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let read_result = unsafe {
            libc::sceIoRead(self.0, buf.as_mut_ptr() as *mut c_void, buf.len() as u32)
        };
        if read_result < 0 {
            return Err(cvt_io_error(read_result));
        } else {
            Ok(buf.len())
        }
    }

    pub fn read_vectored(&self, _bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        unimplemented!()
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let write_result = unsafe {
            libc::sceIoWrite(self.0, buf.as_ptr() as *const c_void, buf.len())
        };
        if write_result < 0 {
            return Err(cvt_io_error(write_result));
        } else {
            Ok(buf.len())
        }
    }

    pub fn write_vectored(&self, _bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        unimplemented!()
    }

    pub fn flush(&self) -> io::Result<()> {
        Ok(())
    }

    pub fn seek(&self, pos: SeekFrom) -> io::Result<u64> {
        //let (whence, pos) = match pos {
            //SeekFrom::Start(off) => (libc::IoWhence::Set, off as i64),
            //SeekFrom::End(off) => (libc::IoWhence::End, off),
            //SeekFrom::Current(off) => (libc::IoWhence::Cur, off),
        //};
        //Ok(unsafe{libc::sceIoLseek(self.0, pos, whence)} as u64)
        // broken somehow
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<File> {
        unsupported()
    }

    pub fn set_permissions(&self, _perm: FilePermissions) -> io::Result<()> {
        unsupported()
    }

    pub fn diverge(&self) -> ! {
        unimplemented!()
    }
}

impl DirBuilder {
    pub fn new() -> DirBuilder {
        DirBuilder {}
    }

    pub fn mkdir(&self, p: &Path) -> io::Result<()> {
        let cstring = cstring(p)?;
        let result = unsafe { libc::sceIoMkdir(cstring.as_c_str().as_ptr() as *const u8, 0o777) };
        if result < 0 {
            return Err(cvt_io_error(result));
        } else {
            Ok(())
        }
    }
}

fn cstring(path: &Path) -> io::Result<CString> {
    Ok(CString::new(path.to_str().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Path to str failed"))?)?)
}

impl fmt::Debug for File {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        unimplemented!()
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe { libc::sceIoClose(self.0) };
    }
}

pub fn readdir(p: &Path) -> io::Result<ReadDir> {
    unsupported()
    //let cstring = cstring(p)?;
    //let open_result = libc::sceIoDopen(cstring.as_c_str().as_ptr() as *const u8);
    //if open_result.0 < 0 {
        //// Need to enumerate errors to return the proper io::ErrorKind
        //unimplemented!()
    //} else {
        //let mut dirent: libc::SceIoDirent = core::mem::zeroed();
        //let read_result = libc::sceIoDread(open_result, &mut dirent); 
        //if read_result < 0 {
            //unimplemented!()
        //} else {
            //Ok(ReadDir(dirent))
        //}
    //}
    // I think maybe this is supposed to recursively build DirEntrys into a linked 
    // list or some shit
}

pub fn unlink(p: &Path) -> io::Result<()> {
    let cstring = cstring(p)?;
    let result = unsafe { libc::sceIoRemove(cstring.as_c_str().as_ptr() as *const u8) };
    if result < 0 {
        return Err(cvt_io_error(result));
    } else {
        Ok(())
    }
}

pub fn rename(old: &Path, new: &Path) -> io::Result<()> {
    let cstring_old = cstring(old)?;
    let cstring_new = cstring(new)?;
    let rename_result = unsafe { libc::sceIoRename(cstring_old.as_c_str().as_ptr() as *const u8, cstring_new.as_c_str().as_ptr() as *const u8) };
    if rename_result < 0 {
        return Err(cvt_io_error(rename_result));
    } else {
        Ok(())
    }
}

pub fn set_perm(p: &Path, perm: FilePermissions) -> io::Result<()> {
    let cstring = cstring(p)?;
    let mut stat: libc::SceIoStat = unsafe { core::mem::zeroed() };
    let getstat_result = unsafe { libc::sceIoGetstat(cstring.as_c_str().as_ptr() as *const u8, &mut stat)};
    if getstat_result < 0 {
        return Err(cvt_io_error(getstat_result));
    } else {
        let non_perm_mode_bits = stat.st_mode & 0x7e00;  
        stat.st_mode = non_perm_mode_bits | perm.0;
        let chstat_result = unsafe { libc::sceIoChstat(cstring.as_c_str().as_ptr() as *const u8, &mut stat, 0x0001) }; 
        if chstat_result < 0 {
            return Err(cvt_io_error(chstat_result));
        } else {
            Ok(())
        }
    }
}

pub fn rmdir(p: &Path) -> io::Result<()> {
    let cstring = cstring(p)?;
    let rm_result = unsafe { libc::sceIoRmdir(cstring.as_c_str().as_ptr() as *const u8) };
    if rm_result < 0 {
        return Err(cvt_io_error(rm_result));
    } else {
        Ok(())
    }
 
}

pub use crate::sys_common::fs::remove_dir_all;

pub fn readlink(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn symlink(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn link(_src: &Path, _dst: &Path) -> io::Result<()> {
    unsupported()
}

pub fn stat(p: &Path) -> io::Result<FileAttr> {
    let mut stat: libc::SceIoStat = unsafe { core::mem::zeroed() }; 
    let cstring = cstring(p)?;
    let stat_result = unsafe {
        libc::sceIoGetstat(cstring.as_c_str().as_ptr() as *const u8, &mut stat)
    };
    if stat_result < 0 {
        return Err(cvt_io_error(stat_result));
    } else {
        Ok(FileAttr(stat))
    }
   
}

pub fn lstat(_p: &Path) -> io::Result<FileAttr> {
    unsupported()
}

pub fn canonicalize(_p: &Path) -> io::Result<PathBuf> {
    unsupported()
}

pub fn copy(_from: &Path, _to: &Path) -> io::Result<u64> {
    unsupported()
}

fn cvt_io_error(err: i32) -> io::Error {
    match err {
        0x80010002 => io::Error::new(io::ErrorKind::NotFound, "Not found"),
        0x800200d3 => io::Error::new(io::ErrorKind::InvalidInput, "Invalid address"),
        0x80020320 => io::Error::new(io::ErrorKind::Other, "Too many open files"),
        0x800200d1 => io::Error::new(io::ErrorKind::PermissionDenied, "Permission denied"),
        0x8002032c => io::Error::new(io::ErrorKind::InvalidInput, "No current working directory"),
        0x8002032d => io::Error::new(io::ErrorKind::InvalidInput, "File name too long"),
        0x80020321 => io::Error::new(io::ErrorKind::InvalidInput, "No such device"),
        0x80020325 => io::Error::new(io::ErrorKind::InvalidInput, "Unsupported operation"),
        0x80020190 => io::Error::new(io::ErrorKind::Other, "No memory"),
        0x80020064 => io::Error::new(io::ErrorKind::Other, "Called from interrupt handler/thread"),
        0x80020323 => io::Error::new(io::ErrorKind::InvalidInput, "Bad file descriptor"),
        0x80020130 => io::Error::new(io::ErrorKind::Other, "Read error"),
        _ => io::Error::new(io::ErrorKind::Other, "Unknown"),
    }
}
