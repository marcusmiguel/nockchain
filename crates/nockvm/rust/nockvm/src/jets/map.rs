/** Map engine jets
 */
use crate::interpreter::{Context};
use crate::jets::util::{slot};
use crate::jets::Result;
use crate::noun::{Noun};

crate::gdb!();

pub fn jet_get_by(context: &mut Context, subject: Noun) -> Result {
    let a = slot(subject, 30)?;
    let b = slot(subject, 6)?;

    util::get_by(context, a, b)
}

pub mod util {
    // use crate::jets::util::BAIL_EXIT;
    use crate::jets::{Result};
    use crate::interpreter::Context;
    // use crate::mem::NockStack;
    use crate::noun::{Noun, D, T, YES, NO};
    use crate::jets::sort::util::{mor, gor};
    use crate::unifying_equality::unifying_equality;
    // use std::result;

    pub fn get_by(context: &mut Context, mut a: Noun, mut b: Noun) -> Result {
      loop {
            if unsafe { a.raw_equals(&D(0)) } {
                return Ok(D(0));
            }

            let [n_a, l_a, r_a] = a.uncell()?;
            let [mut key, val] = n_a.uncell()?;

            if unsafe { unifying_equality(&mut context.stack, &mut key, &mut b) } {
                return Ok(T(&mut context.stack, &[D(0), val]));
            }

            if unsafe { gor(&mut context.stack, b, key).raw_equals(&YES) } {
                a = l_a;
            } else {
                a = r_a;
            }
        }
    }

}
