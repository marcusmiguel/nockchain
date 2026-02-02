use crate::ast::hoon::*;
use nockvm::noun::{D, Noun};
use nockvm_macros::tas;
use nockapp::noun::slab::{slab_mug, slab_noun_equality};
use either::Either::{Left, Right};
use std::cmp;
use std::sync::Arc;
use std::path::PathBuf;
use std::collections::HashMap;
use num_bigint::BigUint;
use std::ops::BitAnd;
use nockapp::noun::slab::NounSlab;
use num_traits::{One, Num, FromPrimitive, ToPrimitive};
use num_traits::identities::Zero;
use ibig::UBig;
use chumsky::{
    span::Span,
    input::MapExtra,
    prelude::*,
};
use crate::atom::*;
use crate::skin_formation::*;

pub type Err<'src> = extra::Full<Rich<'src, char>, (), ()>;

pub trait ParserExt<'src, O>:
    Parser<'src, &'src str, O, Err<'src>> + Clone + 'src
{
}

impl<'src, O, P> ParserExt<'src, O> for P
where
    P: Parser<'src, &'src str, O, Err<'src>> + Clone + 'src,
{
}

// TODO: we need to support all UTF-8
// non-control ASCII (32-255, excluding 127/DEL)
pub fn non_control_char<'src>(
) -> impl Parser<'src, &'src str, char, Err<'src>> {
    any().filter(|c: &char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        bytes.iter().all(|&b| b >= 32 && b != 127)
    })
}

fn gah<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    choice((
        just(' ').ignored(),
        newline(),
    ))
    .labelled("Space or NewLine")
}

pub fn gaw<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    choice((vul(),
            gah()))
    .repeated()
    .ignored()
    .labelled("WhiteSpace")
}

pub fn vul<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    just("::")
        .ignore_then(non_control_char().repeated())
        .ignore_then(newline())
        .ignored()
        .labelled("Comments")
}

fn gaq<'src>() -> impl Parser<'src, &'src str, (), Err<'src>> {
    choice((
        newline().ignored(),

        gah().ignore_then(
                choice((gah().ignored(),
                        vul(),
                        ))
        ),

        vul(),
    ))
    .ignored()
    .labelled("End of Line")
}

pub fn gap<'src>(
) -> impl Parser<'src, &'src str, (), Err<'src>>
{
    gaq()
    .then_ignore(
        choice((
            vul(),
            gah().ignored(),
        ))
        .repeated()
        .or_not()
    )
    .ignored()
    .labelled("Gap")
}

pub fn list_term_hoon<'src>(
    hoon: impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<(String, Hoon)>, Err<'src>>
{
    symbol()
    .then_ignore(gap())
    .then(hoon.clone())
    .then_ignore(gap())
    .repeated()
    .at_least(1)
    .collect::<Vec<(String, Hoon)>>()
}

pub fn list_names_tall<'src>(
) -> impl Parser<'src, &'src str, Vec<String>, Err<'src>>
{
    symbol()
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<_>>()
    .then_ignore(gap().ignore_then(just("==")))
}

pub fn list_names_wide<'src>(
) -> impl Parser<'src, &'src str, Vec<String>, Err<'src>>
{
    symbol()
    .separated_by(just(' '))
    .at_least(1)
    .collect::<Vec<_>>()
    .delimited_by(just('['), just(']'))
}

pub fn winglist<'src>(
) -> impl Parser<'src, &'src str, WingType, Err<'src>>
{
    let name =      //  Name or $
        just('$')
            .to("$".to_string())
            .or(symbol());

    let com =   //  ,
        just(',')
        .to(Limb::Parent(0, None));

    let ket_name =   //  ^^name or name
        just('^')
            .repeated()
            .count()
            .then(name)
            .map(|(cnt, name)| {
                if cnt == 0 {
                    return Limb::Term(name);
                } else {
                    return Limb::Parent(cnt as u64, Some(name));
                }
            });

    let lus_number =   //  +10
            just('+')
                .ignore_then(digits())
                .map(|s| {
                    let num = s.parse::<u64>().unwrap();
                    Limb::Axis(num)
                });

    let pam_number =   //  &10
            just('&')
                .ignore_then(digits())
                .map(|s| {
                    let num = s.parse::<u64>().unwrap();
                    Limb::Axis(left_child(num))
                });

    let bar_number =  //  |10
           just('|').ignore_then(digits())
                .map(|s| {
                    let num = s.parse::<u64>().unwrap();
                    Limb::Axis(right_child(num))
                });

    let dot =  //  .
            just('.').to(Limb::Axis(1));

    let lus =  //  +
        just('+').to(Limb::Axis(3));

    let hep =  //  -
        just('-').to(Limb::Axis(2));

    let sign = any().filter(|c: &char| *c == '+' || *c == '-');
    let angle = any().filter(|c: &char| *c == '<' || *c == '>');

    let lark =   //    +>-<  notation
            sign
                .then(angle)
                .repeated()
                .at_least(1)
                .collect::<Vec<_>>()
            .then(sign.or_not())
            .map(|(pairs, tail)| {
                let mut out = String::new();
                for (s, a) in pairs {
                    out.push(s);
                    out.push(a);
                }
                if let Some(t) = tail {
                    out.push(t);
                }
                out
            })
            .map(|s: String| {
                let mut axis = 1;
                for c in s.chars() {
                    match c {
                        '+' | '>' => axis = peg(axis, 3).unwrap(),
                        '-' | '<' => axis = peg(axis, 2).unwrap(),
                        _ => axis = 1,
                    }
                }
                Limb::Axis(axis)
            }).labelled("Lark Expression");

    choice((
        com,
        ket_name,
        lus_number,
        pam_number,
        bar_number,
        lark,
        dot,
        lus,
        hep,
    )).separated_by(just('.'))
        .at_least(1)
        .collect::<Vec<_>>()
        .labelled("Wing")
}

pub fn variable_name_and_type<'src>(
    spec_wide:   impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, Skin, Err<'src>>
{
    let not_named = just('=')  // =/  =foo
        .ignore_then(spec_wide.clone())
        .try_map(|spec, span| {
            let auto = autoname(spec.clone());
             match auto {
                        None => Err(Rich::custom(span, "cannot autoname")),
                        Some(term) => {
                            Ok(Skin::Name(
                              term,
                                Box::new(Skin::Spec(
                                    Box::new(spec),
                                    Box::new(Skin::Base(BaseType::NounExpr)),
                                )),
                            ))
                        }
                    }
        });

     let name_or_namedspec = symbol()    //  =/  a=foo  ,  =/  a
        .then(just('/').or(just('='))
                    .ignore_then(
                        spec_wide.clone()
                    ).or_not())
        .map(|(term, maybe_spec)|
            match maybe_spec {
                None => Skin::Term(term),
                Some(spec) => Skin::Name(
                    term,
                    Box::new(Skin::Spec(
                        Box::new(spec),
                        Box::new(Skin::Base(BaseType::NounExpr)),
                    )),
                ),
        });

    let just_type = spec_wide.clone() // =/  type
        .map(|s| Skin::Spec(Box::new(s), Box::new(Skin::Base(BaseType::NounExpr))));

    choice((not_named, name_or_namedspec, just_type))
}

pub fn list_wing_hoon_wide<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<(WingType, Hoon)>, Err<'src>>
{
    let pair = winglist()
                .then_ignore(just(' '))
                .then(hoon.clone());

    pair
        .separated_by(just(",").then(just(' ')))
        .at_least(1)
        .collect::<Vec<_>>()
}

