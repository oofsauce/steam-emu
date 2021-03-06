use std::{os::raw::c_char, ffi::c_void, ptr};
use tracing::{info, debug, error};

use vtables::VTable;
use vtables_derive::{VTable, has_vtable};

use crate::{CSteamID, uint64, int32};

use super::SteamAPICall_t;

#[has_vtable]
#[derive(VTable, Debug)]
pub struct SteamUser {
  steam_id: CSteamID,
}

impl SteamUser {
  pub fn new() -> Self {
    Self { vtable: get_vtable(), steam_id: CSteamID::new() }
  }

  pub fn steam_id(&mut self) -> *mut CSteamID {
    ptr::addr_of_mut!(self.steam_id)
  }
}

#[no_mangle]
extern "fastcall" fn BLoggedOn(self_: *mut SteamUser, _edx: *mut c_void) -> bool {
  true
}


#[no_mangle]
unsafe extern "fastcall" fn GetSteamID(self_: *mut SteamUser, _edx: *mut c_void, _kill_me: *mut c_void) -> *mut CSteamID {
  debug!("SteamUser->GetSteamID");
  (*self_).steam_id()
}

unsafe extern "fastcall" fn RequestEncryptedAppTicked(
  self_: *mut SteamUser, _edx: *mut c_void,
  p_data_to_include: *mut c_void,
  cb_data_to_include: int32,
) -> SteamAPICall_t {
  0
}

pub fn get_vtable() -> *mut *mut usize {
  unsafe {
    static mut VTABLE: [*mut usize; 21] = [
      ptr::null_mut(),
      BLoggedOn as _,
      GetSteamID as _,
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      ptr::null_mut(),
      RequestEncryptedAppTicked as _,
    ];
    VTABLE.as_mut_ptr()
  }
}