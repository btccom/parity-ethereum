// Copyright (c) [2019] [BTC.COM]
// Copyright 2015-2019 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use std::fmt;
use rustc_hex::ToHex;
use byteorder::{BigEndian, WriteBytesExt};
use v1::types::Bytes;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ExtraNonce {
	/// A 32 bit extra nonce field
	ExtraNonce32(u32),
	/// Extra nonce bytes
	ExtraNonceBytes(Bytes),
}

impl fmt::Display for ExtraNonce {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.to_vec().to_hex())
	}
}

impl ExtraNonce {
	pub fn to_vec(&self) -> Vec<u8> {
		match self {
			ExtraNonce::ExtraNonce32(nonce32) => {
				let mut nbytes = vec![];
				nbytes.write_u32::<BigEndian>(*nonce32).unwrap();
				nbytes
			}
			ExtraNonce::ExtraNonceBytes(nbytes) => nbytes.clone().into(),
		}
	}
}
