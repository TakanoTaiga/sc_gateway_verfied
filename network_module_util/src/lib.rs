// Copyright 2023 Hakoroboken
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.


pub mod key;

use std::mem;
use std::ptr;

pub fn serialize<T: Sized>(obj: &T, buf: &mut [u8]) {
    let size = mem::size_of::<T>();
    assert!(buf.len() >= size, "Buffer is too small for the serialized data");

    let obj_ptr = obj as *const T as *const u8;
    let obj_slice = unsafe { std::slice::from_raw_parts(obj_ptr, size) };
    buf[..size].copy_from_slice(obj_slice);
}

pub fn deserialize<T: Sized>(buf: &[u8]) -> T {
    assert!(
        buf.len() >= mem::size_of::<T>(),
        "Buffer is too small for the serialized data"
    );
    let obj_ptr = buf.as_ptr() as *const T;
    unsafe { ptr::read(obj_ptr) }
}


// kani struct
#[cfg(kani)]
struct Vec3 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}


#[cfg(kani)]
#[kani::proof]
fn check_correctness(){
    let mut serialized_vec = Vec3 {
        x: kani::any(),
        y: kani::any(),
        z: kani::any(),
    };

    let mut buf= [0; 128];
    serialize(&serialized_vec, &mut buf);
    let deserialized_vec: Vec3 = deserialize(&buf);

    assert!(serialized_vec.x == deserialized_vec.x);
    assert!(serialized_vec.y == deserialized_vec.y);
    assert!(serialized_vec.z == deserialized_vec.z);
}