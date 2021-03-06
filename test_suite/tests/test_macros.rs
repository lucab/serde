// Copyright 2017 Serde Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate serde_derive;

extern crate serde_test;
use self::serde_test::{Token, assert_tokens, assert_ser_tokens, assert_de_tokens,
                       assert_de_tokens_error};

use std::collections::BTreeMap;
use std::marker::PhantomData;

// That tests that the derived Serialize implementation doesn't trigger
// any warning about `serializer` not being used, in case of empty enums.
#[derive(Serialize)]
#[allow(dead_code)]
#[deny(unused_variables)]
enum Void {}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct NamedUnit;

#[derive(Debug, PartialEq, Serialize)]
struct SerNamedTuple<'a, 'b, A: 'a, B: 'b, C>(&'a A, &'b mut B, C);

#[derive(Debug, PartialEq, Deserialize)]
struct DeNamedTuple<A, B, C>(A, B, C);

#[derive(Debug, PartialEq, Serialize)]
struct SerNamedMap<'a, 'b, A: 'a, B: 'b, C> {
    a: &'a A,
    b: &'b mut B,
    c: C,
}

#[derive(Debug, PartialEq, Deserialize)]
struct DeNamedMap<A, B, C> {
    a: A,
    b: B,
    c: C,
}

#[derive(Debug, PartialEq, Serialize)]
enum SerEnum<'a, B: 'a, C: 'a, D>
where
    D: 'a,
{
    Unit,
    Seq(i8, B, &'a C, &'a mut D),
    Map { a: i8, b: B, c: &'a C, d: &'a mut D },

    // Make sure we can support more than one variant.
    _Unit2,
    _Seq2(i8, B, &'a C, &'a mut D),
    _Map2 { a: i8, b: B, c: &'a C, d: &'a mut D },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum DeEnum<B, C, D> {
    Unit,
    Seq(i8, B, C, D),
    Map { a: i8, b: B, c: C, d: D },

    // Make sure we can support more than one variant.
    _Unit2,
    _Seq2(i8, B, C, D),
    _Map2 { a: i8, b: B, c: C, d: D },
}

#[derive(Serialize)]
enum Lifetimes<'a> {
    LifetimeSeq(&'a i32),
    NoLifetimeSeq(i32),
    LifetimeMap { a: &'a i32 },
    NoLifetimeMap { a: i32 },
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericStruct<T> {
    x: T,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericNewTypeStruct<T>(T);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GenericTupleStruct<T, U>(T, U);

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum GenericEnum<T, U> {
    Unit,
    NewType(T),
    Seq(T, U),
    Map { x: T, y: U },
}

trait AssociatedType {
    type X;
}

impl AssociatedType for i32 {
    type X = i32;
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct DefaultTyParam<T: AssociatedType<X = i32> = i32> {
    phantom: PhantomData<T>,
}

#[test]
fn test_named_unit() {
    assert_tokens(&NamedUnit, &[Token::UnitStruct { name: "NamedUnit" }]);
}

#[test]
fn test_ser_named_tuple() {
    let a = 5;
    let mut b = 6;
    let c = 7;
    assert_ser_tokens(
        &SerNamedTuple(&a, &mut b, c),
        &[
            Token::TupleStruct { name: "SerNamedTuple", len: 3 },
            Token::I32(5),
            Token::I32(6),
            Token::I32(7),
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn test_de_named_tuple() {
    assert_de_tokens(
        &DeNamedTuple(5, 6, 7),
        &[
            Token::Seq { len: Some(3) },
            Token::I32(5),
            Token::I32(6),
            Token::I32(7),
            Token::SeqEnd,
        ],
    );

    assert_de_tokens(
        &DeNamedTuple(5, 6, 7),
        &[
            Token::TupleStruct { name: "DeNamedTuple", len: 3 },
            Token::I32(5),
            Token::I32(6),
            Token::I32(7),
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn test_ser_named_map() {
    let a = 5;
    let mut b = 6;
    let c = 7;

    assert_ser_tokens(
        &SerNamedMap {
             a: &a,
             b: &mut b,
             c: c,
         },
        &[
            Token::Struct { name: "SerNamedMap", len: 3 },

            Token::Str("a"),
            Token::I32(5),

            Token::Str("b"),
            Token::I32(6),

            Token::Str("c"),
            Token::I32(7),

            Token::StructEnd,
        ],
    );
}

#[test]
fn test_de_named_map() {
    assert_de_tokens(
        &DeNamedMap { a: 5, b: 6, c: 7 },
        &[
            Token::Struct { name: "DeNamedMap", len: 3 },

            Token::Str("a"),
            Token::I32(5),

            Token::Str("b"),
            Token::I32(6),

            Token::Str("c"),
            Token::I32(7),

            Token::StructEnd,
        ],
    );
}

#[test]
fn test_ser_enum_unit() {
    assert_ser_tokens(
        &SerEnum::Unit::<u32, u32, u32>,
        &[Token::UnitVariant { name: "SerEnum", variant: "Unit" }],
    );
}

#[test]
fn test_ser_enum_seq() {
    let a = 1;
    let b = 2;
    let c = 3;
    let mut d = 4;

    assert_ser_tokens(
        &SerEnum::Seq(a, b, &c, &mut d),
        &[
            Token::TupleVariant { name: "SerEnum", variant: "Seq", len: 4 },
            Token::I8(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::TupleVariantEnd,
        ],
    );
}

#[test]
fn test_ser_enum_map() {
    let a = 1;
    let b = 2;
    let c = 3;
    let mut d = 4;

    assert_ser_tokens(
        &SerEnum::Map {
             a: a,
             b: b,
             c: &c,
             d: &mut d,
         },
        &[
            Token::StructVariant { name: "SerEnum", variant: "Map", len: 4 },

            Token::Str("a"),
            Token::I8(1),

            Token::Str("b"),
            Token::I32(2),

            Token::Str("c"),
            Token::I32(3),

            Token::Str("d"),
            Token::I32(4),

            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_de_enum_unit() {
    assert_tokens(
        &DeEnum::Unit::<u32, u32, u32>,
        &[Token::UnitVariant { name: "DeEnum", variant: "Unit" }],
    );
}

#[test]
fn test_de_enum_seq() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;

    assert_tokens(
        &DeEnum::Seq(a, b, c, d),
        &[
            Token::TupleVariant { name: "DeEnum", variant: "Seq", len: 4 },
            Token::I8(1),
            Token::I32(2),
            Token::I32(3),
            Token::I32(4),
            Token::TupleVariantEnd,
        ],
    );
}

#[test]
fn test_de_enum_map() {
    let a = 1;
    let b = 2;
    let c = 3;
    let d = 4;

    assert_tokens(
        &DeEnum::Map {
             a: a,
             b: b,
             c: c,
             d: d,
         },
        &[
            Token::StructVariant { name: "DeEnum", variant: "Map", len: 4 },

            Token::Str("a"),
            Token::I8(1),

            Token::Str("b"),
            Token::I32(2),

            Token::Str("c"),
            Token::I32(3),

            Token::Str("d"),
            Token::I32(4),

            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_lifetimes() {
    let value = 5;

    assert_ser_tokens(
        &Lifetimes::LifetimeSeq(&value),
        &[
            Token::NewtypeVariant { name: "Lifetimes", variant: "LifetimeSeq" },
            Token::I32(5),
        ],
    );

    assert_ser_tokens(
        &Lifetimes::NoLifetimeSeq(5),
        &[
            Token::NewtypeVariant { name: "Lifetimes", variant: "NoLifetimeSeq" },
            Token::I32(5),
        ],
    );

    assert_ser_tokens(
        &Lifetimes::LifetimeMap { a: &value },
        &[
            Token::StructVariant { name: "Lifetimes", variant: "LifetimeMap", len: 1 },

            Token::Str("a"),
            Token::I32(5),

            Token::StructVariantEnd,
        ],
    );

    assert_ser_tokens(
        &Lifetimes::NoLifetimeMap { a: 5 },
        &[
            Token::StructVariant { name: "Lifetimes", variant: "NoLifetimeMap", len: 1 },

            Token::Str("a"),
            Token::I32(5),

            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_generic_struct() {
    assert_tokens(
        &GenericStruct { x: 5u32 },
        &[
            Token::Struct { name: "GenericStruct", len: 1 },

            Token::Str("x"),
            Token::U32(5),

            Token::StructEnd,
        ],
    );
}

#[test]
fn test_generic_newtype_struct() {
    assert_tokens(
        &GenericNewTypeStruct(5u32),
        &[Token::NewtypeStruct { name: "GenericNewTypeStruct" }, Token::U32(5)],
    );
}

#[test]
fn test_generic_tuple_struct() {
    assert_tokens(
        &GenericTupleStruct(5u32, 6u32),
        &[
            Token::TupleStruct { name: "GenericTupleStruct", len: 2 },
            Token::U32(5),
            Token::U32(6),
            Token::TupleStructEnd,
        ],
    );
}

#[test]
fn test_generic_enum_unit() {
    assert_tokens(
        &GenericEnum::Unit::<u32, u32>,
        &[Token::UnitVariant { name: "GenericEnum", variant: "Unit" }],
    );
}

#[test]
fn test_generic_enum_newtype() {
    assert_tokens(
        &GenericEnum::NewType::<u32, u32>(5),
        &[
            Token::NewtypeVariant { name: "GenericEnum", variant: "NewType" },
            Token::U32(5),
        ],
    );
}

#[test]
fn test_generic_enum_seq() {
    assert_tokens(
        &GenericEnum::Seq::<u32, u32>(5, 6),
        &[
            Token::TupleVariant { name: "GenericEnum", variant: "Seq", len: 2 },
            Token::U32(5),
            Token::U32(6),
            Token::TupleVariantEnd,
        ],
    );
}

#[test]
fn test_generic_enum_map() {
    assert_tokens(
        &GenericEnum::Map::<u32, u32> { x: 5, y: 6 },
        &[
            Token::StructVariant { name: "GenericEnum", variant: "Map", len: 2 },

            Token::Str("x"),
            Token::U32(5),

            Token::Str("y"),
            Token::U32(6),

            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_default_ty_param() {
    assert_tokens(
        &DefaultTyParam::<i32> { phantom: PhantomData },
        &[
            Token::Struct { name: "DefaultTyParam", len: 1 },

            Token::Str("phantom"),
            Token::UnitStruct { name: "PhantomData" },

            Token::StructEnd,
        ],
    );
}

#[test]
fn test_enum_state_field() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum SomeEnum {
        Key { key: char, state: bool },
    }

    assert_tokens(
        &SomeEnum::Key {
             key: 'a',
             state: true,
         },
        &[
            Token::StructVariant { name: "SomeEnum", variant: "Key", len: 2 },

            Token::Str("key"),
            Token::Char('a'),

            Token::Str("state"),
            Token::Bool(true),

            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_untagged_enum() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    enum Untagged {
        A { a: u8 },
        B { b: u8 },
        C,
        D(u8),
        E(String),
        F(u8, u8),
    }

    assert_tokens(
        &Untagged::A { a: 1 },
        &[
            Token::Struct { name: "Untagged", len: 1 },

            Token::Str("a"),
            Token::U8(1),

            Token::StructEnd,
        ],
    );

    assert_tokens(
        &Untagged::B { b: 2 },
        &[
            Token::Struct { name: "Untagged", len: 1 },

            Token::Str("b"),
            Token::U8(2),

            Token::StructEnd,
        ],
    );

    assert_tokens(&Untagged::C, &[Token::Unit]);

    assert_tokens(&Untagged::D(4), &[Token::U8(4)]);
    assert_tokens(&Untagged::E("e".to_owned()), &[Token::Str("e")]);

    assert_tokens(
        &Untagged::F(1, 2),
        &[Token::Tuple { len: 2 }, Token::U8(1), Token::U8(2), Token::TupleEnd],
    );

    assert_de_tokens_error::<Untagged>(
        &[Token::None],
        "data did not match any variant of untagged enum Untagged",
    );

    assert_de_tokens_error::<Untagged>(
        &[Token::Tuple { len: 1 }, Token::U8(1), Token::TupleEnd],
        "data did not match any variant of untagged enum Untagged",
    );

    assert_de_tokens_error::<Untagged>(
        &[
            Token::Tuple { len: 3 },
            Token::U8(1),
            Token::U8(2),
            Token::U8(3),
            Token::TupleEnd,
        ],
        "data did not match any variant of untagged enum Untagged",
    );
}

#[test]
fn test_internally_tagged_enum() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Newtype(BTreeMap<String, String>);

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Struct {
        f: u8,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type")]
    enum InternallyTagged {
        A { a: u8 },
        B { b: u8 },
        C,
        D(BTreeMap<String, String>),
        E(Newtype),
        F(Struct),
    }

    assert_tokens(
        &InternallyTagged::A { a: 1 },
        &[
            Token::Struct { name: "InternallyTagged", len: 2 },

            Token::Str("type"),
            Token::Str("A"),

            Token::Str("a"),
            Token::U8(1),

            Token::StructEnd,
        ],
    );

    assert_tokens(
        &InternallyTagged::B { b: 2 },
        &[
            Token::Struct { name: "InternallyTagged", len: 2 },

            Token::Str("type"),
            Token::Str("B"),

            Token::Str("b"),
            Token::U8(2),

            Token::StructEnd,
        ],
    );

    assert_tokens(
        &InternallyTagged::C,
        &[
            Token::Struct { name: "InternallyTagged", len: 1 },

            Token::Str("type"),
            Token::Str("C"),

            Token::StructEnd,
        ],
    );

    assert_tokens(
        &InternallyTagged::D(BTreeMap::new()),
        &[
            Token::Map { len: Some(1) },

            Token::Str("type"),
            Token::Str("D"),

            Token::MapEnd,
        ],
    );

    assert_tokens(
        &InternallyTagged::E(Newtype(BTreeMap::new())),
        &[
            Token::Map { len: Some(1) },

            Token::Str("type"),
            Token::Str("E"),

            Token::MapEnd,
        ],
    );

    assert_tokens(
        &InternallyTagged::F(Struct { f: 6 }),
        &[
            Token::Struct { name: "Struct", len: 2 },

            Token::Str("type"),
            Token::Str("F"),

            Token::Str("f"),
            Token::U8(6),

            Token::StructEnd,
        ],
    );

    assert_de_tokens_error::<InternallyTagged>(
        &[Token::Map { len: Some(0) }, Token::MapEnd],
        "missing field `type`",
    );

    assert_de_tokens_error::<InternallyTagged>(
        &[
            Token::Map { len: Some(1) },

            Token::Str("type"),
            Token::Str("Z"),

            Token::MapEnd,
        ],
        "unknown variant `Z`, expected one of `A`, `B`, `C`, `D`, `E`, `F`",
    );
}

#[test]
fn test_adjacently_tagged_enum() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "t", content = "c")]
    enum AdjacentlyTagged<T> {
        Unit,
        Newtype(T),
        Tuple(u8, u8),
        Struct { f: u8 },
    }

    // unit with no content
    assert_tokens(
        &AdjacentlyTagged::Unit::<u8>,
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 1 },

            Token::Str("t"),
            Token::Str("Unit"),

            Token::StructEnd,
        ],
    );

    // unit with tag first
    assert_de_tokens(
        &AdjacentlyTagged::Unit::<u8>,
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 1 },

            Token::Str("t"),
            Token::Str("Unit"),

            Token::Str("c"),
            Token::Unit,

            Token::StructEnd,
        ],
    );

    // unit with content first
    assert_de_tokens(
        &AdjacentlyTagged::Unit::<u8>,
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 1 },

            Token::Str("c"),
            Token::Unit,

            Token::Str("t"),
            Token::Str("Unit"),

            Token::StructEnd,
        ],
    );

    // unit with excess content (f, g, h)
    assert_de_tokens(
        &AdjacentlyTagged::Unit::<u8>,
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 3 },

            Token::Str("f"),
            Token::Unit,

            Token::Str("t"),
            Token::Str("Unit"),

            Token::Str("g"),
            Token::Unit,

            Token::Str("c"),
            Token::Unit,

            Token::Str("h"),
            Token::Unit,

            Token::StructEnd,
        ],
    );

    // newtype with tag first
    assert_tokens(
        &AdjacentlyTagged::Newtype::<u8>(1),
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2 },

            Token::Str("t"),
            Token::Str("Newtype"),

            Token::Str("c"),
            Token::U8(1),

            Token::StructEnd,
        ],
    );

    // newtype with content first
    assert_de_tokens(
        &AdjacentlyTagged::Newtype::<u8>(1),
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2 },

            Token::Str("c"),
            Token::U8(1),

            Token::Str("t"),
            Token::Str("Newtype"),

            Token::StructEnd,
        ],
    );

    // tuple with tag first
    assert_tokens(
        &AdjacentlyTagged::Tuple::<u8>(1, 1),
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2 },

            Token::Str("t"),
            Token::Str("Tuple"),

            Token::Str("c"),
            Token::Tuple { len: 2 },
            Token::U8(1),
            Token::U8(1),
            Token::TupleEnd,

            Token::StructEnd,
        ],
    );

    // tuple with content first
    assert_de_tokens(
        &AdjacentlyTagged::Tuple::<u8>(1, 1),
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2 },

            Token::Str("c"),
            Token::Tuple { len: 2 },
            Token::U8(1),
            Token::U8(1),
            Token::TupleEnd,

            Token::Str("t"),
            Token::Str("Tuple"),

            Token::StructEnd,
        ],
    );

    // struct with tag first
    assert_tokens(
        &AdjacentlyTagged::Struct::<u8> { f: 1 },
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2 },

            Token::Str("t"),
            Token::Str("Struct"),

            Token::Str("c"),
            Token::Struct { name: "Struct", len: 1 },
            Token::Str("f"),
            Token::U8(1),
            Token::StructEnd,

            Token::StructEnd,
        ],
    );

    // struct with content first
    assert_de_tokens(
        &AdjacentlyTagged::Struct::<u8> { f: 1 },
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2 },

            Token::Str("c"),
            Token::Struct { name: "Struct", len: 1 },
            Token::Str("f"),
            Token::U8(1),
            Token::StructEnd,

            Token::Str("t"),
            Token::Str("Struct"),

            Token::StructEnd,
        ],
    );
}

#[test]
fn test_adjacently_tagged_enum_deny_unknown_fields() {
    #[derive(Debug, PartialEq, Deserialize)]
    #[serde(tag = "t", content = "c", deny_unknown_fields)]
    enum AdjacentlyTagged {
        Unit,
    }

    assert_de_tokens(
        &AdjacentlyTagged::Unit,
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 2},

            Token::Str("t"),
            Token::Str("Unit"),

            Token::Str("c"),
            Token::Unit,

            Token::StructEnd,
        ],
    );

    assert_de_tokens_error::<AdjacentlyTagged>(
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 3},

            Token::Str("t"),
            Token::Str("Unit"),

            Token::Str("c"),
            Token::Unit,

            Token::Str("h"),
        ],
        r#"invalid value: string "h", expected "t" or "c""#
    );

    assert_de_tokens_error::<AdjacentlyTagged>(
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 3},

            Token::Str("h"),
        ],
        r#"invalid value: string "h", expected "t" or "c""#
    );

    assert_de_tokens_error::<AdjacentlyTagged>(
        &[
            Token::Struct { name: "AdjacentlyTagged", len: 3},

            Token::Str("c"),
            Token::Unit,

            Token::Str("h"),
        ],
        r#"invalid value: string "h", expected "t" or "c""#
    );
}

