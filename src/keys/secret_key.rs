// Copyright 2020 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// https://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

//! Module providing keys, keypairs, and signatures.
//!
//! The easiest way to get a `PublicKey` is to create a random `Keypair` first through one of the
//! `new` functions. A `PublicKey` can't be generated by itself; it must always be derived from a
//! secret key.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Debug, Display, Formatter};
use threshold_crypto::{self, serde_impl::SerdeSecret};
// TODO: remove clones. We need to restructure to hold keypair ones and only require references for this.
/// Wrapper for different secret key types.
#[derive(Debug, Serialize, Deserialize)]
pub enum SecretKey {
    /// Ed25519 secretkey.
    Ed25519(ed25519_dalek::SecretKey),
    /// BLS secretkey.
    Bls(SerdeSecret<threshold_crypto::SecretKey>),
    /// BLS secretkey share.
    BlsShare(SerdeSecret<threshold_crypto::SecretKeyShare>),
}

impl Display for SecretKey {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, formatter)
    }
}

impl From<threshold_crypto::SecretKey> for SecretKey {
    fn from(sk: threshold_crypto::SecretKey) -> Self {
        Self::Bls(SerdeSecret(sk))
    }
}
