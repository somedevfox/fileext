#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use core::{ffi::c_void as void, mem, ptr};
use std::io;

pub mod raw;

pub fn StringToLPCWSTR(string: impl ToString) -> *const u16 {
    let mut lpcwstr = string.to_string().encode_utf16().collect::<Vec<u16>>();
    lpcwstr.push('\0' as u16); // LPCWSTR strings must be null-terminated
    lpcwstr.as_ptr()
}
pub unsafe fn VecIntoString(vector: Vec<u16>) -> String {
    String::from_utf16_lossy(vector.as_slice())
}
pub unsafe fn LPCWSTRIntoString(lpString: *const u16) -> String {
    let len = raw::lstrlenW(lpString) as usize;
    let vector = Vec::from_raw_parts(lpString as *mut u16, len, len);
    VecIntoString(vector)
}

pub unsafe fn RegCreateKey(h_key: Option<isize>, path: impl ToString) -> io::Result<isize> {
    let h_key = h_key.unwrap_or(0);

    let mut out_h_key: isize = 0;

    let res = raw::RegCreateKeyExW(
        h_key,
        StringToLPCWSTR(path),
        0,
        ptr::null(),
        raw::REG_OPTION_NON_VOLATILE,
        raw::KEY_ALL_ACCESS,
        ptr::null(),
        &mut out_h_key,
        ptr::null_mut(),
    );
    if res == 0 {
        Ok(out_h_key)
    } else {
        Err(io::Error::from_raw_os_error(res))
    }
}

pub unsafe fn RegWriteKey(
    h_key: isize,
    value_name: impl ToString,
    h_key_type: u32,
    data: *const u8,
) -> io::Result<()> {
    let value_name = value_name.to_string();
    let value_name = if value_name.is_empty() {
        StringToLPCWSTR(value_name)
    } else {
        ptr::null()
    };
    if raw::RegSetValueExW(
        h_key,
        value_name,
        0,
        h_key_type,
        data,
        mem::size_of::<*const u8>() as u32,
    ) == 0
    {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
pub unsafe fn RegReadKeyValue(
    h_key: isize,
    value_name: impl ToString,
    lp_type: *mut u32,
) -> io::Result<*const u8> {
    let mut buffer = Vec::new();
    let value_name = value_name.to_string();

    let value_name = if value_name.is_empty() {
        ptr::null()
    } else {
        StringToLPCWSTR(value_name)
    };
    let mut size = 0;
    let res = raw::RegQueryValueExW(
        h_key,
        value_name,
        ptr::null(),
        lp_type,
        buffer.as_mut_ptr(),
        &mut size,
    );
    println!("{res}");
    if res == 0 {
        Ok(buffer.as_ptr())
    } else if res == 234 {
        let mut buffer = Vec::with_capacity(size as usize);
        let res = raw::RegQueryValueExW(
            h_key,
            value_name,
            ptr::null(),
            lp_type,
            buffer.as_mut_ptr(),
            &mut size,
        );
        if res == 0 {
            Ok(buffer.as_ptr())
        } else {
            Err(io::Error::from_raw_os_error(res))
        }
    } else {
        Err(io::Error::from_raw_os_error(res))
    }
}

pub unsafe fn RegDeleteKey(h_key: isize, subkey: impl ToString) -> io::Result<()> {
    let subkey = subkey.to_string();

    let res = raw::RegDeleteKeyExW(h_key, StringToLPCWSTR(subkey), raw::KEY_WOW64_32KEY, 0);
    if res == 0 {
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(res))
    }
}

#[derive(Debug, Clone)]
pub struct ProgID {
    pub id: String,
    pub name: String,
    pub default_icon_path: Option<String>,
}

pub unsafe fn CreateProgID(
    id: impl ToString,
    name: impl ToString,
    default_icon_path: impl ToString,
) -> io::Result<ProgID> {
    let id = id.to_string();
    let name = name.to_string();
    let default_icon_path = default_icon_path.to_string();

    let id_key = RegCreateKey(Some(raw::HKEY_CLASSES_ROOT), id.clone())?;
    RegWriteKey(
        id_key,
        "",
        raw::REG_SZ,
        StringToLPCWSTR(name.clone()) as *const u8,
    )?;

    RegWriteKey(
        RegCreateKey(Some(id_key), "CurVer")?,
        "",
        raw::REG_SZ,
        StringToLPCWSTR(id.clone()) as *const u8,
    )?;
    let default_icon_path = if !default_icon_path.is_empty() {
        RegWriteKey(
            RegCreateKey(Some(id_key), "DefaultIcon")?,
            "",
            raw::REG_SZ,
            StringToLPCWSTR(default_icon_path.clone()) as *const u8,
        )?;
        Some(default_icon_path)
    } else {
        None
    };
    raw::RegCloseKey(id_key);

    Ok(ProgID {
        id,
        name,
        default_icon_path,
    })
}

pub unsafe fn GetProcID(id: impl ToString) -> Option<ProgID> {
    let id = id.to_string();

    let mut h_key = 0;

    if raw::RegOpenKeyExW(
        raw::HKEY_CLASSES_ROOT,
        StringToLPCWSTR(id.clone()),
        0,
        raw::KEY_READ,
        &mut h_key,
    ) == 0
    {
        let name = RegReadKeyValue(h_key, "", ptr::null_mut()).ok()? as *const u16;
        let name = LPCWSTRIntoString(name);

        let mut default_icon_key = 0;
        let default_icon_path = if raw::RegOpenKeyExW(
            h_key,
            StringToLPCWSTR("DefaultIcon"),
            0,
            raw::KEY_READ,
            &mut default_icon_key,
        ) == 0
        {
            let default_icon_path =
                RegReadKeyValue(default_icon_key, "", ptr::null_mut()).ok()? as *const u16;
            Some(LPCWSTRIntoString(default_icon_path))
        } else {
            None
        };

        raw::RegCloseKey(default_icon_key);
        raw::RegCloseKey(h_key);

        Some(ProgID {
            id,
            name,
            default_icon_path,
        })
    } else {
        None
    }
}

pub unsafe fn DeleteProcID(id: impl ToString) -> io::Result<()> {
    let id = id.to_string();

    let mut h_key = 0;
    let res = raw::RegOpenKeyExW(
        raw::HKEY_CLASSES_ROOT,
        StringToLPCWSTR(id.clone()),
        0,
        raw::KEY_ALL_ACCESS,
        &mut h_key,
    );
    if res == 0 {
        RegDeleteKey(h_key, "CurVer")?;
        let _ = RegDeleteKey(h_key, "DefaultIcon");
        RegDeleteKey(raw::HKEY_CLASSES_ROOT, id)
    } else {
        Err(io::Error::from_raw_os_error(res as i32))
    }
}
