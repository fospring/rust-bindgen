/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[derive(Debug)]
#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
#[repr(C)]
#[derive(Debug, Copy)]
pub struct A {
    pub c: ::std::os::raw::c_uint,
    pub named_union: A__bindgen_ty_1,
    pub __bindgen_anon_1: A__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct A_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_A_Segment() {
    assert_eq!(::std::mem::size_of::<A_Segment>() , 8usize);
    assert_eq!(::std::mem::align_of::<A_Segment>() , 4usize);
}
impl Clone for A_Segment {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct A__bindgen_ty_1 {
    pub f: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_A__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<A__bindgen_ty_1>() , 4usize);
    assert_eq!(::std::mem::align_of::<A__bindgen_ty_1>() , 4usize);
}
impl Clone for A__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct A__bindgen_ty_2 {
    pub d: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u32,
}
#[test]
fn bindgen_test_layout_A__bindgen_ty_2() {
    assert_eq!(::std::mem::size_of::<A__bindgen_ty_2>() , 4usize);
    assert_eq!(::std::mem::align_of::<A__bindgen_ty_2>() , 4usize);
}
impl Clone for A__bindgen_ty_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_A() {
    assert_eq!(::std::mem::size_of::<A>() , 12usize);
    assert_eq!(::std::mem::align_of::<A>() , 4usize);
}
impl Clone for A {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct B {
    pub d: ::std::os::raw::c_uint,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct B_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_B_Segment() {
    assert_eq!(::std::mem::size_of::<B_Segment>() , 8usize);
    assert_eq!(::std::mem::align_of::<B_Segment>() , 4usize);
}
impl Clone for B_Segment {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_B() {
    assert_eq!(::std::mem::size_of::<B>() , 4usize);
    assert_eq!(::std::mem::align_of::<B>() , 4usize);
}
impl Clone for B {
    fn clone(&self) -> Self { *self }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum StepSyntax {
    Keyword = 0,
    FunctionalWithoutKeyword = 1,
    FunctionalWithStartKeyword = 2,
    FunctionalWithEndKeyword = 3,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C {
    pub d: ::std::os::raw::c_uint,
    pub __bindgen_anon_1: C__bindgen_ty_1,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C__bindgen_ty_1 {
    pub mFunc: __BindgenUnionField<C__bindgen_ty_1_1>,
    pub __bindgen_anon_1: __BindgenUnionField<C__bindgen_ty_1_2>,
    pub bindgen_union_field: [u32; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C__bindgen_ty_1_1 {
    pub mX1: f32,
    pub mY1: f32,
    pub mX2: f32,
    pub mY2: f32,
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1_1() {
    assert_eq!(::std::mem::size_of::<C__bindgen_ty_1_1>() , 16usize);
    assert_eq!(::std::mem::align_of::<C__bindgen_ty_1_1>() , 4usize);
}
impl Clone for C__bindgen_ty_1_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C__bindgen_ty_1_2 {
    pub mStepSyntax: StepSyntax,
    pub mSteps: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1_2() {
    assert_eq!(::std::mem::size_of::<C__bindgen_ty_1_2>() , 8usize);
    assert_eq!(::std::mem::align_of::<C__bindgen_ty_1_2>() , 4usize);
}
impl Clone for C__bindgen_ty_1_2 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_C__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<C__bindgen_ty_1>() , 16usize);
    assert_eq!(::std::mem::align_of::<C__bindgen_ty_1>() , 4usize);
}
impl Clone for C__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct C_Segment {
    pub begin: ::std::os::raw::c_int,
    pub end: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_C_Segment() {
    assert_eq!(::std::mem::size_of::<C_Segment>() , 8usize);
    assert_eq!(::std::mem::align_of::<C_Segment>() , 4usize);
}
impl Clone for C_Segment {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_C() {
    assert_eq!(::std::mem::size_of::<C>() , 20usize);
    assert_eq!(::std::mem::align_of::<C>() , 4usize);
}
impl Clone for C {
    fn clone(&self) -> Self { *self }
}
