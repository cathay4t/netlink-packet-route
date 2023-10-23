// SPDX-License-Identifier: MIT

/// The `netlink-packet-route` crate is designed to abstract Netlink route
/// protocol(`rtnetlink`) packet into Rust data types. The goal of this crate is
/// saving netlink user from reading Kernel Netlink codes.
///
/// This crate grouped Netlink route protocol into these modules:
///  * `link`: NIC interface, similar to to `ip link` command.
///  * `address`: IP address, similar to `ip address` command.
///  * `route`: Route, similar to `ip route` command.
///  * `rule`: Route rule, similar to `ip rule` command.
///  * `tc`: Traffic control, similar to `tc` command.
///  * `neighbour`: Neighbour, similar to `ip neighbour` command.
///  * `neighbour_table`: Neighbour table, similar to `ip ntable` command.
///  * `nsid`: Namespace, similar to `ip netns` command.
///
/// Normally, you should use [`rtnetlink`][rtnetlink_url] instead of using this
/// crate directly.
///
/// [rtnetlink_url]: https://docs.rs/rtnetlink
pub mod address;
pub mod link;
pub mod neighbour;
pub mod neighbour_table;
pub mod nsid;
pub mod route;
pub mod rule;
pub mod tc;

mod message;

pub use self::message::{RouteNetlinkMessage, RouteNetlinkMessageBuffer};
