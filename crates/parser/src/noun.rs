use nockvm_macros::tas;
use parser::ast::hoon::*;
use std::collections::HashMap;
use num_bigint::BigUint;
use nockvm::noun::{D, T, Noun, YES, NO, DirectAtom};
use nockapp::noun::slab::{slab_mug, NounSlab, slab_noun_equality};
use crate::atom::*;
use std::cmp;
use std::cmp::Ordering;
use nockvm::jets::util::slot;
use either::{Left, Right};
use ibig::UBig;
use nockvm::noun::{Atom, DIRECT_MAX};
use nockapp::AtomExt;
use bytes::Bytes;
use num_traits::{Zero, One, Num, FromPrimitive, ToPrimitive};

//
//  AST to Noun(Slab)
//

pub fn hoon_to_noun(slab: &mut NounSlab, hoon: &Hoon) -> Noun {
    use Hoon::*;

    match hoon {
        Pair(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[p, q])
        }
        ZapZap => T(slab, &[D(tas!(b"zpzp")), D(0)]),
        Axis(a) => T(slab, &[D(0), D(*a)]),
        Base(bt) => {
            let bt_noun = basetype_to_noun(slab, bt);
            T(slab, &[D(tas!(b"base")), bt_noun])
        }
        Bust(bt) => {
            let bt_noun = basetype_to_noun(slab, bt);
            T(slab, &[D(tas!(b"bust")), bt_noun])
        }
        Dbug(spot, h) => {
            let spot_noun = spot_to_noun(slab, spot);
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"dbug")), spot_noun, h_noun])
        }
        Eror(msg) => {
            let msg_noun = cord_to_noun(slab, msg);
            T(slab, &[D(tas!(b"eror")), msg_noun])
        }
        Hand(typ, nock) => {
            let typ_noun = type_to_noun(slab, typ);
            let nock_noun = nock_to_noun(slab, nock);
            T(slab, &[D(tas!(b"hand")), typ_noun, nock_noun])
        }
        Note(note, h) => {
            let note_noun = note_to_noun(slab, note);
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"note")), note_noun, h_noun])
        }
        Fits(h, wing) => {
            let h_noun = hoon_to_noun(slab, h);
            let wing_noun = wing_to_noun(slab, wing);
            T(slab, &[D(tas!(b"fits")), h_noun, wing_noun])
        }
        Knit(woofs) => {
            let woofs_noun: Vec<_> = woofs.iter().map(|w| woof_to_noun(slab, w)).collect();
            let list = list_to_noun(slab, woofs_noun);
            T(slab, &[D(tas!(b"knit")), list])
        }
        Leaf(tag, atom) => {
            let tag_noun = term_to_noun(slab, tag);
            let atom_noun = atom_to_noun(slab, atom);
            T(slab, &[D(tas!(b"leaf")), tag_noun, atom_noun])
        }
        Limb(name) => {
            let name_noun = term_to_noun(slab, name);
            T(slab, &[D(tas!(b"limb")), name_noun])
        }
        Lost(h) => {
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"lost")), h_noun])
        }
        Rock(au, expr) => {
            let au_noun = term_to_noun(slab, au);
            let expr_noun = noun_expr_to_noun(slab, expr);
            T(slab, &[D(tas!(b"rock")), au_noun, expr_noun])
        }
        Sand(au, expr) => {
            let au_noun = term_to_noun(slab, au);
            let expr_noun = noun_expr_to_noun(slab, expr);
            T(slab, &[D(tas!(b"sand")), au_noun, expr_noun])
        }
        Tell(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"tell")), list])
        }
        Tune(tune) => {
            let tune_noun = term_or_tune_to_noun(slab, tune);
            T(slab, &[D(tas!(b"tune")), tune_noun])
        }
        Wing(wing) => {
            let wing_noun = wing_to_noun(slab, wing);
            T(slab, &[D(tas!(b"wing")), wing_noun])
        }
        Yell(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"yell")), list])
        }
        Xray(manx) => {
            let manx_noun = manx_to_noun(slab, manx);
            T(slab, &[D(tas!(b"xray")), manx_noun])
        }
        BarBuc(tagnames, spec) => {
            let tags_noun: Vec<_> = tagnames.iter().map(|s| term_to_noun(slab, s)).collect();
            let list = list_to_noun(slab, tags_noun);
            let spec_noun = spec_to_noun(slab, spec);
            T(slab, &[D(tas!(b"brbc")), list, spec_noun])
        }
        BarCab(spec, alas, tomes) => {
            let spec_noun = spec_to_noun(slab, spec);
            let alas_noun = alas_to_noun(slab, alas);

            let mut tomes_pairs = Vec::new();
            for (k, tome) in tomes {
                let k_noun = term_to_noun(slab, k);
                let tome_noun = tome_to_noun(slab, tome);
                tomes_pairs.push((k_noun, tome_noun));
            }
            let tomes_noun = map_to_noun(slab, tomes_pairs);
            T(slab, &[D(tas!(b"brcb")), spec_noun, alas_noun, tomes_noun])
        }
        BarCol(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"brcl")), p, q])
        }
        BarCen(prefix, tomes) => {
            let prefix_noun = prefix.as_ref().map_or_else(|| D(0u64), |s| term_to_noun(slab, s));
            let mut tomes_pairs = Vec::new();
            for (k, tome) in tomes {
                let k_noun = term_to_noun(slab, k);
                let tome_noun = tome_to_noun(slab, tome);
                tomes_pairs.push((k_noun, tome_noun));
            }
            let tomes_noun = map_to_noun(slab, tomes_pairs);
            T(slab, &[D(tas!(b"brcn")), prefix_noun, tomes_noun])
        }
        BarDot(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"brdt")), p])
        }
        BarKet(p, tomes) => {
            let p_noun = hoon_to_noun(slab, p);
            let mut tomes_pairs = Vec::new();
            for (k, tome) in tomes {
                let k_noun = term_to_noun(slab, k);
                let tome_noun = tome_to_noun(slab, tome);
                tomes_pairs.push((k_noun, tome_noun));
            }
            let tomes_noun = map_to_noun(slab, tomes_pairs);
            T(slab, &[D(tas!(b"brkt")), p_noun, tomes_noun])
        }
        BarHep(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"brhp")), p])
        }
        BarSig(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"brsg")), spec_noun, p_noun])
        }
        BarTar(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"brtr")), spec_noun, p_noun])
        }
        BarTis(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"brts")), spec_noun, p_noun])
        }
        BarPat(prefix, tomes) => {
            let prefix_noun = prefix.as_ref().map_or_else(|| D(0u64), |s| term_to_noun(slab, s));
            let mut tomes_pairs = Vec::new();
            for (k, tome) in tomes {
                let k_noun = term_to_noun(slab, k);
                let tome_noun = tome_to_noun(slab, tome);
                tomes_pairs.push((k_noun, tome_noun));
            }
            let tomes_noun = map_to_noun(slab, tomes_pairs);
            T(slab, &[D(tas!(b"brpt")), prefix_noun, tomes_noun])
        }
        BarWut(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"brwt")), p])
        }
        ColCab(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"clcb")), p, q])
        }
        ColKet(a, b, c, d) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            let d = hoon_to_noun(slab, d);
            T(slab, &[D(tas!(b"clkt")), a, b, c, d])
        }
        ColHep(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"clhp")), p, q])
        }
        ColLus(a, b, c) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"clls")), a, b, c])
        }
        ColSig(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"clsg")), list])
        }
        ColTar(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"cltr")), list])
        }
        CenCab(wing, pairs) => {
            let wing_noun = wing_to_noun(slab, wing);
            let pairs_noun: Vec<_> = pairs
                .iter()
                .map(|(w, h)| {
                    let w_noun = wing_to_noun(slab, w);
                    let h_noun = hoon_to_noun(slab, h);
                    T(slab, &[w_noun, h_noun])
                })
                .collect();
            let list = list_to_noun(slab, pairs_noun);
            T(slab, &[D(tas!(b"cncb")), wing_noun, list])
        }
        CenDot(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"cndt")), p, q])
        }
        CenHep(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"cnhp")), p, q])
        }
        CenCol(p, hoons) => {
            let p = hoon_to_noun(slab, p);
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"cncl")), p, list])
        }
        CenTar(wing, p, pairs) => {
            let wing_noun = wing_to_noun(slab, wing);
            let p_noun = hoon_to_noun(slab, p);
            let pairs_noun: Vec<_> = pairs
                .iter()
                .map(|(w, h)| {
                    let w_noun = wing_to_noun(slab, w);
                    let h_noun = hoon_to_noun(slab, h);
                    T(slab, &[w_noun, h_noun])
                })
                .collect();
            let list = list_to_noun(slab, pairs_noun);
            T(slab, &[D(tas!(b"cntr")), wing_noun, p_noun, list])
        }
        CenKet(a, b, c, d) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            let d = hoon_to_noun(slab, d);
            T(slab, &[D(tas!(b"cnkt")), a, b, c, d])
        }
        CenLus(a, b, c) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"cnls")), a, b, c])
        }
        CenSig(wing, p, hoons) => {
            let wing_noun = wing_to_noun(slab, wing);
            let p_noun = hoon_to_noun(slab, p);
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"cnsg")), wing_noun, p_noun, list])
        }
        CenTis(wing, pairs) => {
            let wing_noun = wing_to_noun(slab, wing);
            let pairs_noun: Vec<_> = pairs
                .iter()
                .map(|(w, h)| {
                    let w_noun = wing_to_noun(slab, w);
                    let h_noun = hoon_to_noun(slab, h);
                    T(slab, &[w_noun, h_noun])
                })
                .collect();
            let list = list_to_noun(slab, pairs_noun);
            T(slab, &[D(tas!(b"cnts")), wing_noun, list])
        }
        DotKet(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"dtkt")), spec_noun, p_noun])
        }
        DotLus(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"dtls")), p])
        }
        DotTar(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"dttr")), p, q])
        }
        DotTis(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"dtts")), p, q])
        }
        DotWut(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"dtwt")), p])
        }
        KetBar(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"ktbr")), p])
        }
        KetDot(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"ktdt")), p, q])
        }
        KetLus(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"ktls")), p, q])
        }
        KetHep(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"kthp")), spec_noun, p_noun])
        }
        KetPam(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"ktpm")), p])
        }
        KetSig(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"ktsg")), p])
        }
        KetTis(skin, p) => {
            let skin_noun = skin_to_noun(slab, skin);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"ktts")), skin_noun, p_noun])
        }
        KetWut(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"ktwt")), p])
        }
        KetTar(spec) => {
            let spec_noun = spec_to_noun(slab, spec);
            T(slab, &[D(tas!(b"kttr")), spec_noun])
        }
        KetCol(spec) => {
            let spec_noun = spec_to_noun(slab, spec);
            T(slab, &[D(tas!(b"ktcl")), spec_noun])
        }
        SigBar(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"sgbr")), p, q])
        }
        SigCab(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"sgcb")), p, q])
        }
        SigCen(chum, p, tyre, q) => {
            let chum_noun = chum_to_noun(slab, chum);
            let p_noun = hoon_to_noun(slab, p);
            let tyre_noun = tyre_to_noun(slab, tyre);
            let q_noun = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"sgcn")), chum_noun, p_noun, tyre_noun, q_noun])
        }
        SigFas(chum, p) => {
            let chum_noun = chum_to_noun(slab, chum);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"sgfs")), chum_noun, p_noun])
        }
        SigGal(term_or_pair, p) => {
            let term_noun = term_or_pair_to_noun(slab, term_or_pair);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"sggl")), term_noun, p_noun])
        }
        SigGar(term_or_pair, p) => {
            let term_noun = term_or_pair_to_noun(slab, term_or_pair);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"sggr")), term_noun, p_noun])
        }
        SigBuc(tag, p) => {
            let tag_noun = term_to_noun(slab, tag);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"sbbc")), tag_noun, p_noun])
        }
        SigLus(n, p) => {
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"sgls")), D(*n), p_noun])
        }
        SigPam(n, p, q) => {
            let p_noun = hoon_to_noun(slab, p);
            let q_noun = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"sgpm")), D(*n), p_noun, q_noun])
        }
        SigTis(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"sgts")), p, q])
        }
        SigWut(n, a, b, c) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"sgwt")), D(*n), a, b, c])
        }
        SigZap(p, q) => {
            let p = hoon_to_noun(slab, p);
            let q = hoon_to_noun(slab, q);
            T(slab, &[D(tas!(b"sgzp")), p, q])
        }
        MicTis(marl) => {
            let marl_noun = marl_to_noun(slab, marl);
            T(slab, &[D(tas!(b"mcts")), marl_noun])
        }
        MicCol(p, hoons) => {
            let p = hoon_to_noun(slab, p);
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"mccl")), p, list])
        }
        MicFas(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"mcfs")), p])
        }
        MicGal(spec, a, b, c) => {
            let spec_noun = spec_to_noun(slab, spec);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"mcgl")), spec_noun, a, b, c])
        }
        MicSig(p, hoons) => {
            let p = hoon_to_noun(slab, p);
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"mcsg")), p, list])
        }
        MicMic(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"mcmc")), spec_noun, p_noun])
        }
        TisBar(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"tsbr")), spec_noun, p_noun])
        }
        TisCol(pairs, p) => {
            let pairs_noun: Vec<_> = pairs
                .iter()
                .map(|(w, h)| {
                    let w_noun = wing_to_noun(slab, w);
                    let h_noun = hoon_to_noun(slab, h);
                    T(slab, &[w_noun, h_noun])
                })
                .collect();
            let list = list_to_noun(slab, pairs_noun);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"tscl")), list, p_noun])
        }
        TisFas(skin, a, b) => {
            let skin_noun = skin_to_noun(slab, skin);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tsfs")), skin_noun, a, b])
        }
        TisMic(skin, a, b) => {
            let skin_noun = skin_to_noun(slab, skin);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tsmc")), skin_noun, a, b])
        }
        TisDot(wing, a, b) => {
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tsdt")), wing_noun, a, b])
        }
        TisWut(wing, a, b, c) => {
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"tswt")), wing_noun, a, b, c])
        }
        TisGal(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tsgl")), a, b])
        }
        TisHep(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tshp")), a, b])
        }
        TisGar(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tsgr")), a, b])
        }
        TisKet(skin, wing, a, b) => {
            let skin_noun = skin_to_noun(slab, skin);
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tskt")), skin_noun, wing_noun, a, b])
        }
        TisLus(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tsls")), a, b])
        }
        TisSig(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"tssg")), list])
        }
        TisTar((name, spec_opt), a, b) => {
            let name_noun = term_to_noun(slab, name);
            let spec_noun = spec_opt.as_ref().map_or_else(
                || D(0u64),
                |s| spec_to_noun(slab, s),
            );
            let name_spec = T(slab, &[name_noun, spec_noun]);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tstr")), name_spec, a, b])
        }
        TisCom(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"tscm")), a, b])
        }
        WutBar(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"wtbr")), list])
        }
        WutHep(wing, pairs) => {
            let wing_noun = wing_to_noun(slab, wing);
            let pairs_noun: Vec<_> = pairs
                .iter()
                .map(|(spec, h)| {
                    let spec_noun = spec_to_noun(slab, spec);
                    let h_noun = hoon_to_noun(slab, h);
                    T(slab, &[spec_noun, h_noun])
                })
                .collect();
            let list = list_to_noun(slab, pairs_noun);
            T(slab, &[D(tas!(b"wthp")), wing_noun, list])
        }
        WutCol(a, b, c) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"wtcl")), a, b, c])
        }
        WutDot(a, b, c) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            let c = hoon_to_noun(slab, c);
            T(slab, &[D(tas!(b"wtdt")), a, b, c])
        }
        WutKet(wing, a, b) => {
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"wtkt")), wing_noun, a, b])
        }
        WutGal(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"wtgl")), a, b])
        }
        WutGar(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"wtgr")), a, b])
        }
        WutLus(wing, a, pairs) => {
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let pairs_noun: Vec<_> = pairs
                .iter()
                .map(|(spec, h)| {
                    let spec_noun = spec_to_noun(slab, spec);
                    let h_noun = hoon_to_noun(slab, h);
                    T(slab, &[spec_noun, h_noun])
                })
                .collect();
            let list = list_to_noun(slab, pairs_noun);
            T(slab, &[D(tas!(b"wtls")), wing_noun, a, list])
        }
        WutPam(hoons) => {
            let hoons_noun: Vec<_> = hoons.iter().map(|h| hoon_to_noun(slab, h)).collect();
            let list = list_to_noun(slab, hoons_noun);
            T(slab, &[D(tas!(b"wtpm")), list])
        }
        WutPat(wing, a, b) => {
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"wtpt")), wing_noun, a, b])
        }
        WutSig(wing, a, b) => {
            let wing_noun = wing_to_noun(slab, wing);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"wtsg")), wing_noun, a, b])
        }
        WutHax(skin, wing) => {
            let skin_noun = skin_to_noun(slab, skin);
            let wing_noun = wing_to_noun(slab, wing);
            T(slab, &[D(tas!(b"wthx")), skin_noun, wing_noun])
        }
        WutTis(spec, wing) => {
            let spec_noun = spec_to_noun(slab, spec);
            let wing_noun = wing_to_noun(slab, wing);
            T(slab, &[D(tas!(b"wtts")), spec_noun, wing_noun])
        }
        WutZap(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"wtzp")), p])
        }
        ZapCom(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"zpcm")), a, b])
        }
        ZapGar(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"zpgr")), p])
        }
        ZapGal(spec, p) => {
            let spec_noun = spec_to_noun(slab, spec);
            let p_noun = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"zpgl")), spec_noun, p_noun])
        }
        ZapMic(a, b) => {
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"zpmc")), a, b])
        }
        ZapTis(p) => {
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"zpts")), p])
        }
        ZapPat(wings, a, b) => {
            let wing_nouns: Vec<_> = wings.iter().map(|w| wing_to_noun(slab, w)).collect();
            let wings_noun = list_to_noun(slab, wing_nouns);
            let a = hoon_to_noun(slab, a);
            let b = hoon_to_noun(slab, b);
            T(slab, &[D(tas!(b"zppt")), wings_noun, a, b])
        }
        ZapWut(arg, p) => {
            let arg_noun = zpwt_arg_to_noun(slab, arg);
            let p = hoon_to_noun(slab, p);
            T(slab, &[D(tas!(b"zpwt")), arg_noun, p])
        }
    }
}

