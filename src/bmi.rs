pub trait Bmi {
    fn pext(&self, mask: u64) -> u64;
    fn pdep(&self, mask: u64) -> u64;
    fn blsi(&self) -> u64;
    fn blsr(&self) -> u64;
}

impl Bmi for u64 {
    fn pext(&self, mask: u64) -> u64 {
        unsafe { core::arch::x86_64::_pext_u64(*self, mask) }
    }
    fn pdep(&self, mask: u64) -> u64 {
        unsafe { core::arch::x86_64::_pdep_u64(*self, mask) }
    }

    fn blsi(&self) -> u64 {
        unsafe { core::arch::x86_64::_blsi_u64(*self) }
    }

    fn blsr(&self) -> u64 {
        unsafe { core::arch::x86_64::_blsr_u64(*self) }
    }
}
