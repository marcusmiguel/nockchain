use parser::ast::hoon::*;
use std::collections::HashMap;
use crate::atom::*;
use crate::utils::*;

//
//  Skin Formation Logic
//
//  The entry is +flay which is called to convert Hoon -> Skin.
//  Flay is called by ^=.
//  It depends on many other functions from the Hoon compiler.
//

pub fn flay(gen: Hoon) -> Option<Skin> {
    match gen {
        Hoon::Pair(p, q) => {
            let maybe_p = flay(*p);
            let maybe_q = flay(*q);
            match (maybe_p, maybe_q) {
                (Some(p), Some(q)) => Some(Skin::Cell(Box::new(p), Box::new(q))),
                _ => None,
            }
        }

        Hoon::Base(b) => Some(Skin::Base(b.clone())),

        Hoon::Rock(t, n) => {
            match n {
                NounExpr::ParsedAtom(a) => Some(Skin::Leaf(t.to_string(), a)),
                NounExpr::Cell(_, _) => None,
            }
        }

        Hoon::CenTis(w, l) => {
            match (w, l) {
                (v, l) if l.is_empty() => match v.as_slice() {
                    [Limb::Term(t)] => Some(Skin::Term((*t).to_string())),
                    _ => None,
                },
                _ => None,
            }
        }

        Hoon::TisGar(p, q) => {
            let maybe_wing = reek(*p);
            match maybe_wing {
                Some(w) => {
                    let skin = flay(*q);
                    match skin {
                        None => None,
                        Some(s) => Some(Skin::Over(w, Box::new(s))),
                    }
                }
                None => None,
            }
        }

        Hoon::Limb(t) => {
            Some(Skin::Term(t.to_string()))
        }

        Hoon::Wing(w) => {
            match w.as_slice() {
                [Limb::Term(t)] => Some(Skin::Term(t.clone())),
                _ => {
                    fn recur(w: &[Limb]) -> Option<Skin> {
                        match w {
                            [] => Some(Skin::Wash(0)),
                            [Limb::Parent(0, None), rest @ ..] => recur(rest),
                            _ => None,
                        }
                    }
                    recur(w.as_slice())
                }
            }
        }

        Hoon::KetTar(s) => {
            Some(Skin::Spec(s.clone(), Box::new(Skin::Base(BaseType::NounExpr))))
        }

    Hoon::KetTis(spec, h) => {
            let maybe_skin = flay(*h);
            match maybe_skin {
                Some(s) => {
                    match spec {
                        Skin::Term(ref t) => {
                            Some(Skin::Name(t.to_string(), Box::new(s)))
                        }
                        Skin::Name(ref t, ref b) => {
                            if matches!(**b, Skin::Base(BaseType::NounExpr)) {
                                Some(Skin::Name(t.clone(), Box::new(s)))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                }
                None => None,
            }
        }

        _ => {
            let desugared = open(gen.clone());
            if desugared == gen {
                None
            } else {
                flay(desugared)
            }
        }

    }
}

pub fn basal(bas: BaseType) -> Hoon {
    match bas {
        BaseType::Atom(a) => {
            let literal = if a == "da" {
                ParsedAtom::Small(year(
                    true,
                    2000,
                    1,
                    1,
                    0,
                    0,
                    0,
                    &Vec::new()
                ))
            } else {
                decimal_to_atom("0".to_string())
            };
            Hoon::Sand(a, NounExpr::ParsedAtom(literal))
        }
        BaseType::NounExpr => {
            let rock0 = Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))));
            let rock1 = Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1))));
            let rock0_clone = rock0.clone();
            let rock0_clone2 = rock0.clone();
            Hoon::KetLus(Box::new(Hoon::DotTar(rock0, Box::new(Hoon::Pair(rock0_clone, rock1)))), rock0_clone2)
        }
        BaseType::Cell => {
            let noun = Box::new(basal(BaseType::NounExpr));
            let noun_clone = noun.clone();
            Hoon::Pair(noun, noun_clone)
        }
        BaseType::Flag => {
            let rock0 = Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))));
            let rock0_clone = rock0.clone();
            let rock1_clone = rock0.clone();
            Hoon::KetLus(Box::new(Hoon::DotTis(rock0, rock0_clone)), rock1_clone)
        }
        BaseType::Null => Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
        BaseType::Void => Hoon::ZapZap,
    }
}

pub fn function(
    fun: Spec,
    arg: Spec,
    mod_: &Spec,
    dom: u64,
    hay: &WingType,
    cox: &HashMap<String, Spec>,
    bug: &Vec<Spot>,
    nut: &Option<Note>,
    def: &Option<Hoon>,
) -> Hoon {
    Hoon::TisGar(
        Box::new(Hoon::Pair(
                    Box::new(example(&fun.clone(), dom, hay, cox, &vec![], &None, &None)),
                    Box::new(example(&arg.clone(), dom, hay, cox, &vec![], &None, &None)))),
        Box::new(Hoon::KetBar(Box::new(Hoon::BarCol(Box::new(Hoon::Axis(2)),
                                        Box::new(Hoon::Axis(15)))))),
    )
}

pub fn interface(
    variance: Vair,
    payload: Spec,
    arms: HashMap<String, Spec>,
    mod_: &Spec,
    dom: u64,
    hay: &WingType,
    cox: &HashMap<String, Spec>,
    bug: &Vec<Spot>,
    nut: &Option<Note>,
    def: &Option<Hoon>,
) -> Hoon {

    let map: HashMap<String, Hoon> = arms.into_iter()
        .map(|(term, spec)|
                 (term, example(&spec, dom, hay, cox, &vec![], &None, &None)))
        .collect();
    let brcn = Hoon::BarCen(
        None,
        HashMap::from([("$".to_string(), (None, map))]),
    );

    let example_res = example(&payload, dom, hay, cox, &vec![], &None, &None);
    let tsgr = Hoon::TisGar(Box::new(example_res), Box::new(brcn));
    match variance {
        Vair::Gold => tsgr,
        Vair::Lead => Hoon::KetWut(Box::new(tsgr)),
        Vair::Zinc => Hoon::KetPam(Box::new(tsgr)),
        Vair::Iron => Hoon::KetBar(Box::new(tsgr)),
    }
}

pub fn spore(spec: Spec,
                dom: u64,
                hay: WingType,
                cox: HashMap<String, Spec>,
                bug: Vec<Spot>,
                nut: Option<Note>,
                def: Option<Hoon>) -> Hoon {
    let subject = match def {
        Some(d) => d,
        None => spore_recursion(spec, dom, hay, cox, bug, nut, def),
    };
    let ketlus_tail = home(subject, Vec::new(), dom);
    Hoon::KetLus(Box::new(Hoon::Bust(BaseType::NounExpr)), Box::new(ketlus_tail))
}

