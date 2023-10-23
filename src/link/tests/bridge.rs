// SPDX-License-Identifier: MIT

use crate::{
    link::LinkAttribute, RouteNetlinkMessage, RouteNetlinkMessageBuffer,
};

#[test]
fn test_link_port_attach_to_bridge() {
    use crate::*;
    let data = vec![
        0x28, 0x00, 0x00, 0x00, // length
        0x10, 0x00, // type
        0x05, 0x00, // flags
        0x9c, 0x9d, 0x57, 0x5c, // seq id
        0x00, 0x00, 0x00, 0x00, // pid
        0x00, // interface family
        0x00, // padding
        0x00, 0x00, // device type
        0x06, 0x00, 0x00, 0x00, // interface index
        0x00, 0x00, 0x00, 0x00, // device flags
        0x00, 0x00, 0x00, 0x00, // device change flags
        // NLA (set master)
        0x08, 0x00, // length
        0x0a, 0x00, // type
        0x05, 0x00, 0x00, 0x00, // index of the master interface
    ];
    let nl_buffer = NetlinkBuffer::new(&data).payload();
    let rtnl_buffer = RouteNetlinkMessageBuffer::new(&nl_buffer);
    let actual =
        RouteNetlinkMessage::parse_with_param(&rtnl_buffer, RTM_NEWLINK)
            .unwrap();
    let expected = RouteNetlinkMessage::NewLink(LinkMessage {
        header: LinkHeader {
            interface_family: 0,
            index: 6,
            link_layer_type: 0,
            flags: 0,
            change_mask: 0,
        },
        nlas: vec![LinkAttribute::Master(5)],
    });
    assert_eq!(expected, actual);
}
