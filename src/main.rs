#![allow(clippy::no_effect)]

use indoc::indoc;

fn main() {
    // delete the `.` and type it again
    indoc! {
      "
        a.
      "
    };
}