pub fn spore_recursion(spec: Spec,
                dom: u64,
                hay: WingType,
                cox: HashMap<String, Spec>,
                bug: Vec<Spot>,
                nut: Option<Note>,
                def: Option<Hoon>) -> Hoon {
    match spec {
        Spec::Base(b) => {
            match b {
                BaseType::Void => Hoon::Rock("n".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
                _ => basal(b),
            }
        }
        Spec::BucBuc(s, map) => {
            let mut new_cox = cox;
            new_cox.extend(map);
            new_cox.insert("$".to_string(), *s.clone());
            spore_recursion(*s, dom, hay, new_cox, bug, nut, def)
        }
        Spec::Dbug(spot, spec) => {
            let tail = spore_recursion(*spec, dom, hay, cox, bug, nut, def);
            Hoon::Dbug(spot, Box::new(tail))
        }
        Spec::Leaf(term, atom) => Hoon::Rock(term, NounExpr::ParsedAtom(atom)),
        Spec::Loop(term) => {
            let spec = cox.get(&term).expect("Spec-Loop: Name not found");
            spore_recursion(spec.clone(), dom, hay, cox, bug, nut, def)
        }
        Spec::Like(wing, wings) => {
            let p = unreel(wing, wings);
            spore_recursion(Spec::BucMic(p), dom, hay, cox, bug, nut, def)
        }
        Spec::Made(_, q) => spore_recursion(*q, dom, hay, cox, bug, nut, def),
        Spec::Make(hoon, specs) => {
            let p = unfold(hoon, specs);
            spore_recursion(Spec::BucMic(p), dom, hay, cox, bug, nut, def)
        }
        Spec::Name(term, spec) => spore_recursion(*spec, dom, hay, cox, bug, nut, def),
        Spec::Over(wing, spec) => spore_recursion(*spec, dom, wing, cox, bug, nut, def),
        Spec::BucBar(spec, hoon) => spore_recursion(*spec, dom, hay, cox, bug, nut, def),
        Spec::BucCab(_) => Hoon::Rock("n".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
        Spec::BucCol(spec, specs) => spore_buccol_recursion(*spec, specs, dom, hay, cox, bug, nut, def),
        Spec::BucCen(spec, specs) => spore_buccen_recursion(*spec, specs, dom, hay, cox, bug, nut, def),
        Spec::BucHep(spec, specs) => Hoon::Rock("n".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
        Spec::BucGal(p_spec, q_spec) => spore_recursion(*q_spec, dom, hay, cox, bug, nut, def),
        Spec::BucGar(p_spec, q_spec) => spore_recursion(*q_spec, dom, hay, cox, bug, nut, def),
        Spec::BucKet(p_spec, q_spec) => spore_recursion(*q_spec, dom, hay, cox, bug, nut, def),
        Spec::BucLus(stud, spec) => {
           let tail = spore_recursion(*spec, dom, hay, cox, bug, nut, def);
           Hoon::Note(Note::Know(stud), Box::new(tail))
        }
        Spec::BucMic(hoon) => Hoon::TisGal(Box::new(Hoon::Axis(6)), Box::new(hoon)),
        Spec::BucPam(spec, hoon) => spore_recursion(*spec, dom, hay, cox, bug, nut, def),
        Spec::BucSig(hoon, spec) => Hoon::KetHep(spec, Box::new(hoon)),
        Spec::BucTis(skin, spec) => {
            let tail = spore_recursion(*spec, dom, hay, cox, bug, nut, def);
            Hoon::KetTis(skin, Box::new(tail))
        }
        Spec::BucPat(p_spec, q_spec) => spore_recursion(*p_spec, dom, hay, cox, bug, nut, def),
        Spec::BucWut(spec, specs) => spore_bucwut_recursion(*spec, specs, dom, hay, cox, bug, nut, def),
        Spec::BucDot(..) | Spec::BucFas(..) | Spec::BucTic(..) | Spec::BucZap(..)
         => Hoon::Rock("n".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
    }
}

pub fn spore_buccol_recursion(spec: Spec,
                list_spec: Vec<Spec>,
                dom: u64,
                hay: WingType,
                cox: HashMap<String, Spec>,
                bug: Vec<Spot>,
                nut: Option<Note>,
                def: Option<Hoon>) -> Hoon {
    if list_spec.is_empty() {
        spore_recursion(spec, dom, hay, cox, bug, nut, def)
    } else {
        let head = spore_recursion(spec,
                                    dom.clone(),
                                    hay.clone(),
                                    cox.clone(),
                                    bug.clone(),
                                    nut.clone(),
                                    def.clone());
        let tail = spore_buccol_recursion(list_spec.first().unwrap().clone(),
                                         list_spec[1..].to_vec(),
                                         dom,
                                         hay,
                                         cox,
                                         bug,
                                         nut,
                                         def);
        Hoon::Pair(Box::new(head), Box::new(tail))
    }
}

pub fn spore_bucwut_recursion(spec: Spec,
                list_spec: Vec<Spec>,
                dom: u64,
                hay: WingType,
                cox: HashMap<String, Spec>,
                bug: Vec<Spot>,
                nut: Option<Note>,
                def: Option<Hoon>) -> Hoon {
    if list_spec.is_empty() {
        spore_recursion(spec, dom, hay, cox, bug, nut, def)
    } else {
        spore_bucwut_recursion(list_spec.first().unwrap().clone(),
                               list_spec[1..].to_vec(),
                                dom,
                                hay,
                                cox,
                                bug,
                                nut,
                                def)
    }
}

pub fn spore_buccen_recursion(spec: Spec,
                list_spec: Vec<Spec>,
                dom: u64,
                hay: WingType,
                cox: HashMap<String, Spec>,
                bug: Vec<Spot>,
                nut: Option<Note>,
                def: Option<Hoon>) -> Hoon {
    if list_spec.is_empty() {
        spore_recursion(spec, dom, hay, cox, bug, nut, def)
    } else {
        spore_buccen_recursion(list_spec.first().unwrap().clone(),
                               list_spec[1..].to_vec(),
                                dom,
                                hay,
                                cox,
                                bug,
                                nut,
                                def)
    }
}

pub fn example(
    mod_: &Spec,
    dom: u64,
    hay: &WingType,
    cox: &HashMap<String, Spec>,
    bug: &Vec<Spot>,
    nut: &Option<Note>,
    def: &Option<Hoon>,
) -> Hoon {
    match mod_ {
        Spec::Base(b) => {
            decorate(basal(b.clone()), bug.clone(), nut.clone())
        }
        Spec::Dbug(spot, inner) => {
            let mut bug = bug.clone();
            bug.push(spot.clone());
            example(&inner, dom, hay, cox, &bug, nut, def)
        }
        Spec::Leaf(term, atom) => {
            decorate(Hoon::Rock(term.clone(), NounExpr::ParsedAtom(atom.clone())), bug.clone(), nut.clone())
        }
        Spec::Like(wing, list) => {
            example(&Spec::BucMic(unreel(wing.clone(), list.clone())),
                dom, wing, cox, bug, nut, def)
        }
        Spec::Loop(term) => {
            Hoon::Limb(term.clone())
        }
        Spec::Made((t, list), inner) => {
            let pieces = list
                .iter()
                .map(|s| vec![Limb::Term(s.to_string())])
                .collect();
            example(&inner, dom, hay, cox, bug,
                    &Some(Note::Made(t.to_string(), Some(pieces))), def)
        }
        Spec::Make(head, tail) => {
            example(&Spec::BucMic(unfold(head.clone(), tail.clone())), dom, hay, cox, bug, nut, def)
        }
        Spec::Name(term, inner) => {
            example(&inner, dom, hay, cox, bug, &Some(Note::Made(term.to_string(), None)), def)
        }
        Spec::Over(wing, inner) => {
            example(&inner, dom, wing, cox, bug, nut, def)
        }
        Spec::BucCab(p) => {
            decorate(home(p.clone(), hay.clone(), dom.clone()), bug.clone(), nut.clone())
        }
        Spec::BucCol(head, tail) => {
           let mut result = example(head, dom, hay, cox, &vec![], &None, &None);

            for x in tail.iter().rev() {
                let next = example(&x, dom, hay, cox, &vec![], &None, &None);
                result = Hoon::Pair(Box::new(next), Box::new(result));
            }

            decorate(result, bug.clone(), nut.clone())
        }
        Spec::BucHep(p, q) => {
            let function_res = function(*p.clone(), *q.clone(), mod_, dom, hay, cox, &vec![], &None, &None);
            decorate(
                function_res,
                bug.clone(),
                nut.clone())
        }
        Spec::BucMic(inner) => {
            let tsgl = Hoon::TisGal(
                            Box::new(Hoon::Limb("$".to_string())),
                            Box::new(inner.clone()));
            decorate(home(tsgl, hay.clone(), dom.clone()), bug.clone(), nut.clone())
        }
        Spec::BucSig(inner, list) => {
            Hoon::KetLus(
                Box::new(example(&list, dom, hay, cox, bug, nut, def)),
                Box::new(home(inner.clone(), hay.clone(), dom.clone()))
            )
        }
        Spec::BucLus(stud, inner) => {
            decorate(
                Hoon::Note(
                    Note::Know(stud.clone()),
                    Box::new(example(&inner.clone(), dom, hay, cox, bug, nut, def)),
                ),
                bug.clone(),
                nut.clone())
        }
        Spec::BucTis(skin, inner) => {
            decorate(
                Hoon::KetTis(
                    skin.clone(),
                    Box::new(example(&inner.clone(), dom, hay, cox, bug, nut, def)),
                ),
                bug.clone(),
                nut.clone())
        }
        Spec::BucDot(inner, map) => vair_case(Vair::Gold, *inner.clone(), map.clone(), mod_, dom, hay, cox, bug, nut, def),
        Spec::BucFas(inner, map) => vair_case(Vair::Iron, *inner.clone(), map.clone(), mod_, dom, hay, cox, bug, nut, def),
        Spec::BucZap(inner, map) => vair_case(Vair::Lead, *inner.clone(), map.clone(), mod_, dom, hay, cox, bug, nut, def),
        Spec::BucTic(inner, map) => vair_case(Vair::Zinc, *inner.clone(), map.clone(), mod_, dom, hay, cox, bug, nut, def),
        _ => {
            let spore_result = spore(mod_.clone(),
                                          dom.clone(),
                                          hay.clone(),
                                          cox.clone(),
                                          bug.clone(),
                                          nut.clone(),
                                          def.clone());
            let dom = peg(dom, 3).expect("example +peg failed");
            let relative_result = relative(2, mod_, dom, hay, cox, bug, nut, def);
            Hoon::TisLus(Box::new(spore_result), Box::new(relative_result))
        }
    }
}

// used in +example
fn vair_case(
    vair: Vair,
    payload: Spec,
    arms: HashMap<String, Spec>,
    mod_: &Spec,
    dom: u64,
    hay: &WingType,
    cox: &HashMap<String, Spec>,
    bug: &Vec<Spot>,
    nut: &Option<Note>,
    def: &Option<Hoon>,
) -> Hoon {
    let hoon = interface(vair, payload, arms, mod_, dom, hay, cox, bug, nut, def);
    decorate(home(hoon, hay.clone(), dom.clone()), bug.clone(), nut.clone())
}

pub fn basic(bas: BaseType,
                axe: u64,
                mod_: &Spec,
                dom: u64,
                hay: &WingType,
                cox: &HashMap<String, Spec>,
                bug: &Vec<Spot>,
                nut: &Option<Note>,
                def: &Option<Hoon>) -> Hoon {
    match bas {
        BaseType::Atom(a) => {
            let cnls = Hoon::CenLus(Box::new(Hoon::Limb("ruth".to_string())),
                                    Box::new(Hoon::Sand("ta".to_string(), NounExpr::ParsedAtom(string_to_atom(a)))),
                                    Box::new(Hoon::Axis(axe)));

            let example_res = Box::new(example(mod_, dom, hay, cox, bug, nut, def));

            let wtpt_limb = Limb::Axis(axe);
            let wtpt_wing: Vec<Limb> = vec![wtpt_limb];
            let wtpt = Hoon::WutPat(wtpt_wing, Box::new(Hoon::Axis(axe)), Box::new(Hoon::ZapZap));

            let zppt_limb = Limb::Parent(0, Some("ruth".to_string()));
            let zppt_wing: Vec<Limb> = vec![zppt_limb];
            let zppt_list_wing: Vec<Vec<Limb>> = vec![zppt_wing];
            let zppt = Hoon::ZapPat(zppt_list_wing, Box::new(cnls), Box::new(wtpt));

            Hoon::KetLus(example_res, Box::new(zppt))
        }
        BaseType::Cell => {
            let example_res = Box::new(example(mod_, dom, hay, cox, bug, nut, def));
            let wing = Limb::Axis(axe);
            let wing: Vec<Limb> = vec![wing];
            let mut p = wing.clone();
            p.insert(0, Limb::Axis(2));
            let mut q = wing.clone();
            q.insert(0, Limb::Axis(3));
            let pair = Hoon::Pair(Box::new(Hoon::Wing(p)), Box::new(Hoon::Wing(q)));

            Hoon::KetLus(example_res, Box::new(pair))
        }
        BaseType::Flag => {
            let rock = Box::new(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))));
            let dtts = Box::new(Hoon::DotTis(
                                    Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
                                    Box::new(Hoon::Axis(axe))
                                ));
            let wtgr = Box::new(Hoon::WutGar(
                            Box::new(Hoon::DotTis(
                                Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1)))),
                                Box::new(Hoon::Axis(axe))
                            )),
                            Box::new(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1))))
                        ));
            Hoon::WutCol(dtts, rock, wtgr)
        },
        BaseType::Null => {
            let rock = Box::new(Hoon::Rock("n".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))));
            let dtts = Box::new(Hoon::DotTis(
                                    Box::new(Hoon::Bust(BaseType::NounExpr)),
                                    Box::new(Hoon::Axis(axe))
                                ));
            Hoon::WutGar(dtts, rock)
        }
        BaseType::NounExpr => Hoon::Axis(axe),
        BaseType::Void => Hoon::ZapZap,
    }
}