fn list_to_noun(slab: &mut NounSlab, nouns: Vec<Noun>) -> Noun {
    nouns.into_iter()
        .rev()
        .fold(D(0u64), |tail, head| T(slab, &[head, tail]))
}

fn map_to_noun(slab: &mut NounSlab, pairs: Vec<(Noun, Noun)>) -> Noun {
    let mut map = D(0);

    for (key, val) in pairs {
        map = put_into_map(slab, map, key, val);
    }

    map
}

fn put_into_map(slab: &mut NounSlab, map: Noun, key: Noun, val: Noun) -> Noun {
    if map.is_atom() && slab_noun_equality(&map, &D(0)) {
        let node = T(slab, &[key, val]);
        return T(slab, &[node, D(0), D(0)]);
    }

    let cell = map.as_cell().expect("non-empty map must be a cell");
    let node = cell.head();
    let children = cell.tail();
    let children_cell = children.as_cell().expect("children must be a cell");
    let left = children_cell.head();
    let right = children_cell.tail();

    let node_cell = node.as_cell().expect("node must be [key value]");
    let node_key = node_cell.head();
    let node_val = node_cell.tail();

    if slab_noun_equality(&key, &node_key) {
        if slab_noun_equality(&val, &node_val) {
            return map;
        } else {
            let new_node = T(slab, &[key, val]);
            return T(slab, &[new_node, left, right]);
        }
    }

    if unsafe { gor_slab(key, node_key).raw_equals(&YES) } {
        let new_left = put_into_map(slab, left, key, val);

        if new_left.is_atom() {
            if !slab_noun_equality(&new_left, &D(0)) {
                panic!("put returned unexpected atom");
            }
        }

        let new_left_cell = new_left.as_cell().expect("new_left must be cell after insert");
        let new_left_node = new_left_cell.head();
        let new_left_node_key = new_left_node.as_cell().expect("node must be [k v]").head();

        if unsafe { mor_slab(node_key, new_left_node_key).raw_equals(&YES) } {
            T(slab, &[node, new_left, right])
        } else {
            let new_left_children = new_left_cell.tail();
            let new_left_children_cell = new_left_children.as_cell().expect("children cell");
            let new_left_right = new_left_children_cell.tail();

            let new_right = T(slab, &[node, new_left_right, right]);
            T(slab, &[
                new_left_node,
                new_left_children_cell.head(),
                new_right,
            ])
        }
    } else {
        let new_right = put_into_map(slab, right, key, val);

        if new_right.is_atom() {
            if !slab_noun_equality(&new_right, &D(0)) {
                panic!("unexpected atom in new_right");
            }
        }

        let new_right_cell = new_right.as_cell().expect("new_right must be cell");
        let new_right_node = new_right_cell.head();
        let new_right_node_key = new_right_node.as_cell().expect("node must be [k v]").head();

        if unsafe { mor_slab(node_key, new_right_node_key).raw_equals(&YES) } {
            T(slab, &[node, left, new_right])
        } else {
            let new_right_children = new_right_cell.tail();
            let new_right_children_cell = new_right_children.as_cell().expect("children cell");
            let new_right_left = new_right_children_cell.head();

            let new_left = T(slab, &[node, left, new_right_left]);
            T(slab, &[
                new_right_node,
                new_left,
                new_right_children_cell.tail(),
            ])
        }
    }
}

