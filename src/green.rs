mod node;
mod token;
mod element;
mod builder;

pub(crate) use self::element::GreenElementRef;
use self::element::{GreenElement, PackedGreenElement};

pub use self::{
    builder::{Checkpoint, GreenNodeBuilder, NodeCache},
    node::{Children, GreenNode},
    token::GreenToken,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_send_sync() {
        fn f<T: Send + Sync>() {}
        f::<GreenNode>();
        f::<GreenToken>();
        f::<GreenElement>();
        f::<PackedGreenElement>();
    }

    #[test]
    fn test_size_of() {
        use std::mem::size_of;

        eprintln!("GreenNode          {}", size_of::<GreenNode>());
        eprintln!("GreenToken         {}", size_of::<GreenToken>());
        eprintln!("GreenElement       {}", size_of::<GreenElement>());
        eprintln!("PackedGreenElement {}", size_of::<PackedGreenElement>());
    }
}
