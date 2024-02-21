pub(crate) mod amap;
pub(crate) mod client;
pub(crate) mod kmkc;
pub(crate) mod models;
pub(crate) mod musq;
pub(super) mod parser;
pub(crate) mod rbean;
pub(crate) mod sjv;
pub(crate) mod tools;

/// All available implementations
pub enum Implementations {
    /// KM by KC
    Kmkc,
    /// MU! by SQ
    Musq,
    /// AM by AP
    Amap,
    /// SJ/M by V
    Sjv,
    /// 小豆 by KRKR
    Rbean,
}
