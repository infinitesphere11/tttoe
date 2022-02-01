// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{common::Identifier, PackageAccess};
use leo_span::Span;

use serde::{Deserialize, Serialize};
use std::fmt;

/// Import of `name.(accesses)`, that is, several sub-packages or items within `name`.
#[derive(Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Packages {
    /// The common package that `accesses` are contained within.
    pub name: Identifier,
    /// The packages or items to import within the package `name`.
    pub accesses: Vec<PackageAccess>,
    /// The entire span for `name.(accesses)`.
    pub span: Span,
}

impl Packages {
    /// Formats `self` to `f`.
    fn format(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.(", self.name)?;
        for (i, access) in self.accesses.iter().enumerate() {
            write!(f, "{}", access)?;
            if i < self.accesses.len() - 1 {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

impl fmt::Display for Packages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}

impl fmt::Debug for Packages {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format(f)
    }
}
