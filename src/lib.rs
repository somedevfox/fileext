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

//! Pure Rust library for registering, modifying and deleting file type associations in WinNT and Linux operating systems.

#![warn(clippy::all)]
#![warn(missing_docs)]

pub mod app;
pub mod platform;
pub mod result;
