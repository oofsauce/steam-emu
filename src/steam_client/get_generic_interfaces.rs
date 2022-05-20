use std::ffi::CStr;
use std::{os::raw::c_char, ffi::c_void, ptr};

use tracing::debug;

use crate::steam_client::{SteamAPI_ISteamClient_GetISteamApps, SteamAPI_ISteamClient_GetISteamInput, SteamAPI_ISteamClient_GetISteamParentalSettings, SteamAPI_ISteamClient_GetISteamUtils, SteamAPI_ISteamClient_GetISteamUserStats, SteamAPI_ISteamClient_GetISteamFriends, SteamAPI_ISteamClient_GetISteamRemoteStorage, SteamAPI_ISteamClient_GetISteamHTMLSurface, SteamAPI_ISteamClient_GetISteamInventory, SteamAPI_ISteamClient_GetISteamUGC};
use crate::{HSteamUser, HSteamPipe};

use super::SteamClient;
use super::SteamAPI_ISteamClient_GetISteamUser;


#[no_mangle]
pub unsafe extern "fastcall" fn SteamAPI_ISteamClient_GetISteamGenericInterface(
  self_: *mut SteamClient, 
  _edx: *mut c_void,
  hSteamUser: HSteamUser, 
  hSteamPipe: HSteamPipe, 
  pchVersion: *const c_char
) -> *mut c_void {
  let version = CStr::from_ptr(pchVersion).to_str().unwrap();
  debug!("GetISteamGenericInterface '{:?}'", version);

  // FIXME: there are so many interfaces yet to be added here
  if version.starts_with("SteamUser") {
    return SteamAPI_ISteamClient_GetISteamUser(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamUtils") {
    return SteamAPI_ISteamClient_GetISteamUtils(self_, _edx, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamInput") {
    return SteamAPI_ISteamClient_GetISteamInput(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamFriends") {
    return SteamAPI_ISteamClient_GetISteamFriends(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("SteamNetworkingUtils") {
    return ptr::addr_of_mut!((*self_).steam_networking_utils) as _;
  } else if version.starts_with("STEAMAPPS_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamApps(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMPARENTALSETTINGS_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamParentalSettings(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMUSERSTATS_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamUserStats(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMREMOTESTORAGE_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamRemoteStorage(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMHTMLSURFACE_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamHTMLSurface(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMINVENTORY_INTERFACE") {
    return SteamAPI_ISteamClient_GetISteamInventory(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  } else if version.starts_with("STEAMUGC_INTERFACE_VERSION") {
    return SteamAPI_ISteamClient_GetISteamUGC(self_, _edx, hSteamUser, hSteamPipe, pchVersion) as _;
  }
  
  ptr::null_mut()
}