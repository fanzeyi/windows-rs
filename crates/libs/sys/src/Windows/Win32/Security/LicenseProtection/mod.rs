#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
extern "system" {
    #[doc = "*Required features: 'Win32_Security_LicenseProtection', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterLicenseKeyWithExpiration(licensekey: super::super::Foundation::PWSTR, validityindays: u32, status: *mut LicenseProtectionStatus) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: 'Win32_Security_LicenseProtection', 'Win32_Foundation'*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValidateLicenseKeyProtection(licensekey: super::super::Foundation::PWSTR, notvalidbefore: *mut super::super::Foundation::FILETIME, notvalidafter: *mut super::super::Foundation::FILETIME, status: *mut LicenseProtectionStatus) -> ::windows_sys::core::HRESULT;
}
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub type LicenseProtectionStatus = i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const Success: LicenseProtectionStatus = 0i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyNotFound: LicenseProtectionStatus = 1i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyUnprotected: LicenseProtectionStatus = 2i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyCorrupted: LicenseProtectionStatus = 3i32;
#[doc = "*Required features: 'Win32_Security_LicenseProtection'*"]
pub const LicenseKeyAlreadyExists: LicenseProtectionStatus = 4i32;