pub fn list_hoon_wide<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<Hoon>, Err<'src>>
{
    hoon_wide.clone()
    .separated_by(just(' '))
    .at_least(1)
    .collect::<Vec<Hoon>>()
}

pub fn list_spec_closed_wide<'src>(
    spec_wide:   impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, Vec<Spec>, Err<'src>>
{
    spec_wide.clone()
    .separated_by(just(' '))
    .at_least(1)
    .collect::<Vec<_>>()
    .delimited_by(just('('), just(')'))
}

pub fn list_spec_closed_tall<'src>(
    spec:   impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, Vec<Spec>, Err<'src>>
{
    gap()
    .ignore_then(spec.clone()
                .separated_by(gap())
                .at_least(1)
                .collect::<Vec<_>>()
        )
    .then_ignore(gap())
    .then_ignore(just("=="))
}

pub fn list_wing_hoon_tall<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<(WingType, Hoon)>, Err<'src>>
{
   let pair = winglist()
                .then_ignore(gap())
                .then(hoon.clone())
                .then_ignore(gap());

    pair.repeated().at_least(1).collect::<Vec<(WingType, Hoon)>>()
}

pub fn tiki_wide<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Tiki, Err<'src>>
{
    let with_name = symbol()
        .then_ignore(just('='))
        .then(
            winglist()
                .map(|w| {
                    Box::new(move |t: String| Tiki::Wing((Some(t), w)))
                        as Box<dyn FnOnce(String) -> Tiki>
                })
                .or(hoon_wide.clone()
                    .map(|h| {
                        Box::new(move |t: String| Tiki::Hoon((Some(t), Box::new(h))))
                         as Box<dyn FnOnce(String) -> Tiki>
                }))
        )
        .map(|(t, f)| f(t));

    let no_name = winglist()
        .map(|w| Tiki::Wing((None, w)))
        .or(hoon_wide.clone().map(|h| Tiki::Hoon((None, Box::new(h)))));

    with_name.or(no_name)
}

pub fn tiki_tall<'src>(
    hoon_tall: impl ParserExt<'src, Hoon>,
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Tiki, Err<'src>>
{
    let with_name = symbol()
        .then_ignore(just('='))
        .then(
            winglist()
                .map(|w| {
                    Box::new(move |t: String| Tiki::Wing((Some(t), w)))
                        as Box<dyn FnOnce(String) -> Tiki>
                })
                .or(hoon_tall.clone()
                    .map(|h| {
                        Box::new(move |t: String| Tiki::Hoon((Some(t), Box::new(h))))
                         as Box<dyn FnOnce(String) -> Tiki>
                }))
        )
        .map(|(t, f)| f(t));

    tiki_wide(hoon_wide.clone())    //  the hoon parser has ^= case here but
        .or(
            just("^=").then(gap()).or_not()
            .ignore_then(with_name)
        )
        .or(
            hoon_tall.clone().map(|h| Tiki::Hoon((None, Box::new(h))))
        )
}

///  Parses arms of a Core (grouped by chapters).
///     chapters can be unamed or named with +$
///     arms can be named with ++ or +$
///
pub fn chapters<'src>(
    hoon: impl ParserExt<'src, Hoon>,
    spec: impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, HashMap<String, Tome>, Err<'src>> {
    let luslus = just("++")
            .ignore_then(gap())
            .ignore_then(just('$').to("$".to_string()).or(symbol()))
            .then_ignore(gap())
            .then(hoon.clone())
            .map(|(name, hoon)| {
                (name, hoon)
            }).labelled("Arm ++");

    let lusbuc =  just("+$")
            .ignore_then(gap())
            .ignore_then(symbol())
            .then_ignore(gap())
            .then(spec.clone())
            .map(|(name, spec)| (name.clone(),
                                Hoon::KetCol(Box::new(Spec::Name(name.clone(),
                                                        Box::new(spec)))))).labelled("Arm +$");

    let optional_chapter_label =
        just("+|")
        .then_ignore(gap())
        .then(just("%"))
        .ignore_then(symbol())
        .then_ignore(gap())
        .or_not().labelled("Chapter Label +|");

    let chapter = optional_chapter_label
                    .then(luslus.or(lusbuc)
                          .then_ignore(gap())
                          .repeated().at_least(1).collect::<Vec<_>>());

    chapter.repeated().at_least(1).collect::<Vec<_>>()
        .then_ignore(just("--"))
        .map(|chapters_vec: Vec<(Option<String>, Vec<(String, Hoon)>)>| {
            let mut map_term_tome = HashMap::new();
            for (opt_label, arms_vec) in chapters_vec {
                let mut arms_map = HashMap::new();
                for (name, hoon) in arms_vec {
                    arms_map.insert(name, hoon);
                }
                let key = opt_label.unwrap_or_else(|| "$".to_string());
                map_term_tome.insert(key, (None, arms_map));
            }
            map_term_tome
        })
}

pub fn list_hoon_tall<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<Hoon>, Err<'src>>
{
    hoon.clone()
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<_>>()
}

pub fn term<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>>
{
    just("%")
      .ignore_then(symbol())
}

pub fn jet_hooks<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<(String, Hoon)>, Err<'src>>
{
    just('~').to(Vec::new())
        .or(
            just("==")
            .ignore_then(gap())
            .ignore_then(just("%")
                        .ignore_then(symbol())
                        .then_ignore(gap())
                        .then(hoon.clone())
                        .separated_by(gap())
                        .at_least(1)
                        .collect::<Vec<(String, Hoon)>>()
                        )
            .then_ignore(gap())
            .then_ignore(just("=="))
        )
}

pub fn jet_signature<'src>(
) -> impl Parser<'src, &'src str, Chum, Err<'src>>
{
    let lef = symbol().map(Chum::Lef); //  %k

    let stdkel = symbol()              //  %k.138
                .then_ignore(just('.'))
                .then(decimal_number())
                .map(|(s, n)| Chum::StdKel(s, decimal_to_atom(n)));

    let venprokel =
                symbol()  //  %k:foo.138
                .then_ignore(just(':'))
                .then(symbol())
                .then_ignore(just('.'))
                .then(decimal_number())
                .map(|((s1, s2), n)| Chum::VenProKel(s1, s2, decimal_to_atom(n)));

    let venproverkel =  //  %k:foo:bar..138
                symbol()
                .then_ignore(just(':'))
                .then(symbol())
                .then_ignore(just(".."))
                .then(decimal_number())
                .map(|((s1, s2), n)| Chum::VenProKel(s1, s2, decimal_to_atom(n)));

    just("%")
    .ignore_then(
        choice((
            venproverkel,
            venprokel,
            stdkel,
            lef
        ))).labelled("Jet Signature")
}

//  +lute
//
pub fn noun_tall<'src>(
    hoon:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    hoon
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<_>>()
    .delimited_by(just('[').ignore_then(gap()),
                gap().ignore_then(just(']')))
    .map(|h| Hoon::ColTar(h))
}

pub fn newline<'src>(
) -> impl Parser<'src, &'src str, (), Err<'src>>
{
    just('\n').labelled("Newline").ignored()
}

pub fn sump<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    hoon_wide
    .separated_by(just(' '))
    .at_least(1)
    .collect::<Vec<_>>()
    .delimited_by(just('{'), just('}'))
    .map(|h| Hoon::ColTar(h))
    .labelled("{Hoon}")
}

pub fn woof_to_beer(woof: Woof) -> Beer {
    match woof {
        Woof::ParsedAtom(atom) => Beer::Char(atom),
        Woof::Hoon(hoon) => Beer::Hoon(hoon),
    }
}

