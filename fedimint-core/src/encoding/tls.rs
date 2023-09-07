use bitcoin_hashes::hex::ToHex;
use lightning::io::{Error, Write};
use tokio_rustls::rustls;

use crate::encoding::Encodable;

impl Encodable for rustls::Certificate {
    fn consensus_encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
        self.0.to_hex().consensus_encode(writer)
    }
}
