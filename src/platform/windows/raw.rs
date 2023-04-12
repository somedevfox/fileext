use core::ffi::{c_long as long, c_ulong as ulong, c_void as void};

macro_rules! const_hkey {
    ($name:ident, $address:expr) => {
        #[allow(missing_docs)]
        pub const $name: *const void = (&($address as ulong) as *const ulong) as *const void;
    };
}

/// Create a [language identifier](https://learn.microsoft.com/en-us/windows/win32/intl/language-identifiers) from a primary language identifier and a sublanguage identifier
#[macro_export]
macro_rules! make_lang_id {
    ($p:expr, $s:expr) => {
        ((($p as i16) << 10) | ($s as i16))
    };
}

/// Language neutral
pub const LANG_NEUTRAL: i16 = 0;
/// Language as defined in user's computer settings
pub const LANG_USER_DEFAULT: i16 = make_lang_id!(LANG_NEUTRAL, SUBLANG_DEFAULT);

/// Default sublanguage code
pub const SUBLANG_DEFAULT: i16 = 1;

// * Root keys
pub const HKEY_CLASSES_ROOT: isize = -2147483648;
pub const HKEY_LOCAL_MACHINE: isize = -2147483646;
pub const HKEY_CURRENT_USER: isize = -2147483647;
pub const HKEY_USERS: isize = -2147483645;
pub const HKEY_CURRENT_CONFIG: isize = -2147483643;
// * Registry options
/// If this flag is set, the function ignores the `samDesired` parameter and attempts to open the key with the access required to backup (or restore) the key.
///
/// More information about this key: [MSDN](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexa#REG_OPTION_BACKUP_RESTORE)
pub const REG_OPTION_BACKUP_RESTORE: u32 = 0x00000004;
/// This key is a [**symbolic link**](https://learn.microsoft.com/en-us/windows/win32/fileio/symbolic-links).
///
/// More information about this key: [MSDN](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexa#REG_OPTION_CREATE_LINK)
pub const REG_OPTION_CREATE_LINK: u32 = 0x00000002;
/// The information is stored in a file and is preserved when the system is restarted.
///
/// More information about this key: [MSDN](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexa#REG_OPTION_NON_VOLATILE)
pub const REG_OPTION_NON_VOLATILE: u32 = 0x00000000;
/// The information is stored in memory and is not preserved when the corresponding registry hive in unloaded.
///
/// More information about this key: [MSDN](https://learn.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regcreatekeyexa#REG_OPTION_VOLATILE)
pub const REG_OPTION_VOLATILE: u32 = 0x00000001;
// * SAM
/// Combination of STANDARD_RIGHTS_REQUIRED, [KEY_QUERY_VALUE], [KEY_SET_VALUE], [KEY_CREATE_SUB_KEY], [KEY_ENUMERATE_SUB_KEYS], [KEY_NOTIFY] and [KEY_CREATE_LINK] access rights.
pub const KEY_ALL_ACCESS: u32 = 0xF003F;
/// Reserved for system use.
pub const KEY_CREATE_LINK: u32 = 0x0020;
/// Required to create a subkey of a registry key.
pub const KEY_CREATE_SUB_KEY: u32 = 0x0004;
/// Required to enumerate the subkeys of a registry key.
pub const KEY_ENUMERATE_SUB_KEYS: u32 = 0x0008;
/// Equivalent to KEY_READ.
pub const KEY_EXECUTE: u32 = 0x20019;
/// Required to request change notifications for a registry key or for subkeys of a registry key.
pub const KEY_NOTIFY: u32 = 0x0010;
/// Required to query the values of a registry key.
pub const KEY_QUERY_VALUE: u32 = 0x0001;
/// Combination of STANDARD_RIGHTS_READ, [KEY_QUERY_VALUE], [KEY_ENUMERATE_SUB_KEYS] and [KEY_NOTIFY] access rights.
pub const KEY_READ: u32 = 0x20019;
/// Required to create, delete, or set a registry value.
pub const KEY_SET_VALUE: u32 = 0x0002;
/// Indicates that an application on 64-bit Windows should operate on the 32-bit registry view.
///
/// More information about this key: [MSDN](https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-key-security-and-access-rights#KEY_WOW64_32KEY)
pub const KEY_WOW64_32KEY: u32 = 0x0200;
/// Indicates that an application on 64-bit Windows should operate on the 64-bit registry view.
///
/// More information about this key: [MSDN](https://learn.microsoft.com/en-us/windows/win32/sysinfo/registry-key-security-and-access-rights#KEY_WOW64_64KEY)
pub const KEY_WOW64_64KEY: u32 = 0x0100;
/// Combination of STANDARD_RIGHTS_WRITE, KEY_SET_VALUE and KEY_CREATE_SUB_KEY access rights.
pub const KEY_WRITE: u32 = 0x20006;
// * Registry data types
/// No value
pub const REG_NONE: u32 = 0;
/// A Null-terminated unicode string
pub const REG_SZ: u32 = 1;
/// A Null-terminated unicode string which might contain unexpanded references to environment variables
pub const REG_EXPAND_SZ: u32 = 2;
/// A 32-bit number
pub const REG_DWORD: u32 = REG_DWORD_LITTLE_ENDIAN;
/// A 32-bit number in little-endian format
pub const REG_DWORD_LITTLE_ENDIAN: u32 = 4;
/// A 32-bit number in big-endian format
pub const REG_DWORD_BIG_ENDIAN: u32 = 5;
/// A null-terminated unicode string that contains the target path of a symbolic link that was created by calling [RegCreateKeyExW] function with [REG_OPTION_CREATE_LINK]
pub const REG_LINK: u32 = 6;
/// An rray of null-terminated unicode string
pub const REG_MULTI_SZ: u32 = 7;
/// Resource list in the resource map
pub const REG_RESOURCE_LIST: u32 = 8;
/// Resource list in the hardware description
pub const REG_FULL_RESOURCE_DESCRIPTOR: u32 = 9;
#[allow(missing_docs)]
pub const REG_RESOURCE_REQUIREMENTS_LIST: u32 = 10;
/// A 64-bit number
pub const REG_QWORD: u32 = REG_QWORD_LITTLE_ENDIAN;
/// A 64-bit number in little-endian format
pub const REG_QWORD_LITTLE_ENDIAN: u32 = 11;
// * FormatMessage flags
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 0x00000100;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: u32 = 0x00002000;
pub const FORMAT_MESSAGE_FROM_HMODULE: u32 = 0x00000800;
pub const FORMAT_MESSAGE_FROM_STRING: u32 = 0x00000400;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 0x00001000;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 0x00000200;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *const void,
    pub bInheritHandle: bool,
}

