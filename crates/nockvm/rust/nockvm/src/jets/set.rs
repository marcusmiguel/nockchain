/** Set engine jets
 */
use crate::interpreter::{Context};
use crate::jets::util::{slot};
use crate::jets::Result;
use crate::noun::{Noun};

crate::gdb!();

pub fn jet_has_in(context: &mut Context, subject: Noun) -> Result {
    let a = slot(subject, 30)?;
    let b = slot(subject, 6)?;

    util::has_in(context, a, b)
}

pub fn jet_put_in(context: &mut Context, subject: Noun) -> Result {
    let a = slot(subject, 30)?;
    let b = slot(subject, 6)?;

    util::put_in(context, a, b)
}

pub fn jet_gas_in(context: &mut Context, subject: Noun) -> Result {
   let a = slot(subject, 30)?;
   let b = slot(subject, 6)?;

   util::gas_in(context, a, b)
}

pub fn jet_uni_in(context: &mut Context, subject: Noun) -> Result {
   let a = slot(subject, 30)?;
   let b = slot(subject, 6)?;

   util::uni_in(context, a, b)
}

use crate::noun::D;
pub fn jet_tap_in(context: &mut Context, subject: Noun) -> Result {
    let [_tap_arm, b, _in_arms, a, _in_subject] = subject.uncell()?;

    util::tap_in(context, a, b)
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

    pub fn gas_in(context: &mut Context, mut a: Noun, mut b: Noun) -> Result {

        loop {
            if unsafe { b.raw_equals(&D(0)) } {
                return Ok(a);
            }
            let [b_head, b_tail] = b.uncell()?;
            b = b_tail;
            a = put_in(context, a, b_head)?;
       }
    }

    pub fn has_in(context: &mut Context, mut a: Noun, mut b: Noun) -> Result {

        loop {
            if unsafe { a.raw_equals(&D(0)) } {
                return Ok(NO);
            }

            let [mut n_a, l_a, r_a] = a.uncell()?;

            if unsafe { unifying_equality(&mut context.stack, &mut b, &mut n_a) } {
                return Ok(YES);
            }

            if unsafe { gor(&mut context.stack, b, n_a).raw_equals(&YES) } {
                a = l_a;
            } else {
                a = r_a;
            }
        }
    }

    pub fn put_in(context: &mut Context, a: Noun, mut b: Noun) -> Result {

        if unsafe { a.raw_equals(&D(0)) } {
            return Ok(T(&mut context.stack, &[b, D(0), D(0)]));
        }

        let [mut n_a, l_a, r_a] = a.uncell()?;

        if unsafe { unifying_equality(&mut context.stack, &mut b, &mut n_a) } {
            return Ok(a);
        } else if unsafe { gor(&mut context.stack, b, n_a).raw_equals(&YES) } {
            let c = put_in(context, l_a, b)?;
            let [n_c, l_c, r_c] = c.uncell()?;

            if unsafe { mor(&mut context.stack, n_a, n_c).raw_equals(&YES) } {
                return Ok(T(&mut context.stack, &[n_a, c, r_a]));
            } else  {
                let r_c = T(&mut context.stack, &[n_a, r_c, r_a]);
                return Ok(T(&mut context.stack, &[n_c, l_c, r_c]));
            }
        } else {
            let c = put_in(context, r_a, b)?;
            let [n_c, l_c, r_c] = c.uncell()?;

            if unsafe { mor(&mut context.stack, n_a, n_c).raw_equals(&YES) } {
                return Ok(T(&mut context.stack, &[n_a, l_a, c]));
            } else {
                let l_c = T(&mut context.stack, &[n_a, l_a, l_c]);
                return Ok(T(&mut context.stack, &[n_c, l_c, r_c]));
            }
        }
    }

    pub fn tap_in(context: &mut Context, a: Noun, b: Noun) -> Result {
        if unsafe { a.raw_equals(&D(0)) } {
            return Ok(b);
        }

        let [n_a, l_a, r_a] = a.uncell()?;
        let tap = tap_in(context, l_a, b)?;
        let new_b = T(&mut context.stack, &[n_a, tap]);

        tap_in(context, r_a, new_b)
    }

    pub fn uni_in(context: &mut Context, mut a: Noun, mut b: Noun) -> Result {

        if unsafe { unifying_equality(&mut context.stack, &mut a, &mut b) } {
            return Ok(a);
        }

       uni_in_recursion(context, a, b)
    }

    fn uni_in_recursion(context: &mut Context, a: Noun, b: Noun) -> Result {

        if unsafe { b.raw_equals(&D(0)) } {
            return Ok(a);
        }
        else if unsafe { a.raw_equals(&D(0)) } {
            return Ok(b);
        }

        let [mut n_a, l_a, r_a] = a.uncell()?;
        let [mut n_b, l_b, r_b] = b.uncell()?;

        if unsafe { unifying_equality(&mut context.stack, &mut n_a, &mut n_b) } {
            let l_b = uni_in_recursion(context, l_a, l_b)?;
            let r_b = uni_in_recursion(context, r_a, r_b)?;
            return Ok(T(&mut context.stack, &[n_b, l_b, r_b]));
        } else if unsafe { mor(&mut context.stack, n_a, n_b).raw_equals(&YES) } {
            if unsafe { gor(&mut context.stack, n_b, n_a).raw_equals(&YES) } {
                let inner_inner_b = T(&mut context.stack, &[n_b, l_b, D(0)]);
                let l_a = uni_in_recursion(context, l_a, inner_inner_b)?;
                let inner_a = T(&mut context.stack, &[n_a, l_a, r_a]);
                return Ok(uni_in_recursion(context, inner_a, r_b)?);
            } else {
                let inner_inner_b = T(&mut context.stack, &[n_b, D(0), r_b]);
                let r_a = uni_in_recursion(context, r_a, inner_inner_b)?;
                let inner_a = T(&mut context.stack, &[n_a, l_a, r_a]);
                return Ok(uni_in_recursion(context, inner_a, l_b)?);
            }
        } else {
            if unsafe { gor(&mut context.stack, n_a, n_b).raw_equals(&YES) } {
                let inner_inner_a = T(&mut context.stack, &[n_a, l_a, D(0)]);
                let l_b = uni_in_recursion(context, inner_inner_a, l_b)?;
                let inner_b = T(&mut context.stack, &[n_b, l_b, r_b]);
                return Ok(uni_in_recursion(context, r_a, inner_b)?);
            } else {
                let inner_inner_a = T(&mut context.stack, &[n_a, D(0), r_a]);
                let r_b = uni_in_recursion(context, inner_inner_a, r_b)?;
                let inner_b = T(&mut context.stack, &[n_b, l_b, r_b]);
                return Ok(uni_in_recursion(context, l_a, inner_b)?);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use crate::jets::util::test::{assert_jet, assert_jet_err, init_context};
    // use crate::jets::util::BAIL_EXIT;
    // use crate::noun::{D, T};

}
