// Copyright 2015, The Rust-GNOME Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

/// A Range widget

use ffi;

struct_Widget!(Range);

impl_drop!(Range);
impl_TraitWidget!(Range);

impl ::OrientableTrait for Range {}
impl ::RangeTrait for Range {}
