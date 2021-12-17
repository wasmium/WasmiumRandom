// Range for integers 0-9
pub(crate) const NUMERIC: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

// Range for alphanumeric letters and numbers 0-9,A-Z,a-z
pub(crate) const ALPHANUMERIC: [&str; 62] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "a", "b", "c", "d", "e", "f", "g", "h", "i",
    "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B",
    "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U",
    "V", "W", "X", "Y", "Z",
];

// Range for alphanumeric letters and numbers 0-9,A-Z,a-z
pub(crate) const ALPHABET: [&str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

/// `WasmiumRandom` contains helper method to generate different random byte length.
/// See module level documentation for more details [crate]
pub struct WasmiumRandom;
