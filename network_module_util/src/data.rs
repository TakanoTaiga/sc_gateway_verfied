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