fn term_to_noun(slab: &mut NounSlab, s: &str) -> Noun {
    let atom = term_to_atom(s.to_string());
    atom_to_noun(slab, &atom)
}

fn cord_to_noun(slab: &mut NounSlab, s: &str) -> Noun {
    let atom = string_to_atom(s.to_string());
    atom_to_noun(slab, &atom)
}

fn atom_to_noun(slab: &mut NounSlab, atom: &ParsedAtom) -> Noun {
    match atom {
        ParsedAtom::Small(n) => {
            if *n <= DIRECT_MAX as u128 {
                D(*n as u64)
            } else {
                let bytes = n.to_le_bytes();
                let trimmed_len = bytes.iter().rev().take_while(|&&b| b == 0).count();
                let trimmed = &bytes[..bytes.len() - trimmed_len];
                let bytes_slice = if trimmed.is_empty() { &[0u8] } else { trimmed };
                let bytes = Bytes::copy_from_slice(bytes_slice);
                Atom::from_bytes(slab, &bytes).as_noun()
            }
        }
        ParsedAtom::Big(b) => {
            let ubig: UBig = UBig::from_le_bytes(b.to_bytes_le().as_slice());
            Atom::from_ubig(slab, &ubig).as_noun()
        }
    }
}


fn opt_to_noun<T, F>(slab: &mut NounSlab, opt: &Option<T>, f: F) -> Noun
where
    F: FnOnce(&T) -> Noun,
{
    match opt {
        None => D(0u64),
        Some(x) => {
            let x_noun = f(x);
            T(slab, &[D(0u64), x_noun])
        }
    }
}

fn basetype_to_noun(slab: &mut NounSlab, bt: &BaseType) -> Noun {
    match bt {
        BaseType::NounExpr => D(tas!(b"noun")),
        BaseType::Cell => D(tas!(b"cell")),
        BaseType::Flag => D(tas!(b"flag")),
        BaseType::Null => D(tas!(b"null")),
        BaseType::Void => D(tas!(b"void")),
        BaseType::Atom(au) => {
            let at = term_to_noun(slab, au);
            T(slab, &[D(tas!(b"atom")), at])
        },
    }
}

fn noun_expr_to_noun(slab: &mut NounSlab, expr: &NounExpr) -> Noun {
    match expr {
        NounExpr::ParsedAtom(a) => atom_to_noun(slab, a),
        NounExpr::Cell(l, r) => {
            let l_noun = noun_expr_to_noun(slab, l);
            let r_noun = noun_expr_to_noun(slab, r);
            T(slab, &[l_noun, r_noun])
        }
    }
}