pub fn woofs_to_beers(woofs: Vec<Woof>) -> Vec<Beer> {
    woofs.into_iter().map(woof_to_beer).collect()
}

pub fn soil<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Vec<Woof>, Err<'src>>
{
    let sump = sump(hoon_wide.clone())
                .map(|h| vec![Woof::Hoon(h)]).boxed();

    // non-control, excluding DEL, {,  ", \
    let wide_char = any().filter(|c: &char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        bytes.iter().all(|&b| b >= 32 && b != 127)
            && !matches!(c, '{' | '\"' | '\\')
    })
    .map(|c: char| {
        let mut buf = [0u8; 4];
        c.encode_utf8(&mut buf)
                .as_bytes()
                .iter()
                .map(|&b| Woof::ParsedAtom(ParsedAtom::Small(b as u128)))
                .collect::<Vec<Woof>>()
    });

    //
    //  "foo"
    //
    let wide_tape =
        choice((
            //
            //  escaped \, ", {, hex
            //
            just("\\")
                .ignore_then(
                    choice((
                            just("\\").to(vec![Woof::ParsedAtom(ParsedAtom::Small('\\' as u128))]),
                            just("\"").to(vec![Woof::ParsedAtom(ParsedAtom::Small('\"' as u128))]),
                            just("{").to(vec![Woof::ParsedAtom(ParsedAtom::Small('{' as u128))]),
                            // \HH hex escape
                            any().filter(|c: &char| c.is_ascii_hexdigit())
                                .then(any().filter(|c: &char| c.is_ascii_hexdigit()))
                                .map(|(a, b)| {
                                    let hx = format!("{}{}", a, b);
                                    let byte = u8::from_str_radix(&hx, 16).unwrap();
                                    vec![Woof::ParsedAtom(ParsedAtom::Small(byte as u128))]
                                }),
                            ))
                ),
            //
            //  {hoon}
            //
            sump.clone(),
            ///
            wide_char,
        )).repeated()
        .collect::<Vec<Vec<Woof>>>()
        .map(|v| v.into_iter().flatten().collect::<Vec<Woof>>())
        .delimited_by(just("\""), just("\""))
        .labelled("Tape");

    // non-control, excluding DEL, {,  ", \
    let tall_char = any().filter(|c: &char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        bytes.iter().all(|&b| b >= 32 && b != 127)
            && !matches!(c, '{' | '\"' | '\\')
    })
    .map(|c: char| {
        let mut buf = [0u8; 4];
        c.encode_utf8(&mut buf)
                .as_bytes()
                .iter()
                .map(|&b| Woof::ParsedAtom(ParsedAtom::Small(b as u128)))
                .collect::<Vec<Woof>>()
    });

    let tall_tape_line_content =
            choice((
                //
                //  escaped \, {, hex
                //
                just("\\")
                .ignore_then(
                    choice((just("\\").to(vec![Woof::ParsedAtom(ParsedAtom::Small('\\' as u128))]),
                            just("{").to(vec![Woof::ParsedAtom(ParsedAtom::Small('{' as u128))]),
                            // \HH hex escape
                            any().filter(|c: &char| c.is_ascii_hexdigit())
                                .then(any().filter(|c: &char| c.is_ascii_hexdigit()))
                                .map(|(a, b)| {
                                    let hx = format!("{}{}", a, b);
                                    let byte = u8::from_str_radix(&hx, 16).unwrap();
                                    vec![Woof::ParsedAtom(ParsedAtom::Small(byte as u128))]
                                })
                ))),
            //
                tall_char,
            //
            //  {hoon}
            //
                sump,
            ))
            .repeated()
            .collect::<Vec<Vec<Woof>>>()
            .map(|v| v.into_iter().flatten().collect::<Vec<Woof>>());


    let prefix_spaces =
        just(' ').repeated();

    let tall_tape_open =
        just("\"\"\"")
            .map_with(move |_, extra| {
                let span: SimpleSpan = extra.span();  // get identation
                let (_line, col) = linemap.line_col(span.start);
                if col != 0 {
                    return (col - 1 ) as usize;
                }
                return 0 as usize;
            });

    let tall_tape_close =
        newline()
            .ignore_then(just(' ').repeated().count())
            .then_ignore(just("\"\"\"")).boxed();

    let tall_tape_line =
        tall_tape_close.clone().not()
        .ignore_then(
                newline()
                .ignore_then(just(' ').repeated().count())
                .then(tall_tape_line_content));

    //  """
    //  foo
    //  """
    let tall_tape =
        prefix_spaces
            .ignore_then(tall_tape_open)
            .then(
                tall_tape_line
                .repeated()
                .collect::<Vec<_>>())
           .then(tall_tape_close)
            .validate(|((absolute_indent, lines), close_indent), extra, emit| {
                let span = extra.span();

                if close_indent != absolute_indent {
                    emit.emit(Rich::custom(
                        span,
                        "closing delimiter indentation mismatch",
                    ));
                    return Vec::new();
                }

                let mut out: Vec<Woof> = vec![];
                for (mut indent, mut line) in lines {

                    if indent > absolute_indent {
                        let extra = indent - absolute_indent;
                        indent = absolute_indent;
                        //  extra whitespaces belongs longs to line not indentation
                        let space = Woof::ParsedAtom(ParsedAtom::Small(' ' as u128));
                        line.splice(0..0, std::iter::repeat(space).take(extra));
                    }

                    //  if line is just a linebreak allow it
                    if indent != absolute_indent &&
                        !(line.is_empty() && (indent == 0 as usize)) {
                        emit.emit(Rich::custom(
                            span,
                            "inconsistent indentation in tall tape",
                        ));
                        return Vec::new();
                    }
                    out.push(Woof::ParsedAtom(
                        ParsedAtom::Small('\n' as u128),
                    ));
                    if !line.is_empty() {
                        out.extend(line);
                    }
                }
                // first linebreak after """ should not be in the tape
                out.remove(0);
                out
            })
            .labelled("Tape");

    choice((tall_tape, wide_tape))
}

pub fn tape<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    soil(hoon_wide.clone(), linemap.clone())
    .separated_by(just('.').ignore_then(gap().or_not()))
    .at_least(1)
    .collect::<Vec<_>>()
    .map(|s: Vec<Vec<Woof>>| {
        let wof: Vec<Woof> = s.into_iter().flatten().collect();
        Hoon::Knit(wof)
    }).labelled("Tape")
}

pub fn aura_text<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>>
{
    just('@')
        .ignore_then(
            any()
                .filter(|c: &char| c.is_ascii_lowercase())
                .repeated()
                .collect::<Vec<char>>()
                .then(
                    any()
                        .filter(|c: &char| c.is_ascii_uppercase())
                        .repeated()
                        .collect::<Vec<char>>()
                )
                .map(|(lowers, uppers)| {
                    let mut s = String::new();
                    s.extend(lowers);
                    s.extend(uppers);
                    s
                })
        )
        .labelled("Aura<@foo>")
}

pub fn aura_hoon<'src>(
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    aura_text()
    .map(|s| Hoon::Base(BaseType::Atom(s)))
    .labelled("Aura")
}

pub fn aura_spec<'src>(
) -> impl Parser<'src, &'src str, Spec, Err<'src>>
{
    aura_text()
    .map(|s| Spec::Base(BaseType::Atom(s)))
    .labelled("Aura")
}

