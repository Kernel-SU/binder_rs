/*
 * Copyright (C) 2023, The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! Rust implementation of `MySimpleParcelable`.

use binder::{
    binder_impl::{BorrowedParcel, UnstructuredParcelable},
    impl_deserialize_for_unstructured_parcelable, impl_serialize_for_unstructured_parcelable,
    StatusCode,
};

/// Rust implementation of `MySimpleParcelable`.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MySimpleParcelable {
    /// The name of the parcelable.
    pub name: String,
    /// The number of the parcelable.
    pub number: i32,
}

impl UnstructuredParcelable for MySimpleParcelable {
    fn write_to_parcel(&self, parcel: &mut BorrowedParcel) -> Result<(), StatusCode> {
        parcel.write(&self.name)?;
        parcel.write(&self.number)?;
        Ok(())
    }

    fn from_parcel(parcel: &BorrowedParcel) -> Result<Self, StatusCode> {
        let name = parcel.read()?;
        let number = parcel.read()?;
        Ok(Self { name, number })
    }
}

impl_deserialize_for_unstructured_parcelable!(MySimpleParcelable);
impl_serialize_for_unstructured_parcelable!(MySimpleParcelable);
