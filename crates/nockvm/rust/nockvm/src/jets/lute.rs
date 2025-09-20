/** ++ut jets (compiler backend and pretty-printer)
 */
use crate::interpreter::{interpret, Context};
use crate::jets::util::*;
use crate::jets::{Result};
use crate::noun::{Noun, D, NO, NONE, T, YES};
use nockvm_macros::tas;
use crate::mug::mug;
crate::gdb!();

pub fn jet_ut_crop(context: &mut Context, subject: Noun) -> Result {
    let rff = slot(subject, 6)?;
    let van = slot(subject, 7)?;

    let bat = slot(van, 2)?;
    let sut = slot(van, 6)?;

    let flag = if let Ok(noun) = slot(van, 59) {
        if unsafe { noun.raw_equals(&D(0)) } {
            0u64
        } else {
            1u64
        }
    } else {
        1
    };
    let fun = 141 + tas!(b"crop") + (flag << 8);
    let mut key = T(&mut context.stack, &[D(fun), sut, rff, bat]);

    match context.cache.lookup(&mut context.stack, &mut key) {
        Some(pro) => Ok(pro),
        None => {
            let pro = interpret(context, subject, slot(subject, 2)?)?;
            context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
            Ok(pro)
        }
    }
}

pub fn jet_ut_fish(context: &mut Context, subject: Noun) -> Result {
    //  axe must be Atom, though we use it as Noun
    let axe = slot(subject, 6)?.as_atom()?;
    let van = slot(subject, 7)?;

    let bat = slot(van, 2)?;
    let sut = slot(van, 6)?;

    let flag = if let Ok(noun) = slot(van, 59) {
        if unsafe { noun.raw_equals(&D(0)) } {
            0u64
        } else {
            1u64
        }
    } else {
        1
    };
    let fun = 141 + tas!(b"fish") + (flag << 8);
    let mut key = T(&mut context.stack, &[D(fun), sut, axe.as_noun(), bat]);

    match context.cache.lookup(&mut context.stack, &mut key) {
        Some(pro) => Ok(pro),
        None => {
            let pro = interpret(context, subject, slot(subject, 2)?)?;
            context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
            Ok(pro)
        }
    }
}

pub fn jet_ut_fuse(context: &mut Context, subject: Noun) -> Result {
    let rff = slot(subject, 6)?;
    let van = slot(subject, 7)?;

    let bat = slot(van, 2)?;
    let sut = slot(van, 6)?;

    let flag = if let Ok(noun) = slot(van, 59) {
        if unsafe { noun.raw_equals(&D(0)) } {
            0u64
        } else {
            1u64
        }
    } else {
        1
    };
    let fun = 141 + tas!(b"fuse") + (flag << 8);
    let mut key = T(&mut context.stack, &[D(fun), sut, rff, bat]);

    match context.cache.lookup(&mut context.stack, &mut key) {
        Some(pro) => Ok(pro),
        None => {
            let pro = interpret(context, subject, slot(subject, 2)?)?;
            context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
            Ok(pro)
        }
    }
}

pub fn jet_ut_mint(context: &mut Context, subject: Noun) -> Result {
    let gol = slot(subject, 12)?;
    let gen = slot(subject, 13)?;
    let van = slot(subject, 7)?;

    let bat = slot(van, 2)?;
    let sut = slot(van, 6)?;

    let fun = 141 + tas!(b"mint");
    let vet = slot(van, 59).map_or(NONE, |x| x);
    let mut key = T(&mut context.stack, &[D(fun), vet, sut, gol, gen, bat]);

    match context.cache.lookup(&mut context.stack, &mut key) {
        Some(pro) => Ok(pro),
        None => {
            let pro = interpret(context, subject, slot(subject, 2)?)?;
            context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
            Ok(pro)
        }
    }
}

pub fn jet_ut_mull(context: &mut Context, subject: Noun) -> Result {
    let gol = slot(subject, 12)?;
    let dox = slot(subject, 26)?;
    let gen = slot(subject, 27)?;
    let van = slot(subject, 7)?;

    let bat = slot(van, 2)?;
    let sut = slot(van, 6)?;

    let flag = if let Ok(noun) = slot(van, 59) {
        if unsafe { noun.raw_equals(&D(0)) } {
            0u64
        } else {
            1u64
        }
    } else {
        1
    };
    let fun = 141 + tas!(b"mull") + (flag << 8);
    let mut key = T(&mut context.stack, &[D(fun), sut, gol, dox, gen, bat]);

    match context.cache.lookup(&mut context.stack, &mut key) {
        Some(pro) => Ok(pro),
        None => {
            let pro = interpret(context, subject, slot(subject, 2)?)?;
            context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
            Ok(pro)
        }
    }
}

pub fn jet_ut_nest(context: &mut Context, subject: Noun) -> Result {
    let [bat, sam, van] = subject.uncell()?;
    let [_ut_arms, sut, _ut_subject] = van.uncell()?;
    let [tel, rff] = sam.uncell()?;

    util::nest(context, van, sut, tel, rff)
}

pub fn jet_ut_rest(context: &mut Context, subject: Noun) -> Result {
    let leg = slot(subject, 6)?;
    let van = slot(subject, 7)?;

    util::rest_cached(context, van, leg)
}

pub fn jet_fork(context: &mut Context, subject: Noun) -> Result {
    let yed = slot(subject, 6)?;

    util::fork(context, yed)
}

pub fn jet_comb(context: &mut Context, subject: Noun) -> Result {

    let sam = slot(subject, 6)?;
    let [mal, buz] = sam.uncell()?;

    util::comb(context, mal, buz)
}

pub fn jet_ut_peek(context: &mut Context, subject: Noun) -> Result {
    let [_peek_arm, sam, ut_core] = subject.uncell()?;
    let [way, axe] = sam.uncell()?;

    util::peek(context, ut_core, slot(ut_core, 6)?, way, axe)
}

pub fn jet_ut_fond(context: &mut Context, subject: Noun) -> Result {
    let [_fond_arm, sam, ut_core] = subject.uncell()?;
    let [way, hyp] = sam.uncell()?;

    util::fond(context, ut_core, slot(ut_core, 6)?, way, hyp)
}

pub fn jet_fitz(context: &mut Context, subject: Noun) -> Result {
    let [_fitz_arm, sam, _ut_core] = subject.uncell()?;
    let [yaz, wix] = sam.uncell()?;

    util::fitz(context, yaz, wix)
}

pub fn jet_ut_redo(context: &mut Context, subject: Noun) -> Result {
    let [_arm, rff, ut_core] = subject.uncell()?;

    util::redo(context, ut_core, slot(ut_core, 6)?, rff)
}

pub fn jet_ut_fire(context: &mut Context, subject: Noun) -> Result {
    let [_arm, hag, ut_core] = subject.uncell()?;

    util::fire(context, ut_core, slot(ut_core, 6)?, hag)
}

pub fn jet_ut_play(context: &mut Context, subject: Noun) -> Result {
    let [_arm, gen, ut_core] = subject.uncell()?;

    if unsafe { gen.is_cell() &&  gen.as_cell()?.head().raw_equals(&D(tas!(b"cnts"))) } {
        return Ok(interpret(context, subject, slot(subject, 2)?)?);
    }

    util::play(context, ut_core, slot(ut_core, 6)?, gen)
}

pub fn jet_ut_wrap(context: &mut Context, subject: Noun) -> Result {
    let [_arm, yoz, ut_core] = subject.uncell()?;

    util::wrap(context, ut_core, slot(ut_core, 6)?, yoz)
}

pub mod util {
    use crate::interpreter::{interpret, Context, inc};
    use crate::jets::util::*;
    use crate::jets::BAIL_EXIT;
    use crate::noun::{Atom, IndirectAtom, DirectAtom, Noun, D, T, YES, NO};
    use crate::jets::tree::util::*;
    use crate::jets::map::util::*;
    use crate::jets::set::util::*;
    use crate::mem::NockStack;
    use crate::unifying_equality::unifying_equality;
    use crate::jets::sort::util::*;
    use crate::jets::{Result, JetErr};
    use either::{Right, Left};
    use crate::jets::math::util::*;
    use crate::jets::list::util::*;
    use crate::jets::bits::util::*;
    use nockvm_macros::tas;
    use crate::mug::mug;

    // pub fn open(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut gen: Noun) -> Result {

    //     let [head_gen, tail_gen] = gen.uncell()?;

    //     let tag = head_gen.as_direct()?;

    //     match tag.data() {
    //         0 => {
    //             let [p_gen, q_gen] = tail_gen.uncell()?;
    //             let limb = T(&mut context.stack, &[YES, p_gen]);
    //             let p = T(&mut context.stack, &[limb, D(0)]);
    //             return Ok(T(&mut context.stack, &[D(tas!(b"cnts")), p, D(0)]));
    //         }
    //         tas!(b"base") => call_factory(context, ut_core, gen),
    //         tas!(b"bust") => {
    //             let sam = T(&mut context.stack, &[D(tas!(b"base")), tail_gen]);
    //             return call_example(context, ut_core, sam);
    //         }
    //         tas!(b"ktcl") => call_factory(context, ut_core, tail_gen),
    //         tas!(b"dbug") => call_factory(context, ut_core, slot(tail_gen, 3)?),
    //         tas!(b"eror") => {
    //             let cord = crip(&mut context.stack, tail_gen)?;
    //             println!("{}", String::from_utf8_lossy(cord.as_atom()?.as_ne_bytes()));
    //             return Err(BAIL_EXIT);
    //         }
    //         tas!(b"knit") => {
    //             let kts = T(&mut context.stack, &[D(tas!(b"ktts")), D(tas!(b"v")), D(tas!(b"0")), D(tas!(b"1"))]);
    //             let sam = T(&mut context.stack, &[D(tas!(b"tsgr")), tail_gen]);

    //         }
    //         _ => return Ok(gen),

    //     }
    // }

    fn crip(stack: &mut NockStack, mut tape: Noun) -> Result {
        let l = lent(tape)?;
        if l == 0 {
            return Ok(unsafe { DirectAtom::new_unchecked(0).as_noun() });
        }
        let (mut indirect, buf) = unsafe { IndirectAtom::new_raw_mut_bytes(stack, l) };

        let mut idx = 0;
        loop {
            if let Ok(tape_it) = tape.as_cell() {
                let tape_byte = tape_it.head().as_direct()?;
                tape = tape_it.tail();
                if tape_byte.data() >= 256 {
                    break Err(BAIL_EXIT);
                } else {
                    buf[idx] = tape_byte.data().to_le_bytes()[0];
                    idx += 1;
                }
            } else {
                break Ok(unsafe { indirect.normalize_as_atom().as_noun() });
            }
        }
    }

    // pub fn factory(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut gen: Noun) -> Result {

    // }