pub fn loop_spec<'src>(
) -> impl Parser<'src, &'src str, Spec, Err<'src>>
{
    just('/')
    .ignore_then(
        choice((just('$').to("$".to_string()),
            symbol(),
        )))
    .map(|s| Spec::Loop(s))
}

pub fn concatanate<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    hoon_wide.clone()
      .then_ignore(just('^'))
      .then(hoon_wide.clone())
      .map(|(p, q)| Hoon::Pair(Box::new(p), Box::new(q)))
}

pub fn wing<'src>(
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    winglist()
    .map(|list: WingType| {
        match list.first() {
            Some(Limb::Axis(0))
                | Some(Limb::Term(_))
                | Some(Limb::Parent(_, _)) => {
                Hoon::Wing(list)
            }
            _ => Hoon::CenTis(list, vec![])
        }
    })
    .labelled("Wing")
}

pub fn tell<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    just("<")
        .ignore_then(list_hoon_wide(hoon_wide.clone()))
        .then_ignore(just(">"))
        .map(|list| Hoon::Tell(list))
}


pub fn yell_parser<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    just(">")
        .ignore_then(list_hoon_wide(hoon_wide.clone()))
        .then_ignore(just("<"))
        .map(|list| Hoon::Yell(list))
}

pub fn constant<'src>(
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    let buc =      // %$
        just('$')
        .to(Coin::Dime("tas".to_string(), ParsedAtom::Small(0)));

    let cord =      // %'foo'
        cord(linemap)
        .map(|s| Coin::Dime("t".to_string(), s));

    let coin =      // %123, %~m5, etc.
        nuck();

    let no =
        just('|')
        .to(Coin::Dime("f".to_string(), ParsedAtom::Small(1)));

    let yes =
        just('&')
        .to(Coin::Dime("f".to_string(), ParsedAtom::Small(0)));

    just('%')
    .ignore_then(
        choice((
            buc,
            yes,
            no,
            cord,
            coin,
        )))
        .labelled("Constant<%foo>")
}

pub fn cord<'src>(
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>>
{
    //  \\, \' and \AA were A is a hex digit
    let escape = just('\\')
        .ignore_then(
            choice((
                just('\\').to(vec!['\\' as u8]),
                just('\'').to(vec!['\'' as u8]),
                // \HH hex escape
                any().filter(|c: &char| c.is_ascii_hexdigit())
                    .then(any().filter(|c: &char| c.is_ascii_hexdigit()))
                    .map(|(a, b)| {
                        let hx = format!("{}{}", a, b);
                        let byte = u8::from_str_radix(&hx, 16).unwrap();
                        vec![byte]
                    }),
            ))
        );

    //  chars (excluding controls, DEL, ', \)
    let wide_char = any().filter(|c: &char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        bytes.iter().all(|&b| b >= 32 && b != 127)
            && !matches!(c, '\'' | '\\')
    })
    .map(|c: char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();
        bytes.to_vec()
    });

    //  chars (excluding controls, DEL)
    let tall_char = any().filter(|c: &char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();

        bytes.iter().all(|&b| b >= 32 && b != 127)
    })
    .map(|c: char| {
        let mut buf = [0u8; 4];
        let bytes = c.encode_utf8(&mut buf).as_bytes();
        bytes.to_vec()
    }).boxed();

    let gon = just("\\")  // multiline separator
            .ignore_then(gap())
            .ignore_then(just("/"))
            .ignored()
            .labelled("Cord Multiline Separator");

    let char_in_singled_quoted = choice((escape,
                                         wide_char,
                                        )).labelled("Cord Character");

    let single_quoted =  char_in_singled_quoted.then_ignore(gon.or_not())
                        .repeated()
                        .collect::<Vec<Vec<u8>>>()
                        .map(|v| v.into_iter().flatten().collect::<Vec<u8>>())
                        .delimited_by(just("'"), just("'"))
                        .map(cord_bytes_to_atom);

    let prefix_spaces =
        just(' ').repeated();

    let triple_quoted_open =
        just("'''")
            .map_with(move |_, extra| {
                let span: SimpleSpan = extra.span();  // get identation
                let (_line, col) = linemap.line_col(span.start);
                if col != 0 {
                    return (col - 1 ) as usize;
                }
                return 0 as usize;
            }).then_ignore(vul().or(newline()));

    let triple_quoted_close =
        newline()
            .ignore_then(just(' ').repeated().count())
            .then_ignore(just("'''")).boxed();

    let triple_quoted_content =
                    tall_char
                    .repeated()
                    .collect::<Vec<Vec<u8>>>()
                    .map(|v| v.into_iter().flatten().collect::<Vec<u8>>())
                    .boxed();

    let triple_quoted_first_line =
                    triple_quoted_close.clone().not()
                    .ignore_then(just(' ').repeated().count())
                    .then(triple_quoted_content.clone());

    let triple_quoted_line =
        triple_quoted_close.clone().not()
        .ignore_then(
                newline()
                .ignore_then(just(' ').repeated().count())
                .then(triple_quoted_content));

    let triple_quoted =
            prefix_spaces
            .ignore_then(triple_quoted_open)
            .then(
                triple_quoted_first_line
                .then(triple_quoted_line
                      .repeated()
                      .collect::<Vec<_>>())
            )
           .then(triple_quoted_close)
            .validate(|((absolute_indent, (first, mut rest)), close_indent), extra, emit| {
                let span = extra.span();

                if close_indent != absolute_indent {
                    emit.emit(Rich::custom(
                        span,
                        "closing delimiter indentation mismatch",
                    ));
                    return Vec::new();
                }
                rest.insert(0, first);

                let mut out: Vec<u8> = vec![];
                for (mut indent, mut line) in rest {

                    if indent > absolute_indent {
                        let extra = indent - absolute_indent;
                        indent = absolute_indent;
                        //  extra whitespaces belongs longs to line not indentation
                        line.splice(0..0, std::iter::repeat(32).take(extra));
                    }

                    //  if line is just a linebreak allow it
                    if indent != absolute_indent &&
                        !(line.is_empty() && (indent == 0 as usize)) {
                        emit.emit(Rich::custom(
                            span,
                            "inconsistent indentation in multiline cord",
                        ));
                        return Vec::new();
                    }
                    out.push(10);
                    if !line.is_empty() {
                        out.extend(line);
                    }
                }
                out.remove(0);
                out
            }).map(cord_bytes_to_atom);

    choice((
        triple_quoted,
        single_quoted,
    )).labelled("Cord")
}

pub fn increment<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    just('.').or_not()
        .ignore_then(just("+"))
        .ignore_then(just('('))
        .ignore_then(
            hoon_wide.clone()
        )
        .then_ignore(just(')'))
        .map(|h| Hoon::DotLus(Box::new(h)))
    .labelled("Increment: +(p)")
}

pub fn function_call<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    just('(')
        .ignore_then(hoon.clone())
        .then(
            just(' ')
                .ignore_then(hoon.clone())
                .repeated()
                .collect::<Vec<_>>()
            )
    .then_ignore(just(')'))
    .map(|(func, args)| Hoon::CenCol(Box::new(func), args))
    .labelled("Function Call")
}

///  Alphanumeric with hyphens
///      Start with a lowercase letter
///      Followed by zero or more: lowercase letter, digit, or hyphen
///
pub fn symbol<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    any()
    .filter(|c: &char| c.is_ascii_lowercase())
    .then(
        any()
            .filter(|c: &char| matches!(c, 'a'..='z' | '0'..='9' | '-'))
            .repeated()
            .collect::<Vec<char>>(),
    )
    .map(|(first, rest)| {
        let mut s = String::with_capacity(rest.len() + 1);
        s.push(first);
        s.extend(rest);
        s
    })
    .labelled("Term")
}

