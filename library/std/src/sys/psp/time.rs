use crate::time::Duration;
use libc::{sceKernelGetSystemTimeWide, sceKernelLibcTime};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct Instant(Duration);

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub struct SystemTime(Duration);

pub const UNIX_EPOCH: SystemTime = SystemTime(Duration::from_secs(0));

impl Instant {
    pub fn now() -> Instant {
        let usecs = unsafe { sceKernelGetSystemTimeWide() };
        Self(Duration::from_micros(usecs as u64))
    }

    pub const fn zero() -> Instant {
        Instant(Duration::from_secs(0))
    }

    // See libstd/time.rs - as far as we know, the PSP game clock (not the system time)
    // is monotonic and seems very unlikely to ever go backwards, so we can leave this
    // as-is for now until we see otherwise, and save ourselves an unnecessary Mutex
    // lock + assignment + check.
    pub fn actually_monotonic() -> bool {
        true
    }

    pub fn checked_sub_instant(&self, other: &Instant) -> Option<Duration> {
        self.0.checked_sub(other.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<Instant> {
        Some(Instant(self.0.checked_sub(*other)?))
    }
}

impl SystemTime {
    pub fn now() -> SystemTime {
        let mut t: i32 = 0;
        unsafe {
            sceKernelLibcTime(&mut t as _);
        }
        Self(Duration::from_secs(t as u64))
    }
    
    pub fn from_unix_time(t: u64) -> SystemTime {
        Self(Duration::from_secs(t))
    }

    pub fn try_from_psp_time(t: &libc::ScePspDateTime) -> Result<SystemTime, ()> {
        let mut secs_since_epoch: u64 = 0;
        if unsafe {libc::sceRtcGetTime64_t(t as *const libc::ScePspDateTime, &mut secs_since_epoch as *mut u64)} < 0 {
            Err(())
        } else {
            Ok(Self(Duration::from_secs(secs_since_epoch)))
        }
    }

    pub fn sub_time(&self, other: &SystemTime) -> Result<Duration, Duration> {
        self.0.checked_sub(other.0).ok_or_else(|| other.0 - self.0)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_add(*other)?))
    }

    pub fn checked_sub_duration(&self, other: &Duration) -> Option<SystemTime> {
        Some(SystemTime(self.0.checked_sub(*other)?))
    }
}
