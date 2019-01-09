use crate::game::Asset;
pub mod tree;
pub use self::tree::Tree;

pub trait Object: Asset {
    fn update(&self) {}
}