pub fn digits<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .collect::<String>()
}

pub fn alphanumeric<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_alphanumeric())
        .repeated()
        .at_least(1)
        .collect::<String>()
}

//
// List Functions
//

pub fn reap<T: Clone>(a: usize, b: T) -> Vec<T> {
    vec![b; a]
}

fn snag<T>(index: usize, list: &[T]) -> &T {
    list.get(index).expect("snag: index out of bounds")
}

pub fn weld<T: Clone>(a: impl AsRef<[T]>, b: impl AsRef<[T]>) -> Vec<T> {
    let a = a.as_ref();
    let b = b.as_ref();
    let mut v = Vec::with_capacity(a.len() + b.len());
    v.extend_from_slice(a);
    v.extend_from_slice(b);
    v
}

pub fn scag<T: Clone>(n: usize, list: impl AsRef<[T]>) -> Vec<T> {
    list.as_ref().iter().take(n).cloned().collect()
}

pub fn slag<T: Clone>(n: usize, list: impl AsRef<[T]>) -> Vec<T> {
    list.as_ref().iter().skip(n).cloned().collect()
}

pub fn flop<T: Clone>(list: impl AsRef<[T]>) -> Vec<T> {
    let mut v = list.as_ref().to_vec();
    v.reverse();
    v
}

//  Path parsing

fn poof(pax: Path) -> Vec<Hoon> {
    pax.iter()
        .map(|a| { Hoon::Sand(
            "ta".to_string(),
            NounExpr::ParsedAtom(string_to_atom(a.clone())),
        )})
        .collect()
}

// used to create dbug traces
#[derive(Clone)]
pub struct LineMap {
    starts: Vec<usize>,
}

impl LineMap {
    pub fn new(src: &str) -> Self {
        let mut starts = Vec::with_capacity(128);
        starts.push(0);

        for (i, b) in src.bytes().enumerate() {
            if b == b'\n' {
                starts.push(i + 1);
            }
        }

        Self { starts }
    }

    pub fn line_col(&self, byte: usize) -> (u64, u64) {
        let line = match self.starts.binary_search(&byte) {
            Ok(i) => i,
            Err(i) => i - 1,
        };

        (
            (line + 1) as u64,
            (byte - self.starts[line] + 1) as u64,
        )
    }

    pub fn pint(&self, span: std::ops::Range<usize>) -> Pint {
        Pint {
            p: self.line_col(span.start),
            q: self.line_col(span.end),
        }
    }

    pub fn offset(&self, line: u64, col: u64) -> Option<usize> {
        let line_idx = (line as usize).saturating_sub(1);
        let line_start = *self.starts.get(line_idx)?;
        Some(line_start + (col as usize).saturating_sub(1))
    }
}

fn poon(
    pag: &[Hoon],
    goo: &[Option<Hoon>],
) -> Option<Vec<Hoon>> {
    if goo.is_empty() {
        return Some(vec![]);
    }

    let (goo_hd, goo_tl) = goo.split_first().unwrap();

    let head = match goo_hd {
        Some(x) => x.clone(),
        None => {
            let (pag_hd, _) = pag.split_first()?;
            pag_hd.clone()
        }
    };

    let pag_tl = if pag.is_empty() {
        &[]
    } else {
        &pag[1..]
    };

    let mut rest = poon(pag_tl, goo_tl)?;

    let mut out = Vec::with_capacity(rest.len() + 1);
    out.push(head);
    out.append(&mut rest);

    Some(out)
}

pub fn posh(
    pre: Option<Vec<Option<Hoon>>>,           // (unit tyke)
    pof: Option<(usize, Vec<Option<Hoon>>)>,  // (unit [p=@ud q=tyke])
    wer: Path,
) -> Option<Vec<Hoon>> {

    let wom: Vec<Hoon> = poof(wer);

    let yez = if pre.is_none() {
        Some(wom.clone())
    } else {
        let pre_val = pre.as_ref().unwrap();

        let moz = poon(&wom, pre_val)?;

        if let Some(_) = pof {
            let n  = pre_val.len();
            let sl = slag(n, &wom.clone());
            Some(weld(&moz, &sl))
        } else {
            Some(moz)
        }
    }?;

    if pof.is_none() {
        return Some(yez);
    }

    let (p, q) = pof.unwrap();

    let zey = flop(&yez.clone());

    let moz = scag(p, &zey);
    let gul = slag(p, &zey);

    let zom = poon(&flop(&moz.clone()), &q);

    match zom {
        None => None,
        Some(z) => Some(weld(&flop(&gul), z))
    }
}

pub fn version_pin<'src>(
) -> impl Parser<'src, &'src str, (), Err<'src>>
{
    just("/?")
        .ignore_then(gap())
        .ignore_then(decimal_without_leading_zero().to(()))
        .ignore_then(gap())
        .ignored()
}

pub fn stap<'src>(
) -> impl Parser<'src, &'src str, Path, Err<'src>>
{
    just('/')
    .ignore_then(urs()
                .separated_by(just('/'))
                .collect::<Path>()
                .map(|p: Path| {
                    // if path is empty:  /
                    if p == vec!["".to_string()] {
                        return vec![];
                    }
                    // if last element is empty:  /foo/bar/
                    if p.last().is_some_and(|s| s.is_empty()) {
                        return vec![];
                    }
                    return p;
                })
        ).labelled("Path")
}

pub fn path<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
    wer: Path,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    let wer1 = wer.clone();
    let wer2 = wer.clone();
    let wer3 = wer.clone();
    let wer4 = wer.clone();

    let hasp = choice((
                hoon_wide.clone().delimited_by(just('['), just(']')),
                hoon_wide.clone()
                    .separated_by(just(' '))
                    .at_least(1)
                    .collect::<Vec<_>>()
                    .delimited_by(just('('), just(')'))
                    .map(|list| {
                        let (first, rest) = list.split_first().unwrap();
                        Hoon::CenCol(Box::new(first.clone()), rest.to_vec())
                    }),
                just('$').to(Hoon::Sand("tas".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
                cord(linemap).map(|s| Hoon::Sand("t".to_string(), NounExpr::ParsedAtom(s))),
                nuck().map(|coin| {
                    let aura = match &coin {
                            Coin::Dime(a, _) if a == "tas" => "tas",
                            _ => "ta",
                        };
                    Hoon::Sand(aura.to_string(), NounExpr::ParsedAtom(rent_co(&coin)))
                }),
            ));

    let gasp = choice((
                    just('=')
                        .to(None)
                        .repeated()
                        .collect::<Vec<Option<Hoon>>>()
                    .then(hasp.map(|h| vec![Some(h)]))
                        .then(
                            just('=')
                                .to(None)
                                .repeated()
                                .collect::<Vec<Option<Hoon>>>()
                        )
                        .map(|((mut a, b), c)| {
                            a.extend(b);
                            a.extend(c);
                            a
                        }),
                    just('=')
                        .to(None)
                        .repeated()
                        .at_least(1)
                        .collect::<Vec<Option<Hoon>>>(),
                    ));

    let limp =  just("/").repeated().count()
                .then(gasp)
                .map(|(a, mut b)| {
                    for _ in 0..a {
                        b.insert(0, Some(Hoon::Sand("tas".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))));
                    }
                    b
                });

    let gash = limp
            .separated_by(just("/"))
            .collect::<Vec<Vec<Option<Hoon>>>>()
            .map(|a| a.into_iter().flatten().collect::<Vec<_>>())
            .boxed();

    let porc = just("%").repeated().count()     //  usize
                .then(just("/").ignore_then(gash.clone())); // Vec<Option<Hoon>>

    let poor = gash.clone()
                .map(|pre| Some(pre))
                    .then(just("%")
                            .ignore_then(porc.clone())
                            .or_not());

    let rood = {
        just("/")
        .ignore_then(poor.try_map(move |(pre, pof), span| {
            match posh(pre, pof, wer1.clone()) {
                Some(list) => Ok(Hoon::ColSig(list)),
                None => Err(Rich::custom(span, "error parsing path")),
            }
        })).labelled("Path")
    };

    let cen_fas = {
        porc.try_map(move |(a, b), span| {
            match posh(Some(vec![None]), Some((a, b)), wer2.clone()) {
                Some(list) => Ok(Hoon::ColSig(list)),
                None => Err(Rich::custom(span, "error parsing path")),
            }
        })
    };

    let multi_cen = {
        just("%").repeated().count().try_map(move |n, span| {
            match posh(Some(vec![None]), Some((n, vec![])), wer3.clone()) {
                Some(list) => Ok(Hoon::ColSig(list)),
                None => Err(Rich::custom(span, "error parsing path")),
            }
        })
    };

    let cen_path = just("%").ignore_then(choice((cen_fas, multi_cen))).labelled("Path");

    choice((
        rood.boxed(),       //  /foo/%/foo
        cen_path.boxed(),   //  %/foo  and  %%
    )).labelled("Path")
}


