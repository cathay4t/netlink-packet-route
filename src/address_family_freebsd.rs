// SPDX-License-Identifier: MIT

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[non_exhaustive]
pub enum AddressFamily {
    #[default]
    Unspec,
    Local,
    Unix,
    Inet,
    Implink,
    Pup,
    Chaos,
    Netbios,
    Iso,
    Osi,
    Ecma,
    Datakit,
    Ccitt,
    Sna,
    Decnet,
    Dli,
    Lat,
    Hylink,
    Appletalk,
    Route,
    Link,
    Coip,
    Cnt,
    Ipx,
    Sip,
    Isdn,
    E164,
    Inet6,
    Natm,
    Atm,
    Netgraph,
    Other(u8),
}

impl From<u8> for AddressFamily {
    fn from(d: u8) -> Self {
        match d {
            d if d == libc::AF_UNSPEC as u8 => Self::Unspec,
            d if d == libc::AF_LOCAL as u8 => Self::Local,
            d if d == libc::AF_UNIX as u8 => Self::Unix,
            d if d == libc::AF_INET as u8 => Self::Inet,
            d if d == libc::AF_IMPLINK as u8 => Self::Implink,
            d if d == libc::AF_PUP as u8 => Self::Pup,
            d if d == libc::AF_CHAOS as u8 => Self::Chaos,
            d if d == libc::AF_NETBIOS as u8 => Self::Netbios,
            d if d == libc::AF_ISO as u8 => Self::Iso,
            d if d == libc::AF_OSI as u8 => Self::Osi,
            d if d == libc::AF_ECMA as u8 => Self::Ecma,
            d if d == libc::AF_DATAKIT as u8 => Self::Datakit,
            d if d == libc::AF_CCITT as u8 => Self::Ccitt,
            d if d == libc::AF_SNA as u8 => Self::Sna,
            d if d == libc::AF_DECnet as u8 => Self::Decnet,
            d if d == libc::AF_DLI as u8 => Self::Dli,
            d if d == libc::AF_LAT as u8 => Self::Lat,
            d if d == libc::AF_HYLINK as u8 => Self::Hylink,
            d if d == libc::AF_APPLETALK as u8 => Self::Appletalk,
            d if d == libc::AF_ROUTE as u8 => Self::Route,
            d if d == libc::AF_LINK as u8 => Self::Link,
            d if d == libc::AF_COIP as u8 => Self::Coip,
            d if d == libc::AF_CNT as u8 => Self::Cnt,
            d if d == libc::AF_IPX as u8 => Self::Ipx,
            d if d == libc::AF_SIP as u8 => Self::Sip,
            d if d == libc::AF_ISDN as u8 => Self::Isdn,
            d if d == libc::AF_E164 as u8 => Self::E164,
            d if d == libc::AF_INET6 as u8 => Self::Inet6,
            d if d == libc::AF_NATM as u8 => Self::Natm,
            d if d == libc::AF_ATM as u8 => Self::Atm,
            d if d == libc::AF_NETGRAPH as u8 => Self::Netgraph,
            _ => Self::Other(d),
        }
    }
}

impl From<AddressFamily> for u8 {
    fn from(v: AddressFamily) -> u8 {
        match v {
            AddressFamily::Unspec => libc::AF_UNSPEC as u8,
            AddressFamily::Local => libc::AF_LOCAL as u8,
            AddressFamily::Unix => libc::AF_UNIX as u8,
            AddressFamily::Inet => libc::AF_INET as u8,
            AddressFamily::Implink => libc::AF_IMPLINK as u8,
            AddressFamily::Pup => libc::AF_PUP as u8,
            AddressFamily::Chaos => libc::AF_CHAOS as u8,
            AddressFamily::Netbios => libc::AF_NETBIOS as u8,
            AddressFamily::Iso => libc::AF_ISO as u8,
            AddressFamily::Osi => libc::AF_OSI as u8,
            AddressFamily::Ecma => libc::AF_ECMA as u8,
            AddressFamily::Datakit => libc::AF_DATAKIT as u8,
            AddressFamily::Ccitt => libc::AF_CCITT as u8,
            AddressFamily::Sna => libc::AF_SNA as u8,
            AddressFamily::Decnet => libc::AF_DECnet as u8,
            AddressFamily::Dli => libc::AF_DLI as u8,
            AddressFamily::Lat => libc::AF_LAT as u8,
            AddressFamily::Hylink => libc::AF_HYLINK as u8,
            AddressFamily::Appletalk => libc::AF_APPLETALK as u8,
            AddressFamily::Route => libc::AF_ROUTE as u8,
            AddressFamily::Link => libc::AF_LINK as u8,
            AddressFamily::Coip => libc::AF_COIP as u8,
            AddressFamily::Cnt => libc::AF_CNT as u8,
            AddressFamily::Ipx => libc::AF_IPX as u8,
            AddressFamily::Sip => libc::AF_SIP as u8,
            AddressFamily::Isdn => libc::AF_ISDN as u8,
            AddressFamily::E164 => libc::AF_E164 as u8,
            AddressFamily::Inet6 => libc::AF_INET6 as u8,
            AddressFamily::Natm => libc::AF_NATM as u8,
            AddressFamily::Atm => libc::AF_ATM as u8,
            AddressFamily::Netgraph => libc::AF_NETGRAPH as u8,
            AddressFamily::Other(d) => d,
        }
    }
}