pub fn switch(
    one: Spec,
    mut rep: Vec<Spec>,
    axe: u64,
    mod_: &Spec,
    dom: u64,
    hay: &WingType,
    cox: &HashMap<String, Spec>,
    bug: &Vec<Spot>,
    nut: &Option<Note>,
    def: &Option<Hoon>,
) -> Hoon {
    if rep.is_empty() {
        return relative(axe, &one, dom, hay, cox, &vec![], &None, &None);
    }

    let mut iter = rep.into_iter();
    let i_rep = iter.next().unwrap();
    let t_rep: Vec<Spec> = iter.collect();

    let fin = switch(i_rep.clone(), t_rep, axe, mod_, dom, hay, cox, bug, nut, def);

    let example_res = example(&one.clone(), dom, hay, cox, &vec![], &None, &None);

    let fits = Hoon::Fits(
        Box::new(Hoon::TisGal(
            Box::new(Hoon::Axis(2)),
            Box::new(example_res),
        )),
        vec![Limb::Axis(peg(axe, 2).expect("+switch, peg failed!"))],
    );

    let relative_result = relative(axe, &one, dom, hay, cox, &vec![], &None, &None);

    Hoon::WutCol(Box::new(fits), Box::new(relative_result), Box::new(fin))
}

pub fn choice_(one: Spec,
            mut rep: Vec<Spec>,
            axe: u64,
            mod_: &Spec,
            dom: u64,
            hay: &WingType,
            cox: &HashMap<String, Spec>,
            bug: &Vec<Spot>,
            nut: &Option<Note>,
            def: &Option<Hoon>,
) -> Hoon {
    if rep.is_empty() {
        return relative(axe, &one, dom, hay, cox, &vec![], &None, &None);
    }

    let mut iter = rep.into_iter();
    let i_rep = iter.next().unwrap();
    let t_rep: Vec<Spec> = iter.collect();

    let example_res = example(&one.clone(), dom, hay, cox, &vec![], &None, &None);

    let fits = Hoon::Fits(
        Box::new(example_res),
        vec![Limb::Axis(axe)],
    );

    let relative_result =
            relative(axe,
                        &one.clone(),
                        dom,
                        hay,
                        cox,
                        &vec![],
                        &None,
                        &None);
    let tail = choice_(i_rep.clone(), t_rep, axe, mod_, dom, hay, cox, bug, nut, def);

    Hoon::WutCol(Box::new(fits), Box::new(relative_result), Box::new(tail))
}

pub fn relative(axe: u64,
                mod_: &Spec,
                dom: u64,
                hay: &WingType,
                cox: &HashMap<String, Spec>,
                bug: &Vec<Spot>,
                nut: &Option<Note>,
                def: &Option<Hoon>,
) -> Hoon {
    match &mod_ {
        Spec::Base(p) => decorate(basic(p.clone(), axe, mod_, dom, hay, cox, &vec![], &None, &None), bug.clone(), nut.clone()),
        Spec::Dbug(p, q) => {
            let mut bug = bug.clone();
            bug.push(p.clone());
            relative(axe, &*q, dom, hay, cox, &bug, nut, def)
        },
        Spec::Leaf(p, q) => {
            decorate(
                Hoon::WutGar(
                    Box::new(Hoon::DotTis(Box::new(Hoon::Axis(axe)),
                                          Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(q.clone()))))),
                    Box::new(Hoon::Rock(p.clone(), NounExpr::ParsedAtom(q.clone())))
                ),
                bug.clone(),
                nut.clone(),
            )
        }
        Spec::Make(p, q) => relative(axe, &Spec::BucMic(unfold(p.clone(), q.clone())), dom, hay, cox, bug, nut, def),
        Spec::Like(p, q) => relative(axe, &Spec::BucMic(unreel(p.clone(), q.clone())), dom, hay, cox, bug, nut, def),
        Spec::Loop(p) => decorate(
            Hoon::CenHep(Box::new(Hoon::Limb(p.clone())), Box::new(Hoon::Axis(axe))),
            bug.clone(),
            nut.clone(),
        ),
        Spec::Name(p, q) => relative(axe, &*q, dom, hay, cox, bug, &Some(Note::Made(p.clone(), None)), def),
        Spec::Made((term, list), q) => {
            let pieces = list
                        .iter()
                        .map(|s| vec![Limb::Term(s.to_string())])
                        .collect();
            let nut = Some(Note::Made(term.clone(), Some(pieces)));
            relative(axe, &*q, dom, hay, cox, bug, &nut, def)
        }
        Spec::Over(p, q) => relative(axe, &*q, dom, p, cox, bug, nut, def),
        Spec::BucBuc(p, q) => {
            let new_dom = peg(3, dom).expect("+relative-bucbuc-peg-failed");
            let map: HashMap<String, Hoon> = q.into_iter()
                .map(|(term, spec)| (term.clone(), relative(axe, spec, new_dom, hay, cox, bug, nut, def)))
                .collect();
            Hoon::BarKet(
                Box::new(relative(axe, &*p, new_dom, hay, cox, bug, nut, def)),
                HashMap::from([("$".to_string(), (None, map))]),
            )
        }
        Spec::BucPam(p, q) => Hoon::TisLus(
            Box::new(relative(axe, &*p, dom, hay, cox, bug, nut, def)),
            Box::new(Hoon::TisLus(
                Box::new(Hoon::TisGar(Box::new(Hoon::Axis(3)), Box::new(q.clone()))),
                Box::new(Hoon::TisLus(
                    Box::new(Hoon::CenHep(Box::new(Hoon::Axis(2)), Box::new(Hoon::Axis(6)))),
                    Box::new(Hoon::WutGar(
                        Box::new(Hoon::WutBar(vec![
                            Hoon::DotTis(Box::new(Hoon::Axis(14)), Box::new(Hoon::Axis(2))),
                            Hoon::DotTis(
                                Box::new(Hoon::Axis(2)),
                                Box::new(Hoon::CenHep(Box::new(Hoon::Axis(6)), Box::new(Hoon::Axis(2))))
                            )
                        ])),
                        Box::new(Hoon::Axis(2))
                    ))
                ))
            ))
        ),
        Spec::BucBar(p, q) => Hoon::TisLus(
            Box::new(relative(axe, &*p, dom, hay, cox, bug, nut, def)),
            Box::new(Hoon::WutGar(
                Box::new(Hoon::CenHep(Box::new(Hoon::TisGar(Box::new(Hoon::Axis(3)), Box::new(q.clone()))), Box::new(Hoon::Axis(2)))),
                Box::new(Hoon::Axis(2))
            ))
        ),
        Spec::BucCab(p) => decorate(home(p.clone(), hay.clone(), dom.clone()), bug.clone(), nut.clone()),
        Spec::BucCen(p, t) => decorate(switch(*p.clone(), t.clone(), axe, mod_, dom, hay, cox, bug, nut, def),
                                        bug.clone(),
                                        nut.clone()),
        Spec::BucCol(p, q) => {
            let mut result: Option<Hoon> = None;
            let mut current_axe = axe;

            let first = relative(
                peg(current_axe, 2).expect("+relative-buccol-peg-failed"),
                &*p,
                dom,
                hay,
                cox,
                bug,
                nut,
                def,
            );

            result = Some(first);
            current_axe = peg(current_axe, 3).expect("+relative-buccol-peg-failed");

            for spec in q {
                let hoon = relative(
                    peg(current_axe, 2).expect("+relative-buccol-peg-failed"),
                    spec,
                    dom,
                    hay,
                    cox,
                    bug,
                    nut,
                    def,
                );

                result = Some(Hoon::Pair(
                    Box::new(result.unwrap()),
                    Box::new(hoon),
                ));

                current_axe = peg(current_axe, 3).expect("+relative-buccol-peg-failed");
            }

            decorate(result.unwrap(), bug.clone(), nut.clone())
        }
        Spec::BucGal(p, q) => Hoon::TisLus(
            Box::new(relative(axe, &*q, dom, hay, cox, &vec![], &None, &None)),
            Box::new(Hoon::WutGal(
                Box::new(Hoon::WutTis(
                    Box::new(Spec::Over(vec![Limb::Axis(3)], p.clone())),
                    vec![Limb::Axis(4)]
                )),
                Box::new(Hoon::Axis(2))
            ))
        ),
        Spec::BucGar(p, q) => Hoon::TisLus(
            Box::new(relative(axe, &*q, dom, hay, cox, &vec![], &None, &None)),
            Box::new(Hoon::WutGar(
                Box::new(Hoon::WutTis(
                    Box::new(Spec::Over(vec![Limb::Axis(3)], p.clone())),
                    vec![Limb::Axis(4)],
                )),
                Box::new(Hoon::Axis(2))
            ))
        ),
        Spec::BucHep(p, q) => {
            let function_res = function(*p.clone(), *q.clone(), mod_, dom, hay, cox, &vec![], &None, &None);
            decorate(
                match def {
                    Some(d) => Hoon::KetLus(Box::new(function_res),
                                            Box::new(d.clone())),
                    None => function_res
                },
                bug.clone(),
                nut.clone(),
            )
        }
        Spec::BucKet(p, q) => decorate(
            Hoon::WutCol(
                Box::new(Hoon::DotWut(Box::new(Hoon::Axis(peg(axe, 2).expect("bucket-peg-failed"))))),
                Box::new(relative(axe, &*p, dom, hay, cox, &vec![], &None, &None)),
                Box::new(relative(axe, &*q, dom, hay, cox, &vec![], &None, &None))
            ),
            bug.clone(),
            nut.clone(),
        ),
        Spec::BucMic(p) => decorate(
            Hoon::CenCol(
                Box::new(home(p.clone(), hay.clone(), dom.clone())),
                vec![Hoon::Axis(axe)],
            ),
            bug.clone(),
            nut.clone(),
        ),
        Spec::BucSig(p, q) => relative(axe, &*q, dom, hay, cox, bug, nut, &Some(Hoon::KetHep(q.clone(), Box::new(p.clone())))),
        Spec::BucWut(p, t) => decorate(choice_(*p.clone(), t.clone(), axe, mod_, dom, hay, cox, bug, nut, def), bug.clone(), nut.clone()),
        Spec::BucTis(p, q) => Hoon::KetTis(p.clone(), Box::new(relative(axe, &*q, dom, hay, cox, bug, nut, def))),
        Spec::BucPat(p, q) => decorate(
            Hoon::WutCol(
                Box::new(Hoon::DotWut(Box::new(Hoon::Axis(axe)))),
                Box::new(relative(axe, &*q, dom, hay, cox, &vec![], &None, &None)),
                Box::new(relative(axe, &*p, dom, hay, cox, &vec![], &None, &None)),
            ),
            bug.clone(),
            nut.clone(),
        ),
        Spec::BucLus(p, q) => Hoon::Note(Note::Know(p.clone()),
                                        Box::new(relative(axe, &*q, dom, hay, cox, bug, nut, def))),
        Spec::BucDot(p, q) => {
            let x = interface(Vair::Gold, *p.clone(), q.clone(), mod_, dom, hay, cox, bug, nut, def);
            let y = home(x, hay.clone(), dom.clone());
            decorate(y, bug.clone(), nut.clone())
        }

        Spec::BucFas(p, q) => {
            let x = interface(Vair::Iron, *p.clone(), q.clone(), mod_, dom, hay, cox, bug, nut, def);
            let y = home(x, hay.clone(), dom.clone());
            decorate(y, bug.clone(), nut.clone())
        }

        Spec::BucZap(p, q) => {
            let x = interface(Vair::Lead, *p.clone(), q.clone(), mod_, dom, hay, cox, bug, nut, def);
            let y = home(x, hay.clone(), dom.clone());
            decorate(y, bug.clone(), nut.clone())
        }

        Spec::BucTic(p, q) => {
            let x = interface(Vair::Zinc, *p.clone(), q.clone(), mod_, dom, hay, cox, bug, nut, def);
            let y = home(x, hay.clone(), dom.clone());
            decorate(y, bug.clone(), nut.clone())
        }
    }
}