pub fn blue(tik: Tiki, gen: Hoon) -> Hoon {
    match tik {
        Tiki::Hoon((None, h)) => Hoon::TisGar(Box::new(Hoon::Axis(3)), Box::new(gen)),
        _ =>  gen,
    }
}

pub fn teal(tik: Tiki, mod_: Spec) -> Spec {
    match tik {
        Tiki::Wing((_, _)) => mod_,
        Tiki::Hoon((_, _)) => Spec::Over(vec![Limb::Axis(3)], Box::new(mod_)),
    }
}

pub fn tele(tik: Tiki, syn: Skin) -> Skin {
    match tik {
        Tiki::Wing((_, _)) => syn,
        Tiki::Hoon((_, _)) => Skin::Over(vec![Limb::Axis(3)], Box::new(syn)),
    }
}

pub fn gray(tik: Tiki, gen: Hoon) -> Hoon {
    match tik {
        Tiki::Wing((p, q)) => {
            match p {
                None => gen,
                Some(u) => Hoon::TisTar((u, None),
                                        Box::new(Hoon::Wing(q)),
                                        Box::new(gen)),
            }
        }
        Tiki::Hoon((p, q)) => {
            let arg = match p {
                None => q,
                Some(u) => Box::new(Hoon::KetTis(Skin::Term(u), q)),
            };
            Hoon::TisLus(arg, Box::new(gen))
        }
    }
}

pub fn puce(tik: Tiki) -> WingType {
    match tik {
        Tiki::Wing((p, q)) => match p {
            None => q,
            Some(u) => vec![Limb::Term(u)],
        },
        Tiki::Hoon((_, _)) => vec![Limb::Axis(2)],
    }
}

pub fn wthp(tik: Tiki, opt: Vec<(Spec, Hoon)>) -> Hoon {
    let mapped = opt.into_iter()
                .map(|(a, b)| (a, blue(tik.clone(), b)))
                .collect::<Vec<(Spec, Hoon)>>();
    gray(tik.clone(), Hoon::WutHep(puce(tik.clone()), mapped))
}

pub fn wtkt(tik: Tiki, sic: Hoon, non: Hoon) -> Hoon {
    gray(tik.clone(), Hoon::WutKet(puce(tik.clone()),
              Box::new(blue(tik.clone(), sic)),
              Box::new(blue(tik.clone(), non))))
}

pub fn wtls(tik: Tiki, gen: Hoon, opt: Vec<(Spec, Hoon)>) -> Hoon {
    let mapped = opt.into_iter()
                .map(|(a, b)| (a, blue(tik.clone(), b)))
                .collect::<Vec<(Spec, Hoon)>>();
    gray(tik.clone(), Hoon::WutLus(puce(tik.clone()), Box::new(blue(tik.clone(), gen)), mapped))
}

pub fn wtpt(tik: Tiki, sic: Hoon, non: Hoon) -> Hoon {
    gray(tik.clone(), Hoon::WutPat(puce(tik.clone()),
                            Box::new(blue(tik.clone(), sic)),
                            Box::new(blue(tik.clone(), non))))
}

pub fn wtsg(tik: Tiki, sic: Hoon, non: Hoon) -> Hoon {
    gray(tik.clone(), Hoon::WutSig(puce(tik.clone()),
                            Box::new(blue(tik.clone(), sic)),
                            Box::new(blue(tik.clone(), non))))
}

pub fn wthx(tik: Tiki, syn: Skin) -> Hoon {
    gray(tik.clone(), Hoon::WutHax(tele(tik.clone(), syn), puce(tik.clone())))
}

pub fn wtts(tik: Tiki, mod_: Spec) -> Hoon {
    gray(tik.clone(), Hoon::WutTis(Box::new(teal(tik.clone(), mod_)), puce(tik.clone())))
}

pub fn number<'src>(
) -> impl Parser<'src, &'src str, (String, ParsedAtom), Err<'src>>
{
    let ud_number = decimal_number()
                    .map(|s|
                        ("ud".to_string(), decimal_to_atom(s)));

    let ux_number = hexadecimal_number()
                    .map(|s|
                        ("ux".to_string(), hex_to_atom(s)));

    let uc_number = bitcoin_address()
                    .validate(|s, extra, emit| {
                        let maybe_base58 = base58_to_atom(s);
                        match maybe_base58 {
                            Some(a) => ("uc".to_string(), a),
                            None => {
                                emit.emit(Rich::custom(extra.span(), "Invalid Address."));
                                ("uc".to_string(), (ParsedAtom::Small(0)))
                            }
                        }});

    let ub_number = binary_number()
                    .map(|s|
                        ("ub".to_string(), binary_to_atom(s)));

    let uv_number = base32_number()
                    .map(|a|
                        ("uv".to_string(), a));

    let uw_number = base64_number()
                    .map(|a|
                        ("uw".to_string(), a));

    let ui_number =
    just("0i")
        .ignore_then(digits())
        .map(|s| {
            ("ui".to_string(), decimal_to_atom(s))
        });

    let negative = choice((
                hexadecimal_number().map(|s| ("sx".to_string(), hex_to_atom(s))),
                binary_number().map(|s| ("sb".to_string(), binary_to_atom(s))),
                bitcoin_address()
                        .validate(|s, extra, emit| {
                                let maybe_base58 = base58_to_atom(s);
                                match maybe_base58 {
                                    Some(a) => ("uc".to_string(), a),
                                    None => {
                                        emit.emit(Rich::custom(extra.span(), "Invalid Address."));
                                        ("sc".to_string(), (ParsedAtom::Small(0)))
                                    }
                                }}),
                base32_number().map(|a| ("sv".to_string(), a)),
                base64_number().map(|a| ("sw".to_string(), a)),
                just("0i").ignore_then(digits())
                    .map(|s| ("si".to_string(), decimal_to_atom(s))),
                decimal_number().map(|s| ("sd".to_string(), decimal_to_atom(s))),
            )).boxed();

    let signed_number = // signed: -num and --num
        just('-')
        .ignore_then(
            just('-')
            .ignore_then(negative.clone().map(|(p, q)| (p, apply_sign(true, q))))
            .or(negative.map(|(p, q)| (p, apply_sign(false, q)))));

    choice((
        signed_number,
        ub_number,
        uc_number,
        ui_number,
        ux_number,
        uv_number,
        uw_number,
        ud_number,
    )).labelled("Number")
}