fn type_to_noun(slab: &mut NounSlab, typ: &Type) -> Noun {
    use Type::*;
    match typ {
        NounExpr => D(tas!(b"noun")),
        Void => D(tas!(b"void")),
        ParsedAtom(au, bits) => {
            let au_noun = term_to_noun(slab, au);
            let bits_noun = opt_to_noun(slab, bits, |n| D(*n));
            T(slab, &[D(tas!(b"atom")), au_noun, bits_noun])
        }
        Cell(l, r) => {
            let l = type_to_noun(slab, l);
            let r = type_to_noun(slab, r);
            T(slab, &[D(tas!(b"cell")), l, r])
        }
        Core(face, coil) => {
            let face_noun = type_to_noun(slab, face);
            let coil_noun = coil_to_noun(slab, coil);
            T(slab, &[D(tas!(b"core")), face_noun, coil_noun])
        }
        Face(face_type, inner) => {
            let face_noun = face_type_to_noun(slab, face_type);
            let inner_noun = type_to_noun(slab, inner);
            T(slab, &[D(tas!(b"face")), face_noun, inner_noun])
        }
        Fork(types) => {
            let types_vec: Vec<_> = types.iter().map(|t| type_to_noun(slab, t)).collect();
            let types_noun = list_to_noun(slab, types_vec);
            T(slab, &[D(tas!(b"fork")), types_noun])
        }
        Hint((inner, note), payload) => {
            let inner_noun = type_to_noun(slab, inner);
            let note_noun = note_to_noun(slab, note);
            let payload_noun = type_to_noun(slab, payload);
            let hint_inner = T(slab, &[inner_noun, note_noun]);
            T(slab, &[D(tas!(b"hint")), hint_inner, payload_noun])
        }
        Hold(typ, hoon) => {
            let typ_noun = type_to_noun(slab, typ);
            let hoon_noun = hoon_to_noun(slab, hoon);
            T(slab, &[D(tas!(b"hold")), typ_noun, hoon_noun])
        }
    }
}

fn face_type_to_noun(slab: &mut NounSlab, ft: &FaceType) -> Noun {
    match ft {
        FaceType::Term(s) => term_to_noun(slab, s),
        FaceType::Tune(tune) => {
            let tune_noun = tune_to_noun(slab, tune);
            T(slab, &[D(tas!(b"tune")), tune_noun])
        }
    }
}

fn coil_to_noun(slab: &mut NounSlab, coil: &Coil) -> Noun {
    let garb_noun = garb_to_noun(slab, &coil.p);
    let type_noun = type_to_noun(slab, &coil.q);
    let semi_noun = semi_noun_expr_to_noun(slab, &coil.r.0);

    let tomes_entries: Vec<_> = coil.r.1.iter().map(|(k, v)| {
        let (_what, v) = v;
        let k_noun = term_to_noun(slab, k);
        let inner_entries: Vec<_> = v.iter().map(|(kk, vv)| {
            (term_to_noun(slab, kk), hoon_to_noun(slab, vv))
        }).collect();
        let v_noun = map_to_noun(slab, inner_entries);
        (k_noun, T(slab, &[D(0), v_noun]))
    }).collect();

    let tomes_noun = map_to_noun(slab, tomes_entries);
    T(slab, &[garb_noun, type_noun, semi_noun, tomes_noun])
}

fn garb_to_noun(slab: &mut NounSlab, garb: &Garb) -> Noun {
    let name_noun = {
        if let Some(s) = &garb.name {
            term_to_noun(slab, s)
        } else {
            D(0)
        }
    };
    let poly_noun = poly_to_noun(slab, &garb.poly);
    let vair_noun = vair_to_noun(slab, &garb.vair);
    T(slab, &[name_noun, poly_noun, vair_noun])
}

fn poly_to_noun(_slab: &mut NounSlab, poly: &Poly) -> Noun {
    match poly {
        Poly::Wet => D(tas!(b"wet")),
        Poly::Dry => D(tas!(b"dry")),
    }
}

fn vair_to_noun(_slab: &mut NounSlab, vair: &Vair) -> Noun {
    match vair {
        Vair::Gold => D(tas!(b"gold")),
        Vair::Iron => D(tas!(b"iron")),
        Vair::Lead => D(tas!(b"lead")),
        Vair::Zinc => D(tas!(b"zinc")),
    }
}

fn semi_noun_expr_to_noun(slab: &mut NounSlab, (stencil, expr): &SemiNounExpr) -> Noun {
    let stencil_noun = stencil_to_noun(slab, stencil);
    let expr_noun = noun_expr_to_noun(slab, expr);
    T(slab, &[stencil_noun, expr_noun])
}

fn stencil_to_noun(slab: &mut NounSlab, st: &Stencil) -> Noun {
    match st {
        Stencil::Half { left, rite } => {
            let l = stencil_to_noun(slab, left);
            let r = stencil_to_noun(slab, rite);
            T(slab, &[D(tas!(b"half")), l, r])
        }
        Stencil::Full { blocks } => {
            let blocks_vec: Vec<_> = blocks.iter().map(|b| block_to_noun(slab, b)).collect();
            let blocks_noun = list_to_noun(slab, blocks_vec);
            T(slab, &[D(tas!(b"full")), blocks_noun])
        }
        Stencil::Lazy { fragment, resolve } => {
            let gate_noun = gate_to_noun(slab, resolve);
            T(slab, &[D(tas!(b"lazy")), D(*fragment), gate_noun])
        }
    }
}

fn block_to_noun(slab: &mut NounSlab, block: &Block) -> Noun {
    let paths: Vec<_> = block.iter().map(|path| path_to_noun(slab, path)).collect();
    list_to_noun(slab, paths)
}

fn path_to_noun(slab: &mut NounSlab, path: &Path) -> Noun {
    let knots: Vec<_> = path.iter().map(|k| cord_to_noun(slab, k)).collect();
    list_to_noun(slab, knots)
}

fn gate_to_noun(slab: &mut NounSlab, (spec, body): &Gate) -> Noun {
    let spec_noun = spec_to_noun(slab, spec);
    let body_noun = spec_to_noun(slab, body);
    T(slab, &[spec_noun, body_noun])
}

