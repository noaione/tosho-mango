pub(crate) mod kmkc;
pub(crate) mod musq;

/// All available implementations
pub enum Implementations {
    /// KM by KC
    #[allow(dead_code)]
    Kmkc,
    /// MU! by SQ
    Musq,
}
