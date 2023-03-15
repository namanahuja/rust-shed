/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under both the MIT license found in the
 * LICENSE-MIT file in the root directory of this source tree and the Apache
 * License, Version 2.0 found in the LICENSE-APACHE file in the root directory
 * of this source tree.
 */

use std::marker::PhantomData;

use fbthrift::adapter::ThriftAdapter;
use uuid::Uuid;

pub struct UuidAdapter<T> {
    inner: PhantomData<T>,
}

impl ThriftAdapter for UuidAdapter<Vec<u8>> {
    type OriginalType = Vec<u8>;
    type AdaptedType = Uuid;

    type Error = uuid::Error;

    fn to_original(value: &Self::AdaptedType) -> Self::OriginalType {
        value.as_bytes().to_vec()
    }

    fn from_original(value: Self::OriginalType) -> Result<Self::AdaptedType, Self::Error> {
        if value.is_empty() {
            Ok(Uuid::nil())
        } else {
            Uuid::from_slice(&value)
        }
    }
}