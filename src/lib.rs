//! Pure Rust library for registering, modifying and deleting file type associations in WinNT and Linux operating systems.

#![warn(clippy::all)]
#![warn(missing_docs)]

use platform::*;
use result::{Error, Result};
use std::{env, io, path::PathBuf};

pub mod platform;
pub mod result;

pub struct ApplicationDescriptor {
    pub id: String,
    pub name: String,
    pub icon_path: String,
}

/// Representation of the application to manipulate file type associations in.
#[derive(Debug)]
pub struct Application {
    id: String,
    path: String,
}

impl Application {
    pub fn current(descriptor: ApplicationDescriptor) -> Result<Self> {
        Self::create(descriptor, current_exe_path()?)
    }

    pub fn create(descriptor: ApplicationDescriptor, path: impl ToString) -> Result<Self> {
        #[cfg(windows)]
        unsafe {
            windows::CreateProgID(descriptor.id.clone(), descriptor.name, descriptor.icon_path)?
        };

        Ok(Self::get(descriptor.id, path)?.unwrap())
    }

    pub fn get(id: impl ToString, path: impl ToString) -> Result<Option<Self>> {
        Ok(
            #[cfg(windows)]
            unsafe {
                windows::GetProcID(id.to_string()).map(|id| Self {
                    id: id.id,
                    path: path.to_string(),
                })
            },
        )
    }

    pub fn delete(self) -> Result<()> {
        #[cfg(windows)]
        unsafe {
            windows::DeleteProcID(self.id).map_err(|why| Error::Io(why))
        }
    }
}

pub(crate) fn pathbuf_into_string(pathbuf: PathBuf) -> String {
    pathbuf
        .into_os_string()
        .into_string()
        .unwrap_or(String::new())
}
pub(crate) fn current_exe_path() -> Result<String> {
    env::current_exe()
        .map(|pathbuf| {
            pathbuf
                .into_os_string()
                .into_string()
                .unwrap_or(String::new())
        })
        .map_err(|why| Error::Io(why))
}

#[cfg(test)]
mod tests {
    use crate::{Application, ApplicationDescriptor as Descriptor};

    #[test]
    fn create() {
        let app = Application::current(Descriptor {
            id: String::from("Fileext.Test"),
            name: String::from("fileext crate"),
            icon_path: String::new(),
        })
        .unwrap();

        app.delete().unwrap();
    }
}
