extern "system" {
	fn mmap(
		addr: *const core::ffi::c_void,
		len: usize,
		prot: core::ffi::c_int,
		flags: core::ffi::c_int,
		fd: core::ffi::c_int,
		off: i64
	) -> *mut core::ffi::c_void;

	fn munmap(
		addr: *const core::ffi::c_void,
		len: usize
	) -> core::ffi::c_int;
}

#[derive(Debug)]
pub struct Mmap<'a> {
	// NOTE: This can only be &mut right now because only ::exec is exposed.
	// If other permissions are allowed, need a runtime solution.
	slice: &'a mut [u8]
}

impl<'a> Drop for Mmap<'a> {
	fn drop(&mut self) {
		unsafe { munmap(self.slice.as_ptr() as _, self.slice.len()) };
	}
}

impl<'a> Mmap<'a> {
	fn new(mem: impl AsRef<[u8]>, prot: core::ffi::c_int) -> super::MmapResult<Self> {
		let mem = mem.as_ref();

		let slice = unsafe {
			let map = mmap(
				core::ptr::null(),
				mem.len(),
				prot,
				0x02 /* MAP_PRIVATE */ | 0x20 /* MAP_ANONYMOUS */,
				-1,
				0
			) as *mut u8;

			if (map as isize) == -1 {
				return Err(super::MmapError::Failed);
			}

			let slice = core::slice::from_raw_parts_mut(map, mem.len());
			slice.copy_from_slice(&mem);

			slice
		};

		Ok(Self {
			slice,
		})
	}

	pub fn exec(mem: impl AsRef<[u8]>) -> super::MmapResult<Self> {
		Self::new(mem, 0x2 /* PROT_WRITE */ | 0x4 /* PROT_EXEC */ | 0x1 /* PROT_READ */)
	}

	pub fn as_ptr(&self) -> *const u8 {
		self.slice.as_ptr()
	}
}

impl<'a> AsRef<[u8]> for Mmap<'a> {
	fn as_ref(&self) -> &[u8] {
		&self.slice
	}
}

impl<'a> AsMut<[u8]> for Mmap<'a> {
	fn as_mut(&mut self) -> &mut [u8] {
		&mut self.slice
	}
}
