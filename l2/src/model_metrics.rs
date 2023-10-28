pub const CODE_BITS: usize = 16;
pub const FREQUENCY_BITS: usize = 14;
pub const CODE_MAX: usize = (1 << CODE_BITS) - 1;
pub const FREQUENCY_MAX: usize = (1 << FREQUENCY_BITS) - 1;
pub const CODE_FIRST_QTR: usize = 1 << (CODE_BITS - 2);
pub const CODE_HALF: usize = 2 * CODE_FIRST_QTR;
pub const CODE_THIRD_QTR: usize = 3 * CODE_FIRST_QTR;
