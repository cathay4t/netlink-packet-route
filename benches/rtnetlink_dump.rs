// SPDX-License-Identifier: MIT

use std::fs::File;

use criterion::{criterion_group, criterion_main, Criterion};
use pcap_file::PcapReader;

use netlink_packet_core::NetlinkMessage;
use netlink_packet_route::RouteNetlinkMessage;

fn bench(c: &mut Criterion) {
    let pcap_reader =
        PcapReader::new(File::open("data/rtnetlink.pcap").unwrap()).unwrap();
    let packets: Vec<Vec<u8>> = pcap_reader
        .map(|pkt| pkt.unwrap().data.into_owned().to_vec())
        .collect();

    c.bench_function("parse", move |b| {
        b.iter(|| {
            for (i, buf) in packets.iter().enumerate() {
                NetlinkMessage::<RouteNetlinkMessage>::deserialize(&buf[16..])
                    .unwrap_or_else(|_| panic!("message {} failed", i));
            }
        })
    });
}

criterion_group!(benches, bench);
criterion_main!(benches);