fn spec_to_noun(slab: &mut NounSlab, spec: &Spec) -> Noun {
    use Spec::*;
    match spec {
        Base(bt) => {
            let bt_noun = basetype_to_noun(slab, bt);
            T(slab, &[D(tas!(b"base")), bt_noun])
        }
        Dbug(spot, s) => {
            let spot_noun = spot_to_noun(slab, spot);
            let s_noun = spec_to_noun(slab, s);
            T(slab, &[D(tas!(b"dbug")), spot_noun, s_noun])
        }
        Leaf(tag, atom) => {
            let tag_noun = term_to_noun(slab, tag);
            let atom_noun = atom_to_noun(slab, atom);
            T(slab, &[D(tas!(b"leaf")), tag_noun, atom_noun])
        }
        Like(wing, wings) => {
            let wing_noun = wing_to_noun(slab, wing);
            let wings_vec: Vec<_> = wings.iter().map(|w| wing_to_noun(slab, w)).collect();
            let wings_noun = list_to_noun(slab, wings_vec);
            T(slab, &[D(tas!(b"like")), wing_noun, wings_noun])
        }
        Loop(name) => {
            let name_noun = term_to_noun(slab, name);
            T(slab, &[D(tas!(b"loop")), name_noun])
        }
        Made((name, args), s) => {
            let name_noun = term_to_noun(slab, name);
            let args_vec: Vec<_> = args.iter().map(|a| term_to_noun(slab, a)).collect();
            let args_noun = list_to_noun(slab, args_vec);
            let s_noun = spec_to_noun(slab, s);
            let inner = T(slab, &[name_noun, args_noun]);
            T(slab, &[D(tas!(b"made")), inner, s_noun])
        }
        Make(hoon, specs) => {
            let hoon_noun = hoon_to_noun(slab, hoon);
            let specs_vec: Vec<_> = specs.iter().map(|s| spec_to_noun(slab, s)).collect();
            let specs_noun = list_to_noun(slab, specs_vec);
            T(slab, &[D(tas!(b"make")), hoon_noun, specs_noun])
        }
        Name(name, s) => {
            let name_noun = term_to_noun(slab, name);
            let s_noun = spec_to_noun(slab, s);
            T(slab, &[D(tas!(b"name")), name_noun, s_noun])
        }
        Over(wing, s) => {
            let wing_noun = wing_to_noun(slab, wing);
            let s_noun = spec_to_noun(slab, s);
            T(slab, &[D(tas!(b"over")), wing_noun, s_noun])
        }
        BucGar(a, b) => {
            let a_noun = spec_to_noun(slab, a);
            let b_noun = spec_to_noun(slab, b);
            T(slab, &[D(tas!(b"bcgr")), a_noun, b_noun])
        }
        BucBuc(a, map) => {
            let a_noun = spec_to_noun(slab, a);
            let entries: Vec<_> = map.iter().map(|(k, v)| {
                (term_to_noun(slab, k), spec_to_noun(slab, v))
            }).collect();
            let map_noun = map_to_noun(slab, entries);
            T(slab, &[D(tas!(b"bcbc")), a_noun, map_noun])
        }
        BucBar(a, h) => {
            let a_noun = spec_to_noun(slab, a);
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"bcbr")), a_noun, h_noun])
        },
        BucCab(h) => {
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"bccb")), h_noun])
        }
        BucCol(a, specs) => {
            let a_noun = spec_to_noun(slab, a);
            let specs_vec: Vec<_> = specs.iter().map(|s| spec_to_noun(slab, s)).collect();
            let specs_noun = list_to_noun(slab, specs_vec);
            T(slab, &[D(tas!(b"bccl")), a_noun, specs_noun])
        }
        BucCen(a, specs) => {
            let a_noun = spec_to_noun(slab, a);
            let specs_vec: Vec<_> = specs.iter().map(|s| spec_to_noun(slab, s)).collect();
            let specs_noun = list_to_noun(slab, specs_vec);
            T(slab, &[D(tas!(b"bccn")), a_noun, specs_noun])
        }
        BucDot(a, map) => {
            let a_noun = spec_to_noun(slab, a);
            let entries: Vec<_> = map.iter().map(|(k, v)| {
                (term_to_noun(slab, k), spec_to_noun(slab, v))
            }).collect();
            let map_noun = map_to_noun(slab, entries);
            T(slab, &[D(tas!(b"bcdt")), a_noun, map_noun])
        }
        BucGal(a, b) => {
            let a_noun = spec_to_noun(slab, a);
            let b_noun = spec_to_noun(slab, b);
            T(slab, &[D(tas!(b"bcgl")), a_noun, b_noun])
        }
        BucHep(a, b) => {
            let a_noun = spec_to_noun(slab, a);
            let b_noun = spec_to_noun(slab, b);
            T(slab, &[D(tas!(b"bchp")), a_noun, b_noun])
        }
        BucKet(a, b) => {
            let a_noun = spec_to_noun(slab, a);
            let b_noun = spec_to_noun(slab, b);
            T(slab, &[D(tas!(b"bckt")), a_noun, b_noun])
        }
        BucLus(tag, s) => {
            let tag_noun = term_to_noun(slab, tag);
            let s_noun = spec_to_noun(slab, s);
            T(slab, &[D(tas!(b"bcls")), tag_noun, s_noun])
        },
        BucFas(a, map) => {
            let a_noun = spec_to_noun(slab, a);
            let entries: Vec<_> = map.iter().map(|(k, v)| {
                (term_to_noun(slab, k), spec_to_noun(slab, v))
            }).collect();
            let map_noun = map_to_noun(slab, entries);
            T(slab, &[D(tas!(b"bcfs")), a_noun, map_noun])
        }
        BucMic(h) => {
            let inner = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"bcmc")), inner])
        }
        BucPam(a, h) => {
            let a_noun = spec_to_noun(slab, a);
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"bcpm")), a_noun, h_noun])
        },
        BucSig(h, a) => {
            let h_noun = hoon_to_noun(slab, h);
            let a_noun = spec_to_noun(slab, a);
            T(slab, &[D(tas!(b"bcsg")), h_noun, a_noun])
        },
        BucTic(a, map) => {
            let a_noun = spec_to_noun(slab, a);
            let entries: Vec<_> = map.iter().map(|(k, v)| {
                (term_to_noun(slab, k), spec_to_noun(slab, v))
            }).collect();
            let map_noun = map_to_noun(slab, entries);
            T(slab, &[D(tas!(b"bctc")), a_noun, map_noun])
        }
        BucTis(skin, a) => {
            let skin_noun = skin_to_noun(slab, skin);
            let a_noun = spec_to_noun(slab, a);
            T(slab, &[D(tas!(b"bcts")), skin_noun, a_noun])
        },
        BucPat(a, b) => {
            let a_noun = spec_to_noun(slab, a);
            let b_noun = spec_to_noun(slab, b);
            T(slab, &[D(tas!(b"bcpt")), a_noun, b_noun])
        }
        BucWut(a, specs) => {
            let a_noun = spec_to_noun(slab, a);
            let specs_vec: Vec<_> = specs.iter().map(|s| spec_to_noun(slab, s)).collect();
            let specs_noun = list_to_noun(slab, specs_vec);
            T(slab, &[D(tas!(b"bcwt")), a_noun, specs_noun])
        }
        BucZap(a, map) => {
            let a_noun = spec_to_noun(slab, a);
            let entries: Vec<_> = map.iter().map(|(k, v)| {
                (term_to_noun(slab, k), spec_to_noun(slab, v))
            }).collect();
            let map_noun = map_to_noun(slab, entries);
            T(slab, &[D(tas!(b"bczp")), a_noun, map_noun])
        }
    }
}

fn skin_to_noun(slab: &mut NounSlab, skin: &Skin) -> Noun {
    use Skin::*;
    match skin {
        Term(s) => term_to_noun(slab, s),
        Base(bt) => {
            let inner = basetype_to_noun(slab, bt);
            T(slab, &[D(tas!(b"base")), inner])
        }
        Cell(l, r) => {
            let l = skin_to_noun(slab, l);
            let r = skin_to_noun(slab, r);
            T(slab, &[D(tas!(b"cell")), l, r])
        }
        Dbug(spot, s) => {
            let spot_noun = spot_to_noun(slab, spot);
            let s_noun = skin_to_noun(slab, s);
            T(slab, &[D(tas!(b"dbug")), spot_noun, s_noun])
        }
        Leaf(tag, atom) => {
            let tag_noun = cord_to_noun(slab, tag);
            let atom_noun = atom_to_noun(slab, atom);
            T(slab, &[D(tas!(b"leaf")), tag_noun, atom_noun])
        }
        Name(name, s) => {
            let name_noun = term_to_noun(slab, name);
            let s_noun = skin_to_noun(slab, s);
            T(slab, &[D(tas!(b"name")), name_noun, s_noun])
        }
        Over(wing, s) => {
            let wing_noun = wing_to_noun(slab, wing);
            let s_noun = skin_to_noun(slab, s);
            T(slab, &[D(tas!(b"over")), wing_noun, s_noun])
        }
        Spec(spec, s) => {
            let spec_noun = spec_to_noun(slab, spec);
            let s_noun = skin_to_noun(slab, s);
            T(slab, &[D(tas!(b"spec")), spec_noun, s_noun])
        }
        Wash(n) => T(slab, &[D(tas!(b"wash")), D(*n)]),
    }
}

fn wing_to_noun(slab: &mut NounSlab, wing: &WingType) -> Noun {
    let limbs: Vec<Noun> = wing
        .iter()
        .map(|l| limb_to_noun(slab, l))
        .collect();

    list_to_noun(slab, limbs)
}

fn limb_to_noun(slab: &mut NounSlab, limb: &Limb) -> Noun {
    match limb {
        Limb::Term(s) => term_to_noun(slab, s),

        Limb::Axis(n) => {
            T(slab, &[D(0), D(*n)])
        }

        Limb::Parent(n, opt) => {
            let opt_noun = match opt {
                Some(s) => {
                    let s_noun = term_to_noun(slab, s);
                    T(slab, &[D(0), s_noun])
                }
                None => D(0),
            };

            T(slab, &[D(1), D(*n), opt_noun])
        }
    }
}

fn spot_to_noun(slab: &mut NounSlab, spot: &Spot) -> Noun {
    let path_noun = path_to_noun(slab, &spot.p);
    let pint_noun = pint_to_noun(slab, &spot.q);
    T(slab, &[path_noun, pint_noun])
}