pub fn home(gen: Hoon,
            mut hay: WingType,
            dom: u64) -> Hoon {

    let wing = if  1 != dom {
        hay
    } else {
        hay.push(Limb::Axis(dom));
        hay
    };

    if wing.is_empty() {
        gen
    } else {
        Hoon::TisGar(Box::new(Hoon::Wing(wing)), Box::new(gen))
    }
}

pub fn unreel(one: WingType, res: Vec<WingType>) -> Hoon {
    if res.is_empty() {
        Hoon::Wing(one)
    } else {
        match res.first() {
            Some(first) => {
                let wing_tail = unreel(first.clone(), res[1..].to_vec());
                Hoon::TisGal(Box::new(Hoon::Wing(one)), Box::new(wing_tail))
            }
            None => Hoon::Wing(one),
        }
    }
}


pub fn unfold(fun: Hoon, arg: Vec<Spec>) -> Hoon {
    let cencol_tail: Vec<Hoon> = arg.iter().map(|spec| Hoon::KetCol(Box::new(spec.clone()))).collect();
    Hoon::CenCol(Box::new(fun), cencol_tail)
}

pub fn factory(mod_: Spec,
                dom: u64,
                hay: WingType,
                cox: HashMap<String, Spec>,
                bug: Vec<Spot>,
                nut: Option<Note>,
                def: Option<Hoon>) -> Hoon {
    match mod_ {
        Spec::Dbug(spot, spec) => {
            let mut bug = bug.clone();
            bug.insert(0, spot);
            factory(*spec, dom, hay, cox, bug, nut, def)
        }
        Spec::BucSig(hoon, spec) => {
            let spec_clone = spec.clone();
            let spec_clone2 = spec.clone();
            factory(*spec_clone, dom, hay, cox, bug, nut, Some(Hoon::KetHep(spec_clone2, Box::new(hoon))))
        }
        _ => {
            match (def.clone(), mod_.clone()) {
                (Some(_), Spec::BucMic(h)) => decorate(home(h, hay, dom), bug, nut),
                (Some(_), Spec::Like(wing, vec_wing)) => decorate(home(unreel(wing, vec_wing), hay, dom), bug, nut),
                (Some(_), Spec::Loop(term)) => decorate(home(Hoon::Limb(term), hay, dom), bug, nut),
                (Some(_), Spec::Make(h, s)) => decorate(home(unfold(h, s), hay, dom), bug, nut),
                _ => {
                    let spore_res = spore(mod_.clone(),
                                          dom.clone(),
                                          hay.clone(),
                                          cox.clone(),
                                          bug.clone(),
                                          nut.clone(),
                                          def.clone());

                    let ketsig = Box::new(Hoon::KetSig(Box::new(spore_res)));

                    let descent_axis = peg(7, dom).expect("factory-peg-failed");
                    let tislus =  Hoon::TisLus(Box::new(Hoon::DotTis(Box::new(Hoon::Axis(14)),
                                                            Box::new(Hoon::Axis(2)))),
                                               Box::new(Hoon::Axis(6)));
                    let relative_res = relative(6, &mod_, descent_axis, &hay, &cox, &bug, &nut, &def);
                    let tail = Hoon::TisLus(Box::new(relative_res),
                                            Box::new(tislus));

                    Hoon::BarCol(ketsig, Box::new(tail))
                }
            }
        }
    }
}

