pub(crate) trait Pdep {
    fn pdep(&self, mask: u64) -> u64;
}

impl Pdep for u64 {
    fn pdep(&self, mask: u64) -> u64 {
        unsafe {
            core::arch::x86_64::_pdep_u64(*self, mask)
        }
    }
}