#[test]
fn test_enum_in_internally_tagged_enum() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(tag = "type")]
    enum Outer {
        Inner(Inner),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Inner {
        Unit,
        Newtype(u8),
        Tuple(u8, u8),
        Struct { f: u8 },
    }

    assert_tokens(
        &Outer::Inner(Inner::Unit),
        &[
            Token::Map { len: Some(2) },

            Token::Str("type"),
            Token::Str("Inner"),

            Token::Str("Unit"),
            Token::Unit,

            Token::MapEnd,
        ],
    );

    assert_tokens(
        &Outer::Inner(Inner::Newtype(1)),
        &[
            Token::Map { len: Some(2) },

            Token::Str("type"),
            Token::Str("Inner"),

            Token::Str("Newtype"),
            Token::U8(1),

            Token::MapEnd,
        ],
    );

    assert_tokens(
        &Outer::Inner(Inner::Tuple(1, 1)),
        &[
            Token::Map { len: Some(2) },

            Token::Str("type"),
            Token::Str("Inner"),

            Token::Str("Tuple"),
            Token::TupleStruct { name: "Tuple", len: 2 },
            Token::U8(1),
            Token::U8(1),
            Token::TupleStructEnd,

            Token::MapEnd,
        ],
    );

    assert_tokens(
        &Outer::Inner(Inner::Struct { f: 1 }),
        &[
            Token::Map { len: Some(2) },

            Token::Str("type"),
            Token::Str("Inner"),

            Token::Str("Struct"),
            Token::Struct { name: "Struct", len: 1 },
            Token::Str("f"),
            Token::U8(1),
            Token::StructEnd,

            Token::MapEnd,
        ],
    );
}