pub fn open(gen: Hoon) -> Hoon {
    match gen {
        Hoon::Axis(a) => Hoon::CenTis(vec![Limb::Axis(a)], Vec::new()),
        Hoon::Base(b) => factory(Spec::Base(b), 1, Vec::new(), HashMap::new(), Vec::new(), None, None),
        Hoon::Bust(b) => example(
            &Spec::Base(b), 1, &WingType::default(), &HashMap::new(), &Vec::new(), &None, &None,
        ),
        Hoon::Dbug(_, q) => *q,
        Hoon::Eror(s) => panic!("{}", s),
        Hoon::Knit(woofs) => {
            let ktts = Hoon::KetTis(Skin::Term("v".to_string()), Box::new(Hoon::Axis(1)));

            fn knit_loop(woofs: Vec<Woof>) -> Hoon {
                if woofs.is_empty() {
                    Hoon::Bust(BaseType::Null)
                } else {
                    let head = &woofs[0];
                    let tail = knit_loop(woofs[1..].to_vec());
                    match head {
                        Woof::ParsedAtom(a) => {
                            let sand = Hoon::Sand("tD".to_string(), NounExpr::ParsedAtom(a.clone()));
                            Hoon::Pair(Box::new(sand), Box::new(tail))
                        }
                        Woof::Hoon(p) => {
                            let a = Hoon::Pair(
                                        Box::new(Hoon::KetTis(
                                                Skin::Term("a".to_string()),
                                                Box::new(Hoon::KetLus(
                                                                Box::new(Hoon::Limb("$".to_string())),
                                                                Box::new(Hoon::TisGar(
                                                                        Box::new(Hoon::Limb("v".to_string())),
                                                                        Box::new(p.clone())))
                                                            )))),
                                        Box::new(Hoon::KetTis(Skin::Term("a".to_string()), Box::new(tail)))
                                    );
                            let b = Hoon::BarHep(
                                Box::new(
                                    Hoon::WutPat(
                                        vec![Limb::Term("a".to_string())],
                                        Box::new(Hoon::Limb("b".to_string())),
                                        Box::new(Hoon::Pair(
                                            Box::new(Hoon::TisGal(Box::new(Hoon::Axis(2)),
                                                                  Box::new(Hoon::Limb("a".to_string()))
                                                                )),
                                            Box::new(Hoon::CenTis(vec![Limb::Term("$".to_string())],
                                                                vec![(vec![Limb::Term("a".to_string())],
                                                                        Hoon::TisGal(Box::new(Hoon::Axis(3)),
                                                                            Box::new(Hoon::Limb("a".to_string())),
                                                                        ))])
                                                    ))
                                        )
                                    )
                                ));

                            Hoon::TisLus(
                                Box::new(a),
                                Box::new(b),
                            )

                        }
                    }
                }
            }

            let ktls =
                Hoon::KetLus(
                    Box::new(
                        Hoon::BarHep(Box::new(
                            Hoon::WutCol(
                                Box::new(Hoon::Bust(BaseType::Flag)),
                                Box::new(Hoon::Bust(BaseType::Null)),
                                Box::new(Hoon::Pair(
                                    Box::new(Hoon::KetTis(
                                        Skin::Term("i".to_string()),
                                        Box::new(Hoon::Sand("tD".to_string(),
                                                            NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
                                    )),
                                    Box::new(Hoon::KetTis(
                                        Skin::Term("t".to_string()),
                                        Box::new(Hoon::Limb("$".to_string())),
                                    )),
                                )),
                            )
                        ))),
                    Box::new(knit_loop(woofs))
                );

            let brhp = Hoon::BarHep(Box::new(ktls));

            Hoon::TisGar(
                Box::new(ktts),
                Box::new(brhp),
            )
        }
        Hoon::Leaf(term, atom) => factory(Spec::Leaf(term, atom), 1, Vec::new(), HashMap::new(), Vec::new(), None, None),
        Hoon::Limb(term) => Hoon::CenTis(vec![Limb::Term(term)], Vec::new()),
        Hoon::Wing(wing) => Hoon::CenTis(wing, Vec::new()),
        Hoon::Note(_, q) => *q,

        Hoon::Tell(hoons) => {
            let zpgr = Hoon::ZapGar(Box::new(Hoon::ColTar(hoons)));
            Hoon::CenCol(
                Box::new(Hoon::Limb("noah".to_string())),
                vec![zpgr],
            )
        }

        Hoon::Yell(hoons) => {
            let zpgr = Hoon::ZapGar(Box::new(Hoon::ColTar(hoons)));
            Hoon::CenCol(
                Box::new(Hoon::Limb("cain".to_string())),
                vec![zpgr],
            )
        }

        Hoon::BarBuc(sample, body) => {
            if sample.is_empty() {
                panic!("empty sample in BarBuc");
            }

            let tar = Spec::Base(BaseType::NounExpr);
            let bcsg = Spec::BucSig(
                Hoon::Base(BaseType::NounExpr),
                Box::new(Spec::BucHep(
                    Box::new(tar.clone()),
                    Box::new(tar),
                )),
            );

            let transformed: Vec<Spec> = sample
                .iter()
                .map(|term| Spec::BucTis(Skin::Term(term.clone()), Box::new(bcsg.clone())))
                .collect();

            let (first, rest) = transformed.split_first().unwrap();

            Hoon::BarTar(
                Box::new(Spec::BucCol(
                    Box::new(first.clone()),
                    rest.to_vec(),
                )),
                Box::new(Hoon::KetCol(Box::new(*body))),
            )
        }

        Hoon::BarCab(spec, alas, arms) => {
            let transformed_arms = arms
                .into_iter()
                .map(|(term, tome)| {
                    let (what, tome_map) = tome;
                    let wrapped_pairs: Vec<(String, Hoon)> = tome_map
                            .into_iter()
                            .map(|(face, expr)| {
                                let wrapped_expr = alas.iter().rev().fold(expr, |body, (alas_face, alas_init)| {
                                    Hoon::TisTar(
                                        (alas_face.clone(), None),
                                        Box::new(alas_init.clone()),
                                        Box::new(body),
                                    )
                                });
                                (face, wrapped_expr)
                            })
                            .collect();

                    let tome_map: HashMap<_, _> = wrapped_pairs.into_iter().collect();

                    (term, (what, tome_map))
                })
                .collect();

            Hoon::TisLus(
                Box::new(Hoon::KetTar(spec)),
                Box::new(Hoon::BarCen(None, transformed_arms)),
            )
        }

        Hoon::BarCol(p, q) => Hoon::TisLus(p, Box::new(Hoon::BarDot(q))),

        Hoon::BarDot(p) => {
            let map_term_hoon = {
                let mut m = HashMap::new();
                m.insert("$".to_string(), *p);
                m
            };
            let map_term_tome = {
                let mut m = HashMap::new();
                m.insert("$".to_string(), (None, map_term_hoon));
                m
            };
            Hoon::BarCen(None, map_term_tome)
        }

        Hoon::BarKet(p, arms) => {
            let mut map = arms.clone();
            if let Some(zil) = arms.get(&"$".to_string()) {
                let updated = {
                    let (what, mut inner) = zil.clone();
                    inner.insert("$".to_string(), *p.clone());
                    (what, inner)
                };
                map.insert("$".to_string(), updated);
            } else {
                let mut inner = HashMap::new();
                inner.insert("$".to_string(), *p.clone());
                map.insert("$".to_string(), (None, inner));
            }
            Hoon::TisGal(
                Box::new(Hoon::Limb("$".to_string())),
                Box::new(Hoon::BarCen(None, map)),
            )
        }

        Hoon::BarHep(p) => Hoon::TisGal(Box::new(Hoon::Limb("$".to_string())), Box::new(Hoon::BarDot(Box::new(*p)))),

        Hoon::BarSig(spec, q) => Hoon::KetBar(Box::new(Hoon::BarTis(spec.clone(), q.clone()))),

        Hoon::BarTar(spec, q) => {
            let map_term_hoon = {
                let mut m = HashMap::new();
                m.insert("$".to_string(), *q);
                m
            };
            let map_term_tome = {
                let mut m = HashMap::new();
                m.insert("$".to_string(), (None, map_term_hoon));
                m
            };
            Hoon::TisLus(Box::new(Hoon::KetTar(spec)),
                        Box::new(Hoon::BarPat(None, map_term_tome)))
        }

        Hoon::BarTis(spec, q) => {
            let map_term_hoon = {
                let mut m = HashMap::new();
                m.insert("$".to_string(), *q);
                m
            };
            let map_term_tome = {
                let mut m = HashMap::new();
                m.insert("$".to_string(), (None, map_term_hoon));
                m
            };
            Hoon::BarCab(spec, vec![], map_term_tome)
        }

        Hoon::BarWut(p) => Hoon::KetWut(Box::new(Hoon::BarDot(p))),

        Hoon::ColKet(p, q, r, s) => {
               Hoon::Pair(
                    p,
                    Box::new(Hoon::Pair(
                        q,
                        Box::new(Hoon::Pair(
                            r,
                            s
                        ))
                    ))
                )
            }

        Hoon::ColCab(p, q) => Hoon::Pair(q, p),

        Hoon::ColHep(p, q) => Hoon::Pair(p, q),

        Hoon::ColLus(p, q, r) => {
            Hoon::Pair(
                    p,
                    Box::new(
                        Hoon::Pair(
                            q,
                            r,
                        )
                    )
                )
        }

        Hoon::ColSig(hoons) => {
            match hoons.as_slice() {
                [] => Hoon::Rock("n".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
                [h] => h.clone(),
                [h, tail @ ..] => {
                    let rest = open(Hoon::ColSig(tail.to_vec()));
                    Hoon::Pair(Box::new(h.clone()), Box::new(rest))
                }
            }
        }

        Hoon::ColTar(hoons) => {
            match hoons.as_slice() {
                [] => Hoon::ZapZap,
                [h] => h.clone(),
                [h, tail @ ..] => {
                    let rest = open(Hoon::ColTar(tail.to_vec()));
                    Hoon::Pair(Box::new(h.clone()), Box::new(rest))
                }
            }
        }
        Hoon::KetTar(spec) => Hoon::KetSig(
                                    Box::new(example(&spec, 1, &Vec::new(), &HashMap::new(), &Vec::new(), &None, &None))),

        Hoon::CenCab(wing, pairs) => {
            Hoon::KetLus(Box::new(Hoon::Wing(wing.clone())),
                        Box::new(Hoon::CenTis(wing, pairs)))
        }

        Hoon::CenDot(p, q) => Hoon::CenCol(q, vec![*p]),

        Hoon::CenKet(p, q, r, s) => Hoon::CenCol(p, vec![*q, *r, *s]),

        Hoon::CenLus(p, q, r) => Hoon::CenCol(p, vec![*q, *r]),

        Hoon::CenHep(p, q) => Hoon::CenCol(p, vec![*q]),

        Hoon::CenCol(p, hoons) => {
            Hoon::CenSig(vec![Limb::Term("$".to_string())], p, hoons)
        }

        Hoon::CenSig(wing, p, hoons) => {
            fn compile_r_gen_rec(r_gen: &[Hoon], axe: u64) -> Vec<(Vec<Limb>, Hoon)> {
                match r_gen.split_first() {
                    None => vec![],
                    Some((hoon, rest)) => {
                        let (wing_axe, next_axe) = if rest.is_empty() {
                            (axe, 0)
                        } else {
                            (peg(axe, 2).expect("+open: peg failed"), peg(axe, 3).expect("+open: peg failed"))
                        };

                        let wing = vec![
                            Limb::Parent(0, None),
                            Limb::Axis(wing_axe),
                        ];

                        let mut out = vec![(wing, hoon.clone())];
                        if !rest.is_empty() {
                            out.extend(compile_r_gen_rec(rest, next_axe));
                        }
                        out
                    }
                }
            }
            let list = compile_r_gen_rec(&hoons, 6);
            Hoon::CenTar(wing, p, list)
        }

        Hoon::CenTar(mut wing, p, pairs) => {
            if pairs.is_empty() {
               return Hoon::TisGar(p, Box::new(Hoon::Wing(wing)));
            }
            wing.extend(vec![Limb::Axis(2)]);
            let wrapped = pairs
                    .into_iter()
                    .map(|(p, q)| (p, Hoon::TisGar(Box::new(Hoon::Axis(3)), Box::new(q))))
                    .collect();
            Hoon::TisLus(p,
                    Box::new(
                        Hoon::CenTis(wing, wrapped)
                    ))
        }

        Hoon::KetDot(p, q) => Hoon::KetLus(Box::new(Hoon::CenCol(p, vec![*q.clone()])), q),

        Hoon::KetHep(spec, q) => {
            let example_res =
                example(&spec, 1, &Vec::new(), &HashMap::new(), &Vec::new(), &None, &None);
            Hoon::KetLus(Box::new(example_res), q)
        }

        Hoon::KetTis(skin, p) => grip(skin, *p, vec![]),


        Hoon::SigBar(p, q) => {
            let fek = {
                let fek = feck(*p.clone());
                match fek {
                    Some(s) => Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(s)),
                    None => {
                        Hoon::BarDot(Box::new(Hoon::CenCol(
                            Box::new(Hoon::Limb("cain".to_string())),
                            vec![Hoon::ZapGar(Box::new(Hoon::TisGal(
                                              Box::new(Hoon::Axis(3)), p)))])))
                    }
                }
            };
            let hint = TermOrPair::Pair("mean".to_string(), Box::new(fek));
            Hoon::SigGar(hint, q)
        }

        Hoon::SigCab(p, q) => Hoon::SigGar(
            TermOrPair::Term("mean".to_string()),
            Box::new(Hoon::BarDot(p)),
        ),

        Hoon::SigCen(chum, p, tyre, q) => {
            let clsg_vec = {
                let mut nob = vec![];
                let mut r = tyre;
                while !r.is_empty() {
                    let (p_i, q_i) = r.remove(0);
                    nob.push(Hoon::Pair(
                        Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(string_to_atom(p_i)))),
                        Box::new(Hoon::ZapTis(Box::new(q_i))),
                    ));
                }
                nob
            };
            let clls =
                Hoon::ColLus(
                    Box::new(Hoon::Rock("$".to_string(), chum_to_nounexpr(chum))),
                    Box::new(Hoon::ZapTis(q.clone())),
                    Box::new(Hoon::ColSig(clsg_vec)),
                );
            Hoon::SigGal(
                TermOrPair::Pair("fast".to_string(), Box::new(clls)),
                q,
            )
        }

        Hoon::SigFas(chum, q) => Hoon::SigCen(chum, Box::new(Hoon::Axis(7)), vec![], q),

        Hoon::SigGal(term_or_pair, q) => Hoon::TisGal(Box::new(Hoon::SigGar(term_or_pair, Box::new( Hoon::Axis(1)))), q),

        Hoon::SigBuc(term, q) => Hoon::SigGar(
            TermOrPair::Pair("live".to_string(), Box::new(Hoon::Rock("$".to_string(), NounExpr::ParsedAtom(string_to_atom(term))))),
            q
        ),

        Hoon::SigLus(a, q) => Hoon::SigGar(
            TermOrPair::Pair("memo".to_string(),
                            Box::new(Hoon::Rock("$".to_string(),
                                                NounExpr::ParsedAtom(ParsedAtom::Small(a.into()))))),
            q
        ),

        Hoon::SigPam(a, p, q) => Hoon::SigGar(
            TermOrPair::Pair(
                "slog".to_string(),
                Box::new(Hoon::Pair(
                    Box::new(Hoon::Sand("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(a.into())))),
                    Box::new(Hoon::CenCol(
                    Box::new(Hoon::Limb("cain".to_string())),
                    vec![Hoon::ZapGar(p)]))))
            ),
            q,
        ),

        Hoon::SigTis(p, q) => Hoon::SigGar(
            TermOrPair::Pair("germ".to_string(), p),
            q,
        ),

        Hoon::SigWut(a, p, q, r) => {
            let wtdt = Hoon::WutDot(p, Box::new(Hoon::Bust(BaseType::Null)), Box::new(Hoon::Pair(Box::new(Hoon::Bust(BaseType::Null)),
                                                                                                 Box::new(*q))));
            let sgpm = Hoon::SigPam(a, Box::new(Hoon::Axis(5)), Box::new(Hoon::TisGar(Box::new(Hoon::Axis(3)), r.clone())));
            let wtsg = Hoon::WutSig(vec![Limb::Axis(2)], Box::new(Hoon::TisGar(Box::new(Hoon::Axis(3)), r)), Box::new(sgpm));
            Hoon::TisLus(
                Box::new(wtdt),
                Box::new(wtsg),
                )
        }

        Hoon::MicTis(marl) => {
            fn loop_marl(marl: Marl) -> Hoon {
                match marl.split_first() {
                    None => Hoon::Bust(BaseType::Null),
                    Some((head, tail)) => match head {
                        Tuna::Manx(m) => {
                            Hoon::Pair(Box::new(Hoon::Xray(m.clone())), Box::new(loop_marl(tail.to_vec())))
                        },
                        Tuna::ManxHoon(m) => Hoon::Pair(Box::new(m.clone()), Box::new(loop_marl(tail.to_vec()))),
                        Tuna::Tape(t) => Hoon::Pair(Box::new(Hoon::MicFas(Box::new(t.clone()))),
                                                        Box::new(loop_marl(tail.to_vec()))),
                        Tuna::Call(h) => Hoon::CenCol(Box::new(h.clone()), vec![loop_marl(tail.to_vec())]),
                        Tuna::Marl(sub) => {
                            let tsbr = Box::new(Hoon::TisBar(
                                Box::new(Spec::Base(BaseType::Cell)),
                                Box::new(Hoon::BarPat(None, {
                                    let sug = vec![Limb::Axis(12)];
                                    let wtsg = Hoon::WutSig(sug.clone(),
                                                            Box::new(Hoon::CenTis(sug.clone(), vec![(vec![Limb::Axis(1)], Hoon::Axis(13))])),
                                                            Box::new(Hoon::CenTis(sug.clone(),
                                                                vec![(vec![Limb::Axis(3)],
                                                                    Hoon::CenTis(vec![Limb::Term("$".to_string())],
                                                                        vec![(sug, Hoon::Axis(25))]
                                                                    ))]))
                                                        );
                                    let map_term_hoon = {
                                        let mut m = HashMap::new();
                                        m.insert("$".to_string(), wtsg);
                                        m
                                    };
                                    let map_term_tome = {
                                        let mut m = HashMap::new();
                                        m.insert("$".to_string(), (None, map_term_hoon));
                                        m
                                    };
                                    map_term_tome
                                }))),
                            );
                            Hoon::CenDot(Box::new(Hoon::Pair(Box::new(sub.clone()),
                                                        Box::new(loop_marl(tail.to_vec())))), tsbr)
                        }
                    }
                }
            }
            loop_marl(marl)
        }

        Hoon::MicCol(p, hoons) => {
            match hoons.as_slice() {
                [] => Hoon::ZapZap,
                [h] => h.clone(),
                [h, tail @ ..] => {
                    let yex = hoons;
                    fn loop_yex(yex: &[Hoon]) -> Hoon {
                        match yex {
                            [] => panic!("empty yex"),
                            [h] => Hoon::TisGal(Box::new(Hoon::Axis(3)), Box::new(h.clone())),
                            [h, t @ ..] => Hoon::CenCol(
                                Box::new(Hoon::Axis(2)),
                                vec![Hoon::TisGar(Box::new(Hoon::Axis(3)), Box::new(h.clone())),
                                loop_yex(t)]),
                            _ => panic!("miccol error"),
                        }
                    }
                    Hoon::TisLus(p, Box::new(loop_yex(&yex)))
                }
            }
        }

        Hoon::MicFas(p) => {
            let zoy = Hoon::Rock("ta".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)));
            Hoon::ColSig(vec![Hoon::Pair(
                                Box::new(zoy.clone()),
                                 Box::new(Hoon::ColSig(vec![Hoon::Pair(
                                        Box::new(zoy.clone()),
                                        p.clone())])))])
        }

        Hoon::MicGal(spec, q, r, s) => {
            let ktcl_p = Hoon::KetCol(spec.clone());
            let cnhp = Hoon::CenHep(q, Box::new(ktcl_p));
            let brts = Hoon::BarTis(spec, Box::new(Hoon::TisGar(Box::new(Hoon::Axis(3)), s)));
            Hoon::CenLus(
                Box::new(cnhp),
                r,
                Box::new(brts)
            )
        }

        Hoon::MicSig(p, q) => {
            fn loop_tail(p: Box<Hoon>, q: Vec<Hoon>) -> Hoon {
                match q.as_slice() {
                    [] => {
                        panic!("open-mcsg")
                    }
                    [first, rest @ ..] => {
                        if rest.is_empty() {
                            return Hoon::TisGar(Box::new(Hoon::Limb("v".to_string())), Box::new(first.clone()));
                        }
                        let a_bind = Hoon::KetTis(Skin::Term("a".to_string()),
                                                    Box::new(loop_tail(p.clone(), rest.to_vec())));

                        let b_expr = Hoon::TisGar(
                            Box::new(Hoon::Limb("v".to_string())),
                            Box::new(first.clone()),
                        );
                        let b_bind =
                            Hoon::KetTis(
                            Skin::Term("b".to_string()),
                                Box::new(Hoon::TisGar(Box::new(Hoon::Limb("v".to_string())), Box::new(first.clone())))
                            );

                        let wing_c = vec![
                            Limb::Parent(0, None),
                            Limb::Axis(6),
                        ];
                        let c_expr = Hoon::TisGal(
                            Box::new(Hoon::Wing(wing_c)),
                            Box::new(Hoon::Limb("b".to_string())),
                        );
                        let c_bind =
                            Hoon::KetTis(
                                Skin::Term("c".to_string()),
                                Box::new(Hoon::TisGal(
                                            Box::new(Hoon::Wing(vec![Limb::Parent(0, None), Limb::Axis(6)])),
                                            Box::new(Hoon::Limb("b".to_string())))));

                        let tsgr_v_p = Hoon::TisGar(
                            Box::new(Hoon::Limb("v".to_string())),
                            p.clone(),
                        );
                        let cncl_b_c = Hoon::CenCol(
                            Box::new(Hoon::Limb("b".to_string())),
                            vec![Hoon::Limb("c".to_string())],
                        );
                        let cnts_wing = vec![
                            Limb::Parent(0, None),
                            Limb::Axis(6),
                        ];
                        let cnts = Hoon::CenTis(
                            vec![Limb::Term("a".to_string())],
                            vec![(cnts_wing, Hoon::Limb("c".to_string()))],
                        );
                        let cnls = Hoon::CenLus(
                            Box::new(tsgr_v_p),
                            Box::new(cncl_b_c),
                            Box::new(cnts),
                        );

                        Hoon::TisLus(
                            Box::new(a_bind),
                            Box::new(Hoon::TisLus(
                                Box::new(b_bind),
                                Box::new(Hoon::TisLus(
                                    Box::new(c_bind),
                                    Box::new(Hoon::BarDot(
                                        Box::new(cnls),
                                    ))
                                ))
                            ))
                        )
                    }
                }
            }

            let tail = loop_tail(p, q);

            Hoon::TisGar(
                Box::new(Hoon::KetTis(Skin::Term("$".to_string()), Box::new(Hoon::Axis(1)))),
                Box::new(tail),
            )
        },

        Hoon::MicMic(spec, q) => Hoon::CenHep(
            Box::new(factory(*spec, 1, Vec::new(), HashMap::new(), Vec::new(), None, None)),
            q,
        ),

        Hoon::TisBar(spec, q) => Hoon::TisLus(Box::new(Hoon::KetTar(spec)), q),

        Hoon::TisTar((term, spec_opt), p, q) => {
            let inner = match spec_opt {
                None => *p,
                Some(spec_box) => Hoon::KetHep(spec_box, p),
            };
            let mut m = HashMap::new();
            m.insert(term, Some(inner));
            Hoon::TisGal(q, Box::new(Hoon::Tune(TermOrTune::Tune((m, vec![])))))
        }

        Hoon::TisCol(pairs, q) => {
            let wing = vec![Limb::Term("$".to_string())];
            Hoon::TisGar(Box::new(Hoon::CenCab(wing, pairs)), q)
        }

        Hoon::TisFas(skin, p, q) => Hoon::TisLus(Box::new(Hoon::KetTis(skin, p)), q),

        Hoon::TisMic(skin, p, q) => Hoon::TisFas(skin, q, p),

        Hoon::TisDot(wing, p, q) => Hoon::TisGar(Box::new(Hoon::CenCab(vec![Limb::Axis(1)], vec![(wing, *p)])), q),

        Hoon::TisWut(wing, p, q, r) => {
            let wtcl = Hoon::WutCol(p, q, Box::new(Hoon::Wing(wing.clone())));
            Hoon::TisDot(wing, Box::new(wtcl), r)
        }

        Hoon::TisGal(p, q) => Hoon::TisGar(q, p),

        Hoon::TisHep(p, q) => Hoon::TisLus(q, p),

        Hoon::TisKet(skin, wing, p, q) => {
            let wuy = weld(wing.clone(), vec![Limb::Term("v".to_string())]);
            let v_bind =
                Hoon::KetTis(Skin::Term("v".to_string()),  Box::new(Hoon::Axis(1)));
            let a_bind =
                Hoon::KetTis(Skin::Term("a".to_string()),
                        Box::new(Hoon::TisGar(
                            Box::new(Hoon::Limb("v".to_string())), p.clone())));
            let tsdt =
                Box::new(Hoon::TisDot(
                    wuy.clone(),
                    Box::new(Hoon::TisGal(
                        Box::new(Hoon::Axis(3)),
                        Box::new(Hoon::Limb("a".to_string())),
                    )),
                    Box::new(Hoon::TisGar(
                        Box::new(Hoon::Pair(
                            Box::new(Hoon::KetTis(
                                Skin::Over(vec![Limb::Term("v".to_string())],  Box::new(skin)),
                                Box::new(Hoon::TisGal(
                                    Box::new(Hoon::Axis(2)),
                                    Box::new(Hoon::Limb("a".to_string())),
                                )))),
                            Box::new(Hoon::Limb("v".to_string())),
                        )),
                        q
                    )),
                ));
            Hoon::TisGar(
                Box::new(v_bind),
                Box::new(Hoon::TisLus(
                    Box::new(a_bind),
                    tsdt,
                )))
        }

        Hoon::TisLus(p, q) => Hoon::TisGar(Box::new(
                                            Hoon::Pair(p,
                                                        Box::new(Hoon::Axis(1)))),
                                                    q),

        Hoon::TisSig(hoons) => {
            match hoons.as_slice() {
                [] => Hoon::Axis(1),
                [h] => h.clone(),
                [h, tail @ ..] => {
                    let rest = open(Hoon::TisSig(tail.to_vec()));
                    Hoon::TisGar(Box::new(h.clone()), Box::new(rest))
                }
            }
        }
        Hoon::WutBar(p) => {
            match p.as_slice() {
                [] => Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1))),
                [head, tail @ ..] => {
                    let recurse = open(Hoon::WutBar(tail.to_vec()));
                    Hoon::WutCol(
                        Box::new(head.clone()),
                        Box::new(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
                        Box::new(recurse),
                    )
                }
            }
        },

    Hoon::WutDot(p, q, r) => {
        Hoon::WutCol(
            Box::new(*p),
            r,
            q,
        )
    },

    Hoon::WutGal(p, q) => {
        Hoon::WutCol(
            Box::new(*p),
            Box::new(Hoon::ZapZap),
            q,
        )
    },

    Hoon::WutGar(p, q) => {
        Hoon::WutCol(
            Box::new(*p),
            q,
            Box::new(Hoon::ZapZap),
        )
    },

    Hoon::WutKet(p, q, r) => {
        let wuttis = Hoon::WutTis(
            Box::new(Spec::Base(BaseType::Atom("$".to_string()))),
            p,
        );
        Hoon::WutCol(
            Box::new(wuttis),
            r,
            q,
        )
    },

    Hoon::WutHep(p, q) => {
        match q.as_slice() {
            [] => {
                Hoon::Lost(Box::new(Hoon::Wing(p)))
            }
            [(spec, head), tail @ ..] => {
                let wtts = Hoon::WutTis(Box::new(spec.clone()), p.clone());
                let recurse = open(Hoon::WutHep(p.clone(), tail.to_vec()));
                Hoon::WutCol(
                    Box::new(wtts),
                    Box::new(head.clone()),
                    Box::new(recurse),
                )
            }
        }
    },

    Hoon::WutLus(p, q, r) => {
        let mut new_r = r.clone();
        new_r.push((Spec::Base(BaseType::NounExpr), *q));
        Hoon::WutHep(p, new_r)
    },

    Hoon::WutPam(p) => {
        match p.as_slice() {
            [] => Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0))),
            [head, tail @ ..] => {
                let recurse = open(Hoon::WutPam(tail.to_vec()));
                Hoon::WutCol(
                    Box::new(head.clone()),
                    Box::new(recurse),
                    Box::new(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1)))),
                )
            }
        }
    },

    Hoon::Xray(manx) => {
        let open_mane = match &manx.g.n {
            Mane::Tag(s) => Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(s.clone()))),
            Mane::TagSpace(a, b) => {
                let left = Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(a.clone())));
                let right = Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(b.clone())));
                Hoon::Pair(Box::new(left), Box::new(right))
            }
        };

        let clsg_items: Vec<Hoon> = manx.g.a
            .iter()
            .map(|(mane, beers)| {
                let n_hoon = match &mane {
                    Mane::Tag(s) => Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(s.clone()))),
                    Mane::TagSpace(a, b) => {
                        let left = Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(a.clone())));
                        let right = Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(b.clone())));
                        Hoon::Pair(Box::new(left), Box::new(right))
                    }
                };
                let woofs: Vec<Woof> = beers
                    .iter()
                    .map(|b| match b {
                        Beer::Char(cord) => Woof::ParsedAtom(cord.clone()),
                        Beer::Hoon(hoon) => Woof::Hoon(hoon.clone()),
                    })
                    .collect();

                Hoon::Pair(
                    Box::new(n_hoon),
                    Box::new(Hoon::Knit(woofs)),
                )
            })
            .collect();

        let clsg = Hoon::ColSig(clsg_items);
        let head = Hoon::Pair(Box::new(open_mane), Box::new(clsg));
        let tail = Hoon::MicTis(manx.c);

        Hoon::Pair(Box::new(head), Box::new(tail))
    },

    Hoon::WutPat(p, q, r) => {
        let wtts = Hoon::WutTis(
            Box::new(Spec::Base(BaseType::Atom("$".to_string()))),
            p,
        );
        Hoon::WutCol(
            Box::new(wtts),
            q,
            r,
        )
    },

    Hoon::WutSig(p, q, r) => {
        let wtts = Hoon::WutTis(
            Box::new(Spec::Base(BaseType::Null)),
            p,
        );
        Hoon::WutCol(
            Box::new(wtts),
            q,
            r,
        )
    },

    Hoon::WutTis(spec, q) => {
        let example_res = example(&spec, 1, &Vec::new(), &HashMap::new(), &Vec::new(), &None, &None);
        Hoon::Fits(
            Box::new(example_res),
            q,
        )
    },

    Hoon::WutZap(p) => {
        Hoon::WutCol(
            p,
            Box::new(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1)))),
            Box::new(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
        )
    },

    Hoon::ZapGar(p) => {
        let limb_onan = Hoon::Limb("onan".to_string());
        let limb_abel = Hoon::Limb("abel".to_string());
        let bcmc = Spec::BucMic(limb_abel);
        let kttr = Hoon::KetTar(Box::new(bcmc));
        let zpmc = Hoon::ZapMic(Box::new(kttr), p);

        Hoon::CenCol(Box::new(limb_onan), vec![zpmc])
    },

    Hoon::ZapWut(arg, q) => {
        const HOON_VERSION: u64 = 138;  // hardcoded...

        let version_ok = match &arg {
            ZpwtArg::ParsedAtom(s) => {
                s.parse::<u64>().map_or(false, |v| HOON_VERSION <= v)
            }
            ZpwtArg::Pair(min_s, max_s) => {
                match (min_s.parse::<u64>(), max_s.parse::<u64>()) {
                    (Ok(min), Ok(max)) => min <= HOON_VERSION && HOON_VERSION <= max,
                    _ => false,
                }
            }
        };

        if version_ok {
            *q
        } else {
            panic!("hoon-version")
        }
    },

        _ => gen,
    }
}

