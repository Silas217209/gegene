pub(crate) trait Pext {
    fn pext(&self, mask: u64) -> u64;
}

impl Pext for u64 {
    fn pext(&self, mask: u64) -> u64 {
        unsafe { core::arch::x86_64::_pext_u64(*self, mask) }
    }
}
