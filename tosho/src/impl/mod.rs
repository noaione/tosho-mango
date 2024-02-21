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

const WINDOWS_RESERVED: [&str; 22] = [
    "con", "prn", "aux", "nul", "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8",
    "com9", "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9",
];

/// A quick helper to clean up filename strings
pub(crate) fn clean_filename(filename: &str) -> String {
    #[cfg(windows)]
    let matchers = vec![
        ('/', "-"),
        ('\\', "-"),
        ('?', ""),
        ('*', ""),
        ('<', ""),
        ('>', ""),
        (':', ""),
        ('|', ""),
        ('"', ""),
    ];
    #[cfg(not(windows))]
    let matchers = vec![('/', "-")];

    let mut cleaned = filename.to_string();
    for (from, to) in matchers {
        cleaned = cleaned.replace(from, to);
    }

    // disallow windows reserved names
    for reserved in WINDOWS_RESERVED {
        if cleaned.eq_ignore_ascii_case(reserved) {
            // append a dash to the end
            return format!("tosho-{}", cleaned);
        }
    }

    cleaned
}
