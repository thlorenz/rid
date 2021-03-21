#![allow(unused_imports, unused)]
use std::{mem, ops::Deref, ptr};

use rid_ffi::{ManDropVec, RidVec};

fn get_len<T>(rvec: &RidVec<T>) -> usize {
    let vec: ManDropVec<T> = rvec.into();
    vec.len()
}

fn get_capacity<T>(rvec: &RidVec<T>) -> usize {
    let vec: ManDropVec<T> = rvec.into();
    vec.capacity()
}

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

#[derive(Debug, PartialEq)]
struct Small(String);

#[derive(Debug, PartialEq)]
struct Something {
    id: u8,
    label: String,
    small: Vec<Small>,
}

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
                small: vec![
                    Small("Hello".to_string()),
                    Small("World".to_string())
                ],
            },
            "get item 1"
        );
        assert_eq!(
            item2,
            &Something {
                id: 2,
                label: "something else".to_string(),
                small: vec![
                    Small("Hola".to_string()),
                    Small("Mundo".to_string())
                ],
            },
            "get item 1"
        );
    }
    rvec.free()
}

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
        let filtered: Vec<&Something> =
            vec.iter().filter(|&x| x.id == 2).collect();
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
                small: vec![
                    Small("Hola".to_string()),
                    Small("Mundo".to_string())
                ],
            },
            "get item 1"
        );
    }
    rvec.free()
}

fn main() {
    vec_u8();
    vec_string();
    vec_struct();
}