//  +rump: name/hoon or name+hoon
//
pub fn constant_separator_hoon<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    choice((
        just('$').to(Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
        symbol().map(|s| Hoon::Rock("tas".to_string(), NounExpr::ParsedAtom(string_to_atom(s)))),
        number().map(|(p, q)| Hoon::Rock(p, NounExpr::ParsedAtom(q))),
        just('&').to(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
        just('|').to(Hoon::Rock("f".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(1)))),
        just('~').to(Hoon::Bust(BaseType::Null)),
    ))
    .then(just('+').or(just('/'))
            .ignore_then(hoon.clone()))
    .map(|(p, hoon)| Hoon::Pair(Box::new(p), Box::new(hoon)))
}

//  `@p`q
//
pub fn tic_aura<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    aura_text()
    .then_ignore(just("`"))
    .then(hoon_wide.clone())
    .map(|(a, b)| {
        Hoon::KetLus(
            Box::new(Hoon::Sand(a, NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
            Box::new(Hoon::KetLus(Box::new(Hoon::Sand("$".to_string(), NounExpr::ParsedAtom(ParsedAtom::Small(0)))), Box::new(b))),
        )})
}

pub fn tic_cell_construction<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    hoon_wide.clone()
        .map(|h| Hoon::Pair(Box::new(Hoon::Rock("n".to_string(),
                                                    NounExpr::ParsedAtom(ParsedAtom::Small(0)))),
                                 Box::new(h)))
}

pub fn parenthesis_spec<'src>(
    hoon_wide:   impl ParserExt<'src, Hoon>,
    spec_wide:   impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, Spec, Err<'src>>
{
    hoon_wide.clone()
        .then(
            just(' ')
            .ignore_then(spec_wide.clone())
                .repeated()
                .collect::<Vec<_>>()
                .or_not()
                .map(|specs| specs.unwrap_or_default())
        )
    .delimited_by(just('('), just(')'))
    .map(|(name, specs)| Spec::Make(name, specs))
}

pub fn reference_spec<'src>(
) -> impl Parser<'src, &'src str, Spec, Err<'src>>
{
    let lower =
        any().filter(|c: &char| matches!(c, 'a'..='z'));

    let ident_tail =
        any().filter(|c: &char| c.is_ascii_alphanumeric());

    let ident =
        lower
            .then(ident_tail.repeated().collect::<Vec<char>>())
            .to(());

    let special =
        any()
            .filter(|c: &char| matches!(c, '$' | '^' | ','))
            .to(());

    let guard =
        ident
            .or(special)
            .rewind();

    // prevents this parser from matching
    //  inputs that starts with: "([a-z][a-zA-Z0-9]*)|[\$\^\,]"
    guard
    .rewind()
    .ignore_then(
            winglist()
            .separated_by(just(':'))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|wings: Vec<WingType>| {
                        let (first, rest) = wings.split_first().unwrap();
                        Spec::Like(first.clone(), rest.to_vec())
                    })
        )
}

pub fn two_hoons_tall<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, (Hoon, Hoon), Err<'src>>
{
    gap()
    .ignore_then(hoon.clone())
    .then_ignore(gap())
    .then(hoon.clone())
}

pub fn two_hoons_wide<'src>(
    hoon_wide:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, (Hoon, Hoon), Err<'src>>
{
    hoon_wide.clone()
    .then_ignore(just(' '))
    .then(hoon_wide.clone())
}

pub fn three_hoons_tall<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, ((Hoon, Hoon), Hoon), Err<'src>>
{
    gap()
    .ignore_then(hoon.clone())
    .then_ignore(gap())
    .then(hoon.clone())
    .then_ignore(gap())
    .then(hoon.clone())
}

pub fn three_hoons_wide<'src>(
    hoon_wide:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, ((Hoon, Hoon), Hoon), Err<'src>>
{
    hoon_wide.clone()
    .then_ignore(just(' '))
    .then(hoon_wide.clone())
    .then_ignore(just(' '))
    .then(hoon_wide.clone())
}

pub fn two_specs_tall<'src>(
    spec:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Spec, Spec), Err<'src>>
{
    gap()
    .ignore_then(spec.clone())
    .then_ignore(gap())
    .then(spec.clone())
}

pub fn two_specs_closed_tall<'src>(
    spec:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Spec, Spec), Err<'src>>
{
    two_specs_tall(spec.clone())
    .then_ignore(gap())
    .then_ignore(just("=="))
}

pub fn two_specs_closed_wide<'src>(
    spec_wide:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Spec, Spec), Err<'src>>
{
    spec_wide.clone()
    .then_ignore(just(' '))
    .then(spec_wide.clone())
    .delimited_by(just('('), just(')'))
}

pub fn hoon_spec_wide<'src>(
    hoon_wide:        impl ParserExt<'src, Hoon>,
    spec_wide:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Hoon, Spec), Err<'src>>
{
    hoon_wide.clone()
    .then_ignore(just(' '))
    .then(spec_wide.clone())
    .delimited_by(just('('), just(')'))
}

pub fn hoon_spec_tall<'src>(
    hoon:           impl ParserExt<'src, Hoon>,
    spec:           impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Hoon, Spec), Err<'src>>
{
    gap()
    .ignore_then(hoon.clone())
    .then_ignore(gap())
    .then(spec.clone())
}

pub fn spec_hoon_tall<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
    spec:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Spec, Hoon), Err<'src>>
{
    gap()
    .ignore_then(spec.clone())
    .then_ignore(gap())
    .then(hoon.clone())
}

pub fn spec_hoon_wide<'src>(
    hoon_wide:        impl ParserExt<'src, Hoon>,
    spec_wide:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (Spec, Hoon), Err<'src>>
{
    spec_wide.clone()
    .then_ignore(just(' '))
    .then(hoon_wide.clone())
}

pub fn name_spec_tall<'src>(
    spec:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src,  &'src str, (String, Spec), Err<'src>>
{
    gap()
    .ignore_then(symbol())
    .then_ignore(gap())
    .then(spec.clone())
}

pub fn name_spec_closed_tall<'src>(
    spec:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, (String, Spec), Err<'src>>
{
    gap()
    .ignore_then(symbol())
    .then_ignore(gap())
    .then(spec.clone())
    .then_ignore(just("=="))
}

pub fn name_spec_wide<'src>(
    spec_wide:        impl ParserExt<'src, Spec> + Clone,
) -> impl Parser<'src, &'src str, (String, Spec), Err<'src>>
{
    symbol()
    .then_ignore(just(' '))
    .then(spec_wide.clone())
    .delimited_by(just('('), just(')'))
}

pub fn one_hoon_closed_wide<'src>(
    hoon_wide:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    hoon_wide.clone()
    .delimited_by(just('('), just(')'))
}

pub fn one_hoon_closed_tall<'src>(
    hoon:        impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>>
{
    gap()
    .ignore_then(hoon.clone())
    .then_ignore(gap())
    .delimited_by(just('='), just('='))
}

pub fn one_spec_closed_wide<'src>(
    spec_wide:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, Spec, Err<'src>>
{
    spec_wide.clone()
    .delimited_by(just('('), just(')'))
}

