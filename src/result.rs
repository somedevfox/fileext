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

use std::io;

#[derive(Debug)]
pub enum Error {
    ExecutableDoesntExist,
    Io(io::Error),
}
impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