    pub fn play(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut gen: Noun) -> Result {
        ut_core = replace_at_axis(context, ut_core, 59, NO)?;

        let [head_gen, tail_gen] = gen.uncell()?;

        if head_gen.is_cell() {
            let play_head = play(context, ut_core, sut, head_gen)?;
            let play_tail = play(context, ut_core, sut, tail_gen)?;
            return Ok(cell(context, play_head, play_tail)?);
        }

        let tag = head_gen.as_direct()?;
        // println!("{:?}", head_gen);
        match tag.data() {
            tas!(b"brcn") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                let garb = T(&mut context.stack, &[p_gen, D(tas!(b"dry")), D(tas!(b"gold"))]);
                let mask = T(&mut context.stack, &[D(tas!(b"full")), D(0), D(0), D(0)]);
                let seminoun = T(&mut context.stack, &[mask, D(0)]);
                let coil = T(&mut context.stack, &[garb, sut, seminoun, q_gen]);
                Ok(core(context, sut, coil)?)
            }
            tas!(b"brpt") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                let garb = T(&mut context.stack, &[p_gen, D(tas!(b"wet")), D(tas!(b"gold"))]);
                let mask = T(&mut context.stack, &[D(tas!(b"full")), D(0), D(0), D(0)]);
                let seminoun = T(&mut context.stack, &[mask, D(0)]);
                let coil = T(&mut context.stack, &[garb, sut, seminoun, q_gen]);
                Ok(core(context, sut, coil)?)
            }
            tas!(b"cnts") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                call_play_et(context, ut_core, p_gen, q_gen)
            }
            tas!(b"dtkt") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                gen = T(&mut context.stack, &[D(tas!(b"kttr")), p_gen]);
                play(context, ut_core, sut, gen)
            }
            tas!(b"dtls") => Ok(T(&mut context.stack, &[D(tas!(b"atom")), D(0), D(0)])),
            tas!(b"rock") => {
                fn play_rock_recursion(context: &mut Context, p_gen: Noun, q_gen: Noun) -> Result {
                    if q_gen.is_atom() {
                        return Ok(T(&mut context.stack, &[D(tas!(b"atom")), p_gen, D(0), q_gen]));
                    }
                    let head = play_rock_recursion(context, p_gen, slot(q_gen, 2)?)?;
                    let tail = play_rock_recursion(context, p_gen, slot(q_gen, 3)?)?;
                    Ok(T(&mut context.stack, &[D(tas!(b"cell")), head, tail]))
                }
                let [_tag_gen, p_gen, q_gen] = gen.uncell()?;
                return play_rock_recursion(context, p_gen, q_gen);
            }
            tas!(b"sand") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                if q_gen.is_atom() {
                    if unsafe { p_gen.raw_equals(&D(tas!(b"n"))) } {
                        if unsafe { !q_gen.raw_equals(&D(0)) } {
                            return Err(BAIL_EXIT);
                        }
                        return Ok(T(&mut context.stack, &[D(tas!(b"atom")), p_gen, D(0), q_gen]));
                    } else if unsafe { p_gen.raw_equals(&D(tas!(b"f"))) } {
                        if q_gen.as_atom()?.as_u64()? > 1 {
                            return Err(BAIL_EXIT);
                        }
                        return bool(context);
                    }
                    return Ok(T(&mut context.stack, &[D(tas!(b"atom")), p_gen, D(0)]));
                }
                gen = T(&mut context.stack, &[D(tas!(b"rock")), tail_gen]);
                play(context, ut_core, sut, gen)
            }
            tas!(b"tune") => face(context, tail_gen, sut),
            tas!(b"dttr") => Ok(D(tas!(b"noun"))),
            tas!(b"dtts") => bool(context),
            tas!(b"dtwt") => bool(context),
            tas!(b"hand") => Ok(slot(tail_gen, 2)?),
            tas!(b"ktbr") => {
                let wrap_p_gen = play(context, ut_core, sut, tail_gen)?;
                return wrap(context, ut_core, wrap_p_gen, D(tas!(b"iron")));
            }
            tas!(b"ktls") => play(context, ut_core, sut, slot(gen, 6)?),
            tas!(b"ktpm") => {
                let wrap_p_gen = play(context, ut_core, sut, tail_gen)?;
                return wrap(context, ut_core, wrap_p_gen, D(tas!(b"zinc")));
            }
            tas!(b"ktsg") => play(context, ut_core, sut, slot(gen, 3)?),
            tas!(b"ktwt") => {
                sut = play(context, ut_core, sut, tail_gen)?;
                return wrap(context, ut_core, sut, D(tas!(b"lead")));
            }
            tas!(b"note") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                let p = T(&mut context.stack, &[sut, p_gen]);
                let q = play(context, ut_core, sut, q_gen)?;
                return hint(context, p, q);
            }
            tas!(b"sgzp") => {   //  TODO: enable sigcab
                let [p_gen, q_gen] = tail_gen.uncell()?;
                return play(context, ut_core, sut, q_gen);
            }
            tas!(b"sggr") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                return play(context, ut_core, sut, q_gen);
            }
            tas!(b"tsgr") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                sut = play(context, ut_core, sut, p_gen)?;
                ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                return play(context, ut_core, sut, q_gen);
            }
            tas!(b"tscm") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                sut = busk(context, sut, p_gen)?;
                ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                return play(context, ut_core, sut, q_gen);
            }
            tas!(b"wtcl") => {
                let [p_gen, q_gen, r_gen] = tail_gen.uncell()?;
                ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                let fex = call_arm(context, ut_core, 380, p_gen)?;   //  +gain
                let wux = call_arm(context, ut_core, 98172, p_gen)?; //  +lose
                let fox =
                    if unsafe { fex.raw_equals(&D(tas!(b"void"))) } {
                        D(tas!(b"void"))
                    } else {
                        ut_core = replace_at_axis(context, ut_core, 6, fex)?;
                        play(context, ut_core, fex, q_gen)?
                    };
                let wox =
                    if unsafe { wux.raw_equals(&D(tas!(b"void"))) } {
                        D(tas!(b"void"))
                    } else {
                        ut_core = replace_at_axis(context, ut_core, 6, wux)?;
                        play(context, ut_core, wux, r_gen)?
                    };
                let types = T(&mut context.stack, &[fox, wox, D(0)]);
                return fork(context, types);
            }
            tas!(b"fits") => bool(context),
            tas!(b"wthx") => bool(context),
            tas!(b"dbug") => {  //  TODO: enable sigcab
                let [p_gen, q_gen] = tail_gen.uncell()?;
                // let sam = T(&mut context.stack, &[D(tas!(b"o")), p_gen]);
                // ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                // let show_fol = call_show(context, ut_core, sut)?; //  +show
                play(context, ut_core, sut, q_gen)
            }
            tas!(b"zpcm") => play(context, ut_core, sut, slot(gen, 6)?),
            tas!(b"lost") => Ok(D(tas!(b"void"))),
            tas!(b"zpmc") => {
                let [p_gen, q_gen] = tail_gen.uncell()?;
                let play_head = play(context, ut_core, sut, p_gen)?;
                let play_tail = play(context, ut_core, sut, q_gen)?;
                return cell(context, play_head, play_tail);
            }
            tas!(b"zpgl") => {
               let kttr = T(&mut context.stack, &[D(tas!(b"kttr")), slot(gen, 6)?]);
               play(context, ut_core, sut, kttr)
            }
            tas!(b"zpts") =>  Ok(D(tas!(b"noun"))),
            tas!(b"zppt") => {
                let [p_gen, q_gen, r_gen] = tail_gen.uncell()?;
                ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                let feel = call_arm(context, ut_core, 1502, p_gen)?;   //  +feel
                if unsafe { feel.raw_equals(&YES) } {
                    return play(context, ut_core, sut, q_gen);
                }
                play(context, ut_core, sut, r_gen)
            }
            tas!(b"zpzp") =>  Ok(D(tas!(b"void"))),
            _ => {
                let mut doz = call_open(context, ut_core, gen)?;
                if unsafe {
                    unifying_equality(&mut context.stack, &mut doz, &mut gen) }
                {
                    println!("play-open");
                    return Err(BAIL_EXIT);
                }
                return play(context, ut_core, sut, doz);
            }
        }
    }

    pub fn busk(context: &mut Context, mut sut: Noun, mut gen: Noun) -> Result {
        let p = T(&mut context.stack, &[D(0), gen, D(0)]);
        Ok(T(&mut context.stack, &[D(tas!(b"face")), p, sut]))
    }

    pub fn wrap(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut yoz: Noun) -> Result {
        if sut.is_atom() {
            return Ok(sut);
        }

        let [head_sut, tail_sut] = sut.uncell()?;

        let tag = head_sut.as_direct()?;

        match tag.data() {
            tas!(b"cell") => {
                let [p_sut, q_sut] = tail_sut.uncell()?;
                let head = wrap(context, ut_core, p_sut, yoz)?;
                let tail = wrap(context, ut_core, q_sut, yoz)?;
                return Ok(cell(context, head, tail)?);
            }
            tas!(b"core") => {
                // [%core p=type q=[p=garb q=type r=[p=seminoun q=(map term tome)]]]
                let [sut_tag, mut p_sut, mut p_q_sut, q_q_sut, p_r_q_sut, mut q_r_q_sut] = sut.uncell()?;
                let [p_p_q_sut, mut q_p_q_sut, r_p_q_sut] = p_q_sut.uncell()?;  // [(unit term) poly vair]

                if unsafe { !( r_p_q_sut.raw_equals(&D(tas!(b"gold")))
                            || yoz.raw_equals(&D(tas!(b"lead"))) ) } {
                    println!("wrap");
                    return Err(BAIL_EXIT);
                }
                p_q_sut = T(&mut context.stack, &[p_p_q_sut, q_p_q_sut, yoz]);
                return Ok(T(&mut context.stack, &[sut_tag, p_sut, p_q_sut, q_q_sut, p_r_q_sut, q_r_q_sut]));

            }
            tas!(b"face") => {
                let [p_sut, q_sut] = tail_sut.uncell()?;
                let der = wrap(context, ut_core, q_sut, yoz)?;
                Ok(face(context, p_sut, der)?)
            }
            tas!(b"fork") => {
                let mut tap_res = tap_in(context, tail_sut, D(0))?;
                let mut types = D(0);
                while tap_res.is_cell() {
                    let [head, tail] = tap_res.uncell()?;
                    let wrap_res = wrap(context, ut_core, head, yoz)?;
                    types = T(&mut context.stack, &[wrap_res, types]);
                    tap_res = tail;
                }
                fork(context, types)
            }
            tas!(b"hint") => {
                let [p_sut, q_sut] = tail_sut.uncell()?;
                let tail = wrap(context, ut_core, q_sut, yoz)?;
                hint(context, p_sut, tail)
            }
            tas!(b"hold") => {
                let leg = T(&mut context.stack, &[tail_sut, D(0)]);
                sut = rest_cached(context, ut_core, leg)?;
                wrap(context, ut_core, sut, yoz)
            }
            _ => Err(BAIL_EXIT)
        }

    }

    pub fn bool(context: &mut Context) -> Result {
        let atom_f_y = T(&mut context.stack, &[D(tas!(b"atom")), D(tas!(b"f")), D(0), YES]);
        let atom_f_n = T(&mut context.stack, &[D(tas!(b"atom")), D(tas!(b"f")), D(0), NO]);
        let types = T(&mut context.stack, &[atom_f_y, atom_f_n, D(0)]);
        return Ok(fork(context, types)?);
    }

    pub fn fire(context: &mut Context, mut ut_core: Noun, sut: Noun, mut hag: Noun) -> Result {

        if unsafe { lent(hag)? == 1 } {
            let [i_hag, _t_hag] = hag.uncell()?;           // (list [type foot])
            let [p_i_hag, mut q_i_hag] = i_hag.uncell()?;  // [type foot]

            let mut wet_foot = T(&mut context.stack, &[D(tas!(b"wet")), D(0), D(1)]);

            if unsafe {
                unifying_equality(&mut context.stack, &mut q_i_hag, &mut wet_foot) } {
                return Ok(p_i_hag);
            }
        }

        let mut res = D(0);

        while hag.is_cell() {
            let [i_hag, t_hag] = hag.uncell()?;    // (list [type foot])
            let [mut p, q] = i_hag.uncell()?;      // [type foot]

            if unsafe { !(p.is_cell() && p.as_cell()?.head().raw_equals(&D(tas!(b"core")))) } {
                println!("expected-fork-to-be-core");
                println!("fire-core");
                return Err(BAIL_EXIT);
            }

            // [%core p=type q=[p=garb q=type r=[p=seminoun q=(map term tome)]]]
            let [p_tag, mut p_p, p_q_p, q_q_p, p_r_q_p, mut q_r_q_p] = p.uncell()?;
            let [p_p_q_p, mut q_p_q_p, r_p_q_p] = p_q_p.uncell()?;  // [(unit term) poly vair]

            let new_p_q_p = replace_at_axis(context, p_q_p, 7, D(tas!(b"gold")))?;
            let dox = T(&mut context.stack, &[D(tas!(b"core")), q_q_p, new_p_q_p, q_q_p, p_r_q_p, q_r_q_p]);
            let vet = slot(ut_core, 59)?;
            let [q_tag, p_q] = q.uncell()?;

            if unsafe { q_tag.raw_equals(&(D(tas!(b"dry")))) } {
                let ut_core = replace_at_axis(context, ut_core, 6, q_q_p)?;
                if unsafe { vet.raw_equals(&YES) && nest(context, ut_core, q_q_p, YES, p_p)?.raw_equals(&NO) } {
                   return Err(BAIL_EXIT);
                }

                let fired = T(&mut context.stack, &[D(tas!(b"hold")), dox, p_q]);
                res = T(&mut context.stack, &[fired, res]);
                hag = t_hag;
                continue;
            }

            let ut_core = replace_at_axis(context, ut_core, 6, p_p)?;
            p_p = redo(context, ut_core, p_p, q_q_p)?;
            p = replace_at_axis(context, p, 6, p_p)?;

            let mut rib = slot(ut_core, 58)?;
            let cache_entry = T(&mut context.stack, &[sut, dox, p_q]);
            if unsafe { vet.raw_equals(&YES) && has_in(context, rib, cache_entry)?.raw_equals(&NO)
            } {
                rib = put_in(context, rib, cache_entry)?;
                let mut ut_core = replace_at_axis(context, ut_core, 58, rib)?;
                ut_core = replace_at_axis(context, ut_core, 6, p)?;
                let sam = T(&mut context.stack, &[D(tas!(b"noun")), dox, p_q]);
                let _mull_res = call_arm(context, ut_core, 24020, sam); //  +mull
                //  no crash on mull == success
            }

            let fired = T(&mut context.stack, &[D(tas!(b"hold")), p, p_q]);
            res = T(&mut context.stack, &[fired, res]);
            hag = t_hag;
        }

        Ok(fork(context, res)?)
    }

    pub fn redo(context: &mut Context, mut ut_core: Noun, mut sut: Noun, rff: Noun) -> Result {
        let hos = D(0);
        let wec = T(&mut context.stack, &[D(0), D(0), D(0)]);
        let gil = D(0);

        redo_dext(context, ut_core, sut, rff, hos, wec, gil)
    }

    pub fn redo_dext(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut rff: Noun, mut hos: Noun, wec: Noun, mut gil: Noun) -> Result {

        if unsafe { unifying_equality(&mut context.stack, &mut sut, &mut rff)
                    ||  rff.raw_equals(&D(tas!(b"noun")))
                    ||  rff.raw_equals(&D(tas!(b"void")))
                    || ( rff.is_cell() && (rff.as_cell()?.head().raw_equals(&D(tas!(b"atom")))
                            || rff.as_cell()?.head().raw_equals(&D(tas!(b"core")))) )
                    } {
            redo_done(context, ut_core, sut, rff, hos, wec)
        } else {
            match sut.as_either_atom_cell() {
                Left(atom) => {
                    let [rff, wec] = redo_sint(context, ut_core, sut, rff, wec, true)?.uncell()?;
                    redo_done(context, ut_core, sut, rff, hos, wec)
                }
                Right(cell) => {
                    if unsafe { cell.head().raw_equals(&D(tas!(b"cell"))) } {

                        let [rff, mut wec] = redo_sint(context, ut_core, sut, rff, wec, true)?.uncell()?;

                        let new_hos = D(0);
                        let new_wec = T(&mut context.stack, &[D(0), D(0), D(0)]);

                        let [_tag_sut, p_sut, q_sut] = sut.uncell()?;

                        ut_core = replace_at_axis(context, ut_core, 6, rff)?;

                        let peek_head = peek(context, ut_core, rff, D(tas!(b"free")), D(2))?;
                        let dext_head = redo_dext(context, ut_core, p_sut, peek_head, new_hos, new_wec, gil)?;

                        let peek_tail = peek(context, ut_core, rff, D(tas!(b"free")), D(3))?;
                        let dext_tail = redo_dext(context, ut_core, q_sut, peek_tail, new_hos, new_wec, gil)?;

                        sut = T(&mut context.stack, &[D(tas!(b"cell")), dext_head, dext_tail]);

                        redo_done(context, ut_core, sut, rff, hos, wec)

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"face"))) } {
                        let [_tag_sut, p_sut, q_sut] = sut.uncell()?;

                        hos = T(&mut context.stack, &[p_sut, hos]);
                        redo_dext(context, ut_core, q_sut, rff, hos, wec, gil)

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"hint"))) } {
                        let [_tag_sut, p_sut, q_sut] = sut.uncell()?;

                        let dext_res = redo_dext(context, ut_core, q_sut, rff, hos, wec, gil)?;

                        hint(context, p_sut, dext_res)

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"fork"))) } {

                        let mut list = tap_in(context, slot(sut, 3)?, D(0))?;
                        let mut new_list = D(0);

                        while list.is_cell() {
                            let [head, tail] = list.uncell()?;
                            let dext_res = redo_dext(context, ut_core, head, rff, hos, wec, gil)?;
                            new_list = T(&mut context.stack, &[dext_res, new_list]);
                            list = tail;
                        }

                        fork(context, new_list)

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"hold"))) } {

                        let [rff, wec] = redo_sint(context, ut_core, sut, rff, wec, false)?.uncell()?;

                        let fan = slot(ut_core, 28)?;

                        if unsafe { has_in(context, fan, cell.tail())?.raw_equals(&YES) } {
                            let [rff, wec] = redo_sint(context, ut_core, sut, rff, wec, true)?.uncell()?;
                            redo_done(context, ut_core, sut, rff, hos, wec)

                        } else {
                            let sut_rff = T(&mut context.stack, &[sut, rff]);

                            if unsafe { has_in(context, gil, sut_rff)?.raw_equals(&YES) } {
                                let [rff, wec] = redo_sint(context, ut_core, sut, rff, wec, false)?.uncell()?;
                                redo_done(context, ut_core, sut, rff, hos, wec)

                            } else {
                                let leg = T(&mut context.stack, &[cell.tail(), D(0)]);
                                let mut rest_res = rest_cached(context, ut_core, leg)?;
                                gil = put_in(context, gil, sut_rff)?;
                                let mut dext_res = redo_dext(context, ut_core, rest_res, rff, hos, wec, gil)?;

                                if unsafe { unifying_equality(&mut context.stack, &mut rest_res, &mut dext_res) } {
                                    Ok(sut)
                                } else {
                                    Ok(dext_res)
                                }
                            }
                        }
                    } else {
                        let [rff, wec] = redo_sint(context, ut_core, sut, rff, wec, true)?.uncell()?;
                        redo_done(context, ut_core, sut, rff, hos, wec)
                    }
                }
            }
        }
    }

    pub fn redo_sint(context: &mut Context, mut ut_core: Noun, sut: Noun, mut rff: Noun, wec: Noun, hod: bool) -> Result {

         match rff.as_either_atom_cell() {
                Left(atom) => {
                    Ok(T(&mut context.stack, &[rff, wec]))
                }
                Right(cell) => {
                    if unsafe { cell.head().raw_equals(&D(tas!(b"hint"))) } {
                        redo_sint(context, ut_core, sut, slot(rff, 7)?, wec, hod)

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"face"))) } {
                        let [_tag_rff, p_rff, q_rff] = rff.uncell()?;

                        let mut list = tap_in(context, wec, D(0))?;

                        let mut new_wec = D(0);
                        while list.is_cell() {
                            let [head, tail] = list.uncell()?;
                            let tool = T(&mut context.stack, &[p_rff, head]);
                            new_wec = put_in(context, new_wec, tool)?;
                            list = tail;
                        }

                        redo_sint(context, ut_core, sut, q_rff, new_wec, hod)

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"fork"))) } {
                        let moy = tap_in(context, slot(rff, 3)?, D(0))?;

                        let [wec, types] = redo_sint_fork(context, ut_core, sut, rff, wec, moy, hod)?.uncell()?;

                        rff = fork(context, types)?;
                        Ok(T(&mut context.stack, &[rff, wec]))

                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"hold"))) } {
                        if hod {
                            let leg = T(&mut context.stack, &[cell.tail(), D(0)]);
                            rff = rest_cached(context, ut_core, leg)?;
                            return redo_sint(context, ut_core, sut, rff, wec, hod);
                        }
                        return Ok(T(&mut context.stack, &[rff, wec]));
                    } else {
                        return Ok(T(&mut context.stack, &[rff, wec]));
                    }
                }
        }
    }

    pub fn redo_sint_fork(context: &mut Context, mut ut_core: Noun, sut: Noun, mut rff: Noun, mut wec: Noun, moy: Noun, hod: bool) -> Result {
        if unsafe { moy.raw_equals(&D(0)) } {
            return Ok(T(&mut context.stack, &[D(0), D(0)]));
        }
        let [i_moy, t_moy] = moy.uncell()?;

        let mor = redo_sint_fork(context, ut_core, sut, rff, wec, t_moy, hod)?;
        let [p_mor, q_mor] = mor.uncell()?;

        if  unsafe { miss(context, ut_core, sut, i_moy)?.raw_equals(&YES) } {
            return Ok(mor);
        }

        let [rff, mut wec] = redo_sint(context, ut_core, sut, i_moy, wec, hod)?.uncell()?;

        wec = uni_in(context, p_mor, wec)?;

        return Ok(T(&mut context.stack, &[wec, rff, q_mor]));
    }

    pub fn redo_done(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut rff: Noun, hos: Noun, wec: Noun) -> Result {
        let mut lov = {
            let lov = redo_dear(context, ut_core, hos, wec)?;

            if unsafe { lov.raw_equals(&D(0)) } {
                return Err(BAIL_EXIT);
            }
            slot(lov, 3)?
        };

        loop {
            if unsafe { lov.raw_equals(&D(0)) } {
               return Ok(sut);
            }

            let [i_lov, t_lov] = lov.uncell()?;
            sut = face(context, i_lov, sut)?;
            lov = t_lov;
        }
    }

    pub fn core(context: &mut Context, pac: Noun, con: Noun) -> Result {
        if unsafe { pac.raw_equals(&D(tas!(b"void")))  } {
            return Ok(D(tas!(b"void")));
        }
        return Ok(T(&mut context.stack, &[D(tas!(b"core")), pac, con]));
    }

    pub fn cell(context: &mut Context, hed: Noun, tal: Noun) -> Result {
        if unsafe { hed.raw_equals(&D(tas!(b"void")))  ||
                    tal.raw_equals(&D(tas!(b"void")))
                } {
            return Ok(D(tas!(b"void")));
        }
        return Ok(T(&mut context.stack, &[D(tas!(b"cell")), hed, tal]));

    }

    pub fn face(context: &mut Context, giz: Noun, der: Noun) -> Result {
        if unsafe { der.raw_equals(&D(tas!(b"void"))) } {
            return Ok(D(tas!(b"void")));
        }
       return Ok(T(&mut context.stack, &[D(tas!(b"face")), giz, der]));
    }

    pub fn hint(context: &mut Context, p: Noun, q: Noun) -> Result {
        if unsafe { q.raw_equals(&D(tas!(b"void")))
                    ||  q.raw_equals(&D(tas!(b"noun"))) }
                  {
            return Ok(q);
        }
        Ok(T(&mut context.stack, &[D(tas!(b"hint")), p, q]))
    }

    pub fn redo_dear(context: &mut Context, ut_core: Noun, hos: Noun, wec: Noun) -> Result {
        if unsafe { wec.raw_equals(&D(0)) } {
            return Ok(T(&mut context.stack, &[D(0), D(0)]));
        } else {
            let [n_wec, l_wec, r_wec] = wec.uncell()?;
            if  unsafe { !l_wec.raw_equals(&D(0)) || !r_wec.raw_equals(&D(0)) } {
                return Ok(D(0));
            } else {
                let har = n_wec;
                let p_len = lent(hos)?;
                let q_len = lent(har)?;
                let mut lip = 0;

                let mut lup = D(0);
                let mut lop = 0;

                loop {
                    if lop > p_len || lop > q_len {
                        if lup.is_cell() {
                            lip = slot(lup, 3)?.as_atom()?.as_u64()?;
                        } else {
                            lip = 0;
                        }
                        break;
                    }

                    let sub_res = p_len - lop;
                    let mut lep = slag(context, D(sub_res as u64).as_atom()?, hos)?;
                    let mut lap = scag(context, D(lop as u64).as_atom()?, har)?;

                    lup = {
                        if unsafe { !unifying_equality(&mut context.stack, &mut lep, &mut lap) } {
                            lup
                        } else {
                            T(&mut context.stack, &[D(0), D(lop as u64)])
                        }
                    };

                    lop += 1;
                }

                let slag_res = slag(context, D(lip).as_atom()?, har)?;
                let weld_res = weld(&mut context.stack, hos, slag_res)?;
                Ok(T(&mut context.stack, &[D(0), weld_res]))
            }
        }
    }

    pub fn miss(context: &mut Context, mut ut_core: Noun, sut: Noun, rff: Noun) -> Result {
        let gil = D(0);
        miss_dext(context, ut_core, sut, rff, gil)
    }

    pub fn miss_dext(context: &mut Context, mut ut_core: Noun, mut sut: Noun, mut rff: Noun, mut gil: Noun) -> Result {

        if unsafe { unifying_equality(&mut context.stack, &mut sut, &mut rff) } {
            ut_core = replace_at_axis(context, ut_core, 6, D(tas!(b"void")))?;
            return nest(context, ut_core, D(tas!(b"void")), NO, sut);
        } else {
            match sut.as_either_atom_cell() {
                Left(atom) => {
                    if unsafe { atom.as_noun().raw_equals(&D(tas!(b"void"))) } {
                        return Ok(YES);
                    } else if unsafe { atom.as_noun().raw_equals(&D(tas!(b"noun"))) } {
                        ut_core = replace_at_axis(context, ut_core, 6, D(tas!(b"void")))?;
                        return nest(context, ut_core, D(tas!(b"void")), NO, rff);
                    } else {
                        return Err(BAIL_EXIT);
                    }
                }
                Right(cell) => {
                    if unsafe { cell.head().raw_equals(&D(tas!(b"atom"))) } {
                        miss_sint(context, ut_core, sut, rff, gil)
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"cell"))) } {
                        miss_sint(context, ut_core, sut, rff, gil)
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"core"))) } {
                        sut = T(&mut context.stack, &[D(tas!(b"cell")), D(tas!(b"noun")), D(tas!(b"noun"))]);
                        miss_sint(context, ut_core, sut, rff, gil)
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"fork"))) } {
                        let [_sut_tag, p_sut] = sut.uncell()?;
                        let mut tap_res = tap_in(context, p_sut, D(0))?;
                        while tap_res.is_cell() {
                            let [head, tail] = tap_res.uncell()?;
                            let dext_res = miss_dext(context, ut_core, head, rff, gil)?;
                            if unsafe { dext_res.raw_equals(&NO) } {
                                return Ok(NO);
                            }
                            tap_res = tail;
                        }
                        return Ok(YES);
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"hold"))) } {
                        let list = T(&mut context.stack, &[sut, rff, D(0)]);
                        let set = gas_in(context, D(0), list)?;
                        if  unsafe { has_in(context, gil, set)?.raw_equals(&YES) } {
                            return Ok(YES);
                        } else {
                            gil = put_in(context, gil, set)?;
                            let leg =  T(&mut context.stack, &[cell.tail(), D(0)]);
                            sut = rest_cached(context, ut_core, leg)?;
                            return miss_dext(context, ut_core, sut, rff, gil);
                        }
                    } else {  // hint/face
                       return miss_dext(context, ut_core, slot(sut, 7)?, rff, gil);
                    }
                }
            }
        }
    }

    pub fn miss_sint(context: &mut Context, ut_core: Noun, sut: Noun, rff: Noun, gil: Noun) -> Result {
        match rff.as_either_atom_cell() {
            Left(_atom) => {
                return miss_dext(context, ut_core, rff, sut, gil);
            }
            Right(cell) => {
                    if unsafe { cell.head().raw_equals(&D(tas!(b"atom"))) } {
                        if unsafe { sut.as_cell()?.head().raw_equals(&D(tas!(b"atom"))) } {
                            let [_sut_tag, _p_sut, mut q_sut] = sut.uncell()?;
                            let [_rff_tag, _p_rff, mut q_rff] = rff.uncell()?;
                            if unsafe { q_sut.is_cell()  && q_rff.is_cell()
                                && !unifying_equality(&mut context.stack, &mut q_sut, &mut q_rff) } {
                                return Ok(YES);
                            } else {
                                return Ok(NO);
                            }
                        } else {
                            return Ok(YES);
                        }
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"cell"))) } {
                        if unsafe { sut.as_cell()?.head().raw_equals(&D(tas!(b"cell"))) } {
                            let [_sut_tag, p_sut, q_sut] = sut.uncell()?;
                            let [_rff_tag, p_rff, q_rff] = rff.uncell()?;
                            if  unsafe {
                                    miss_dext(context, ut_core, p_sut, p_rff, gil)?.raw_equals(&YES) ||
                                    miss_dext(context, ut_core, q_sut, q_rff, gil)?.raw_equals(&YES)
                                } {
                                return Ok(YES);
                            } else {
                                return Ok(NO);
                            }
                        } else {
                            return Ok(YES);
                        }
                    } else {
                        return miss_dext(context, ut_core, rff, sut, gil);
                    }
            }
        }
    }

    pub fn nest(context: &mut Context, ut_core: Noun, sut: Noun, tel: Noun, rff: Noun) -> Result {

        let seg = D(0);
        let reg = D(0);
        let gil = D(0);
        let tel = if unsafe { tel.raw_equals(&YES) } { true } else { false };

        nest_dext_cached(context, ut_core, sut, tel, rff, seg, gil, reg)
    }

    pub fn nest_dext_cached(context: &mut Context, ut_core: Noun, sut: Noun, tel: bool, rff: Noun, seg: Noun, gil: Noun, reg: Noun) -> Result {

        let flag = if let Ok(noun) = slot(ut_core, 59) {
            if unsafe { noun.raw_equals(&D(0)) } {
                0u64
            } else {
                1u64
            }
        } else {
            1
        };
        let fun = (141 + tas!(b"dext")) + (flag << 8);
        let mut key = T(&mut context.stack, &[D(fun), sut, rff]);

        match context.cache.lookup(&mut context.stack, &mut key) {
            Some(pro) => Ok(pro),
            None => {
                let pro = nest_dext(context, ut_core, sut, tel, rff, seg, gil, reg)?;

                if unsafe { pro.raw_equals(&YES) && reg.raw_equals(&D(0)) }
                    || unsafe { pro.raw_equals(&NO) && seg.raw_equals(&D(0)) }
                {
                    context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
                }
                Ok(pro)
            }
        }
    }
    pub fn nest_dext(context: &mut Context, mut ut_core: Noun, mut sut: Noun, tel: bool, mut rff: Noun, mut seg: Noun, mut gil: Noun, reg: Noun) -> Result {

        if unsafe { unifying_equality(&mut context.stack, &mut sut, &mut rff) } {
            Ok(YES)
        } else {
           let res = match sut.as_either_atom_cell() {
                Left(atom) => {
                    if unsafe { atom.as_noun().raw_equals(&D(tas!(b"void"))) } {
                        nest_sint(context, ut_core, sut, tel, rff, seg, gil, reg)
                    } else if unsafe { atom.as_noun().raw_equals(&D(tas!(b"noun"))) } {
                        return Ok(YES);
                    } else {
                        return Err(BAIL_EXIT);
                    }
                }
                Right(cell) => {
                    if unsafe { cell.head().raw_equals(&D(tas!(b"atom"))) } {
                        if unsafe { !( rff.is_cell() && rff.as_cell()?.head().raw_equals(&D(tas!(b"atom"))) ) } {
                            nest_sint(context, ut_core, sut, tel, rff, seg, gil, reg)
                        } else {
                            let [_sut_tag, p_sut, mut q_sut] = sut.uncell()?;
                            let [_rff_tag, p_rff, mut q_rff] = rff.uncell()?;
                            if unsafe {
                                fitz(context, p_sut, p_rff)?.raw_equals(&YES) &&
                                ( q_sut.raw_equals(&D(0)) ||
                                unifying_equality(&mut context.stack, &mut q_sut, &mut q_rff) )
                            } {
                                return Ok(YES);
                            } else {
                                return Ok(NO);
                            }
                        }
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"cell"))) } {
                        if unsafe { !( rff.is_cell() && rff.as_cell()?.head().raw_equals(&D(tas!(b"cell"))) ) } {
                            nest_sint(context, ut_core, sut, tel, rff, seg, gil, reg)
                        } else {
                            let [_sut_tag, p_sut, q_sut] = sut.uncell()?;
                            let [_rff_tag, p_rff, q_rff] = rff.uncell()?;
                            let nest_head = nest_dext_cached(context, ut_core, p_sut, tel, p_rff, D(0), gil, D(0))?;
                            if unsafe { nest_head.raw_equals(&YES) } {
                                Ok(nest_dext_cached(context, ut_core, q_sut, tel, q_rff, D(0), gil, D(0))?)
                            } else {
                                Ok(NO)
                            }
                        }
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"core"))) } {
                        if unsafe { !( rff.is_cell() && rff.as_cell()?.head().raw_equals(&D(tas!(b"core"))) ) } {
                            nest_sint(context, ut_core, sut, tel, rff, seg, gil, reg)
                        } else {
                            let [_sut_tag, p_sut, mut q_sut] = sut.uncell()?;
                            let [_rff_tag, p_rff, mut q_rff] = rff.uncell()?;
                            if unsafe { unifying_equality(&mut context.stack, &mut q_sut, &mut q_rff) } {
                                nest_dext_cached(context, ut_core, p_sut, tel, p_rff, seg, gil, reg)
                            } else {
                                // [%core p=type q=[p=garb q=type r=[p=seminoun q=(map term tome)]]]
                                let [_sut_tag, p_sut, p_q_sut, q_q_sut, _p_r_q_sut, mut q_r_q_sut] = sut.uncell()?;
                                let [_rff_tag, p_rff, p_q_rff, q_q_rff, _p_r_q_rff, mut q_r_q_rff] = rff.uncell()?;
                                let [_p_p_q_sut, mut q_p_q_sut, r_p_q_sut] = p_q_sut.uncell()?;  // [(unit term) poly vair]
                                let [_p_p_q_rff, mut q_p_q_rff, r_p_q_rff] = p_q_rff.uncell()?;
                                if unsafe { unifying_equality(&mut context.stack, &mut q_p_q_sut, &mut q_p_q_rff)
                                    &&  nest_meet(context, ut_core, q_q_sut, tel, p_sut, seg, gil, reg)?.raw_equals(&YES)
                                    &&  nest_dext_cached(context, ut_core, q_q_rff, tel, p_rff, seg, gil, reg)?.raw_equals(&YES)
                                    &&  nest_deem(context, ut_core, q_q_sut, tel, q_q_rff, seg, gil, reg, r_p_q_sut, r_p_q_rff)?.raw_equals(&YES)
                                } {
                                    if unsafe { q_p_q_sut.raw_equals(&D(tas!(b"wet"))) }  {
                                        if unsafe { unifying_equality(&mut context.stack, &mut q_r_q_sut, &mut q_r_q_rff) } {
                                            Ok(YES)
                                        } else {
                                            Ok(NO)
                                        }
                                    } else {
                                        let sut_rff = T(&mut context.stack, &[sut, rff]);
                                        if unsafe {
                                            has_in(context, gil, sut_rff)?.raw_equals(&YES)
                                        } {
                                            Ok(YES)
                                        } else {
                                            gil = put_in(context, gil, sut_rff)?;
                                            sut = replace_at_axis(context, sut, 59, D(tas!(b"gold")))?;
                                            sut = replace_at_axis(context, sut, 6, q_q_sut)?;
                                            rff = replace_at_axis(context, rff, 59, D(tas!(b"gold")))?;
                                            rff = replace_at_axis(context, rff, 6, q_q_rff)?;
                                            nest_deep(context, ut_core, sut, tel, rff, seg, gil, reg, q_r_q_sut, q_r_q_rff)
                                        }
                                    }
                                } else {
                                    Ok(NO)
                                }
                            }
                        }
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"fork"))) } {
                        if unsafe { rff.raw_equals(&D(tas!(b"noun"))) ||
                                    ( rff.is_cell() &&
                                        ( rff.as_cell()?.head().raw_equals(&D(tas!(b"atom")))
                                        || rff.as_cell()?.head().raw_equals(&D(tas!(b"cell")))
                                        || rff.as_cell()?.head().raw_equals(&D(tas!(b"core")))) )
                                 } {
                            let [_sut_tag, p_sut] = sut.uncell()?;
                            let mut tap_res = tap_in(context, p_sut, D(0))?;
                            while tap_res.is_cell() {
                                let [head, tail] = tap_res.uncell()?;
                                let dext_res = nest_dext_cached(context, ut_core, head, false, rff, seg, gil, reg)?;
                                if unsafe { dext_res.raw_equals(&YES) } {
                                    return Ok(YES);
                                }
                                tap_res = tail;
                            }
                            Ok(NO)
                        } else {
                            nest_sint(context, ut_core, sut, tel, rff, seg, gil, reg)
                        }
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"hold"))) } {
                        if unsafe { has_in(context, seg, sut)?.raw_equals(&YES) } {
                           return Ok(NO);
                        }
                        let sut_rff = T(&mut context.stack, &[sut, rff]);
                        if unsafe { has_in(context, gil, sut_rff)?.raw_equals(&YES) } {
                           return Ok(YES);
                        } else {
                            seg = put_in(context, seg, sut)?;
                            gil = put_in(context, gil, sut_rff)?;
                            let leg = T(&mut context.stack, &[cell.tail(), D(0)]);
                            ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                            sut = rest_cached(context, ut_core, leg)?;
                            nest_dext_cached(context, ut_core, sut, tel, rff, seg, gil, reg)
                        }
                    } else {  // hint/face
                        nest_dext_cached(context, ut_core, slot(sut, 7)?, tel, rff, seg, gil, reg)
                    }
                }
            };

            if unsafe { res?.raw_equals(&YES)} {
                return Ok(YES);
            } else if !tel {
                return Ok(NO);
            } else {
                println!("nest-fail");   //  change this...
                return Err(BAIL_EXIT);
            }

        }
    }

    pub fn nest_deep(context: &mut Context, ut_core: Noun, sut: Noun, tel: bool, mut rff: Noun, seg: Noun, gil: Noun, reg: Noun, dom: Noun, vim: Noun)  -> Result {
        if unsafe { dom.raw_equals(&D(0)) } {
            if unsafe { vim.raw_equals(&D(0))} {
                Ok(YES)
            } else {
                Ok(NO)
            }
        } else {
            if unsafe { vim.raw_equals(&D(0))} {
                Ok(NO)
            } else {
                let [n_dom, l_dom, r_dom] = dom.uncell()?;
                let [n_vim, l_vim, r_vim] = vim.uncell()?;
                let [mut p_n_dom, _p_q_n_dom, q_q_n_dom] = n_dom.uncell()?;  //  [p=term q=[p=what q=(map term hoon))]]
                let [mut p_n_vim, _p_q_n_dom, q_q_n_vim] = n_vim.uncell()?;

                if unsafe { unifying_equality(&mut context.stack, &mut p_n_dom, &mut p_n_vim)
                    && nest_deep(context, ut_core, sut, tel, rff, seg, gil, reg, l_dom, l_vim)?.raw_equals(&YES)
                    && nest_deep(context, ut_core, sut, tel, rff, seg, gil, reg, r_dom, r_vim)?.raw_equals(&YES)
                } {
                    nest_deep_recursion(context, ut_core, sut, tel, rff, seg, gil, reg, q_q_n_dom, q_q_n_vim)
                } else {
                    Ok(NO)
                }
            }
        }
    }

    pub fn nest_deep_recursion(context: &mut Context, mut ut_core: Noun, sut: Noun, tel: bool, rff: Noun, seg: Noun, gil: Noun, reg: Noun, dab: Noun, hem: Noun)  -> Result {
        if unsafe { dab.raw_equals(&D(0))} {  // q=(map term hoon)
            if unsafe { hem.raw_equals(&D(0)) } {
                Ok(YES)
            } else {
                Ok(NO)
            }
        } else {
            if unsafe { hem.raw_equals(&D(0))} {
                Ok(NO)
            } else {
                let [n_dab, l_dab, r_dab] = dab.uncell()?;  // (map term hoon)
                let [n_hem, l_hem, r_hem] = hem.uncell()?;
                let [mut p_n_dab, q_n_dab] = n_dab.uncell()?;  //  [term hoon]
                let [mut p_n_hem, q_n_hem] = n_hem.uncell()?;

                if unsafe { unifying_equality(&mut context.stack, &mut p_n_dab, &mut p_n_hem)
                    && nest_deep_recursion(context, ut_core, sut, tel, rff, seg, gil, reg, l_dab, l_hem)?.raw_equals(&YES)
                    && nest_deep_recursion(context, ut_core, sut, tel, rff, seg, gil, reg, r_dab, r_hem)?.raw_equals(&YES)
                } {
                    ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                    let play_sut = play(context, ut_core, sut, q_n_dab)?;  //  +play
                    //  let play_sut = call_arm(context, ut_core, 3006, q_n_dab)?; //  +play

                    ut_core = replace_at_axis(context, ut_core, 6, rff)?;
                    let play_rff = play(context, ut_core, rff, q_n_hem)?;  //  +play
                    // let play_rff = call_arm(context, ut_core, 3006, q_n_hem)?; //  +play


                    nest_dext_cached(context, ut_core, play_sut, tel, play_rff, seg, gil, reg)
                } else {
                    Ok(NO)
                }
            }
        }
    }

    pub fn nest_deem(context: &mut Context, mut ut_core: Noun, sut: Noun, tel: bool, rff: Noun, seg: Noun, gil: Noun, reg: Noun, mut mel: Noun, mut ram: Noun)  -> Result {
        if  unsafe {
            unifying_equality(&mut context.stack, &mut mel, &mut ram)
            ||  mel.raw_equals(&D(tas!(b"lead")))
            ||  ram.raw_equals(&D(tas!(b"gold")))
        } {
            match mel.as_atom()?.as_u64()? {
                tas!(b"lead") => { Ok(YES) },
                tas!(b"gold") => { nest_meet(context, ut_core, sut, tel, rff, seg, gil, reg) },
                tas!(b"iron") => {
                    ut_core = replace_at_axis(context, ut_core, 6, rff)?;
                    let sut_peek = peek(context, ut_core, rff, D(tas!(b"rite")), D(2))?;
                    ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                    let rff_peek = peek(context, ut_core, sut, D(tas!(b"rite")), D(2))?;
                    nest_dext_cached(context, ut_core, sut_peek, tel, rff_peek, seg, gil, reg)
                },
                tas!(b"zinc") => {
                    ut_core = replace_at_axis(context, ut_core, 6, rff)?;
                    let rff_peek = peek(context, ut_core, rff, D(tas!(b"read")), D(2))?;
                    ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                    let sut_peek = peek(context, ut_core, sut, D(tas!(b"read")), D(2))?;
                    nest_dext_cached(context, ut_core, sut_peek, tel, rff_peek, seg, gil, reg)
                }
                _ => Err(BAIL_EXIT)
            }
        } else {
            Ok(NO)
        }
    }

    pub fn nest_meet(context: &mut Context, ut_core: Noun, sut: Noun, tel: bool, rff: Noun, seg: Noun, gil: Noun, reg: Noun)  -> Result {
       if unsafe { nest_dext_cached(context, ut_core, sut, tel, rff, seg, gil, reg)?.raw_equals(&YES) &&
                   nest_dext_cached(context, ut_core, rff, tel, sut, seg, gil, reg)?.raw_equals(&YES)
                } {
            Ok(YES)
       } else {
            Ok(NO)
       }
    }

    pub fn nest_sint(context: &mut Context, mut ut_core: Noun, sut: Noun, tel: bool, mut rff: Noun, seg: Noun, mut gil: Noun, mut reg: Noun)  -> Result {
        match rff.as_either_atom_cell() {
            Left(atom) => {
                    if unsafe { atom.as_noun().raw_equals(&D(tas!(b"void"))) } {
                        return Ok(YES);
                    } else if unsafe { atom.as_noun().raw_equals(&D(tas!(b"noun"))) } {
                        return Ok(NO);
                    } else {
                        return Err(BAIL_EXIT);
                    }
                }
                Right(cell) => {
                    if unsafe { cell.head().raw_equals(&D(tas!(b"atom"))) } {
                        return Ok(NO);
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"cell"))) } {
                        return Ok(NO);
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"core"))) } {
                        let [_rff_tag, p_rff, _q_rff] = rff.uncell()?;
                        rff = T(&mut context.stack, &[D(tas!(b"cell")), D(tas!(b"noun")), p_rff]);
                        nest_dext_cached(context, ut_core, sut, tel, rff, seg, gil, reg)
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"fork"))) } {
                        let [_rff_tag, p_rff] = rff.uncell()?;
                        let mut tap_res = tap_in(context, p_rff, D(0))?;
                        while tap_res.is_cell() {
                            let [head, tail] = tap_res.uncell()?;
                            let dext_res = nest_dext_cached(context, ut_core, sut, tel, head, seg, gil, reg)?;
                            if unsafe { dext_res.raw_equals(&NO) } {
                                return Ok(NO);
                            }
                            tap_res = tail;
                        }
                        Ok(YES)
                    } else if unsafe { cell.head().raw_equals(&D(tas!(b"hold"))) } {
                        if unsafe { has_in(context, reg, rff)?.raw_equals(&YES) } {
                           return Ok(YES);
                        }
                        let sut_rff = T(&mut context.stack, &[sut, rff]);
                        if unsafe { has_in(context, gil, sut_rff)?.raw_equals(&YES) } {
                           return Ok(YES);
                        } else {
                            reg = put_in(context, reg, rff)?;
                            gil = put_in(context, gil, sut_rff)?;
                            ut_core = replace_at_axis(context, ut_core, 6, rff)?;
                            let leg = T(&mut context.stack, &[cell.tail(), D(0)]);
                            rff = rest_cached(context, ut_core, leg)?;
                            nest_dext_cached(context, ut_core, sut, tel, rff, seg, gil, reg)
                        }
                    } else { // face, hint
                        nest_dext_cached(context, ut_core, sut, tel, slot(rff, 7)?, seg, gil, reg)
                    }
            }
        }
    }

    pub fn fitz(context: &mut Context, yaz: Noun, wix: Noun) -> Result {

        fn fiz(context: &mut Context, mot: Atom) -> Result {
            let len = met(3, mot);
            if len == 0 {
                return Ok(T(&mut context.stack, &[D(0), D(0)]));
            }

            let dec_res = dec(context, D(len as u64).as_atom()?)?.as_atom()?.as_u64()?;
            let tyl = rsh(&mut context.stack, 3, dec_res as usize, mot)?.as_atom()?;

            let is_gte = unsafe {gte(&mut context.stack, tyl, D(tas!(b"A")).as_atom()?).raw_equals(&YES)};
            let is_lte = unsafe {lte(&mut context.stack, tyl, D(tas!(b"Z")).as_atom()?).raw_equals(&YES)};

            if is_gte && is_lte {
                let sub_res = sub(&mut context.stack, tyl, D(64).as_atom()?)?;
                let end_res = end(&mut context.stack, 3, dec_res as usize, mot)?;
                Ok(T(&mut context.stack, &[sub_res.as_noun(), end_res]))
            } else {
                Ok(T(&mut context.stack, &[D(0), mot.as_noun()]))
            }
        }

        let [p_yoz, q_yoz] = fiz(context, yaz.as_atom()?)?.uncell()?;
        let [p_wux, q_wux] = fiz(context, wix.as_atom()?)?.uncell()?;

        if unsafe { ( p_yoz.raw_equals(&D(0))
                    || p_wux.raw_equals(&D(0))
                    || ( !p_wux.raw_equals(&D(0)) &&
                    lte(&mut context.stack, p_wux.as_atom()?, p_yoz.as_atom()?).raw_equals(&YES))
                    )
                && fitz_recursion(context, q_yoz, q_wux)?.raw_equals(&YES)
        } {
            Ok(YES)
        } else {
            Ok(NO)
        }
    }

    fn fitz_recursion(context: &mut Context, q_yoz: Noun, q_wux: Noun) -> Result {
        if unsafe { q_yoz.raw_equals(&D(0))
                   || q_wux.raw_equals(&D(0))
        } {
            return Ok(YES);
        } else {
            let mut end_yoz = end(&mut context.stack, 3, 1, q_yoz.as_atom()?)?;
            let mut end_wux = end(&mut context.stack, 3, 1, q_wux.as_atom()?)?;
            let rsh_yoz = rsh(&mut context.stack, 3, 1, q_yoz.as_atom()?)?;
            let rsh_wux = rsh(&mut context.stack, 3, 1, q_wux.as_atom()?)?;
            let rec_res = fitz_recursion(context, rsh_yoz, rsh_wux)?;
            let bool_res =  if unsafe {
                                unifying_equality(&mut context.stack, &mut end_yoz, &mut end_wux)
                                && rec_res.raw_equals(&YES) } {
                              YES
                            } else {
                                NO
                            };
            return Ok(bool_res);
        }
    }

    pub fn fond(context: &mut Context, mut ut_core: Noun, mut sut: Noun, way: Noun, hyp: Noun) -> Result {

        if unsafe { hyp.raw_equals(&D(0)) } {
            return Ok(T(&mut context.stack, &[YES, D(0), YES, sut]));
        }

        let [hyp_head, hyp_tail] = hyp.uncell()?;  //  [limb (list limb)]
        let mor = fond(context, ut_core, sut, way, hyp_tail)?;
        let [mor_head, mor_tail] = mor.uncell()?;  // [?([%.y palo] [%.n ?([%.y @ud] [%.n [type nock]])])]

        if unsafe { mor_head.raw_equals(&NO) } {
            let [mor_tail_head, mor_tail_tail] = mor_tail.uncell()?;  //  ?([%.y @ud] [%.n [type nock]])

            if unsafe { mor_tail_head.raw_equals(&YES) } {
                return Ok(mor);
            } else {
                let sam = T(&mut context.stack, &[D(tas!(b"noun")), D(tas!(b"wing")), hyp_head, D(0)]);
                let [type_mor, nock_mor] = mor_tail_tail.uncell()?;  //  [%.n %.n type nock]

                ut_core = replace_at_axis(context, ut_core, 6, type_mor)?;
                let fex = call_arm(context, ut_core, 49083, sam)?;  // +mint
                let [p_fex, q_fex] = fex.uncell()?;

                let comb_res = comb(context, nock_mor, q_fex)?;
                Ok(T(&mut context.stack, &[NO, NO, p_fex, comb_res]))
            }
        } else {
            let [vein, opal] = mor_tail.uncell()?;   //  [p=(list (unit axis)) q=?([%.y type] [%.n axis (set [type foot])])]
            let [opal_head, opal_tail] = opal.uncell()?; //  ?([%.y type] [%.n axis (set [type foot])])
            sut = if unsafe { opal_head.raw_equals(&YES) } {
                opal_tail
            } else {
                let [_axis, set] = opal_tail.uncell()?;
                let mut tap = tap_in(context, set, D(0))?;  // (list [type foot])
                let mut type_list = D(0);
                while tap.is_cell() {
                    let [head, tail] = tap.uncell()?;
                    let [p_head, _q_head] = head.uncell()?; //  [type foot]
                    type_list = T(&mut context.stack, &[p_head, type_list]);
                    tap = tail;
                }
                fork(context, type_list)?
            };
            let axe = D(1).as_atom()?;
            let lon = vein;
            let heg = if hyp_head.is_cell() {
                hyp_head
            } else {
                T(&mut context.stack, &[NO, D(0), D(0), hyp_head])
            };

            let [heg_head, heg_tail] = heg.uncell()?;  //  ?([%.y p=axis] [%.n @ud (unit term)])

            if unsafe { heg_head.raw_equals(&YES) } {
                let unit_p_heg = T(&mut context.stack, &[D(0), heg_tail]);
                let vein = T(&mut context.stack, &[unit_p_heg, lon]);

                ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                let peek_res = peek(context, ut_core, sut, way, heg_tail)?;

                Ok(T(&mut context.stack, &[YES, vein, YES, peek_res]))
            } else {
                let [p_heg, q_heg] = heg_tail.uncell()?;  //  [@ud (unit term)]
                fond_buc(context, ut_core, sut, way, hyp, axe, lon, p_heg, q_heg, D(0))
            }
        }
    }

    // pub fn fond_buc(context: &mut Context, mut ut_core: Noun, mut sut: Noun, way: Noun, hyp: Noun, mut axe: Atom, lon: Noun, mut p_heg: Noun, q_heg: Noun, mut gil: Noun) -> Result {

    //     let flag = if let Ok(noun) = slot(ut_core, 59) {
    //         if unsafe { noun.raw_equals(&D(0)) } {
    //             0u64
    //         } else {
    //             1u64
    //         }
    //     } else {
    //         1
    //     };
    //     let fun = 141 + tas!(b"fonds") + (flag << 8);
    //     let mut key = T(&mut context.stack, &[D(fun), sut, way, hyp, axe.as_noun()]);

    //     match context.cache.lookup(&mut context.stack, &mut key) {
    //         Some(pro) => Ok(pro),
    //         None => {
    //             let pro = fond_buc2(context, ut_core, sut, way, hyp, axe, lon, p_heg, q_heg, gil)?;
    //             if unsafe { gil.raw_equals(&D(0)) } {
    //                 context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
    //             }
    //             Ok(pro)
    //         }
    //     }
    // }

    pub fn fond_buc(context: &mut Context, mut ut_core: Noun, mut sut: Noun, way: Noun, hyp: Noun, mut axe: Atom, lon: Noun, mut p_heg: Noun, q_heg: Noun, mut gil: Noun) -> Result {

        match sut.as_either_atom_cell() {
            Left(atom) => {
                if unsafe { atom.as_noun().raw_equals(&D(tas!(b"void"))) } {
                    return Ok(D(0));
                } else if unsafe { atom.as_noun().raw_equals(&D(tas!(b"noun"))) } {
                    return fond_stop(context, sut, axe, lon, p_heg, q_heg);
                } else {
                    return Err(BAIL_EXIT);
                }
            }
            Right(cell) => {
                if unsafe { cell.head().raw_equals(&D(tas!(b"atom"))) } {
                    return fond_stop(context, sut, axe, lon, p_heg, q_heg);
                } else if unsafe { cell.head().raw_equals(&D(tas!(b"cell"))) } {
                    if unsafe { q_heg.raw_equals(&D(0)) } {
                       return fond_here(context, sut, axe, lon, p_heg);
                    }
                    let [_tag, p_sut, q_sut] = sut.uncell()?;
                    let new_axe = peg(context, axe, D(2).as_atom()?)?.as_atom()?;
                    //  ?(~ [?([%.y palo] [%.n ?([%.y @ud] [%.n [type nock]])])])
                    let taf = fond_buc(context, ut_core, p_sut, way, hyp, new_axe, lon, p_heg, q_heg, gil)?;

                    if unsafe { taf.raw_equals(&D(0)) } {
                       return Ok(D(0));
                    }

                    let [taf_head, taf_tail_head, taf_tail_tail] = taf.uncell()?;

                    if unsafe { taf_head.raw_equals(&YES) || taf_tail_head.raw_equals(&NO) } {
                        return Ok(taf);
                    } else {
                        let new_axe = peg(context, axe, D(3).as_atom()?)?.as_atom()?;
                        return fond_buc(context, ut_core, q_sut, way, hyp, new_axe, lon, taf_tail_tail, q_heg, gil);
                    }
                } else if unsafe { cell.head().raw_equals(&D(tas!(b"core"))) } {
                    if unsafe { q_heg.raw_equals(&D(0)) } {
                        return fond_here(context, sut, axe, lon, p_heg);
                    }

                    let [_null, u_q_heg] = q_heg.uncell()?;  //  (unit term)

                    // [%core p=type q=[p=garb q=type r=[p=seminoun q=(map term tome)]]]
                    let [_tag, p_sut, p_q_sut, _q_q_sut, _p_r_q_sut, q_r_q_sut] = sut.uncell()?;

                    let loot_res = loot(context, u_q_heg, q_r_q_sut, D(1).as_atom()?)?;  // (unit [p=axis q=hoon])
                    let zem = if unsafe { loot_res.raw_equals(&D(0)) } {
                        D(0)
                    } else{
                        if unsafe { p_heg.raw_equals(&D(0)) } {
                            loot_res
                        } else {
                            p_heg = dec(context, p_heg.as_atom()?)?;
                            D(0)
                        }
                    };
                    let [_unit, poly, vair] = p_q_sut.uncell()?;

                    if zem.is_cell() {
                        let axe_unit = T(&mut context.stack, &[D(0), axe.as_noun()]);
                        let vein = T(&mut context.stack, &[axe_unit, lon]);
                        let [_null, axis, hoon] = zem.uncell()?;
                        let peg_res = peg(context, D(2).as_atom()?, axis.as_atom()?)?;
                        let zut = if unsafe { poly.raw_equals(&D(tas!(b"wet"))) } {  //   (trel (unit term) poly vair)
                                                T(&mut context.stack, &[D(tas!(b"wet")), hoon])
                                            } else {  // %dry
                                                T(&mut context.stack, &[D(tas!(b"dry")), hoon])
                                            };
                        let type_hoon = T(&mut context.stack, &[sut, zut]);
                        Ok(T(&mut context.stack, &[YES, vein, NO, peg_res, type_hoon, D(0), D(0)]))
                    } else {
                        let [sam_pec, con_pec] = peel(context, way, vair)?.uncell()?;

                        if unsafe { !sam_pec.raw_equals(&YES) } {
                            fond_lose(context, p_heg)
                        } else if unsafe { con_pec.raw_equals(&YES) } {
                            axe = peg(context, axe, D(3).as_atom()?)?.as_atom()?;
                            fond_buc(context, ut_core, p_sut, way, hyp, axe, lon, p_heg, q_heg, gil)
                        } else {
                            ut_core = replace_at_axis(context, ut_core, 6, p_sut)?;
                            sut = peek(context, ut_core, p_sut, way, D(2))?;
                            axe = peg(context, axe, D(6).as_atom()?)?.as_atom()?;
                            fond_buc(context, ut_core, sut, way, hyp, axe, lon, p_heg, q_heg, gil)
                        }
                    }
                } else if unsafe { cell.head().raw_equals(&D(tas!(b"hint"))) } {
                    fond_buc(context, ut_core, slot(sut, 7)?, way, hyp, axe, lon, p_heg, q_heg, gil)
                } else if unsafe { cell.head().raw_equals(&D(tas!(b"face"))) } {
                    let [_sut_head, mut p_sut, q_sut] = sut.uncell()?;

                    if unsafe { q_heg.raw_equals(&D(0)) } {
                        return fond_here(context, q_sut, axe, lon, p_heg);
                    }

                    if p_sut.is_atom() {
                        let [_null, mut q_u_heg] = q_heg.uncell()?;  //  (unit term)
                        if unsafe { unifying_equality(&mut context.stack, &mut p_sut, &mut q_u_heg) } {
                            return fond_here(context, q_sut, axe, lon, p_heg);
                        } else {
                            return fond_lose(context, p_heg);
                        }
                    } else {
                        let zot = p_sut;
                        fond_buc_face_main(context, ut_core, sut, zot, way, hyp, axe, lon, p_heg, q_heg, gil)
                    }
                } else if unsafe { cell.head().raw_equals(&D(tas!(b"fork"))) } {
                    let mut tap = tap_in(context, slot(sut, 3)?, D(0))?;
                    let mut wiz = D(0);
                    while unsafe { !tap.raw_equals(&D(0)) } {
                        let [tap_head, tap_tail] = tap.uncell()?;
                        let res = fond_buc(context, ut_core, tap_head, way, hyp, axe, lon, p_heg, q_heg, gil)?;
                        wiz = T(&mut context.stack, &[res, wiz]);
                        tap = tap_tail;
                    }
                    if unsafe { wiz.raw_equals(&D(0)) } {
                        return Ok(D(0));
                    } else {
                        wiz = flop(&mut context.stack, wiz)?;  //  remove this
                        fond_buc_fork_recursion(context, wiz)
                    }
                }
                else  { // %hold
                    if unsafe { has_in(context, gil, sut)?.raw_equals(&YES) } {
                        return Ok(D(0));
                    } else {
                        gil = put_in(context, gil, sut)?;
                        ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                        let leg = T(&mut context.stack, &[cell.tail(), D(0)]);
                        sut = rest_cached(context, ut_core, leg)?;
                        fond_buc(context, ut_core, sut, way, hyp, axe, lon, p_heg, q_heg, gil)
                    }
                }
            }
        }
    }

    pub fn fond_buc_face_main(context: &mut Context, mut ut_core: Noun, sut: Noun, zot: Noun, way: Noun, hyp: Noun, axe: Atom, lon: Noun, mut p_heg: Noun, q_heg: Noun, gil: Noun) -> Result {
        let [p_zot, _q_zot] = zot.uncell()?;  //  [(map term (unit hoon)) (list hoon)]
        let [_null, q_u_heg] = q_heg.uncell()?;  //  (unit term)  (null case was handled before)

        let tyr = get_by(context, p_zot, q_u_heg)?;

        if  unsafe { tyr.raw_equals(&D(0)) } {
            return Ok(fond_buc_face_next(context, ut_core, sut, zot, way, hyp, axe, lon, p_heg, q_heg, gil)?);
        }

        let [_null, u_tyr] = tyr.uncell()?;  // (unit (unit hoon))

        if  unsafe { u_tyr.raw_equals(&D(0)) } {
            let unit_lon = T(&mut context.stack, &[D(0), lon]);
            p_heg = inc(&mut context.stack, p_heg.as_atom()?).as_noun();
            fond_buc_face_main(context, ut_core, slot(sut, 7)?, zot, way, hyp, axe, unit_lon, p_heg, q_heg, gil)
        }  else if  unsafe { !p_heg.raw_equals(&D(0)) } {
            p_heg = dec(context, p_heg.as_atom()?)?;
            fond_buc_face_next(context, ut_core, sut, zot, way, hyp, axe, lon, p_heg, q_heg, gil)
        } else {
            let [_null, u_u_tyr] = u_tyr.uncell()?;  // (unit hoon)

            ut_core = replace_at_axis(context, ut_core, 6, sut)?;
            let sam = T(&mut context.stack, &[way, u_u_tyr]);
            let [tor_head, tor_tail] = call_arm(context, ut_core, 6014, sam)?.uncell()?; //  +fund : (each palo (pair type nock))

            if  unsafe { tor_head.raw_equals(&YES) } {
                let [mut vein, opal] = tor_tail.uncell()?;
                let unit_axe = T(&mut context.stack, &[D(0), axe.as_noun()]);
                let list = T(&mut context.stack, &[D(0), unit_axe, lon]);
                vein = weld(&mut context.stack, vein, list)?;
                Ok(T(&mut context.stack, &[YES, vein, opal]))
            } else {
                let [tor_type, tor_nock] = tor_tail.uncell()?;
                let zero_axis = T(&mut context.stack, &[D(0), axe.as_noun()]);
                let comb_res = comb(context, zero_axis, tor_nock)?;
                Ok(T(&mut context.stack, &[NO, NO, tor_type, comb_res]))
            }
        }

    }

    pub fn fond_buc_face_next(context: &mut Context, mut ut_core: Noun, sut: Noun, mut zot: Noun, way: Noun, hyp: Noun, axe: Atom, lon: Noun, p_heg: Noun, q_heg: Noun, gil: Noun) -> Result {
        let [p_zot, q_zot] = zot.uncell()?;  //  [(map term (unit hoon)) (list hoon)]

        if  unsafe { q_zot.raw_equals(&D(0)) } {
            let unit_lon = T(&mut context.stack, &[D(0), lon]);
            fond_buc(context, ut_core, slot(sut, 7)?, way, hyp, axe, unit_lon, p_heg, q_heg, gil)
        } else {
            let [i_q_zot, t_q_zot] = q_zot.uncell()?;

            ut_core = replace_at_axis(context, ut_core, 6, sut)?;
            let sam = T(&mut context.stack, &[D(tas!(b"noun")), i_q_zot]);
            let [tiv_head, tiv_tail] = call_arm(context, ut_core, 49083, sam)?.uncell()?;  // +mint [type nock]

            let fid = fond_buc(context, ut_core, tiv_head, way, hyp, D(1).as_atom()?, D(0), p_heg, q_heg, D(0))?;

            if unsafe { fid.raw_equals(&D(0)) } {
                return Ok(D(0));
            } else {
                // [?([%.y palo=[vein opal]] [%.n ?([%.y @ud] [%.n [type nock]])])]
                let [fid_head, fid_tail] = fid.uncell()?;

                if unsafe { fid_head.raw_equals(&NO) && fid_tail.as_cell()?.head().raw_equals(&YES) } {
                    zot = T(&mut context.stack, &[p_zot, t_q_zot]);
                    fond_buc_face_next(context, ut_core, sut, zot, way, hyp, axe, lon, fid_tail.as_cell()?.tail(), q_heg, gil)
                } else {
                    let vat = if unsafe { fid_head.raw_equals(&YES) } {
                        let sam = T(&mut context.stack, &[YES, fid_tail]);
                        ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                        call_arm(context, ut_core, 98173, sam)?  //  +fine
                    } else {
                        let sam = T(&mut context.stack, &[NO, fid_tail.as_cell()?.tail()]);
                        ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                        call_arm(context, ut_core, 98173, sam)?  //  +fine
                    };
                    let [vat_head, vat_tail] = vat.uncell()?;  // [type nock]
                    let zero_axis = T(&mut context.stack, &[D(0), axe.as_noun()]);
                    let comb_res = comb(context, zero_axis, tiv_tail)?;
                    let comb_res_2 = comb(context, comb_res, vat_tail)?;
                    Ok(T(&mut context.stack, &[NO, NO, vat_head, comb_res_2]))
                }
            }
        }

    }

    pub fn fond_buc_fork_recursion(context: &mut Context, wiz: Noun) -> Result {
        let [wiz_head, wiz_tail] = wiz.uncell()?;
        if unsafe { wiz_tail.raw_equals(&D(0)) } {
           return Ok(wiz_head);
        } else {
            let rec = fond_buc_fork_recursion(context, wiz_tail)?;
            fond_twin(context, wiz_head, rec)
        }
    }

    pub fn fond_twin(context: &mut Context, mut hax: Noun, mut yor: Noun) -> Result {

        if unsafe { unifying_equality(&mut context.stack, &mut hax, &mut yor) } {
            return Ok(hax);
        } else if unsafe { hax.raw_equals(&D(0)) } {
            return Ok(yor);
        } else if unsafe { yor.raw_equals(&D(0)) } {
            return Ok(hax);
        }

        let [hax_head, hax_tail] = hax.uncell()?;

        if unsafe { hax_head.raw_equals(&NO) } {
            let [_hax_tail_head, hax_type, hax_nock] = hax_tail.uncell()?;  //  [%.n p=[type nock]]
            let [_yor_head, _yor_tail_head, yor_type, _yor_nock] = yor.uncell()?;

            let fork_arg = T(&mut context.stack, &[hax_type, yor_type, D(0)]);
            let fork_res = fork(context, fork_arg)?;
            Ok(T(&mut context.stack, &[NO, NO, fork_res, hax_nock]))
        } else {
            let [p_p_hax, q_p_hax_head, q_p_hax_tail] = hax_tail.uncell()?;  //  [p=vein q=?([%.y type] [%.n p=axis q=(set [type foot])])]
            let [_yor_head, _p_p_yor, q_p_yor_head, q_p_yor_tail] = yor.uncell()?;

            if unsafe { q_p_hax_head.raw_equals(&YES) && q_p_yor_head.raw_equals(&YES) } {
                let fork_arg = T(&mut context.stack, &[q_p_hax_tail, q_p_yor_tail, D(0)]);
                let fork_res = fork(context, fork_arg)?;
                Ok(T(&mut context.stack, &[YES, p_p_hax, YES, fork_res]))
            } else {
                let [hax_axis, hax_set] = q_p_hax_tail.uncell()?;  // [p=axis q=(set [type foot])])]
                let [_yor_axis, yor_set] = q_p_yor_tail.uncell()?;

                let wal = uni_in(context, hax_set, yor_set)?;
                Ok(T(&mut context.stack, &[YES, p_p_hax, NO, hax_axis, wal]))
            }
        }
    }

    pub fn fond_stop(context: &mut Context, sut: Noun, axe: Atom, lon: Noun, p_heg: Noun, q_heg: Noun) -> Result {
        if unsafe { q_heg.raw_equals(&D(0)) } {
            fond_here(context, sut, axe, lon, p_heg)
        } else {
            fond_lose(context, p_heg)
        }
    }

    pub fn fond_here(context: &mut Context, sut: Noun, axe: Atom, lon: Noun, mut p_heg: Noun) -> Result {
        if unsafe { p_heg.raw_equals(&D(0)) } {
            let axe_limb = T(&mut context.stack, &[D(0), axe.as_noun()]);
            let vein = T(&mut context.stack, &[D(0), axe_limb, lon]);
            Ok(T(&mut context.stack, &[YES, vein, YES, sut]))
        } else {
            p_heg = dec(context, p_heg.as_atom()?)?;
            Ok(T(&mut context.stack, &[NO, YES, p_heg]))
        }
    }

    pub fn fond_lose(context: &mut Context, p_heg: Noun) -> Result {
        Ok(T(&mut context.stack, &[NO, YES, p_heg]))
    }

    pub fn loot(context: &mut Context, cog: Noun, dom: Noun, axe: Atom) -> Result {

        if unsafe { dom.raw_equals(&D(0)) } {
            return Ok(D(0));
        }

        let [n_dom, l_dom, r_dom] = dom.uncell()?;
        let [_key, _term, hoon] = n_dom.uncell()?;

        if unsafe { l_dom.raw_equals(&D(0)) && r_dom.raw_equals(&D(0)) } {
            let lok = look(context, cog, hoon, D(1).as_atom()?)?;
            if  unsafe { lok.raw_equals(&D(0)) } {
                return Ok(D(0));
            }
            let [_null, axis_lok, hoon_lok] = lok.uncell()?;
            let peg_heg = peg(context, axe, axis_lok.as_atom()?)?;
            Ok(T(&mut context.stack, &[D(0), peg_heg, hoon_lok]))
        }  else {
            let yed = look(context, cog, hoon, D(1).as_atom()?)?;
            if unsafe { !yed.raw_equals(&D(0)) } {
                let [_null, axis_yed, hoon_yed] = yed.uncell()?;
                let peg_res =  peg(context, axe, D(2).as_atom()?)?.as_atom()?;
                let peg_res_2 = peg(context, peg_res, axis_yed.as_atom()?)?;
                Ok(T(&mut context.stack, &[D(0), peg_res_2, hoon_yed]))
            } else if unsafe { l_dom.raw_equals(&D(0)) } {
                let peg_res = peg(context, axe, D(3).as_atom()?)?.as_atom()?;
                loot(context, cog, r_dom, peg_res)
            } else if unsafe { r_dom.raw_equals(&D(0)) } {
                let peg_res = peg(context, axe, D(3).as_atom()?)?.as_atom()?;
                loot(context, cog, l_dom, peg_res)
            }  else {
                let peg_res = peg(context, axe, D(6).as_atom()?)?.as_atom()?;
                let pey = loot(context, cog, l_dom, peg_res)?;
                if unsafe { !pey.raw_equals(&D(0)) } {
                    return Ok(pey);
                } else {
                    let peg_res = peg(context, axe, D(7).as_atom()?)?.as_atom()?;
                    loot(context, cog, r_dom, peg_res)
                }
            }
        }
    }

    pub fn look(context: &mut Context, mut cog: Noun, dab: Noun, axe: Atom) -> Result {

        if unsafe { dab.raw_equals(&D(0)) } {
            return Ok(D(0));
        }

        let [n_dab, l_dab, r_dab] = dab.uncell()?;
        let [mut p_n_dab, q_n_dab] = n_dab.uncell()?;

        if unsafe { l_dab.raw_equals(&D(0)) && r_dab.raw_equals(&D(0)) } {
            if unsafe { unifying_equality(&mut context.stack, &mut cog, &mut p_n_dab) } {
                Ok(T(&mut context.stack, &[D(0), axe.as_noun(), q_n_dab]))
            } else {
                Ok(D(0))
            }
        } else if unsafe { l_dab.raw_equals(&D(0)) } {
            if unsafe { unifying_equality(&mut context.stack, &mut cog, &mut p_n_dab) } {
                let peg_axe = peg(context, axe, D(2).as_atom()?)?;
                Ok(T(&mut context.stack, &[D(0), peg_axe, q_n_dab]))
            } else if  unsafe { gor(&mut context.stack, cog, p_n_dab).raw_equals(&YES) } {
                Ok(D(0))
            } else {
                let peg_axe = peg(context, axe, D(3).as_atom()?)?.as_atom()?;
                look(context, cog, r_dab, peg_axe)
            }
        } else if unsafe { r_dab.raw_equals(&D(0)) } {
            if unsafe { unifying_equality(&mut context.stack, &mut cog, &mut p_n_dab) } {
                let peg_axe = peg(context, axe, D(2).as_atom()?)?;
                Ok(T(&mut context.stack, &[D(0), peg_axe, q_n_dab]))
            } else if  unsafe { gor(&mut context.stack, cog, p_n_dab).raw_equals(&YES) } {
                let peg_axe = peg(context, axe, D(3).as_atom()?)?.as_atom()?;
                look(context, cog, l_dab, peg_axe)
            } else {
                Ok(D(0))
            }
        } else {
            if unsafe { unifying_equality(&mut context.stack, &mut cog, &mut p_n_dab)} {
                let peg_axe = peg(context, axe, D(2).as_atom()?)?;
                Ok(T(&mut context.stack, &[D(0), peg_axe, q_n_dab]))
            } else if unsafe { gor(&mut context.stack, cog, p_n_dab).raw_equals(&YES) } {
                let peg_axe = peg(context, axe, D(6).as_atom()?)?.as_atom()?;
                look(context, cog, l_dab, peg_axe)
            } else {
                let peg_axe = peg(context, axe, D(7).as_atom()?)?.as_atom()?;
                look(context, cog, r_dab, peg_axe)
            }
        }
    }

    pub fn rest_cached(context: &mut Context, ut_core: Noun, leg: Noun) -> Result {

        let flag = if let Ok(noun) = slot(ut_core, 59) {
            if unsafe { noun.raw_equals(&D(0)) } {
                0u64
            } else {
                1u64
            }
        } else {
            1
        };
        let fun = 141 + tas!(b"rest") + (flag << 8);
        let mut key = T(&mut context.stack, &[D(fun), slot(ut_core, 6)?, leg]);

        match context.cache.lookup(&mut context.stack, &mut key) {
            Some(pro) => Ok(pro),
            None => {
                let pro = rest(context, ut_core, leg)?;
                context.cache = context.cache.insert(&mut context.stack, &mut key, pro);
                Ok(pro)
            }
        }
    }

    pub fn rest(context: &mut Context, mut ut_core: Noun, leg: Noun) -> Result {
        let mut fan = slot(ut_core, 28)?;
        let mut list = leg;

        while list.is_cell() {
            let [head, tail] = list.uncell()?;

            if unsafe { has_in(context, fan, head)?.raw_equals(&YES) } {
                return Err(BAIL_EXIT);
            }
            list = tail;
        }

        fan = gas_in(context, fan, leg)?;
        ut_core = replace_at_axis(context, ut_core, 28, fan)?;

        let mut list_result = D(0);
        list = leg;

        while list.is_cell() {
            let [head, tail] = list.uncell()?;
            let [type_arg, hoon_arg] = head.uncell()?;
            let new_ut_core = replace_at_axis(context, ut_core, 6, type_arg)?;
            // let play_result = call_arm(context, new_ut_core, 3006, hoon_arg)?;  //  +play
            let play_result = play(context, new_ut_core, type_arg, hoon_arg)?;
            list_result = T(&mut context.stack, &[play_result, list_result]);
            list = tail;
        }

        let gas = gas_in(context, D(0), list_result)?;
        let tap = tap_in(context, gas, D(0))?;

        fork(context, tap)
    }
    // [11 [1.851.876.717 [1 1 1.717.658.988 0] 0 1] 1 q]
    pub fn play_run_sigcab(context: &mut Context, ut_core: Noun, p: Noun, q: Noun) -> Result {
        let y = T(&mut context.stack, &[D(1), D(1), p]);
        let x = T(&mut context.stack, &[D(1851876717), y, D(0), D(1)]);
        let q = call_arm_formula(context, 3006, q)?;
        let fol = T(&mut context.stack, &[D(11), x, D(1), q]);
        Ok(interpret(context, ut_core, fol)?)
    }