fn pint_to_noun(slab: &mut NounSlab, pint: &Pint) -> Noun {
    let p = T(slab, &[D(pint.p.0), D(pint.p.1)]);
    let q = T(slab, &[D(pint.q.0), D(pint.q.1)]);
    T(slab, &[p, q])
}

fn note_to_noun(slab: &mut NounSlab, note: &Note) -> Noun {
    match note {
        Note::Know(s) => {
            let s_noun = term_to_noun(slab, s);
            T(slab, &[D(tas!(b"know")), s_noun])
        }

        Note::Made(s, opt_wings) => {
            let s_noun = term_to_noun(slab, s);

            let wings_noun = opt_wings.as_ref().map(|wings| {
                let wing_nouns: Vec<Noun> = wings
                    .iter()
                    .map(|w| wing_to_noun(slab, w))
                    .collect();

                list_to_noun(slab, wing_nouns)
            });

            let wings_noun = match wings_noun {
                None => { D(0)},
                Some(p) => { T(slab, &[D(0), p]) },
            };

            T(slab, &[D(tas!(b"made")), s_noun, wings_noun])
        }
    }
}

fn woof_to_noun(slab: &mut NounSlab, woof: &Woof) -> Noun {
    match woof {
        Woof::ParsedAtom(a) => {
            let val = atom_to_noun(slab, a);
            val
        }
        Woof::Hoon(h) => {
            let val = hoon_to_noun(slab, h);
            T(slab, &[D(0), val])
        }
    }
}

fn tome_to_noun(slab: &mut NounSlab, tome: &Tome) -> Noun {
    // let what = term_to_noun(slab, tome.0); // unused
    let pairs: Vec<_> = tome.1.iter()
        .map(|(k, v)| (
            term_to_noun(slab, k),
            hoon_to_noun(slab, v),
        ))
        .collect();
    let map = map_to_noun(slab, pairs);
    T(slab, &[D(0), map])
}

fn alas_to_noun(slab: &mut NounSlab, alas: &Alas) -> Noun {
    let pairs: Vec<_> = alas
        .iter()
        .map(|(k, v)| {
            let k_noun = term_to_noun(slab, k);
            let v_noun = hoon_to_noun(slab, v);
            (k_noun, v_noun)
        })
        .collect();
    map_to_noun(slab, pairs)
}

fn tyre_to_noun(slab: &mut NounSlab, tyre: &Tyre) -> Noun {
    let pairs: Vec<Noun> = tyre
        .iter()
        .map(|(k, v)| {
            let k_noun = term_to_noun(slab, k);
            let v_noun = hoon_to_noun(slab, v);
            T(slab, &[k_noun, v_noun])
        })
        .collect();
    list_to_noun(slab, pairs)
}

fn chum_to_noun(slab: &mut NounSlab, chum: &Chum) -> Noun {
    match chum {
        Chum::Lef(s) => term_to_noun(slab, s),
        Chum::StdKel(s, a) => {
            let s_noun = term_to_noun(slab, s);
            let a_noun = atom_to_noun(slab, a);
            T(slab, &[s_noun, a_noun])
        }
        Chum::VenProKel(v, p, a) => {
            let v_noun = term_to_noun(slab, v);
            let p_noun = term_to_noun(slab, p);
            let a_noun = atom_to_noun(slab, a);
            T(slab, &[v_noun, p_noun, a_noun])
        }
        Chum::VenProVerKel(v, p, a1, a2) => {
            let v_noun = term_to_noun(slab, v);
            let p_noun = term_to_noun(slab, p);
            let a1_noun = atom_to_noun(slab, a1);
            let a2_noun = atom_to_noun(slab, a2);
            T(slab, &[v_noun, p_noun, a1_noun, a2_noun])
        }
    }
}

fn nock_to_noun(slab: &mut NounSlab, nock: &Nock) -> Noun {
    use Nock::*;
    match nock {
        Pair(a, b) => {
            let a_noun = nock_to_noun(slab, a);
            let b_noun = nock_to_noun(slab, b);
            T(slab, &[D(2u64), a_noun, b_noun])
        }
        Const(expr) => {
            let expr_noun = noun_expr_to_noun(slab, expr);
            T(slab, &[D(1u64), expr_noun])
        }
        Compose(f, g) => {
            let f_noun = nock_to_noun(slab, f);
            let g_noun = nock_to_noun(slab, g);
            T(slab, &[D(7u64), f_noun, g_noun])
        }
        CellTest(n) => {
            let n_noun = nock_to_noun(slab, n);
            T(slab, &[D(3u64), n_noun])
        }
        Increment(n) => {
            let n_noun = nock_to_noun(slab, n);
            T(slab, &[D(4u64), n_noun])
        }
        Equality(a, b) => {
            let a_noun = nock_to_noun(slab, a);
            let b_noun = nock_to_noun(slab, b);
            T(slab, &[D(5u64), a_noun, b_noun])
        }
        IfThenElse(cond, yes, no) => {
            let cond_noun = nock_to_noun(slab, cond);
            let yes_noun = nock_to_noun(slab, yes);
            let no_noun = nock_to_noun(slab, no);
            T(slab, &[D(6u64), cond_noun, yes_noun, no_noun])
        }
        Edit((axis, new), core) => {
            let new_noun = nock_to_noun(slab, new);
            let core_noun = nock_to_noun(slab, core);
            let axis_cell = T(slab, &[D(*axis), new_noun]);
            T(slab, &[D(11u64), axis_cell, core_noun])
        }
        Hint(hint, n) => {
            let hint_noun = nock_hint_to_noun(slab, hint);
            let n_noun = nock_to_noun(slab, n);
            T(slab, &[D(12u64), hint_noun, n_noun])
        }
        SerialCompose(f, g) => {
            let f = nock_to_noun(slab, f);
            let g = nock_to_noun(slab, g);
            T(slab, &[D(8u64), f, g])
        }
                PushSubject(n, subj) => {
            let n = nock_to_noun(slab, n);
            let subj = nock_to_noun(slab, subj);
            T(slab, &[D(9u64), n, subj])
        }
        SelectArm(axis, core) => {
            let core = nock_to_noun(slab, core);
            T(slab, &[D(10u64), D(*axis), core])
        }
        GrabData(core, path) => {
            let core = nock_to_noun(slab, core);
            let path = nock_to_noun(slab, path);
            T(slab, &[D(13u64), core, path])
        }
        AxisSelect(axis) => D(*axis),
    }
}

fn nock_hint_to_noun(slab: &mut NounSlab, hint: &NockHint) -> Noun {
    match hint {
        NockHint::ParsedAtom(a) => D(*a),
        NockHint::Pair(tag, n) => {
            let n_noun = nock_to_noun(slab, n);
            T(slab, &[D(*tag), n_noun])
        }
    }
}

fn term_or_tune_to_noun(slab: &mut NounSlab, tot: &TermOrTune) -> Noun {
    match tot {
        TermOrTune::Term(s) => term_to_noun(slab, s),
        TermOrTune::Tune(tune) => tune_to_noun(slab, tune),
    }
}

fn tune_to_noun(slab: &mut NounSlab, (map, vec): &Tune) -> Noun {
    let map_pairs: Vec<_> = map.iter().map(|(k, opt_v)| {
        let k_noun = term_to_noun(slab, k);
        let v_noun = if let Some(v) = opt_v {
            hoon_to_noun(slab, v)
        } else {
            D(0)
        };
        (k_noun, v_noun)
    }).collect();

    let map_noun = map_to_noun(slab, map_pairs);

    let vec_nouns: Vec<_> = vec.iter().map(|v| hoon_to_noun(slab, v)).collect();

    let vec_noun = list_to_noun(slab, vec_nouns);

    T(slab, &[map_noun, vec_noun])
}

fn term_or_pair_to_noun(slab: &mut NounSlab, top: &TermOrPair) -> Noun {
    match top {
        TermOrPair::Term(s) => term_to_noun(slab, s),
        TermOrPair::Pair(s, h) => {
            let s_noun = term_to_noun(slab, s);
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[s_noun, h_noun])
        }
    }
}

