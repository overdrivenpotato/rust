use crate::convert::TryFrom;
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr, SocketAddrV4};
use crate::sys::{unsupported, Void};
use crate::sys_common::IntoInner;
use crate::time::Duration;
use crate::ffi::CString;

use core::ffi::c_void;

pub struct Socket(i32);

impl Drop for Socket {
    fn drop(&mut self) {
        unsafe { libc::sceNetInetClose(self.0); }
    }
}

pub struct TcpStream {
    socket: Socket,
    peer_addr: SocketAddrV4,
}

impl TcpStream {
    pub fn connect(addr: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        let sock = unsafe { libc::sceNetInetSocket(netc::AF_INET as i32, netc::SOCK_STREAM, 0) };
        if sock < 0 {
            todo!()
        } else {
            let addr = addr?;
            match addr {
                SocketAddr::V4(v4) => {
                    let (addr, len) = addr.into_inner();
                    if unsafe { libc::sceNetInetConnect(sock, addr, len as u32) } < 0  {
                        todo!("Error Handling")
                    } else {
                        Ok(TcpStream{socket: Socket(sock), peer_addr: *v4})
                    }
                }
                SocketAddr::V6(_) => {
                    unsupported()
                }
            }
        }
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe { libc::sceNetInetRecv(self.socket.0, buf.as_mut_ptr() as *mut c_void, buf.len(), 0) };
        if result < 0 {
            let err = unsafe { libc::sceNetInetGetErrno() };
            Err(io::Error::new(io::ErrorKind::Other, err.to_string()))    
        } else {
            Ok(result)
        }
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_read_vectored(&self) -> bool {
        false
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe { libc::sceNetInetSend(self.socket.0, buf.as_ptr() as *const c_void, buf.len(), 0) };
        if result < 0 {
            todo!("Error Handling")
        } else {
            Ok(result)
        }
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        unsupported()
    }

    pub fn is_write_vectored(&self) -> bool {
        false
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        Ok(SocketAddr::V4(self.peer_addr))
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unsupported")
    }
}

pub struct TcpListener(Socket);

impl TcpListener {
    pub fn bind(addr: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        let sock = unsafe { libc::sceNetInetSocket(netc::AF_INET as i32, netc::SOCK_STREAM, 0) };
        if sock < 0 {
            todo!()
        } else {
            let addr = addr?;
            match addr {
                SocketAddr::V4(v4) => {
                    let (addr, len) = addr.into_inner();
                    if unsafe { libc::sceNetInetBind(sock, addr, len as u32) } < 0  {
                        todo!("Error Handling")
                    } else {
                        if unsafe { libc::sceNetInetListen(sock, 128) } < 0 {
                            todo!("Error Handling")
                        } else {
                            Ok(TcpListener(Socket(sock)))
                        }
                    }
                }
                SocketAddr::V6(_) => {
                    unsupported()
                }
            }
        }
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        let mut addr: netc::sockaddr = unsafe { core::mem::zeroed() };
        let mut addr_len: netc::socklen_t = 0; 
        let sock = unsafe { libc::sceNetInetAccept(self.0.0, &mut addr, &mut addr_len) };
        if sock < 0 {
            todo!("Error Handling")
        } else {
            let addr = unsafe { core::mem::transmute::<netc::sockaddr, netc::sockaddr_in>(addr) };
            let port = addr.sin_port;
            let octets = u32::to_le_bytes(addr.sin_addr.s_addr);
            let sockaddr = SocketAddrV4::new(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]), port);
            let stream = TcpStream {
                socket: Socket(sock),
                peer_addr: sockaddr,
            };
            return Ok((stream, SocketAddr::V4(sockaddr)))
        }
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unsupported")
    }
}

pub struct UdpSocket(Void);

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        unsupported()
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        unsupported()
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        unsupported()
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        unsupported()
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        unsupported()
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        unsupported()
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        unsupported()
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        unsupported()
    }

    pub fn ttl(&self) -> io::Result<u32> {
        unsupported()
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        unsupported()
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        unsupported()
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        unsupported()
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        unsupported()
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {} 
    }
}

pub struct LookupHost {
    hostname: CString,
    port: u16,
    resolver_id: i32,
    resolver_buf: [u8; 1024],
    timeout: u32,
    retries: i32,
    // TODO is this necessary? Just don't want to get caught in an infinite loop
    // returning the same address with no errors returned from sceNetResolverStartNtoA
    prev_result: u32,
}

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.port
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        let mut in_addr: libc::in_addr = unsafe { core::mem::zeroed() };
        let result =  unsafe { libc::sceNetResolverStartNtoA(self.resolver_id, self.hostname.as_ptr() as *const u8, &mut in_addr, self.timeout, self.retries) };
        if result < 0 || in_addr.s_addr == self.prev_result {
            None
        } else {
            self.prev_result = in_addr.s_addr;
            let octets = u32::to_le_bytes(in_addr.s_addr);
            Some(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3]), self.port)))
        }
    }
}

