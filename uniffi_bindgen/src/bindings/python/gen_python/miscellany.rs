/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use crate::backend::{CodeOracle, CodeType, Literal};
use paste::paste;

macro_rules! impl_code_type_for_miscellany {
    ($T:ty, $canonical_name:literal) => {
        paste! {
            pub struct $T;

            impl CodeType for $T  {
                fn type_label(&self, _oracle: &dyn CodeOracle) -> String {
                    format!("{}", $canonical_name)
                }

                fn canonical_name(&self, _oracle: &dyn CodeOracle) -> String {
                    format!("{}", $canonical_name)
                }

                fn literal(&self, _oracle: &dyn CodeOracle, _literal: &Literal) -> String {
                    unreachable!()
                }

                fn coerce(&self, _oracle: &dyn CodeOracle, nm: &str) -> String {
                    nm.to_string()
                }
            }
        }
    };
}

impl_code_type_for_miscellany!(TimestampCodeType, "Timestamp");

impl_code_type_for_miscellany!(DurationCodeType, "Duration");