#[test]
fn test_enum_in_untagged_enum() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    #[serde(untagged)]
    enum Outer {
        Inner(Inner),
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Inner {
        Unit,
        Newtype(u8),
        Tuple(u8, u8),
        Struct { f: u8 },
    }

    assert_tokens(
        &Outer::Inner(Inner::Unit),
        &[Token::UnitVariant { name: "Inner", variant: "Unit" }],
    );

    assert_tokens(
        &Outer::Inner(Inner::Newtype(1)),
        &[Token::NewtypeVariant { name: "Inner", variant: "Newtype" }, Token::U8(1)],
    );

    assert_tokens(
        &Outer::Inner(Inner::Tuple(1, 1)),
        &[
            Token::TupleVariant { name: "Inner", variant: "Tuple", len: 2 },
            Token::U8(1),
            Token::U8(1),
            Token::TupleVariantEnd,
        ],
    );

    assert_tokens(
        &Outer::Inner(Inner::Struct { f: 1 }),
        &[
            Token::StructVariant { name: "Inner", variant: "Struct", len: 1 },

            Token::Str("f"),
            Token::U8(1),

            Token::StructVariantEnd,
        ],
    );
}

#[test]
fn test_rename_all() {
    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "snake_case")]
    enum E {
        #[serde(rename_all = "camelCase")]
        Serialize {
            serialize: bool,
            serialize_seq: bool,
        },
        #[serde(rename_all = "kebab-case")]
        SerializeSeq {
            serialize: bool,
            serialize_seq: bool,
        },
        #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
        SerializeMap {
            serialize: bool,
            serialize_seq: bool,
        },
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    #[serde(rename_all = "PascalCase")]
    struct S {
        serialize: bool,
        serialize_seq: bool,
    }

    assert_tokens(
        &E::Serialize {
             serialize: true,
             serialize_seq: true,
         },
        &[
            Token::StructVariant { name: "E", variant: "serialize", len: 2 },
            Token::Str("serialize"),
            Token::Bool(true),
            Token::Str("serializeSeq"),
            Token::Bool(true),
            Token::StructVariantEnd,
        ],
    );

    assert_tokens(
        &E::SerializeSeq {
             serialize: true,
             serialize_seq: true,
         },
        &[
            Token::StructVariant { name: "E", variant: "serialize_seq", len: 2 },
            Token::Str("serialize"),
            Token::Bool(true),
            Token::Str("serialize-seq"),
            Token::Bool(true),
            Token::StructVariantEnd,
        ],
    );

    assert_tokens(
        &E::SerializeMap {
             serialize: true,
             serialize_seq: true,
         },
        &[
            Token::StructVariant { name: "E", variant: "serialize_map", len: 2 },
            Token::Str("SERIALIZE"),
            Token::Bool(true),
            Token::Str("SERIALIZE_SEQ"),
            Token::Bool(true),
            Token::StructVariantEnd,
        ],
    );

    assert_tokens(
        &S {
             serialize: true,
             serialize_seq: true,
         },
        &[
            Token::Struct { name: "S", len: 2 },
            Token::Str("Serialize"),
            Token::Bool(true),
            Token::Str("SerializeSeq"),
            Token::Bool(true),
            Token::StructEnd,
        ],
    );
}
