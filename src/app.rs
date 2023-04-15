// Copyright (C) 2023 Egor Poleshko
//
// fileext is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fileext is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with fileext. If not, see <http://www.gnu.org/licenses/>.
use crate::platform::*;
use crate::result::{Error, Result};
use std::{env, path::PathBuf};

pub(crate) const APP_READ: u32 = 0b00000001;
pub(crate) const APP_WRITE: u32 = 0b00000010;
pub(crate) const APP_STRICT: u32 = 0b00000100;
pub(crate) fn bitflag_eq(lhs: u32, rhs: u32) -> bool {
    (lhs & rhs) == rhs
}

#[derive(Debug, Clone)]
pub struct OpenOptions {
    flags: u32,
    path: String,
}
impl OpenOptions {
    pub fn new(path: impl ToString) -> Self {
        Self {
            flags: 0,
            path: path.to_string(),
        }
    }
    pub fn current() -> Self {
        Self {
            flags: 0,
            path: current_exe_path().unwrap(),
        }
    }

    fn flag_set(mut self, flag: u32, yes: bool) -> Self {
        if yes {
            self.flags |= flag;
        } else {
            self.flags &= !flag;
        }
        self
    }

    pub fn read(mut self, yes: bool) -> Self {
        self.flag_set(APP_READ, yes)
    }
    pub fn write(mut self, yes: bool) -> Self {
        self.flag_set(APP_WRITE, yes)
    }
    pub fn strict(mut self, yes: bool) -> Self {
        self.flag_set(APP_STRICT, yes)
    }

    pub fn create(self, descriptor: ApplicationDescriptor) -> Result<Application> {
        if bitflag_eq(self.flags, APP_WRITE) {
            #[cfg(windows)]
            unsafe {
                windows::CreateProgID(descriptor.id.clone(), descriptor.name, descriptor.icon_path)?
            };

            Ok(self.get(descriptor.id)?.unwrap())
        } else {
            Err(Error::WritePermissionRequired)
        }
    }
    pub fn get(self, id: impl ToString) -> Result<Option<Application>> {
        Ok(
            #[cfg(windows)]
            unsafe {
                windows::GetProcID(id.to_string()).map(|id| Application {
                    id: id.id,
                    path: self.path,
                    flags: self.flags,
                })
            },
        )
    }
}

pub struct ApplicationDescriptor {
    pub id: String,
    pub name: String,
    pub icon_path: String,
}

/// Representation of the application to manipulate file type associations in.
#[derive(Debug)]
pub struct Application {
    pub(crate) id: String,
    pub(crate) path: String,
    pub(crate) flags: u32,
}

impl Application {
    pub fn enumerate_associations(&self) -> Result<impl Iterator<Item = String>> {
        if bitflag_eq(self.flags, APP_READ) {
            Ok(
                #[cfg(windows)]
                unsafe {
                    windows::EnumerateFileTypeAssociations(self.id.clone())
                        .map_err(|why| Error::Io(why))?
                        .into_iter()
                },
            )
        } else {
            Err(Error::ReadPermissionRequired)
        }
    }
    pub fn set_file_type_association(&self) -> Result<()> {
        if bitflag_eq(self.flags, APP_WRITE) {
            Ok(())
        } else {
            Err(Error::WritePermissionRequired)
        }
    }

    pub fn delete(self) -> Result<()> {
        if bitflag_eq(self.flags, APP_WRITE) {
            #[cfg(windows)]
            unsafe {
                windows::DeleteProcID(self.id).map_err(|why| Error::Io(why))
            }
        } else {
            Err(Error::WritePermissionRequired)
        }
    }
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
    use crate::app::{Application, ApplicationDescriptor as Descriptor};

    mod options {
        use crate::app;

        #[test]
        fn write() {
            let options = app::OpenOptions::current().write(true).strict(true);

            assert!(app::bitflag_eq(options.flags, app::APP_WRITE));
            assert!(app::bitflag_eq(options.flags, app::APP_STRICT));
            assert!(!app::bitflag_eq(options.flags, app::APP_READ));
        }

        #[test]
        fn read() {
            let options = app::OpenOptions::current().read(true).strict(true);

            assert!(!app::bitflag_eq(options.flags, app::APP_WRITE));
            assert!(app::bitflag_eq(options.flags, app::APP_STRICT));
            assert!(app::bitflag_eq(options.flags, app::APP_READ));
        }
    }

    mod app {
        use crate::app;

        pub const ID: &str = "Fileext.Test";

        #[test]
        fn create() {
            app::OpenOptions::current()
                .write(true)
                .create(app::ApplicationDescriptor {
                    id: String::from(ID),
                    name: String::from("fileext crate"),
                    icon_path: String::new(),
                })
                .unwrap();
        }

        #[test]
        fn delete() {
            let application = match app::OpenOptions::current().write(true).get(ID).unwrap() {
                Some(app) => app,
                None => {
                    create();
                    app::OpenOptions::current()
                        .write(true)
                        .get(ID)
                        .unwrap()
                        .unwrap()
                }
            };

            application.delete().unwrap()
        }
    }
}
