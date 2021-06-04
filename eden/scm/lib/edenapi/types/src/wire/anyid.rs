/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This software may be used and distributed according to the terms of the
 * GNU General Public License version 2.
 */

#[cfg(any(test, feature = "for-tests"))]
use serde_derive::{Deserialize, Serialize};

use crate::anyid::{AnyId, LookupRequest, LookupResponse};
use crate::wire::{
    is_default, ToApi, ToWire, WireAnyFileContentId, WireHgId, WireToApiConversionError,
    WireUploadToken,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum WireAnyId {
    #[serde(rename = "1")]
    WireAnyFileContentId(WireAnyFileContentId),

    #[serde(rename = "2")]
    WireHgFilenodeId(WireHgId),

    #[serde(rename = "3")]
    WireHgTreeId(WireHgId),

    #[serde(rename = "4")]
    WireHgChangesetId(WireHgId),

    #[serde(other, rename = "0")]
    Unknown,
}

impl Default for WireAnyId {
    fn default() -> Self {
        Self::WireAnyFileContentId(WireAnyFileContentId::default())
    }
}

impl ToWire for AnyId {
    type Wire = WireAnyId;

    fn to_wire(self) -> Self::Wire {
        use AnyId::*;
        match self {
            AnyFileContentId(id) => WireAnyId::WireAnyFileContentId(id.to_wire()),
            HgFilenodeId(id) => WireAnyId::WireHgFilenodeId(id.to_wire()),
            HgTreeId(id) => WireAnyId::WireHgTreeId(id.to_wire()),
            HgChangesetId(id) => WireAnyId::WireHgChangesetId(id.to_wire()),
        }
    }
}

impl ToApi for WireAnyId {
    type Api = AnyId;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        use WireAnyId::*;
        Ok(match self {
            Unknown => {
                return Err(WireToApiConversionError::UnrecognizedEnumVariant(
                    "WireAnyId",
                ));
            }
            WireAnyFileContentId(id) => AnyId::AnyFileContentId(id.to_api()?),
            WireHgFilenodeId(id) => AnyId::HgFilenodeId(id.to_api()?),
            WireHgTreeId(id) => AnyId::HgTreeId(id.to_api()?),
            WireHgChangesetId(id) => AnyId::HgChangesetId(id.to_api()?),
        })
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct WireLookupRequest {
    #[serde(rename = "1", default, skip_serializing_if = "is_default")]
    pub id: WireAnyId,
}

impl ToWire for LookupRequest {
    type Wire = WireLookupRequest;

    fn to_wire(self) -> Self::Wire {
        WireLookupRequest {
            id: self.id.to_wire(),
        }
    }
}

impl ToApi for WireLookupRequest {
    type Api = LookupRequest;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(LookupRequest {
            id: self.id.to_api()?,
        })
    }
}

#[derive(Clone, Default, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct WireLookupResponse {
    #[serde(rename = "1")]
    pub index: usize,

    #[serde(rename = "2")]
    pub token: Option<WireUploadToken>,
}

impl ToWire for LookupResponse {
    type Wire = WireLookupResponse;

    fn to_wire(self) -> Self::Wire {
        WireLookupResponse {
            index: self.index,
            token: self.token.to_wire(),
        }
    }
}

impl ToApi for WireLookupResponse {
    type Api = LookupResponse;
    type Error = WireToApiConversionError;

    fn to_api(self) -> Result<Self::Api, Self::Error> {
        Ok(LookupResponse {
            index: self.index,
            token: self.token.to_api()?,
        })
    }
}
