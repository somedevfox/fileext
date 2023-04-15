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

use std::error;
use std::fmt::Display;
use std::io;

/// Error type for Input/Output operations.
#[derive(Debug)]
pub enum Error {
    /// Read permission is required to get data about a type association or get list of them under a specific application.
    ReadPermissionRequired,
    /// Write permission is require to create or delete an application or register a file type association.
    WritePermissionRequired,
    /// The executable which the application is trying to get doesn't exist in the filesystem.
    ExecutableDoesntExist,
    /// Operating System I/O Error
    Io(io::Error),
}

// Display traits
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::{ExecutableDoesntExist, Io, ReadPermissionRequired, WritePermissionRequired};

        if let Io(why) = self {
            why.fmt(f)
        } else {
            write!(
                f,
                "{}",
                match self {
                    ReadPermissionRequired => "read permission required",
                    WritePermissionRequired => "write permission required",
                    ExecutableDoesntExist => "executable doesn't exist",
                    _ => unreachable!(),
                }
            )
        }
    }
}
impl error::Error for Error {}

// Constructor traits
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

/// A type that represents either success ([Ok](std::result::Result::Ok)) or failure ([Error]).
pub type Result<T> = core::result::Result<T, Error>;
