use super::group_trait::{AdditiveGroup, MultiplicativeGroup};

pub trait Field: AdditiveGroup + MultiplicativeGroup {}