pub fn chum_to_nounexpr(chum: Chum) -> NounExpr {
    match chum {
        Chum::Lef(term) => {
            NounExpr::ParsedAtom(string_to_atom(term))
        }
        Chum::StdKel(term, u) => {
            NounExpr::Cell(
                Box::new(NounExpr::ParsedAtom(string_to_atom(term))),
                Box::new(NounExpr::ParsedAtom(u)),
            )
        }
        Chum::VenProKel(t1, t2, u) => {
            NounExpr::Cell(
                Box::new(NounExpr::ParsedAtom(string_to_atom(t1))),
                Box::new(NounExpr::Cell(
                    Box::new(NounExpr::ParsedAtom(string_to_atom(t2))),
                    Box::new(NounExpr::ParsedAtom(u)),
                )),
            )
        }
        Chum::VenProVerKel(t1, t2, u1, u2) => {
            NounExpr::Cell(
                Box::new(NounExpr::ParsedAtom(string_to_atom(t1))),
                Box::new(NounExpr::Cell(
                    Box::new(NounExpr::ParsedAtom(string_to_atom(t2))),
                    Box::new(NounExpr::Cell(
                        Box::new(NounExpr::ParsedAtom(u1)),
                        Box::new(NounExpr::ParsedAtom(u2)),
                    )),
                )),
            )
        }
    }
}

