use bitflags::bitflags;

bitflags! {
    /// Flags how they are serialized to a storage location
    #[derive(Copy, Clone, Debug)]
    pub struct Flags: u16 {
        /// A portion of a the flags that encodes the length of the path that follows.
        const PATH_LEN = 0x0fff;
        const STAGE_MASK = 0x3000;
        /// If set, there is more extended flags past this one
        const EXTENDED = 0x4000;
        /// If set, the entry be assumed to match with the version on the working tree, as a way to avoid `lstat()`  checks.
        const ASSUME_VALID = 0x8000;
    }
}

impl Flags {
    pub fn as_u16(&self) -> u16 {
        self.bits()
    }

    pub fn to_memory(&self) -> Flags {
        Flags::from_bits_retain(self.bits())
    }
}