//TODO less unwrapping
impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(v: &str) -> io::Result<LookupHost> {
        let mut split = v.split(":");
        let host = split.next().unwrap();
        let mut port: u16 = 80;
        let next = split.next();
        if next.is_some() {
            port = next.unwrap().parse::<u16>().unwrap();
        }
        let cstring = crate::ffi::CString::new(host).unwrap();
        
        let mut rid: i32 = 0;
        let mut dns_buf: [u8; 1024] = [0u8; 1024];
        if unsafe { libc::sceNetResolverCreate(&mut rid, &mut dns_buf[0] as *mut _ as *mut _, dns_buf.len() as u32) } < 0 {
            todo!("Error Handling");
        } else {
            Ok(LookupHost {
                hostname: cstring,
                port,
                resolver_id: rid,
                resolver_buf: dns_buf,
                retries: 5,
                timeout: 5,
                prev_result: 0,
            })
        }
    }
}

impl TryFrom<(&str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(v: (&str, u16)) -> io::Result<LookupHost> {
        let cstring = crate::ffi::CString::new(v.0).unwrap();
        let mut rid: i32 = 0;
        let mut dns_buf: [u8; 1024] = [0u8; 1024];
        if unsafe { libc::sceNetResolverCreate(&mut rid, &mut dns_buf[0] as *mut _ as *mut _, dns_buf.len() as u32) } < 0 {
            todo!("Error Handling");
        } else {
            Ok(LookupHost {
                hostname: cstring,
                port: v.1,
                resolver_id: rid,
                resolver_buf: dns_buf,
                retries: 5,
                timeout: 5,
                prev_result: 0,
            })
        }    
    }
}

impl Drop for LookupHost {
    fn drop(&mut self) {
        unsafe { libc::sceNetResolverDelete(self.resolver_id) };
    }
}

#[allow(nonstandard_style)]
pub mod netc {
    pub const AF_UNSPEC: u8 = 0;
    pub const AF_LOCAL: u8 = 1;
    pub const AF_UNIX: u8 = AF_LOCAL;
    pub const AF_INET: u8 = 2;
    pub const AF_IMPLINK: u8 = 3;
    pub const AF_PUP: u8 = 4;
    pub const AF_CHAOS: u8 = 5;
    pub const AF_NS: u8 = 6;
    pub const AF_ISO: u8 = 7;
    pub const AF_OSI: u8 = AF_ISO;
    pub const AF_ECMA: u8 = 8;
    pub const AF_DATAKIT: u8 = 9;
    pub const AF_CCITT: u8 = 10;
    pub const AF_SNA: u8 = 11;
    pub const AF_DECnet: u8 = 12;
    pub const AF_DLI: u8 = 13;
    pub const AF_LAT: u8 = 14;
    pub const AF_HYLINK: u8 = 15;
    pub const AF_APPLETALK: u8 = 16;
    pub const AF_ROUTE: u8 = 17;
    pub const AF_LINK: u8 = 18;
    pub const AF_COIP: u8 = 20;
    pub const AF_CNT: u8 = 21;
    pub const AF_IPX: u8 = 23;
    pub const AF_INET6: u8 = 24;
    pub const AF_ISDN: u8 = 26;
    pub const AF_E164: u8 = AF_ISDN;
    pub const AF_NATM: u8 = 27;
    pub const AF_ARP: u8 = 28;
    pub const AF_MAX: u8 = 31;

    pub const SOCK_STREAM: i32 = 1;
    pub const SOCK_DGRAM: i32 = 2;
    pub const SOCK_RAW: i32 = 3;
    pub const SOCK_RDM: i32 = 4;
    pub const SOCK_SEQPACKET: i32 = 5;
    pub type sa_family_t = u8;

    pub use libc::in_addr;

    pub use libc::sockaddr;

    pub use libc::socklen_t;

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub struct sockaddr_in {
        pub sin_len: u8,
        pub sin_family: u8,
        pub sin_port: u16,
        pub sin_addr: in_addr,
        pub sin_zero: [u8; 8]
    }

    #[derive(Copy, Clone)]
    pub struct in6_addr {
        pub s6_addr: [u8; 16],
    }

    #[derive(Copy, Clone)]
    pub struct sockaddr_in6 {
        pub sin6_family: sa_family_t,
        pub sin6_port: u16,
        pub sin6_addr: in6_addr,
        pub sin6_flowinfo: u32,
        pub sin6_scope_id: u32,
    }
}