pub fn feck(gen: Hoon) -> Option<ParsedAtom> {
    match gen {
        Hoon::Sand(term, noun) => {
            if term == "tas" {
                match noun {
                    NounExpr::ParsedAtom(s) => Some(s),
                    NounExpr::Cell(_, _) => None,
                }
            } else {
                None
            }
        }

        Hoon::Dbug(_spot, expr) => feck(*expr),

        _ => None,
    }
}

pub fn grip(skin: Skin, gen: Hoon, rel: WingType) -> Hoon {
    match skin {
        Skin::Term(term) => {
            Hoon::TisGal(
                Box::new(Hoon::Tune(TermOrTune::Term(term))),
                Box::new(gen),
            )
        }

        Skin::Base(base) => {
            if base == BaseType::NounExpr {
                gen
            } else {
                Hoon::KetHep(
                    Box::new(Spec::Base(base)),
                    Box::new(gen),
                )
            }
        }

        Skin::Cell(car_skin, cdr_skin) => {
            let haf = half(gen.clone());
            match haf {
                None => {
                    let car_gen = Hoon::Axis(4);
                    let cdr_gen = Hoon::Axis(5);
                    let pair = Hoon::Pair(
                        Box::new(grip(*car_skin, car_gen, rel.clone())),
                        Box::new(grip(*cdr_skin, cdr_gen, rel.clone())),
                    );
                    Hoon::TisLus(Box::new(gen), Box::new(pair))
                }
                Some((p, q)) => {
                    Hoon::Pair(
                        Box::new(grip(*car_skin, p, rel.clone())),
                        Box::new(grip(*cdr_skin, q, rel.clone())),
                    )
                }
            }
        }

        Skin::Dbug(spot, inner_skin) => {
            Hoon::Dbug(
                spot,
                Box::new(grip(*inner_skin, gen, rel)),
            )
        }

        Skin::Leaf(aura, atom) => {
            Hoon::KetHep(
                Box::new(Spec::Leaf(aura, atom)),
                Box::new(gen),
            )
        }

        Skin::Name(term, inner_skin) => {
            Hoon::TisGal(
                Box::new(Hoon::Tune(TermOrTune::Term(term))),
                Box::new(grip(*inner_skin, gen, rel)),
            )
        }

        Skin::Over(mut wing, inner_skin) => {
            wing.extend(rel);
            grip(*inner_skin, gen, wing)
        }

        Skin::Spec(spec, inner_skin) => {
            let check_skin = if rel.is_empty() {
                spec
            } else {
                Box::new(Spec::Over(rel.clone(), spec))
            };

            let inner = grip(*inner_skin, gen, rel);

            Hoon::KetHep(
                check_skin,
                Box::new(inner),
            )
        }

        Skin::Wash(depth) => {
            let wing: WingType = (0..depth)
                    .map(|_| Limb::Parent(0, None))
                    .collect();
            Hoon::TisGal(
                Box::new(Hoon::Wing(wing)),
                Box::new(gen),
            )
        }
    }
}

