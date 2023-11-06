// SPDX-License-Identifier: MIT

#[cfg(test)]
mod bond;
#[cfg(test)]
mod bridge;
#[cfg(test)]
mod hsr;
#[cfg(test)]
mod ipvlan;
#[cfg(test)]
mod loopback;
#[cfg(test)]
mod macsec;
#[cfg(test)]
mod macvlan;
#[cfg(test)]
mod message;
#[cfg(test)]
mod prop_list;
#[cfg(test)]
mod veth;
#[cfg(test)]
mod vlan;
#[cfg(test)]
mod vrf;
#[cfg(test)]
mod vxlan;
#[cfg(test)]
mod xfrm;

//TODO(Gris Ge): capture netlink message for ipoib
//TODO(Gris Ge): need test for Icmp6Stats and Inet6Stats