fn zpwt_arg_to_noun(slab: &mut NounSlab, arg: &ZpwtArg) -> Noun {
    match arg {
        ZpwtArg::ParsedAtom(s) => {
            let tag = D(tas!(b"atom"));
            let s_noun = cord_to_noun(slab, s);
            T(slab, &[tag, s_noun])
        }
        ZpwtArg::Pair(s1, s2) => {
            let tag = D(tas!(b"pair"));
            let s1_noun = cord_to_noun(slab, s1);
            let s2_noun = cord_to_noun(slab, s2);
            T(slab, &[tag, s1_noun, s2_noun])
        }
    }
}

fn mane_to_noun(slab: &mut NounSlab, mane: &Mane) -> Noun {
    match mane {
        Mane::Tag(s) => term_to_noun(slab, s),
        Mane::TagSpace(s1, s2) => {
            let s1_noun = term_to_noun(slab, s1);
            let s2_noun = term_to_noun(slab, s2);
            T(slab, &[s1_noun, s2_noun])
        }
    }
}

fn marx_to_noun(slab: &mut NounSlab, marx: &Marx) -> Noun {
    let n = mane_to_noun(slab, &marx.n);
    let a = mart_to_noun(slab, &marx.a);
    T(slab, &[n, a])
}

fn manx_to_noun(slab: &mut NounSlab, manx: &Manx) -> Noun {
    let g = marx_to_noun(slab, &manx.g);
    let c = marl_to_noun(slab, &manx.c);
    T(slab, &[g, c])
}

fn mart_to_noun(slab: &mut NounSlab, mart: &Mart) -> Noun {
    let cells: Vec<Noun> = mart
        .iter()
        .map(|(mane, beers)| {
            let mane_noun = mane_to_noun(slab, mane);

            let beer_nouns: Vec<Noun> = beers
                .iter()
                .map(|b| beer_to_noun(slab, b))
                .collect();

            let beers_noun = list_to_noun(slab, beer_nouns);

            T(slab, &[mane_noun, beers_noun])
        })
        .collect();

    list_to_noun(slab, cells)
}

fn beer_to_noun(slab: &mut NounSlab, beer: &Beer) -> Noun {
    match beer {
        Beer::Char(cord) => cord_to_noun(slab, cord),
        Beer::Hoon(h) => {
            let hoon_noun = hoon_to_noun(slab, h);
            T(slab, &[D(0), hoon_noun])
        }
    }
}

fn marl_to_noun(slab: &mut NounSlab, marl: &Marl) -> Noun {
    let items: Vec<Noun> = marl
        .iter()
        .map(|t| tuna_to_noun(slab, t))
        .collect();

    list_to_noun(slab, items)
}

fn tuna_to_noun(slab: &mut NounSlab, tuna: &Tuna) -> Noun {
    match tuna {
        Tuna::Manx(m) => {
            manx_to_noun(slab, m)
        }
        Tuna::TunaTail(tail) => {
            tuna_tail_to_noun(slab, tail)
        }
    }
}

fn tuna_tail_to_noun(slab: &mut NounSlab, tail: &TunaTail) -> Noun {
    match tail {
        TunaTail::Tape(h) => {
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"tape")), h_noun])
        }
        TunaTail::Manx(h) => {
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"manx")), h_noun])
        }
        TunaTail::Marl(h) => {
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"marl")), h_noun])
        }
        TunaTail::Call(h) => {
            let h_noun = hoon_to_noun(slab, h);
            T(slab, &[D(tas!(b"call")), h_noun])
        }
    }
}

pub fn dor_slab(a: Noun, b: Noun) -> Noun {
    if unsafe { a.raw_equals(&b) } {
        YES
    } else {
        match (a.as_either_atom_cell(), b.as_either_atom_cell()) {
            (Left(atom_a), Left(atom_b)) => atom_less_than(atom_a, atom_b),
            (Left(_), Right(_)) => YES,
            (Right(_), Left(_)) => NO,
            (Right(cell_a), Right(cell_b)) => {
                let a_head = match slot(cell_a.as_noun(), 2) {
                    Ok(n) => n,
                    Err(_) => return NO,
                };
                let b_head = slot(cell_b.as_noun(), 2).unwrap_or_else(|err| {
                    panic!(
                        "Panicked with {err:?} at {}:{} (git sha: {:?})",
                        file!(),
                        line!(),
                        option_env!("GIT_SHA")
                    )
                });
                let a_tail = slot(cell_a.as_noun(), 3).unwrap_or_else(|err| {
                    panic!(
                        "Panicked with {err:?} at {}:{} (git sha: {:?})",
                        file!(),
                        line!(),
                        option_env!("GIT_SHA")
                    )
                });
                let b_tail = slot(cell_b.as_noun(), 3).unwrap_or_else(|err| {
                    panic!(
                        "Panicked with {err:?} at {}:{} (git sha: {:?})",
                        file!(),
                        line!(),
                        option_env!("GIT_SHA")
                    )
                });
                if unsafe { a_head.raw_equals(&b_head) } {
                    dor_slab(a_tail, b_tail)
                } else {
                    dor_slab(a_head, b_head)
                }
            }
        }
    }
}

pub fn gor_slab(a: Noun, b: Noun) -> Noun {
    let c = unsafe { DirectAtom::new_unchecked(slab_mug(a) as u64) };
    let d = unsafe { DirectAtom::new_unchecked(slab_mug(b) as u64) };

    match c.data().cmp(&d.data()) {
        Ordering::Greater => NO,
        Ordering::Less => YES,
        Ordering::Equal => dor_slab(a, b),
    }
}

pub fn mor_slab(a: Noun, b: Noun) -> Noun {
    let c = unsafe { DirectAtom::new_unchecked(slab_mug(a) as u64) };
    let d = unsafe { DirectAtom::new_unchecked(slab_mug(b) as u64) };

    let e = unsafe { DirectAtom::new_unchecked(slab_mug(c.as_noun()) as u64) };
    let f = unsafe { DirectAtom::new_unchecked(slab_mug(d.as_noun()) as u64) };

    match e.data().cmp(&f.data()) {
        Ordering::Greater => NO,
        Ordering::Less => YES,
        Ordering::Equal => dor_slab(a, b),
    }
}

pub fn pile_to_noun(slab: &mut NounSlab, pile: &Pile) -> Noun {
    let sur_noun = {
        let mut items = Vec::new();
        for (opt_term, term) in pile.sur.clone() {
            let opt_noun = opt_term.map_or_else(|| D(0u64), |t| term_to_noun(slab, &t));
            let term_noun = term_to_noun(slab, &term);
            items.push(T(slab, &[opt_noun, term_noun]));
        }
        list_to_noun(slab, items)
    };

    let lib_noun = {
        let mut items = Vec::new();
        for (opt_term, term) in pile.lib.clone() {
            let opt_noun = opt_term.map_or_else(|| D(0u64), |t| term_to_noun(slab, &t));
            let term_noun = term_to_noun(slab, &term);
            items.push(T(slab, &[opt_noun, term_noun]));
        }
        list_to_noun(slab, items)
    };

    let raw_noun = {
        let mut items = Vec::new();
        for (opt_term, path) in pile.raw.clone() {
            let opt_noun = opt_term.map_or_else(|| D(0u64), |t| term_to_noun(slab, &t));
            let path_noun = path_to_noun(slab, &path);
            items.push(T(slab, &[opt_noun, path_noun]));
        }
        list_to_noun(slab, items)
    };

    let bar_noun = {
        let mut items = Vec::new();
        for (term1, term2, path) in pile.bar.clone() {
            let t1_noun = term_to_noun(slab, &term1);
            let t2_noun = term_to_noun(slab, &term2);
            let path_noun = path_to_noun(slab, &path);
            items.push(T(slab, &[t1_noun, t2_noun, path_noun]));
        }
        list_to_noun(slab, items)
    };

    let hax_noun = {
        let mut items = Vec::new();
        for (opt_term, term) in pile.hax.clone() {
            let opt_noun = opt_term.map_or_else(|| D(0u64), |t| term_to_noun(slab, &t));
            let term_noun = term_to_noun(slab, &term);
            items.push(T(slab, &[opt_noun, term_noun]));
        }
        list_to_noun(slab, items)
    };

    let hoon_noun = hoon_to_noun(slab, &pile.hoon);

    T(slab, &[sur_noun, lib_noun, raw_noun, bar_noun, hax_noun, hoon_noun])
}


//
//  Cue / Jam
//

