// SPDX-License-Identifier: MIT

use netlink_packet_utils::{nla::NlaBuffer, Emitable, ParseableParametrized};

use crate::link::{
    LinkAttribute, LinkVfInfo, VfInfo, VfInfoBroadcast, VfInfoLinkState,
    VfInfoMac, VfInfoRate, VfInfoRssQueryEn, VfInfoSpoofCheck, VfInfoTrust,
    VfInfoTxRate, VfInfoVlan, VfLinkState, VfStats, VfVlan, VfVlanInfo,
    VlanProtocol,
};
use crate::AddressFamily;

// Wireshark capture of nlmon on a PF NIC with 2 SR-IOV VF enabled.
// Only the IFLA_VFINFO_LIST included.
#[test]
fn test_parsing_link_sriov() {
    let raw = vec![
        0x54, 0x02, 0x16, 0x00, 0x28, 0x01, 0x01, 0x00, 0x28, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x4e, 0x60, 0x71, 0x72, 0xbd, 0x1d, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x24, 0x00, 0x0d, 0x00, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x09, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x0c, 0x00,
        0x14, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x81, 0x00, 0x00, 0x00, 0x64, 0x00, 0x08, 0x00,
        0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x0c, 0x00, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x28, 0x01, 0x01, 0x00, 0x28, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00,
        0xde, 0x48, 0xe1, 0xe0, 0xbb, 0xec, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x24, 0x00, 0x0d, 0x00,
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x02, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x10, 0x00, 0x06, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x03, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x04, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x05, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x07, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x09, 0x00, 0x01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x18, 0x00, 0x0c, 0x00, 0x14, 0x00, 0x01, 0x00,
        0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x81, 0x00, 0x00, 0x00, 0x64, 0x00, 0x08, 0x00, 0x0c, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x01, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x02, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x03, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x04, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x05, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x07, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x08, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    ];

    let expected = LinkAttribute::VfInfoList(vec![
        LinkVfInfo(vec![
            VfInfo::Mac(VfInfoMac::new(0, &[78, 96, 113, 114, 189, 29])),
            VfInfo::Broadcast(VfInfoBroadcast::new(&[
                255, 255, 255, 255, 255, 255,
            ])),
            VfInfo::Vlan(VfInfoVlan {
                vf_id: 0,
                vlan_id: 0,
                qos: 0,
            }),
            VfInfo::Rate(VfInfoRate {
                vf_id: 0,
                min_tx_rate: 0,
                max_tx_rate: 0,
            }),
            VfInfo::TxRate(VfInfoTxRate { vf_id: 0, rate: 0 }),
            VfInfo::SpoofCheck(VfInfoSpoofCheck {
                vf_id: 0,
                enabled: true,
            }),
            VfInfo::LinkState(VfInfoLinkState {
                vf_id: 0,
                state: VfLinkState::Auto,
            }),
            VfInfo::RssQueryEn(VfInfoRssQueryEn {
                vf_id: 0,
                enabled: false,
            }),
            VfInfo::Trust(VfInfoTrust {
                vf_id: 0,
                enabled: false,
            }),
            VfInfo::VlanList(vec![VfVlan::Info(VfVlanInfo {
                vf_id: 0,
                vlan_id: 0,
                qos: 0,
                protocol: VlanProtocol::Ieee8021Q,
            })]),
            VfInfo::Stats(vec![
                VfStats::RxPackets(0),
                VfStats::TxPackets(0),
                VfStats::RxBytes(0),
                VfStats::TxBytes(0),
                VfStats::Broadcast(0),
                VfStats::Multicast(0),
                VfStats::RxDropped(0),
                VfStats::TxDropped(0),
            ]),
        ]),
        LinkVfInfo(vec![
            VfInfo::Mac(VfInfoMac::new(1, &[222, 72, 225, 224, 187, 236])),
            VfInfo::Broadcast(VfInfoBroadcast::new(&[
                255, 255, 255, 255, 255, 255,
            ])),
            VfInfo::Vlan(VfInfoVlan {
                vf_id: 1,
                vlan_id: 0,
                qos: 0,
            }),
            VfInfo::Rate(VfInfoRate {
                vf_id: 1,
                min_tx_rate: 0,
                max_tx_rate: 0,
            }),
            VfInfo::TxRate(VfInfoTxRate { vf_id: 1, rate: 0 }),
            VfInfo::SpoofCheck(VfInfoSpoofCheck {
                vf_id: 1,
                enabled: true,
            }),
            VfInfo::LinkState(VfInfoLinkState {
                vf_id: 1,
                state: VfLinkState::Auto,
            }),
            VfInfo::RssQueryEn(VfInfoRssQueryEn {
                vf_id: 1,
                enabled: false,
            }),
            VfInfo::Trust(VfInfoTrust {
                vf_id: 1,
                enabled: false,
            }),
            VfInfo::VlanList(vec![VfVlan::Info(VfVlanInfo {
                vf_id: 1,
                vlan_id: 0,
                qos: 0,
                protocol: VlanProtocol::Ieee8021Q,
            })]),
            VfInfo::Stats(vec![
                VfStats::RxPackets(0),
                VfStats::TxPackets(0),
                VfStats::RxBytes(0),
                VfStats::TxBytes(0),
                VfStats::Broadcast(0),
                VfStats::Multicast(0),
                VfStats::RxDropped(0),
                VfStats::TxDropped(0),
            ]),
        ]),
    ]);

    assert_eq!(
        expected,
        LinkAttribute::parse_with_param(
            &NlaBuffer::new(&raw),
            AddressFamily::Unspec
        )
        .unwrap(),
    );

    let mut buf = vec![0; expected.buffer_len()];

    expected.emit(&mut buf);

    assert_eq!(buf, raw);
}

// tcpdump capture of nlmon on a netdevsim device without VF info configure.
// Only IFLA_VFINFO_LIST was included.
#[test]
fn test_parsing_empty_link_sriov_vf_info() {
    let raw = vec![0x04, 0x00, 0x16, 0x00];
    let expected = LinkAttribute::VfInfoList(vec![]);

    assert_eq!(
        expected,
        LinkAttribute::parse_with_param(
            &NlaBuffer::new_checked(&raw).unwrap(),
            AddressFamily::Unspec
        )
        .unwrap(),
    );
}
