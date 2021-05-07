extern crate block_grid;
extern crate serde;
extern crate serde_json;

use std::iter::repeat_with;

use block_grid::*;
use serde::{Deserialize, Serialize};

#[allow(clippy::upper_case_acronyms)]
type BG<T, B> = BlockGrid<T, B>;

#[test]
fn test_serdes_u8() {
    let bg = BG::<u8, U2>::filled(4, 8, 7).unwrap();
    let s = serde_json::to_string(&bg).unwrap();
    let ds = serde_json::from_str::<BG<_, U2>>(&s).unwrap();
    assert_eq!(ds, bg);
    assert!(serde_json::from_str::<BG<u8, U4>>(&s).is_err());

    let bg = BG::<u8, U32>::new(128, 32).unwrap();
    let s = serde_json::to_string(&bg).unwrap();
    let ds = serde_json::from_str::<BG<u8, U32>>(&s).unwrap();
    assert_eq!(ds, bg);
    assert!(serde_json::from_str::<BG<u8, U16>>(&s).is_err());
}

#[test]
fn test_serdes_i64() {
    let data: Vec<_> = repeat_with(|| fastrand::i64(..)).take(8 * 8).collect();
    let bg = BG::<_, U8>::from_raw_vec(8, 8, data).unwrap();
    let s = serde_json::to_string(&bg).unwrap();
    let ds = serde_json::from_str::<BG<i64, U8>>(&s).unwrap();
    assert_eq!(ds, bg);
    assert!(serde_json::from_str::<BG<i64, U2>>(&s).is_err());
    assert!(serde_json::from_str::<BG<u64, U8>>(&s).is_err());
    assert!(serde_json::from_str::<BG<u16, U8>>(&s).is_err());
}

#[test]
fn test_serdes_f32() {
    let data: Vec<_> = repeat_with(fastrand::f32).take(4 * 4).collect();
    let bg = BG::<_, U4>::from_raw_vec(4, 4, data).unwrap();
    let s = serde_json::to_string(&bg).unwrap();
    let ds = serde_json::from_str::<BG<f32, U4>>(&s).unwrap();
    assert_eq!(ds, bg);
    assert!(serde_json::from_str::<BG<f32, U2>>(&s).is_err());
    assert!(serde_json::from_str::<BG<i64, U4>>(&s).is_err());
}

#[test]
fn test_serdes_rgb() {
    #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
    struct Rgb {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }
    let bg = {
        let mut data = Vec::with_capacity(2 * 2);
        for _ in 0..(2 * 2) {
            data.push(Rgb {
                r: fastrand::u8(..),
                g: fastrand::u8(..),
                b: fastrand::u8(..),
            });
        }
        BG::<_, U2>::from_raw_vec(2, 2, data).unwrap()
    };
    let s = serde_json::to_string(&bg).unwrap();
    let ds = serde_json::from_str::<BG<Rgb, U2>>(&s).unwrap();
    assert_eq!(bg, ds);
}