pub fn cue_simple(buffer: ParsedAtom) -> Result<NounExpr, Box<dyn std::error::Error>> {
    let bits = atom_to_bits(&buffer);
    let mut backrefs = HashMap::new();
    let (noun, _) = cue_inner(&bits, 0, &mut backrefs)?;
    Ok(noun)
}

pub fn jam_simple(noun: NounExpr) -> ParsedAtom {
    let mut bits = Vec::new();
    let mut backrefs = HashMap::new();
    let mut stack = vec![noun];

    while let Some(current) = stack.pop() {
        if let Some(&offset) = backrefs.get(&current) {
            let use_backref = match &current {
                NounExpr::ParsedAtom(atom) => {
                    let atom_bits = mat_bits(atom).len();
                    let offset_bits = mat_bits(&offset_to_atom(offset)).len();
                    offset_bits < atom_bits
                }
                NounExpr::Cell(_, _) => true,
            };

            if use_backref {
                bits.push(true);
                bits.push(true);
                bits.extend(mat_bits(&offset_to_atom(offset)));
                continue;
            }
        }

        let offset = bits.len();
        backrefs.insert(current.clone(), offset);

        match current {
            NounExpr::ParsedAtom(atom) => {
                bits.push(false);
                bits.extend(mat_bits(&atom));
            }
            NounExpr::Cell(head, tail) => {
                bits.push(true);
                bits.push(false);
                stack.push(*tail);
                stack.push(*head);
            }
        }
    }

    bits_to_atom(&bits)
}

fn offset_to_atom(offset: usize) -> ParsedAtom {
    if offset <= u128::MAX as usize {
        ParsedAtom::Small(offset as u128)
    } else {
        ParsedAtom::Big(BigUint::from(offset))
    }
}

fn mat_bits(atom: &ParsedAtom) -> Vec<bool> {
    let n = atom_bit_len(atom);

    let mut bits = Vec::new();

    if n == 0 {
        bits.push(true);
        return bits;
    }

    let k = usize_bit_len(n);

    bits.extend(std::iter::repeat(false).take(k));

    bits.push(true);

    if k > 1 {
        let offset = n - (1usize << (k - 1));
        for i in 0..(k - 1) {
            bits.push((offset >> i) & 1 == 1);
        }
    }

    for i in 0..n {
        bits.push(atom_get_bit(atom, i as u64));
    }

    bits
}

fn usize_bit_len(x: usize) -> usize {
    if x == 0 { 1 } else { (usize::BITS - x.leading_zeros()) as usize }
}

fn atom_bit_len(atom: &ParsedAtom) -> usize {
    match atom {
        ParsedAtom::Small(0) => 0,
        ParsedAtom::Small(x) => 128 - x.leading_zeros() as usize,
        ParsedAtom::Big(x) => x.bits() as usize,
    }
}

fn atom_get_bit(atom: &ParsedAtom, i: u64) -> bool {
    match atom {
        ParsedAtom::Small(x) => i < 128 && ((x >> i) & 1 == 1),
        ParsedAtom::Big(x) => {
            let byte_index = (i / 8) as usize;
            let bit_index = (i % 8) as u8;
            let bytes = x.to_bytes_le();
            if byte_index < bytes.len() {
                let byte = bytes[byte_index];
                (byte >> bit_index) & 1 == 1
            } else {
                false
            }
        }
    }
}

fn bits_to_atom(bits: &[bool]) -> ParsedAtom {
    if bits.is_empty() {
        return ParsedAtom::Small(0);
    }

    let len = bits.len();

    if len <= 128 {
        let mut val: u128 = 0;
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                val |= 1u128 << i;
            }
        }
        ParsedAtom::Small(val)
    } else {
        let mut big = BigUint::from(0u32);
        for (i, &bit) in bits.iter().enumerate() {
            if bit {
                big += BigUint::from(1u32) << i;
            }
        }
        ParsedAtom::Big(big)
    }
}

fn rub_backref(bits: &[bool], cursor: &mut usize) -> Result<u64, Box<dyn std::error::Error>> {
    let size = get_size(bits, cursor)?;
    if size == 0 {
        return Ok(0);
    }
    if size > 64 {
        return Err("backref offset too large (>64 bits)".into());
    }
    if *cursor + size as usize > bits.len() {
        return Err("not enough bits for backref".into());
    }

    let mut val: u64 = 0;
    for i in 0..size {
        if bits[*cursor + i as usize] {
            val |= 1u64 << i;
        }
    }
    *cursor += size as usize;
    Ok(val)
}

fn rub_atom(bits: &[bool], cursor: &mut usize) -> Result<ParsedAtom, Box<dyn std::error::Error>> {
    let size = get_size(bits, cursor)?;

    if size == 0 {
        return Ok(ParsedAtom::Small(0));
    }

    if *cursor + size as usize > bits.len() {
        return Err("not enough bits for rub atom payload".into());
    }

    if size <= 128 {
        let mut val: u128 = 0;
        for i in 0..size {
            if bits[*cursor + i as usize] {
                val |= 1u128 << i;
            }
        }
        *cursor += size as usize;
        Ok(ParsedAtom::Small(val))
    } else {
        let mut big = BigUint::from(0u32);
        for i in 0..size {
            if bits[*cursor + i as usize] {
                big += BigUint::from(1u32) << i;
            }
        }
        *cursor += size as usize;
        Ok(ParsedAtom::Big(big))
    }
}

fn get_size(bits: &[bool], cursor: &mut usize) -> Result<u64, &'static str> {
    let start = *cursor;
    while *cursor < bits.len() && !bits[*cursor] {
        *cursor += 1;
    }

    if *cursor >= bits.len() {
        return Err("unexpected EOF in rub size prefix");
    }

    let c = (*cursor - start) as u32; // number of leading zeros
    *cursor += 1; // consume the '1'

    if c == 0 {
        Ok(0)
    } else {
        if *cursor + (c - 1) as usize > bits.len() {
            return Err("not enough bits for rub size field");
        }

        let mut x = 0u64;
        for i in 0..(c - 1) {
            if bits[*cursor + i as usize] {
                x |= 1u64 << i; // LSB-first: first bit = 2^0
            }
        }
        *cursor += (c - 1) as usize;

        let size = (1u64 << (c - 1)) + x;
        Ok(size)
    }
}

fn atom_to_bits(atom: &ParsedAtom) -> Vec<bool> {
    match atom {
        ParsedAtom::Small(x) => {
            let mut bits = Vec::with_capacity(128);
            for i in 0..128 {
                bits.push((x >> i) & 1 == 1);
            }
            bits
        }
        ParsedAtom::Big(x) => {
            let bytes = x.to_bytes_le();
            let mut bits = Vec::new();
            for &byte in &bytes {
                for i in 0..8 {
                    bits.push((byte >> i) & 1 == 1);
                }
            }
            bits
        }
    }
}

fn cue_inner( // rename
    bits: &[bool],
    cursor: usize,
    backrefs: &mut HashMap<u64, NounExpr>,
) -> Result<(NounExpr, usize), Box<dyn std::error::Error>> {
    if cursor >= bits.len() {
        return Err("unexpected EOF".into());
    }

    let tag0 = bits[cursor];
    if !tag0 {
        let mut cur = cursor + 1;
        let atom = rub_atom(bits, &mut cur)?;
        let noun = NounExpr::ParsedAtom(atom);
        backrefs.insert(cursor as u64, noun.clone());
        Ok((noun, cur))
    } else {
        if cursor + 1 >= bits.len() {
            return Err("unexpected EOF after tag 1".into());
        }
        let tag1 = bits[cursor + 1];
        if !tag1 {
            let mut cur = cursor + 2;
            let (head, next) = cue_inner(bits, cur, backrefs)?;
            cur = next;
            let (tail, next2) = cue_inner(bits, cur, backrefs)?;
            cur = next2;
            let noun = NounExpr::Cell(Box::new(head), Box::new(tail));
            backrefs.insert(cursor as u64, noun.clone());
            Ok((noun, cur))
        } else {
            let mut cur = cursor + 2;
            let offset = rub_backref(bits, &mut cur)?;

            let noun = backrefs
                .get(&(offset))
                .cloned()
                .ok_or_else(|| format!("backref to {} not found", offset))?;
            Ok((noun, cur))
        }
    }
}
