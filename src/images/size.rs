use std::fmt::Debug;

pub trait Size: Send + Sync + Debug + Clone + 'static {
    const WIDTH: u32;
    const HEIGHT: u32;
}

pub trait MultipleOf<T>: Size {}

pub trait SmallerThan<T>: Size {}

// =========== Piece image's size ===============
#[derive(Debug, Clone)]
pub struct Size30x30;

impl Size for Size30x30 {
    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 30;
}

impl SmallerThan<Size1500x1500> for Size30x30 {}
impl SmallerThan<Size3000x3000> for Size30x30 {}

#[derive(Debug, Clone)]
pub struct Size50x50;

impl Size for Size50x50 {
    const WIDTH: u32 = 50;
    const HEIGHT: u32 = 50;
}

impl SmallerThan<Size1500x1500> for Size50x50 {}
impl SmallerThan<Size3000x3000> for Size50x50 {}

#[derive(Debug, Clone)]
pub struct Size100x100;

impl Size for Size100x100 {
    const WIDTH: u32 = 100;
    const HEIGHT: u32 = 100;
}

impl SmallerThan<Size1500x1500> for Size100x100 {}
impl SmallerThan<Size3000x3000> for Size100x100 {}

// =========== Original image's size ===============
#[derive(Debug, Clone)]
pub struct Size1500x1500;

impl Size for Size1500x1500 {
    const WIDTH: u32 = 1500;
    const HEIGHT: u32 = 1500;
}

impl MultipleOf<Size30x30> for Size1500x1500 {}
impl MultipleOf<Size50x50> for Size1500x1500 {}
impl MultipleOf<Size100x100> for Size1500x1500 {}

#[derive(Debug, Clone)]
pub struct Size3000x3000;

impl Size for Size3000x3000 {
    const WIDTH: u32 = 3000;
    const HEIGHT: u32 = 3000;
}

impl MultipleOf<Size30x30> for Size3000x3000 {}
impl MultipleOf<Size50x50> for Size3000x3000 {}
impl MultipleOf<Size100x100> for Size3000x3000 {}
