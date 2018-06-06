pub trait Size {
    const WIDTH: u32;
    const HEIGHT: u32;
}

pub trait MultipleOf<T>: Size {}

pub trait SmallerThan<T>: Size {}

pub struct Size30x30;

impl Size for Size30x30 {
    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 30;
}

impl SmallerThan<Size1500x1500> for Size30x30 {}

pub struct Size1500x1500;

impl Size for Size1500x1500 {
    const WIDTH: u32 = 1500;
    const HEIGHT: u32 = 1500;
}

impl MultipleOf<Size30x30> for Size1500x1500 {}
