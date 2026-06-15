pub mod limb;
pub mod word;


pub trait Layout {
    const N: usize;
    type Ls;
    type Ms;
}