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
        let path = current_exe_path()?;
        if let Some(app) = Self::get(descriptor.id.clone(), path.clone())? {
            Ok(app)
        } else {
            Self::create(descriptor, path)
        }
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
    use crate::app::{Application, ApplicationDescriptor as Descriptor};

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