pub fn one_spec_closed_tall<'src>(
    spec:        impl ParserExt<'src, Spec>,
) -> impl Parser<'src, &'src str, Spec, Err<'src>>
{
    gap()
    .ignore_then(spec.clone())
    .then_ignore(gap())
    .delimited_by(just('='), just('='))
}

pub fn wrap_hoon_with_trace(
    wer: Path,
    linemap: Arc<LineMap>,
) -> impl for<'src> Fn(
        Hoon,
        &mut MapExtra<'src, '_, &'src str, Err<'src>>,
    ) -> Hoon
    + Clone
{
    move |node, e| {
        let spot = chumsky_spot_to_hoon_spot(
            (e.span().start(), e.span().end()),
            &wer,
            &linemap,
        );

        match node {
            Hoon::Dbug(existing_spot, inner) => {
                if existing_spot == spot {
                    Hoon::Dbug(existing_spot, inner)
                } else {
                    Hoon::Dbug(spot, Box::new(Hoon::Dbug(existing_spot, inner)))
                }
            }
            other => Hoon::Dbug(spot, Box::new(other)),
        }
    }
}

pub fn wrap_spec_with_trace(
    wer: Path,
    linemap: Arc<LineMap>,
) -> impl for<'src> Fn(
        Spec,
        &mut MapExtra<'src, '_, &'src str, Err<'src>>,
    ) -> Spec
    + Clone
{
    move |node, e| {
        let spot = chumsky_spot_to_hoon_spot(
            (e.span().start(), e.span().end()),
            &wer,
            &linemap,
        );

        match node {
            Spec::Dbug(existing_spot, inner) => {
                if existing_spot == spot {
                    Spec::Dbug(existing_spot, inner)
                } else {
                    Spec::Dbug(spot, Box::new(Spec::Dbug(existing_spot, inner)))
                }
            }
            other => Spec::Dbug(spot, Box::new(other)),
        }
    }
}

fn chumsky_spot_to_hoon_spot(
    span: (usize, usize),
    wer: &Path,
    linemap: &Arc<LineMap>,
) -> Spot {
    let (start, end) = span;

    let (sl, sc) = linemap.line_col(start);
    let (el, ec) = linemap.line_col(end);

    Spot {
        p: wer.clone(),
        q: Pint {
            p: (sl as u64, sc as u64),
            q: (el as u64, ec as u64),
        },
    }
}

pub fn print_noun(
    noun: &Noun,
    max_depth: usize,
    current_depth: usize,
) -> String {
    if current_depth >= max_depth {
        return "...".to_string();
    }

    match noun.as_either_atom_cell() {
        Left(atom) => format!("{:?}", atom),

        Right(cell) => {
            let head = cell.head();
            let tail = cell.tail();

            let head_is_atom = head.as_either_atom_cell().is_left();
            let tail_is_atom = tail.as_either_atom_cell().is_left();

            if head_is_atom && tail_is_atom {
                format!(
                    "[{} {}]",
                    print_noun(&head, max_depth, current_depth + 1),
                    print_noun(&tail, max_depth, current_depth + 1),
                )
            } else {
                let indent = "  ".repeat(current_depth);
                let inner_indent = "  ".repeat(current_depth + 1);

                format!(
                    "[\n{}{}\n{}{}\n{}]",
                    inner_indent,
                    print_noun(&head, max_depth, current_depth + 1),
                    inner_indent,
                    print_noun(&tail, max_depth, current_depth + 1),
                    indent,
                )
            }
        }
    }
}

// pub fn print_noun(
//     noun: &Noun,
//     max_depth: usize,
//     current_depth: usize,
// ) -> String {
//     if current_depth >= max_depth {
//         return "...".to_string();
//     }

//     let indent = "  ".repeat(current_depth);

//     match noun.as_either_atom_cell() {
//         Left(atom) => format!("{:?}", atom),
//         Right(cell) => format!(
//             "[\n{}  {}\n{}  {}\n{}]",
//             indent,
//             print_noun(&cell.head(), max_depth, current_depth + 1),
//             indent,
//             print_noun(&cell.tail(), max_depth, current_depth + 1),
//             indent,
//         ),
//     }
// }

fn skip_dbug(mut n: Noun) -> Noun {
    loop {
        let cell = match n.cell() {
            Some(c) => c,
            None => return n,
        };

        let head = match cell.head().as_atom() {
            Ok(a) => a,
            Err(_) => return n,
        };

        if unsafe { !head.as_noun().raw_equals(&D(tas!(b"dbug"))) } {
            return n;
        }

        let tail_cell = match cell.tail().as_cell() {
            Ok(c) => c,
            Err(_) => return n,
        };

        n = tail_cell.tail();
    }
}

pub fn diff_noun(a: &Noun, b: &Noun, printed: &mut bool) -> Result<(), ()> {
    let a = skip_dbug(*a);
    let b = skip_dbug(*b);

    if slab_noun_equality(&a, &b) {
        return Ok(());
    }

    match (a.as_either_atom_cell(), b.as_either_atom_cell()) {
        (Right(ac), Right(bc)) => {
            if diff_noun(&ac.head(), &bc.head(), printed).is_err() {
                if !*printed {
                    print_context(&a, &b);
                    *printed = true;
                }
                return Err(());
            }

            if diff_noun(&ac.tail(), &bc.tail(), printed).is_err() {
                if !*printed {
                    print_context(&a, &b);
                    *printed = true;
                }
                return Err(());
            }

            Ok(())
        }

        _ => Err(()),
    }
}

fn print_context(a: &Noun, b: &Noun) {
    println!("Mismatch in subtree:");
    println!("expected: {}", print_noun(a, 40, 0));
    println!("actual:   {}", print_noun(b, 40, 0));
}

pub fn diff_and_report(a: &Noun, b: &Noun) {
    let mut printed = false;
    if diff_noun(a, b, &mut printed).is_ok() {
        println!("Test passed!");
    }
}

// fn atom_to_tas_string(atom: &DirectAtom) -> String {
//     let val: u128 = atom.data() as u128;
//     if val == 0 { return String::new(); }

//     let bytes = val.to_le_bytes();
//     let mut null_seen = false;
//     let mut valid = true;
//     let mut len = 0;

//     for &b in &bytes {
//         if b == 0 {
//             null_seen = true;
//         } else if null_seen {
//             valid = false;
//             break;
//         } else if !b.is_ascii_lowercase() && b != b'-' {
//             valid = false;
//             break;
//         } else {
//             len += 1;
//         }

//         if len > 126 { valid = false; break; }
//     }

//     if valid && len > 0 {
//         format!("%{}", unsafe { std::str::from_utf8_unchecked(&bytes[..len]) })
//     } else {
//         String::new()
//     }
// }

pub fn collect_inputs(path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_inputs_inner(path, &mut files);
    files.sort();
    files
}

fn collect_inputs_inner(path: &PathBuf, out: &mut Vec<PathBuf>) {
    if path.is_file() {
        if path.extension().and_then(|e| e.to_str()) == Some("hoon") {
            out.push(path.to_path_buf());
        }
    } else if path.is_dir() {
        let entries = std::fs::read_dir(path).unwrap_or_else(|e| {
            eprintln!("Failed to read directory '{}': {}", path.display(), e);
            std::process::exit(1);
        });

        for entry in entries {
            let entry = entry.unwrap_or_else(|e| {
                eprintln!("Failed to read directory entry in '{}': {}", path.display(), e);
                std::process::exit(1);
            });

            collect_inputs_inner(&entry.path(), out);
        }
    } else {
        eprintln!("Invalid input path: {}", path.display());
        std::process::exit(2);
    }
}