pub fn half(gen: Hoon) -> Option<(Hoon, Hoon)> {
    match gen {
         Hoon::Pair(car, cdr) => {
            Some((*car, *cdr))
        }

        Hoon::Dbug(_spot, expr) => {
            half(*expr)
        }

        Hoon::ColCab(car, cdr) => {
            Some((*cdr, *car))
        }

        Hoon::ColHep(car, cdr) => {
            Some((*car, *cdr))
        }

        Hoon::ColKet(a, b, c, d) => {
            let tail = Hoon::ColLus(b, c, d);
            Some((*a, tail))
        }

        Hoon::ColSig(mut items) => {
            if items.is_empty() {
                None
            } else {
                let head = items.remove(0);
                Some((head, Hoon::ColSig(items)))
            }
        }

        Hoon::ColTar(mut items) => {
            if items.is_empty() {
                None
            } else if items.len() == 1 {
                half(items.remove(0))
            } else {
                let head = items.remove(0);
                let tail = Hoon::ColTar(items);
                Some((head, tail))
            }
        }

        _ => None,
    }
}

pub fn reek(gen: Hoon) -> Option<WingType> {
    match gen {
        Hoon::Pair(p, _q) => {
            match *p {
                Hoon::Axis(a) => Some(vec![Limb::Axis(a)]),
                _ => None,
            }
        }
        Hoon::Limb(t) => Some(vec![Limb::Term(t.clone())]),
        Hoon::Wing(w) => Some(w.to_vec()),
        Hoon::Dbug(_s, h) => reek(*h),
        _ => None
    }
}

pub fn name_ax(gen: Hoon) ->  Option<String> {
    match gen {
        Hoon::Wing(p) => {
            if p.is_empty() {
                None
            } else if let Some(i) = p.first() {
                match i {
                    Limb::Axis(_) => None,
                    Limb::Term(q) =>  Some(q.to_string()),
                    Limb::Parent(_, q) => q.clone(),
                }
            } else {
                None
            }
        }
        Hoon::Limb(p) => Some(p),
        Hoon::Dbug(_, q) => name_ax(*q),
        Hoon::TisGal(p, q) => name_ax(Hoon::TisGar(q, p)),
        Hoon::TisGar(_, q) => name_ax(*q),
        _ => None
    }
}

pub fn autoname(mod_spec: Spec) -> Option<String> {  //  ++autoname:ax
    match mod_spec {
        Spec::Base(base) => match base {
            BaseType::Atom(aura) => {
                if aura == "$" {    //  how empty terms will be represented here in rust land?...
                    Some("atom".to_string())
                } else {
                    Some(aura)
                }
            }
            _ => None,
        },
        Spec::Dbug(_, q) => autoname(*q),
        Spec::Leaf(p, _) => Some(p),
        Spec::Loop(p) => Some(p),
        Spec::Like(wing, _list_wing) => {
            if wing.is_empty() {
                None
            } else if let Some(i) = wing.first() {
                match i {
                    Limb::Axis(_) => None,
                    Limb::Term(q) =>  Some(q.to_string()),
                    Limb::Parent(_, q) => q.clone(),
                }
            } else {
                None
            }
        },
        Spec::Make(p, _) => name_ax(p),
        Spec::Made(_, q) => autoname(*q),
        Spec::Name(_, q) => autoname(*q),
        Spec::Over(_, q) => autoname(*q),
        Spec::BucBuc(p, _) => autoname(*p),
        Spec::BucBar(p, _) => autoname(*p),
        Spec::BucCab(p) => name_ax(p),
        Spec::BucCol(i, _) => autoname(*i),
        Spec::BucCen(i, _) => autoname(*i),
        Spec::BucDot(_, _) => None,
        Spec::BucGal(_, q) => autoname(*q),
        Spec::BucGar(_, q) => autoname(*q),
        Spec::BucHep(p, _) => autoname(*p),
        Spec::BucKet(_, q) => autoname(*q),
        Spec::BucLus(_, q) => autoname(*q),
        Spec::BucFas(_, _) => None,
        Spec::BucMic(p) => name_ax(p),
        Spec::BucPam(p, _) => autoname(*p),
        Spec::BucSig(_, q) => autoname(*q),
        Spec::BucTic(_, _) => None,
        Spec::BucTis(_, q) => autoname(*q),
        Spec::BucPat(_, q) => autoname(*q),
        Spec::BucWut(i, _) => autoname(*i),
        Spec::BucZap(_, _) => None,
    }
}

pub fn decorate(gen: Hoon, bug: Vec<Spot>, nut: Option<Note>) -> Hoon {
    let mut out = gen;

    for spot in bug.into_iter().rev() {
        out = Hoon::Dbug(spot, Box::new(out));
    }

    match nut {
        None => out,
        Some(note) => Hoon::Note(note, Box::new(out)),
    }
}
