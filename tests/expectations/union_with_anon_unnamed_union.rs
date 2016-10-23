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
pub struct foo {
    pub a: __BindgenUnionField<::std::os::raw::c_uint>,
    pub __bindgen_anon_1: __BindgenUnionField<foo__bindgen_ty_1>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct foo__bindgen_ty_1 {
    pub b: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub c: __BindgenUnionField<::std::os::raw::c_uchar>,
    pub bindgen_union_field: u16,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<foo__bindgen_ty_1>() , 2usize);
    assert_eq!(::std::mem::align_of::<foo__bindgen_ty_1>() , 2usize);
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(::std::mem::size_of::<foo>() , 4usize);
    assert_eq!(::std::mem::align_of::<foo>() , 4usize);
}
impl Clone for foo {
    fn clone(&self) -> Self { *self }
}
