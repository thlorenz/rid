#![allow(unused_imports)]
use std::{
    mem::{self, ManuallyDrop},
    ops::Index,
    ptr,
};

use rid_common::resolve_ptr;

// Similar impl with more features and limited to primitive types since
// it needs to guarantee reprC layout for the Vector.
// Rid sends opaque structs so we don't need that part.
// https://github.com/getditto/safer_ffi/blob/master/src/vec.rs

pub type ManDropVec<T> = mem::ManuallyDrop<Vec<T>>;

#[derive(Debug)]
#[repr(C)]
pub struct RidVec<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub capacity: usize,
}

// Needs to match the struct above and is included by rid_build in order to expose it to cbindgen
pub fn code_rid_vec() -> String {
    let s = stringify! {
    mod rid_ffi_rid_vec {
        #[repr(C)]
        pub struct RidVec<T> {
            pub ptr: *mut T,
            pub len: usize,
            pub capacity: usize,
        }
    }
    };
    s.to_string()
}

impl<T> From<Vec<T>> for RidVec<T> {
    fn from(v: Vec<T>) -> Self {
        let len = v.len();
        let capacity = v.capacity();
        let mut ptr = mem::ManuallyDrop::new(v);
        let ptr = ptr.as_mut_ptr();
        RidVec { ptr, len, capacity }
    }
}

impl<T> From<&RidVec<T>> for ManDropVec<T> {
    fn from(rv: &RidVec<T>) -> Self {
        let vec = unsafe { Vec::from_raw_parts(rv.ptr, rv.len, rv.capacity) };
        mem::ManuallyDrop::new(vec)
    }
}

impl<T> Index<usize> for RidVec<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &Self::Output {
        let vec: ManDropVec<T> = self.into();
        let ptr = &vec[idx] as *const _ as *mut T;
        resolve_ptr(ptr)
    }
}

impl<T> RidVec<T> {
    pub fn free(self) {
        let _ = unsafe { Vec::from_raw_parts(self.ptr, self.len, self.capacity) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_len<T>(rvec: &RidVec<T>) -> usize {
        let vec: ManDropVec<T> = rvec.into();
        vec.len()
    }

    fn get_capacity<T>(rvec: &RidVec<T>) -> usize {
        let vec: ManDropVec<T> = rvec.into();
        vec.capacity()
    }

    #[test]
    fn vec_u8() {
        let rvec: RidVec<u8> = {
            let vec = vec![1, 2, 3];
            vec.into()
        };

        {
            assert_eq!(get_len(&rvec), rvec.len, "len");
            assert_eq!(get_capacity(&rvec), rvec.capacity, "capacity");

            assert_eq!(rvec[0], 1, "get item 1");
            assert_eq!(rvec[1], 2, "get item 2");
            assert_eq!(rvec[2], 3, "get item 3");
        }
        rvec.free()
    }

    #[test]
    fn vec_string() {
        let rvec: RidVec<String> = {
            let vec = vec![
                "hello".to_string(),
                "world".to_string(),
                "Rust Integrated Dart".to_string(),
            ];
            vec.into()
        };

        {
            assert_eq!(get_len(&rvec), rvec.len, "len");
            assert_eq!(get_capacity(&rvec), rvec.capacity, "capacity");

            assert_eq!(rvec[0], "hello", "get item 1");
            assert_eq!(rvec[1], "world", "get item 2");
            assert_eq!(rvec[2], "Rust Integrated Dart", "get item 3");
        }
        rvec.free()
    }

    //
    // struct
    //

    #[derive(Debug, PartialEq)]
    struct Small(String);

    #[derive(Debug, PartialEq)]
    struct Something {
        id: u8,
        label: String,
        small: Vec<Small>,
    }

    #[test]
    fn vec_struct() {
        let rvec: RidVec<Something> = {
            let s1 = Something {
                id: 1,
                label: "something".to_string(),
                small: vec![Small("Hello".to_string()), Small("World".to_string())],
            };
            let s2 = Something {
                id: 2,
                label: "something else".to_string(),
                small: vec![Small("Hola".to_string()), Small("Mundo".to_string())],
            };
            let vec = vec![s1, s2];

            vec.into()
        };

        {
            assert_eq!(get_len(&rvec), rvec.len, "len");
            assert_eq!(get_capacity(&rvec), rvec.capacity, "capacity");

            let item1 = &rvec[0];
            let item2 = &rvec[1];

            assert_eq!(
                item1,
                &Something {
                    id: 1,
                    label: "something".to_string(),
                    small: vec![Small("Hello".to_string()), Small("World".to_string())],
                },
                "get item 1"
            );
            assert_eq!(
                item2,
                &Something {
                    id: 2,
                    label: "something else".to_string(),
                    small: vec![Small("Hola".to_string()), Small("Mundo".to_string())],
                },
                "get item 1"
            );
        }
        rvec.free()
    }

    #[test]
    fn vec_struct_refs() {
        let s1 = Something {
            id: 1,
            label: "something".to_string(),
            small: vec![Small("Hello".to_string()), Small("World".to_string())],
        };
        let s2 = Something {
            id: 2,
            label: "something else".to_string(),
            small: vec![Small("Hola".to_string()), Small("Mundo".to_string())],
        };
        let vec = vec![s1, s2];

        let rvec: RidVec<&Something> = {
            let filtered: Vec<&Something> = vec.iter().filter(|&x| x.id == 2).collect();
            filtered.into()
        };

        {
            assert_eq!(get_len(&rvec), rvec.len, "len");
            assert_eq!(get_capacity(&rvec), rvec.capacity, "capacity");

            let item1 = rvec[0];
            assert_eq!(
                item1,
                &Something {
                    id: 2,
                    label: "something else".to_string(),
                    small: vec![Small("Hola".to_string()), Small("Mundo".to_string())],
                },
                "get item 1"
            );
        }
        rvec.free()
    }
}
