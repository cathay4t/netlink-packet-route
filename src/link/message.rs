// SPDX-License-Identifier: MIT

use anyhow::Context;
use netlink_packet_utils::{
    traits::{Emitable, Parseable, ParseableParametrized},
    DecodeError,
};

use crate::{link::LinkAttribute, LinkHeader, LinkMessageBuffer};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
#[non_exhaustive]
pub struct LinkMessage {
    pub header: LinkHeader,
    pub nlas: Vec<LinkAttribute>,
}

impl Emitable for LinkMessage {
    fn buffer_len(&self) -> usize {
        self.header.buffer_len() + self.nlas.as_slice().buffer_len()
    }

    fn emit(&self, buffer: &mut [u8]) {
        self.header.emit(buffer);
        self.nlas
            .as_slice()
            .emit(&mut buffer[self.header.buffer_len()..]);
    }
}

impl<'a, T: AsRef<[u8]> + 'a> Parseable<LinkMessageBuffer<&'a T>>
    for LinkMessage
{
    fn parse(buf: &LinkMessageBuffer<&'a T>) -> Result<Self, DecodeError> {
        let header = LinkHeader::parse(buf)
            .context("failed to parse link message header")?;
        let interface_family = header.interface_family;
        let nlas =
            Vec::<LinkAttribute>::parse_with_param(buf, interface_family)
                .context("failed to parse link message NLAs")?;
        Ok(LinkMessage { header, nlas })
    }
}

impl<'a, T: AsRef<[u8]> + 'a>
    ParseableParametrized<LinkMessageBuffer<&'a T>, u16>
    for Vec<LinkAttribute>
{
    fn parse_with_param(
        buf: &LinkMessageBuffer<&'a T>,
        family: u16,
    ) -> Result<Self, DecodeError> {
        let mut nlas = vec![];
        for nla_buf in buf.nlas() {
            nlas.push(LinkAttribute::parse_with_param(&nla_buf?, family)?);
        }
        Ok(nlas)
    }
}

impl<'a, T: AsRef<[u8]> + 'a>
    ParseableParametrized<LinkMessageBuffer<&'a T>, u8> for Vec<LinkAttribute>
{
    fn parse_with_param(
        buf: &LinkMessageBuffer<&'a T>,
        family: u8,
    ) -> Result<Self, DecodeError> {
        Vec::<LinkAttribute>::parse_with_param(buf, u16::from(family))
    }
}