// [11 [1.851.876.717 [1 1 3.356.214] 0 1] 8 [9 3.006 0 1] 9 2 10 [6 7 [0 3] 1 b] 0 2]
    // [8 [9 174 0 1] 9 10 10 [6 [7 [0 3] 1 p] 7 [0 3] 1 q] 0 2]
    pub fn call_play_et(context: &mut Context, ut_core: Noun, p: Noun, q: Noun) -> Result {
        let z = T(&mut context.stack, &[D(9), D(174), D(0), D(1)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let k = T(&mut context.stack, &[D(7), y, D(1), p]);
        let x = T(&mut context.stack, &[D(6), k, D(7), y, D(1), q]);

        let fol = T(&mut context.stack, &[D(8), z, D(9), D(10), D(10), x, D(0), D(2)]);

        Ok(interpret(context, ut_core, fol)?)
    }

    pub fn call_put_formula(context: &mut Context, a: Noun, b: Noun) -> Result {
        let z = T(&mut context.stack, &[D(9), D(6102), D(0), D(127)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), a]);
        let call_arm = T(&mut context.stack, &[D(8), z, D(9), D(84), D(10), x, D(0), D(2)]);
        let k = T(&mut context.stack, &[D(6), D(7), y, D(1), b]);
        Ok(T(&mut context.stack, &[D(8), call_arm, D(9), D(2), D(10), k, D(0), D(2)]))
    }

    pub fn call_has_formula(context: &mut Context, a: Noun, b: Noun) -> Result {
        let z = T(&mut context.stack, &[D(9), D(6102), D(0), D(127)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), a]);
        let call_arm = T(&mut context.stack, &[D(8), z, D(9), D(381), D(10), x, D(0), D(2)]);
        let k = T(&mut context.stack, &[D(6), D(7), y, D(1), b]);
        Ok(T(&mut context.stack, &[D(8), call_arm, D(9), D(2), D(10), k, D(0), D(2)]))
    }

    pub fn call_gas_formula(context: &mut Context, a: Noun, b: Noun) -> Result {
        let z = T(&mut context.stack, &[D(9), D(6102), D(0), D(127)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), a]);
        let call_arm = T(&mut context.stack, &[D(8), z, D(9), D(187), D(10), x, D(0), D(2)]);
        let k = T(&mut context.stack, &[D(6), D(7), y, D(1), b]);
        Ok(T(&mut context.stack, &[D(8), call_arm, D(9), D(2), D(10), k, D(0), D(2)]))
    }

    pub fn call_tap_formula(context: &mut Context, a: Noun) -> Result {
        let z = T(&mut context.stack, &[D(9), D(6102), D(0), D(127)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), a]);
        Ok(T(&mut context.stack, &[D(8), z, D(9), D(186), D(10), x, D(0), D(2)]))
    }

    pub fn call_uni_formula(context: &mut Context, a: Noun, b: Noun) -> Result {
        let z = T(&mut context.stack, &[D(9), D(6102), D(0), D(127)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), a]);
        let call_arm = T(&mut context.stack, &[D(8), z, D(9), D(174), D(10), x, D(0), D(2)]);
        let k = T(&mut context.stack, &[D(6), D(7), y, D(1), b]);
        Ok(T(&mut context.stack, &[D(8), call_arm, D(9), D(2), D(10), k, D(0), D(2)]))
    }

    pub fn call_fork_formula(context: &mut Context, sample: Noun) -> Result {
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), sample]);
        let z = T(&mut context.stack, &[D(9), D(1524), D(0), D(15)]);
        Ok(T(&mut context.stack, &[D(8), z, D(9), D(2), D(10), x, D(0), D(2)]))
    }

    pub fn call_scag_formula(context: &mut Context, sample: Noun) -> Result {
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), sample]);
        let z = T(&mut context.stack, &[D(9), D(50061270), D(0), D(127)]);
        Ok(T(&mut context.stack, &[D(8), z, D(9), D(2), D(10), x, D(0), D(2)]))
    }

    pub fn call_slag_formula(context: &mut Context, sample: Noun) -> Result {
        //  [8 [9 arm 0 1] 9 2 10 [6 7 [0 3] 1 sample] 0 2]
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), sample]);
        let z = T(&mut context.stack, &[D(9), D(782174), D(0), D(127)]);
        Ok(T(&mut context.stack, &[D(8), z, D(9), D(2), D(10), x, D(0), D(2)]))
    }
    pub fn call_arm(context: &mut Context, ut_core: Noun, arm: u64, sam: Noun) -> Result {
        let fol = call_arm_formula(context, arm, sam)?;
        Ok(interpret(context, ut_core, fol)?)
    }

    pub fn call_show(context: &mut Context, ut_core: Noun, sample: Noun) -> Result {
     //  [8 [9 188 0 31] 9 2 10 [6 7 [0 3] 1 sample] 0 2]
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), sample]);
        let z = T(&mut context.stack, &[D(9), D(188), D(0), D(31)]);
        Ok(T(&mut context.stack, &[D(8), z, D(9), D(2), D(10), x, D(0), D(2)]))
    }

     pub fn call_example(context: &mut Context, ut_core: Noun, arg: Noun) -> Result {
        // [8 [9 91 0 15] 9 86 10 [6 7 [0 3] 1 1.702.060.386 1.819.047.278] 0 2]
        let z = T(&mut context.stack, &[D(9), D(91), D(0), D(15)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), arg]);
        let fol = T(&mut context.stack, &[D(8), z, D(9), D(86), D(10), x, D(0), D(2)]);
        Ok(interpret(context, ut_core, fol)?)
    }

    pub fn call_factory(context: &mut Context, ut_core: Noun, arg: Noun) -> Result {
        //  [8 [9 91 0 15] 9 383 10 [6 7 [0 3] 1 1.702.060.386 1.819.047.278] 0 2]
        let z = T(&mut context.stack, &[D(9), D(91), D(0), D(15)]);
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), arg]);
        let fol = T(&mut context.stack, &[D(8), z, D(9), D(383), D(10), x, D(0), D(2)]);
        Ok(interpret(context, ut_core, fol)?)
    }

    pub fn call_open(context: &mut Context, ut_core: Noun, gen: Noun) -> Result {
        //  [8 [9 44 0 15] 9 10 10 [6 7 [0 3] 1 sample] 0 2]
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), gen]);
        let z = T(&mut context.stack, &[D(9), D(44), D(0), D(15)]);
        let fol = T(&mut context.stack, &[D(8), z, D(9), D(10), D(10), x, D(0), D(2)]);
        Ok(interpret(context, ut_core, fol)?)
    }

    pub fn call_arm_formula(context: &mut Context, arm: u64, sample: Noun) -> Result {
        //  [8 [9 arm 0 1] 9 2 10 [6 7 [0 3] 1 sample] 0 2]
        let y = T(&mut context.stack, &[D(0), D(3)]);
        let x = T(&mut context.stack, &[D(6), D(7), y, D(1), sample]);
        let z = T(&mut context.stack, &[D(9), D(arm), D(0), D(1)]);
        Ok(T(&mut context.stack, &[D(8), z, D(9), D(2), D(10), x, D(0), D(2)]))
    }

    pub fn peek(context: &mut Context, ut_core: Noun, sut: Noun, way: Noun, axe: Noun) -> Result {
        if unsafe { axe.raw_equals(&D(1)) } {
            return Ok(sut);
        }

        let axe_atom = axe.as_atom()?;
        let now = cap(axe_atom)?;
        let lat = mas(context, axe_atom)?;
        let gil = D(0);

        peek_recursion(context, ut_core, sut, way, axe, now, lat, gil)
    }

    fn peek_recursion(context: &mut Context, mut ut_core: Noun, mut sut: Noun, way: Noun, axe: Noun, now: Noun, lat: Noun, mut gil: Noun) -> Result {

        match sut.as_either_atom_cell() {
            Left(sut_atom) => {
                if unsafe { sut_atom.as_noun().raw_equals(&D(tas!(b"void"))) } {
                    return Ok(D(tas!(b"void")));
                } else if unsafe { sut_atom.as_noun().raw_equals(&D(tas!(b"noun"))) } {
                    return Ok(D(tas!(b"noun")));
                } else {
                    return Err(BAIL_EXIT);
                }
            }
            Right(sut_cell) => {
                if unsafe { sut_cell.head().raw_equals(&D(tas!(b"atom"))) } {

                    return Ok(D(tas!(b"void")));

                } else if unsafe { sut_cell.head().raw_equals(&D(tas!(b"cell"))) } {

                    let [_sut_head, p_sut, q_sut] = sut.uncell()?;
                    if unsafe { now.raw_equals(&D(2)) } {
                        return Ok(peek(context, ut_core, p_sut, way, lat)?);
                    } else {
                        return Ok(peek(context, ut_core, q_sut, way, lat)?);
                    }

                } else if unsafe { sut_cell.head().raw_equals(&D(tas!(b"core"))) } {

                    if unsafe { !now.raw_equals(&D(3)) } {
                        return Ok(D(tas!(b"noun")));
                    }
                    let pec = peel(context, way, slot(sut, 59)?)?;

                    let tow = if unsafe { lat.raw_equals(&D(1)) } {
                                D(1)
                              } else {
                                cap(lat.as_atom()?)?
                              };

                    let [sam, con] = pec.uncell()?;
                    let sam_is_true = unsafe { sam.raw_equals(&YES) };
                    let con_is_true = unsafe { con.raw_equals(&YES) };

                    if  unsafe {
                        ( sam_is_true && con_is_true )
                        ||  ( sam_is_true && tow.raw_equals(&D(2)))
                        ||  ( con_is_true && tow.raw_equals(&D(3)))
                    }   {
                        return Ok(peek(context, ut_core, slot(sut, 6)?, way, lat)?);
                    }
                    else if unsafe { !way.raw_equals(&D(tas!(b"read"))) } {
                       return Err(BAIL_EXIT);
                    } else {
                        let p_cell = if !sam_is_true  { D(tas!(b"noun")) }
                                    else {
                                        peek(context, ut_core, slot(sut, 6)?, way, D(2))?
                                    };
                        let q_cell = if !con_is_true { D(tas!(b"noun")) }
                                    else {
                                        peek(context, ut_core, slot(sut, 6)?, way, D(3))?
                                    };
                        sut = cell(context, p_cell, q_cell)?;
                        return Ok(peek(context, ut_core, sut, way, lat)?);
                    }
                } else if unsafe { sut_cell.head().raw_equals(&D(tas!(b"fork"))) } {
                    let mut tap = tap_in(context, slot(sut, 3)?, D(0))?;
                    let mut list = D(0);
                    while unsafe { !tap.raw_equals(&D(0)) } {
                        let [tap_head, tap_tail] = tap.uncell()?;
                        let res = peek_recursion(context, ut_core, tap_head, way, axe, now, lat, gil)?;
                        list = T(&mut context.stack, &[res, list]);
                        tap = tap_tail;
                    }
                    fork(context, list)

                } else if unsafe { sut_cell.head().raw_equals(&D(tas!(b"hold"))) } {
                        if unsafe { has_in(context, gil, sut)?.raw_equals(&YES) } {
                            return Ok(D(tas!(b"void")));
                        } else {
                            gil = put_in(context, gil, sut)?;
                            let leg = T(&mut context.stack, &[sut_cell.tail(), D(0)]);
                            ut_core = replace_at_axis(context, ut_core, 6, sut)?;
                            sut = rest_cached(context, ut_core, leg)?;
                            peek_recursion(context, ut_core, sut, way, axe, now, lat, gil)
                        }
                } else if unsafe { sut_cell.head().raw_equals(&D(tas!(b"hint")))
                                  || sut_cell.head().raw_equals(&D(tas!(b"face"))) } {
                    peek_recursion(context, ut_core, slot(sut, 7)?, way, axe, now, lat, gil)
                } else {
                    return Err(BAIL_EXIT);
                }
            }
        }
    }

    pub fn peel(context: &mut Context, way: Noun, met: Noun) -> Result {

        if unsafe { met.raw_equals(&D(tas!(b"gold"))) } {
            return Ok(T(&mut context.stack, &[D(0), D(0)]));
        }

        match way.as_atom()?.as_u64()? {
            tas!(b"both") => return Ok(T(&mut context.stack, &[NO, NO])),
            tas!(b"free") => return Ok(T(&mut context.stack, &[YES, YES])),
            tas!(b"read") => {
                let met_is_zinc = if unsafe { met.raw_equals(&D(tas!(b"zinc"))) } { YES } else { NO };
                return Ok(T(&mut context.stack, &[met_is_zinc, NO]));
            },
            tas!(b"rite") => {
                let met_is_iron = if unsafe { met.raw_equals(&D(tas!(b"iron"))) } { YES } else { NO };
                return Ok(T(&mut context.stack, &[met_is_iron, NO]));
            },
            _ => Err(BAIL_EXIT)
        }
    }

    pub fn fork(context: &mut Context, mut yed: Noun) -> Result {
        let mut lez = D(0);

        loop {
            if unsafe { yed.raw_equals(&D(0)) } {
                if unsafe { lez.raw_equals(&D(0)) } {
                    return Ok(D(tas!(b"void")));
                }
                let [n_lez, l_lez, r_lez] = lez.uncell()?;

                if unsafe { l_lez.raw_equals(&D(0)) && r_lez.raw_equals(&D(0)) } {
                    return Ok(n_lez);
                } else {
                    return Ok(T(&mut context.stack, &[D(tas!(b"fork")), lez]));
                }
            } else {
                let [yed_head, yed_tail] = yed.uncell()?;
                yed = yed_tail;

                if  unsafe { yed_head.raw_equals(&D(tas!(b"void"))) } {
                    continue;
                }  else if unsafe { yed_head.is_cell() &&
                           yed_head.as_cell()?.head().raw_equals(&D(tas!(b"fork"))) } {
                    lez = uni_in(context, lez, yed_head.as_cell()?.tail())?;
                }  else {
                    lez = put_in(context, lez, yed_head)?;
                }
            }
        }
    }

    pub fn comb(context: &mut Context, mal: Noun, mut buz: Noun) -> Result {
        let (mal_opcode, p_mal, _q_mal, _r_mal) = parse_formula(mal)?;
        let (buz_opcode, p_buz, q_buz, _r_buz) = parse_formula(buz)?;

        if unsafe { mal_opcode == 0 && !p_mal.unwrap().raw_equals(&D(0)) } {
            if unsafe { buz_opcode == 0 && !p_buz.unwrap().raw_equals(&D(0)) } {
                let peg_res = peg(context, p_mal.unwrap().as_atom()?, p_buz.unwrap().as_atom()?)?;
                return Ok(T(&mut context.stack, &[D(0), peg_res]));
            } else if unsafe {
                buz_opcode == 2 &&
                p_buz.unwrap().as_cell()?.head().raw_equals(&D(0)) &&
                q_buz.unwrap().as_cell()?.head().raw_equals(&D(0))
            } {
                let (_p_buz_opcode, p_p_buz, _q_p_buz, _r_p_buz) = parse_formula(p_buz.unwrap())?;
                let (_q_buz_opcode, p_q_buz, _q_q_buz, _r_q_buz) = parse_formula(q_buz.unwrap())?;
                let peg_p_p = peg(context, p_mal.unwrap().as_atom()?, p_p_buz.unwrap().as_atom()?)?;
                let p_res = T(&mut context.stack, &[D(0), peg_p_p]);
                let peg_p_q = peg(context, p_mal.unwrap().as_atom()?, p_q_buz.unwrap().as_atom()?)?;
                let q_res = T(&mut context.stack, &[D(0), peg_p_q]);
                return Ok(T(&mut context.stack, &[D(2), p_res, q_res]));
            } else {
                return Ok(T(&mut context.stack, &[D(7), mal, buz]));
            }
        }
        let mut zero_one = T(&mut context.stack, &[D(0), D(1)]);
        let mal_cell = mal.as_cell()?;
        if unsafe { mal_cell.head().is_cell() &&
                    unifying_equality(&mut context.stack, &mut zero_one, &mut mal_cell.tail())
            } {
            return Ok(T(&mut context.stack, &[D(8), p_mal.unwrap(), buz]));
        } else if unsafe { unifying_equality(&mut context.stack, &mut zero_one, &mut buz) } {
            return Ok(mal);
        } else {
            return Ok(T(&mut context.stack, &[D(7), mal, buz]));
        }
    }

    pub fn parse_formula(noun: Noun) -> std::result::Result<(u64, Option<Noun>, Option<Noun>, Option<Noun>), JetErr> {

        let [formula_head, formula_tail]  = noun.uncell()?;

        if formula_head.is_cell() {  //  autocons
            return Ok((13, Some(formula_head), Some(formula_tail), None));
        }
        let opcode = formula_head.as_atom()?.as_u64()?;
        match opcode {
            0 | 1 | 3 | 4 => {  // [opcode p]
                Ok((opcode, Some(formula_tail), None, None))
            }
            2 | 5 | 7 | 8 | 9 | 12 => {  // [opcode p q]
                let [p, q] = formula_tail.uncell()?;
                Ok((opcode, Some(p), Some(q), None))
            }
            6 => {  // [6 p q r]
                let [p, q, r] = formula_tail.uncell()?;
                Ok((opcode, Some(p), Some(q), Some(r)))
            }
            10 => {   // [10 [p q] r]
                let [head, r] = formula_tail.uncell()?;
                let [p, q] = head.uncell()?;
                Ok((opcode, Some(p), Some(q), Some(r)))
            }
            11 => {
                let [head, tail] = formula_tail.uncell()?;
                if head.is_atom() {
                    Ok((opcode, Some(head), Some(tail), None)) // [11 p q]
                } else {
                    let [p, q] = head.uncell()?;
                    Ok((opcode, Some(p), Some(q), Some(tail))) // [11 [p q] r]
                }
            }
            _ => Err(BAIL_EXIT)
        }
    }
}
