// Copyright Rivtower Technologies LLC.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! version management

use super::ContractCallExt;
use std::str::FromStr;

use crate::contracts::tools::method as method_tools;
use crate::libexecutor::executor::Executor;
use crate::types::block_number::BlockTag;
use crate::types::reserved_addresses;
use crate::contracts::tools::decode as decode_tools;
use cita_types::Address;

lazy_static! {
    static ref GET_CRL_HASH: Vec<u8> = method_tools::encode_to_vec(b"getCrl()");
    static ref CONTRACT_ADDRESS: Address =
        Address::from_str(reserved_addresses::CERT_REVOKE_MANAGER).unwrap();
}

pub struct CertRevokeManager<'a> {
    executor: &'a Executor,
}

impl<'a> CertRevokeManager<'a> {
    pub fn new(executor: &'a Executor) -> Self {
        CertRevokeManager { executor }
    }

    pub fn get_crl(&self, block_tag: BlockTag) -> Option<Vec<Address>> {
        self.executor
            .call_method(
                &*CONTRACT_ADDRESS,
                &*GET_CRL_HASH.as_slice(),
                None,
                block_tag,
            )
            .ok()
            .and_then(|output| decode_tools::to_address_vec(&output))
    }
}