#[link(name = "Advapi32")]
extern "system" {
    /// Creates the specified registry key. If the key already exists, it will be opened.
    ///
    /// # Notes:
    /// - Key names are *not* case sensitive
    ///
    /// # Errors:
    /// - ERROR_ACCESS_DENIED  - You're not allowed to create a key in specified path, ask user for the [**Administrative Privileges**](https://learn.microsoft.com/en-us/windows/win32/secbp/running-with-administrator-privileges)
    /// - ERROR_INVALID_HANDLE - Specified parent key handle (`hKey` argument) is not valid
    pub fn RegCreateKeyExW(
        hKey: isize,
        lpSubKey: *const u16,
        Reserved: u32,
        lpClass: *const u16,
        dwOptions: u32,
        samDesired: u32,
        lpSecurityAttributes: *const SECURITY_ATTRIBUTES,
        phkResult: *mut isize,
        lpdwDisposition: *mut u32,
    ) -> long;
    pub fn RegOpenKeyExW(
        h_key: isize,
        lpSubKey: *const u16,
        ulOptions: u32,
        samDesired: u32,
        phkResult: *mut isize,
    ) -> long;

    pub fn RegSetValueExW(
        hKey: isize,
        lpValueName: *const u16,
        Reserved: u32,
        dwType: u32,
        lpData: *const u8,
        cbData: *const u32,
    ) -> long;
    pub fn RegQueryValueExW(
        h_key: isize,
        lpValueName: *const u16,
        lpReserved: *const u32,
        lpType: *mut u32,
        lpData: *mut u8,
        lpcbData: *mut u32,
    ) -> long;

    /// Closes a handle to the specified registry key
    ///
    /// ## Errors:
    /// - [ERROR_INVALID_HANDLE](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-#ERROR_INVALID_HANDLE) if the specified key handle is invalid or the key no longer exists.
    pub fn RegCloseKey(h_key: isize) -> long;
}

#[link(name = "Kernel32")]
extern "system" {
    /// Determines length a UTF-16 null-terminated string (LPCWSTR), excluding the null terminator.
    ///
    /// ## Example:
    /// ```
    /// # use fileext::platform::windows::raw::lstrlenW;
    /// # fn main() {
    /// let string = String::new("Hello, world!");
    /// let utf16 = string
    /// 	.encode_utf16()
    /// 	.collect::<Vec<u16>>()
    /// 	.as_ptr(); // Turn UTF-8 string into an LPCWSTR
    ///
    /// let len = unsafe { lstrlenW(utf16) };
    ///
    /// assert_eq!(string.len(), len);
    /// # }
    /// ```
    pub fn lstrlenW(lpString: *const u16) -> i32;
    /// Returns last Input/Output error
    ///
    /// Read ["System Error Codes" article on MSDN](https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-) for the list of errors Win32 API methods can throw.
    pub fn GetLastError() -> u32;
    pub fn FormatMessageW(
        dwFlags: u32,
        lpSource: *const void,
        dwMessageId: u32,
        dwLanguageId: u32,
        lpBuffer: *mut u16,
        nSize: u32,
        arguments: *const *const i8,
    ) -> u32;
}
