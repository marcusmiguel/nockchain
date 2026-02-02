use crate::ast::hoon::*;
use crate::utils::*;
use crate::atom::*;
use crate::runes::*;
use crate::skin_formation::flay;
use std::sync::Arc;
use either::Either::{self, Left, Right};
use chumsky::{
    prelude::*,
    error::Simple,
    input::InputRef
};

use crate::parser_main::{spec_wide_parser,
                            hoon_wide_parser,
                            hoon_tall_parser,
                            spec_parser};

pub fn setup_hoon_parsers<'src>(
    source: &str,
) -> (impl ParserExt<'src, Hoon>,
      impl ParserExt<'src, Hoon>,
      Arc<LineMap>)
{
    // TODO: get file path and dbug flag all
    //       the way down here for better traces?

    let wer: Vec<String> = vec![];

    let linemap = Arc::new(LineMap::new(&source));

    let (t, w) = create_hoon_parsers(wer, false, linemap.clone());

    (t, w, linemap)
}

pub fn create_hoon_parsers<'src>(
    wer: Path,
    bug: bool,
    linemap: Arc<LineMap>,
) -> (impl ParserExt<'src, Hoon>,
      impl ParserExt<'src, Hoon>) {

    let mut hoon                = Recursive::declare();
    let mut hoon_wide           = Recursive::declare();
    let mut spec                = Recursive::declare();
    let mut spec_wide           = Recursive::declare();

    let mut hoon_no_trace       = Recursive::declare();
    let mut hoon_wide_no_trace  = Recursive::declare();
    let mut spec_no_trace       = Recursive::declare();
    let mut spec_wide_no_trace  = Recursive::declare();

    let spec_body = spec_parser(hoon.clone(),
                                hoon_wide.clone(),
                                spec.clone(),
                                spec_wide.clone())
                                .map_with(wrap_spec_with_trace(wer.clone(), linemap.clone()))
                                .labelled("Spec")
                                .boxed();

    spec.define(spec_body);

    let spec_wide_body =
            spec_wide_parser(spec_wide.clone(),
                             hoon_wide.clone(),
                             linemap.clone())
                            .map_with(wrap_spec_with_trace(wer.clone(), linemap.clone()))
                            .labelled("Spec Wide")
                            .boxed();

    spec_wide.define(spec_wide_body);

    let hoon_wide_body = hoon_wide_parser(
                                hoon.clone(),
                                hoon_wide.clone(),
                                spec_wide.clone(),
                                hoon_wide.clone(),
                                hoon_wide_no_trace.clone(),
                                wer.clone(),
                                linemap.clone(),
                            )
                            .map_with(wrap_hoon_with_trace(wer.clone(), linemap.clone()))
                            .labelled("Hoon Wide")
                            .boxed();

    hoon_wide.define(hoon_wide_body);

    let hoon_body =
            hoon_tall_parser(hoon.clone(),
                        hoon_wide.clone(),
                        spec.clone(),
                        spec_wide.clone(),
                        hoon.clone(),
                        hoon_no_trace.clone(),
                        hoon_wide.clone(),
                        hoon_wide_no_trace.clone(),
                        linemap.clone())
                        .map_with(wrap_hoon_with_trace(wer.clone(), linemap.clone()))
                        .labelled("Hoon")
                        .boxed();

    hoon.define(hoon_body);

    let hoon_no_trace_body =
            hoon_tall_parser(hoon_no_trace.clone(),
                        hoon_wide_no_trace.clone(),
                        spec_no_trace.clone(),
                        spec_wide_no_trace.clone(),
                        hoon.clone(),
                        hoon_no_trace.clone(),
                        hoon_wide.clone(),
                        hoon_wide_no_trace.clone(),
                        linemap.clone())
                        .labelled("Hoon")
                        .boxed();

    hoon_no_trace.define(hoon_no_trace_body);

    let hoon_wide_no_trace_body
                    = hoon_wide_parser(
                                        hoon_no_trace.clone(),
                                        hoon_wide_no_trace.clone(),
                                        spec_wide_no_trace.clone(),
                                        hoon_wide.clone(),
                                        hoon_wide_no_trace.clone(),
                                        wer.clone(),
                                        linemap.clone(),
                                    )
                                    .labelled("Hoon Wide")
                                    .boxed();

    hoon_wide_no_trace.define(hoon_wide_no_trace_body);

    let spec_body_no_trace = spec_parser(hoon_no_trace.clone(),
                                         hoon_wide_no_trace.clone(),
                                         spec_no_trace.clone(),
                                         spec_wide_no_trace.clone())
                                        .labelled("Spec")
                                        .boxed();

    spec_no_trace.define(spec_body_no_trace);

    let spec_wide_no_trace_body =
            spec_wide_parser(spec_wide_no_trace.clone(),
                             hoon_wide_no_trace.clone(),
                            linemap)
                            .labelled("Spec Wide")
                             .boxed();

    spec_wide_no_trace.define(spec_wide_no_trace_body);

    let hoon      = if bug { hoon } else { hoon_no_trace };
    let hoon_wide = if bug { hoon_wide } else { hoon_wide_no_trace };

    (hoon, hoon_wide)
}

pub fn setup_sail_parsers<'src>(
    hoon: impl ParserExt<'src, Hoon>,
    hoon_wide: impl ParserExt<'src, Hoon>,
    linemap: Arc<LineMap>,
) -> (impl ParserExt<'src, Either<Manx, Marl>> + Clone,
      impl ParserExt<'src, Either<Manx, Marl>> + Clone) {

    let mut sail_tall = Recursive::declare();
    let mut sail_wide = Recursive::declare();

    let tall_body = tall_top(hoon.clone(),
                                hoon_wide.clone(),
                                sail_tall.clone(),
                                sail_wide.clone(),
                                linemap.clone(),
                            ).boxed();

    let wide_body = wide_top(hoon_wide.clone(),
                                sail_tall.clone(),
                                sail_wide.clone(),
                                linemap.clone(),
                            ).boxed();

    sail_tall.define(tall_body);
    sail_wide.define(wide_body);

    (sail_tall, sail_wide)

}

pub fn sail_tall_parser<'src>(
    hoon: impl ParserExt<'src, Hoon>,
    hoon_wide: impl ParserExt<'src, Hoon>,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>> {
    let (sail_tall, _sail_wide) = setup_sail_parsers(hoon, hoon_wide, linemap);
    sail_tall.clone()
    .map(|res: Either<Manx, Marl>| {
        match res {
            Left(m) => Hoon::Xray(m),
            Right(m) => Hoon::MicTis(m),
        }
    })
    .labelled("Sail Tall")
}

pub fn sail_wide_parser<'src>(
    hoon: impl ParserExt<'src, Hoon>,
    hoon_wide: impl ParserExt<'src, Hoon>,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Hoon, Err<'src>> {
    let (_sail_tall, sail_wide) = setup_sail_parsers(hoon, hoon_wide, linemap);

    sail_wide.clone()
    .map(|res: Either<Manx, Marl>| {
        match res {
            Left(m) => Hoon::Xray(m),
            Right(m) => Hoon::MicTis(m),
        }
    })
    .labelled("Sail Wide")
}

fn debug_remaining<'src>(label: &'src str) -> impl Parser<'src, &'src str, (), Err<'src>> + Clone {
    custom(move |input: &mut InputRef<'src, '_, &'src str, Err<'src>>| {
        let start = input.cursor();
        let remaining = input.slice_from(&start..);
        println!("{}: {:?}", label, remaining);
        Ok(())
    })
}

fn tall_top<'src>(
    hoon: impl ParserExt<'src, Hoon>,
    hoon_wide: impl ParserExt<'src, Hoon>,
    sail_tall: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Either<Manx, Marl>, Err<'src>> {
    choice((
            just(' ').repeated().at_least(1)
            .ignore_then(quote_innards(hoon_wide.clone(),
                                        sail_wide.clone(),
                                        linemap.clone(),
                                        true,
                                        false))
            .map(|innards| {
                Right(collapse_chars(true, innards))
            }),
        script_or_style(hoon_wide.clone())
            .then(script_style_tail())
            .map(|(marx, marl)| {
                Left(Manx { g: marx, c: marl })
            }),
        tall_elem(hoon.clone(),
                    hoon_wide.clone(),
                    sail_tall.clone(),
                    sail_wide.clone(),
                    linemap.clone()).map(Left),
        wide_quote(hoon_wide.clone(),
                   sail_wide.clone(),
                   linemap.clone(),
                   true).map(Right),
        just('=').ignore_then(tall_tail(hoon.clone(),
                                hoon_wide.clone(),
                                sail_tall.clone(),
                                sail_wide.clone(),
                                linemap.clone())).map(Right),
        just('<').ignore_then(gap())
                    .ignore_then(cram(linemap.clone()))
                    .map(Right),
        tuna_mode()
            .then_ignore(gap())
            .then(hoon.clone())
            .map(|(mode, h)| {
                match mode {
                    TunaMode::Tape => Right(vec![Tuna::Tape(h)]),
                    TunaMode::Marl => Right(vec![Tuna::Marl(h)]),
                    TunaMode::Manx => Right(vec![Tuna::ManxHoon(h)]),
                    TunaMode::Call => Right(vec![Tuna::Call(h)]),
                }
            }),
        empty().to({
            Right(vec![micfas(vec![ParsedAtom::Small(10)])])
        }),
    )).boxed()
    .labelled("Sail Tall")
}

fn wide_top<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_tall:  impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    sail_wide:  impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Either<Manx, Marl>, Err<'src>> {

    let tagged = tag_head(hoon_wide.clone(), linemap.clone())
        .then(wide_tail(hoon_wide.clone(),
                        sail_wide.clone(),
                        linemap.clone()))
        .map(|(head, tail)| {
            let manx = Manx { g: head, c: tail };
            Left(manx)
        }).boxed();

    choice((wide_quote(hoon_wide.clone(),
                        sail_wide.clone(),
                        linemap.clone(),
                        false).map(Right),
            wide_paren_elems(hoon_wide.clone(),
                             sail_wide.clone(),
                             linemap.clone()).map(Right),
            tagged,
            )).boxed()
    .labelled("Sail Wide")

}

fn cram<'src>(
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    custom(move |input: &mut InputRef<'src, '_, &'src str, Err<'src>>| {
        let start = input.cursor();

        let start_txt = input.slice_from(&start..);

        let byte_offset = input.span_since(&start).start;
        let (line, col) = linemap.line_col(byte_offset);

        let start_loc = (line as u64, col as u64);
        let mut state = CramState::new(start_txt, start_loc);
        let (hair, result) = state.resolve();

        match result {
            Some((marl, end_loc, remaining_txt)) => {

                if marl.is_empty() {
                    let span = input.span_since(&start);
                    return Err(Rich::custom(span,
                            "Markdown parse error".to_string()).into());
                };

                let consumed_chars = start_txt.len() - remaining_txt.len();

                for _ in 0..consumed_chars {
                    input.next();
                }

                Ok(marl)
            },
            None => {
                let start_offset = input.span_since(&start).start;

                let fail_offset = linemap.offset(hair.0, hair.1)
                    .unwrap_or(start_offset);

                let fail_span: SimpleSpan = (fail_offset..fail_offset).into();

                Err(Rich::custom(fail_span,
                    format!("Markdown syntax error at line {}, column {}",
                        hair.0, hair.1)))
            }
        }
    }).labelled("Markdown")
}

pub fn collapse_chars(tall: bool, reb: Vec<Either<Tuna, ParsedAtom>>) -> Marl {
    let mut sim = Vec::new();
    let mut tuz = Vec::new();

    for item in reb {
        match item {
            Right(atom) => {
                sim.push(atom);
            }
            Left(tuna) => {
                if !sim.is_empty() {
                    tuz.push(micfas(std::mem::take(&mut sim)));
                }
                tuz.push(tuna);
            }
        }
    }

    if tall {
        while let Some(ParsedAtom::Small(32)) = sim.last() {
            sim.pop();
        }
        sim.push(ParsedAtom::Small(10));
        tuz.push(micfas(sim));
    } else {
        if !sim.is_empty() {
            tuz.push(micfas(sim));
        }
    }

    tuz
}

pub fn micfas(atoms: Vec<ParsedAtom>) -> Tuna {
    let beers: Vec<Beer> = atoms.into_iter().map(Beer::Char).collect();
    Tuna::Manx(Manx {
        g: Marx {
            n: Mane::Tag("".to_string()),
            a: vec![(Mane::Tag("".to_string()), beers)],
        },
        c: vec![],
    })
}

#[derive(serde::Serialize, PartialEq, Debug, Clone)]
pub enum TunaMode {
    Tape,
    Manx,
    Marl,
    Call
}

fn bracketed_elem<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Manx, Err<'src>> {

    tag_head(hoon_wide.clone(), linemap.clone())
    .then(wide_elems(hoon_wide.clone(), sail_wide.clone(), linemap))
    .map(|(p, q)| {
        Manx {
            g: p,
            c: q,
        }
    })
    .delimited_by(just("{"), just("}"))
    .labelled("Bracketed Elem")

}

fn drop_top(a: Either<Tuna, Marl>) -> Marl {
    match a {
        Left(tuna) => vec![tuna],
        Right(marl) => marl,
    }
}

fn join_tops(a: Vec<Either<Tuna, Marl>>) -> Marl {
    a.into_iter()
        .map(drop_top)
        .flatten()
        .collect()
}

fn wide_elems<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    just(' ')
    .ignore_then(wide_inner_top(hoon_wide,
                                sail_wide.clone(),
                                linemap))
    .repeated()
    .at_least(1)
    .collect::<Vec<Either<Tuna, Marl>>>()
    .map(|w| join_tops(w))
}

fn wide_attrs<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Mart, Err<'src>> {
    a_mane()
        .then_ignore(just(' '))
        .then(hopefully_quote(hoon_wide.clone()))
    .separated_by(just(", "))
    .collect::<Mart>()
    .delimited_by(just("("), just(")"))
    .or_not()
    .map(|opt| opt.unwrap_or_default())
    .labelled("Wide Attrs")
}

fn hopefully_quote<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
) -> impl Parser<'src, &'src str, Vec<Beer>, Err<'src>> {
    hoon_wide
    .map(|a| {
        match a {
            Hoon::Knit(p) => woofs_to_beers(p),
            _ => vec![Beer::Hoon(a)],
        }
    })
}

fn tag_head<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marx, Err<'src>> {
    let id = just('#')                //  #id
            .to(Mane::Tag("id".to_string()))
            .then(symbol()
                    .map(|s| {
                        s.chars()
                        .map(|c| Beer::Char(string_to_atom(c.to_string())))
                        .collect::<Vec<Beer>>()
                    }));

    let class = just('.')
                .ignore_then(symbol())
                .repeated()
                .collect::<Vec<String>>()
                .map(|v: Vec<String>| {
                    if v.is_empty() {
                        None
                    } else {
                        let joined = v.join(" ");
                        let embeds: Vec<Beer> = joined
                            .chars()
                            .map(|c| Beer::Char(string_to_atom(c.to_string())))
                            .collect();
                        Some((Mane::Tag("class".to_string()), embeds))
                    }
                });

    let href_or_src = choice((just('/').to(Mane::Tag("href".to_string())),
                            just('@').to(Mane::Tag("src".to_string()))
                        ))
                        .then(soil(hoon_wide.clone(), linemap)
                        .map(|woof| woofs_to_beers(woof)));

    a_mane()
    .then(id.or_not()
        .then(class) // option
        .then(href_or_src.or_not())
        .map(|((a, b), c)| {
            [a, b, c].into_iter().flatten().collect::<Mart>()
        }))
    .then(wide_attrs(hoon_wide.clone()))
    .map(|((a, mut b), c): ((Mane, Mart), Mart)| {
        b.extend(c);
        Marx { n: a,
               a: b,
            }
    })
    .labelled("Tag Head")
}

pub fn mixed_case_symbol<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    any().filter(|c: &char| c.is_ascii_alphabetic())
    .then(any()
          .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '-')
          .repeated()
          .collect::<String>())
    .map(|(first, rest)| format!("{first}{rest}"))
}

fn a_mane<'src>(
) -> impl Parser<'src, &'src str, Mane, Err<'src>> {
    mixed_case_symbol()
    .then(
        just('_')
        .ignore_then(mixed_case_symbol())
        .or_not()
    )
    .map(|(p, maybe_sym)|{
        match maybe_sym {
            None => Mane::Tag(p),
            Some(q) => Mane::TagSpace(p, q),
        }
    })
    .labelled("Tag Symbol")
}

fn tuna_mode<'src>(
) -> impl Parser<'src, &'src str, TunaMode, Err<'src>> {
    choice((
        just('-').to(TunaMode::Tape),
        just('+').to(TunaMode::Manx),
        just('*').to(TunaMode::Marl),
        just('%').to(TunaMode::Call),
    ))
}

fn inline_embed<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Tuna, Err<'src>> {
    choice((
        just(";")
        .ignore_then(
            bracketed_elem(hoon_wide.clone(),
                            sail_wide.clone(),
                            linemap)
            .map(|manx| Tuna::Manx(manx))
        ),
        tuna_mode()
        .then(sump(hoon_wide.clone()))
        .map(|(mode, h)| {
            match mode {
                TunaMode::Tape => Tuna::Tape(h),
                TunaMode::Marl => Tuna::Marl(h),
                TunaMode::Manx => Tuna::ManxHoon(h),
                TunaMode::Call => Tuna::Call(h),
            }
        }),
        sump(hoon_wide.clone()).map(|h| {
            Tuna::Tape(h)
        }),
    ))
}

fn quote_innards<'src>(
    hoon_wide: impl ParserExt<'src, Hoon>,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
    tall: bool,
    allow_linebreak: bool,
) -> impl Parser<'src, &'src str, Vec<Either<Tuna, ParsedAtom>>, Err<'src>> {
    let escaped =
            just('\\').ignore_then(
            choice((one_of("--+*%;{{").map(|c| c as u128),
                    just('\\').to(92 as u128),
                    just('"').to(34 as u128),
                    // \HH hex escape
                    any().filter(|c: &char| c.is_ascii_hexdigit())
                        .then(any().filter(|c: &char| c.is_ascii_hexdigit()))
                        .map(|(a, b)| {
                            let hx = format!("{}{}", a, b);
                            let byte = u8::from_str_radix(&hx, 16).unwrap();
                            byte as u128
                        })
                    )))
                    .map(|n| {
                        Right(ParsedAtom::Small(n))
                    })
                    .boxed();

    //  chars from 32-256 (excluding DEL, {, \)
    let tall_char  = any().filter(|c: &char| {
        let x = *c as u32;
        (0x20..=0x7E).contains(&x)
            && x != 0x7B    // {
            && x != 0x5C    // \
        || (0x80..=0xFF).contains(&x)
    });

    //  chars from 32-256 (excluding DEL, {, \, ")
    let wide_char = any().filter(|c: &char| {
        let x = *c as u32;
        (0x20..=0x7E).contains(&x)
            && x != 0x7B    // {
            && x != 0x5C    // \
            && x != 0x22    // "
        || (0x80..=0xFF).contains(&x)
    });

    let char =
        if tall {
            tall_char
            .map(|c| Right(ParsedAtom::Small(c as u128)))
            .boxed()
        } else {
            wide_char
            .map(|c| Right(ParsedAtom::Small(c as u128)))
            .boxed()
        };

    let embed = inline_embed(hoon_wide.clone(), sail_wide.clone(), linemap)
                .map(|tuna| {
                    Left(tuna)
                }).labelled("Embed");

    if allow_linebreak {
        let linebreak = just("\n\"\"\"").not()
                        .ignore_then(newline()
                                     .to(Right(ParsedAtom::Small(10 as u128))));

        choice((escaped,
                embed,
                char,
                linebreak))
        .repeated()
        .collect::<Vec<Either<Tuna, ParsedAtom>>>()
        .labelled("Escaped or Char or Embed or Linebreak")
        .boxed()
    } else {
        choice((escaped,
        embed,
        char))
        .repeated()
        .collect::<Vec<Either<Tuna, ParsedAtom>>>()
        .labelled("Escaped or Char or Embed")
        .boxed()
    }
}

fn wide_inner_top<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Either<Tuna, Marl>, Err<'src>> {
    let wide =
        sail_wide.clone()
        .map(|w| {
            match w {
                Right(marl) => Right(marl),
                Left(manx)  => Left(Tuna::Manx(manx)),
            }
        }).boxed();

    let tuna_wide =
        tuna_mode()
        .then(sump(hoon_wide.clone()))
        .map(|(mode, h)| {
            match mode {
                TunaMode::Tape => Tuna::Tape(h),
                TunaMode::Marl => Tuna::Marl(h),
                TunaMode::Manx => Tuna::ManxHoon(h),
                TunaMode::Call => Tuna::Call(h),
            }
        });

    choice((
        wide,
        tuna_wide.map(Left)
    ))
}

fn wide_tail<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    let colon_case = just(':')
        .ignore_then(wrapped_elems(hoon_wide.clone(), sail_wide.clone(), linemap.clone())).boxed();
    let mic_case = just(';').to(vec![]);
    let default = empty().to(vec![]);

    choice((colon_case,
            mic_case,
            default))
}

fn wrapped_elems<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    let paren = wide_paren_elems(hoon_wide.clone(), sail_wide.clone(), linemap.clone());
    let text =
        cord(linemap.clone())
        .map(|c| {
            let s = trip(c).into_iter().map(string_to_atom).collect();
            vec![micfas(s)]
        });
    let top = sail_wide.clone()
              .map(|w| match w {
                    Right(marl) => drop_top(Right(marl)),
                    Left(manx)  => drop_top(Left(Tuna::Manx(manx))),
                });

    choice((paren,
            text,
            top))
}

fn wide_paren_elems<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    let inner = wide_inner_top(hoon_wide, sail_wide.clone(), linemap);
    just('(')
        .ignore_then(inner
                    .separated_by(just(' '))
                    .collect::<Vec<Either<Tuna, Marl>>>()
                )
        .then_ignore(just(')'))
        .map(join_tops)
}

fn wide_quote<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
    tall: bool,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    let single_line =
            just("\"\"\"").not()
                .ignore_then(quote_innards(hoon_wide.clone(), sail_wide.clone(), linemap.clone(), tall, false))
                .map(move |innards| {
                    collapse_chars(tall, innards)
                })
                .delimited_by(just("\""), just("\""));

    let multi_line_prefix_spaces = just(' ').repeated();
    let linemap2 = linemap.clone();
    let multi_line_open =
        just("\"\"\"").map_with(move |_, extra| {
            let span: SimpleSpan = extra.span();
            let (_line, col) = linemap2.line_col(span.start);
            if col > 0 { (col - 1) as usize } else { 0 }
        });

    let multi_line_close =
        newline()
            .ignore_then(just(' ').repeated().count())
            .then_ignore(just("\"\"\"")).boxed();

    let multi_line_content =
        quote_innards(hoon_wide.clone(),
                        sail_wide.clone(),
                        linemap.clone(),
                        tall,
                        true);

    let line =
        multi_line_close.clone().not()
        .ignore_then(
            newline()
            .ignore_then(just(' ').repeated().count())
            .then(multi_line_content));

    let multi_line =
        multi_line_prefix_spaces
        .ignore_then(multi_line_open)
        .then(line
                .repeated()
                .collect::<Vec<(usize, Vec<Either<Tuna, ParsedAtom>>)>>())
        .then(multi_line_close)
        .validate(move |((base_indent, lines), close_indent), extra, emit| {
            let span = extra.span();

            if close_indent != base_indent {
                emit.emit(Rich::custom(
                    span,
                    "closing delimiter indentation mismatch",
                ));
                return vec![];
            }

            let mut out = Vec::new();

            for (mut indent, mut line) in lines {
                if indent > base_indent {
                    let extra = indent - base_indent;
                    indent = base_indent;
                    let space = Right(ParsedAtom::Small(' ' as u128));
                    line.splice(0..0, std::iter::repeat(space).take(extra));
                }

                if indent != base_indent && !(line.is_empty() && indent == 0) {
                    emit.emit(Rich::custom(
                        span,
                        "inconsistent indentation in wide quote",
                    ));
                    return vec![];
                }

                out.push(Right(ParsedAtom::Small('\n' as u128)));
                if !line.is_empty() {
                    out.extend(line);
                }
            }

            if !out.is_empty() {
                out.remove(0);
            }
            collapse_chars(tall, out)
        });

    choice((single_line,
            multi_line)).labelled("Quote")
}

fn tall_attrs<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
) -> impl Parser<'src, &'src str, Mart, Err<'src>> {
    let attr_pair = gap()
        .ignore_then(just('='))
        .ignore_then(
            a_mane()
            .then_ignore(gap())
            .then(hopefully_quote(hoon_wide))
        );

    attr_pair
    .repeated()
    .collect::<Mart>()
    .labelled("Tall Attrs")
}

fn tall_kids<'src>(
    hoon: impl ParserExt<'src, Hoon> + Clone,
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_tall: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {

    let sail = just(';')
                .ignore_then(sail_tall.clone())
                .map(|e| {
                    match e {
                        Left(manx) => Left(Tuna::Manx(manx)),
                        Right(marl) => Right(marl),
                    }})
                .labelled("Sail Tall");

    let markdown = cram(linemap.clone())
                            .map(Right)
                            .labelled("MarkDown");

    choice((sail, markdown))
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<Either<Tuna, Marl>>>()
    .map(join_tops)
    .labelled("Sail Tall or Markdown")
}

fn tall_tail<'src>(
    hoon: impl ParserExt<'src, Hoon> + Clone,
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_tall: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    let empty_case = just(';').to(vec![]);

    let wrapped_case = just(':')
        .ignore_then(wrapped_elems(hoon_wide.clone(),
                                    sail_wide.clone(),
                                    linemap.clone()));

    let quote_case = just(": ")
        .ignore_then(
            quote_innards(hoon_wide.clone(),
                          sail_wide.clone(),
                          linemap.clone(),
                          true,
                          false)
                .map(|v| collapse_chars(false, v)));

    let indented_case =
        tall_kids(hoon.clone(),
                  hoon_wide.clone(),
                  sail_tall.clone(),
                  linemap)
        .delimited_by(gap(), gap().ignore_then(just("==")));

    choice((empty_case,
            wrapped_case,
            quote_case,
            indented_case))
    .labelled("Tall Tail")
}

fn tall_elem<'src>(
    hoon: impl ParserExt<'src, Hoon> + Clone,
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_tall: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Manx, Err<'src>> {

    tag_head(hoon_wide.clone(), linemap.clone())
    .then(tall_attrs(hoon_wide.clone()))
    .then(tall_tail(hoon.clone(),
                    hoon_wide.clone(),
                    sail_tall.clone(),
                    sail_wide.clone(),
                    linemap))
    .map(|((head, attrs), tail)| {
        let mut all_attrs = head.a;
        all_attrs.extend(attrs);
        Manx {
            g: Marx { n: head.n, a: all_attrs },
            c: tail,
        }
    })

}

fn script_or_style<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
) -> impl Parser<'src, &'src str, Marx, Err<'src>> {
    let tag = just("script").or(just("style"));
    tag.then(wide_attrs(hoon_wide))
         .map(|(name, attrs)| {
            Marx {
                n: Mane::Tag(name.to_string()),
                a: attrs,
            }
        })
}

fn script_style_tail<'src>(
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    let content =
        choice((just(' ')
                    .ignore_then(non_control_char()
                    .repeated()
                    .collect::<String>()),
                newline().to('\n'.to_string())
            ))
        .repeated()
        .collect::<Vec<String>>()
        .map(|parts| parts.concat());

    content.map(|s: String| {
        let atoms: Vec<ParsedAtom> = s
            .bytes()
            .map(|b| ParsedAtom::Small(b as u128))
            .collect();
        micfas(atoms)
    })
    .separated_by(gap())
    .at_least(1)
    .collect::<Marl>()
    .delimited_by(gap(), gap().ignore_then(just("==")))
}

#[derive(Debug, Clone, PartialEq)]
pub enum Graf {
    Bold(Vec<Graf>),           // *bold*
    Talc(Vec<Graf>),           // _italics_
    Quod(Vec<Graf>),           // "double quote"
    Code(String),              // `code` literal (tape = String)
    Text(String),              // plain text (tape = String)
    Link(Vec<Graf>, String),   // [text](url)
    Mage(String, String),      // ![alt](url) → (alt, url)
    Expr(Tuna),                // interpolated Hoon expression
}

trait GrafText {
    fn text(&self) -> Option<String>;
}

impl GrafText for &Graf {
    fn text(&self) -> Option<String> {
        match self {
            Graf::Text(s) => Some(s.to_string()),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trig {
    pub col: u64,
    pub sty: TrigStyle,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EndType {
    Done,
    Stet,
    Dent,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OneType {
    Rule,
    Fens,
    Expr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrigStyle {
    End(EndType),
    One(OneType),
    New(TrigNew),
    OldText,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TrigNew {
    Lite,
    Lint,
    Head,
    Bloc,
    Poem,
}

pub type Item = (Mite, Marl);

#[derive(Debug, Clone, PartialEq)]
pub enum Mite {
    Down,
    Lunt,
    Lime,
    Lord,
    Poem,
    Bloc,
    Head,
}

pub type Hair = (u64, u64);
type Tape = String;
type Wall = Vec<Tape>;

fn item(graf: &Graf) -> Marl {
    match graf {
        Graf::Text(_) => unreachable!(),

        Graf::Expr(tuna) => vec![tuna.clone()],

        Graf::Bold(inner_grafs) => {
            let kids = graf_to_tuna(inner_grafs.clone());
            vec![Tuna::Manx(Manx {
                g: Marx {
                    n: Mane::Tag("b".to_string()),
                    a: vec![],
                },
                c: kids,
            })]
        }

        Graf::Talc(inner_grafs) => {
            let kids = graf_to_tuna(inner_grafs.clone());
            vec![Tuna::Manx(Manx {
                g: Marx {
                    n: Mane::Tag("i".to_string()),
                    a: vec![],
                },
                c: kids,
            })]
        }

        Graf::Code(s) => {
            vec![Tuna::Manx(Manx {
                g: Marx {
                    n: Mane::Tag("code".to_string()),
                    a: vec![],
                },
                c: vec![micfas(
                    s.bytes().map(|b| ParsedAtom::Small(b as u128)).collect()
                )],
            })]
        }

        Graf::Quod(inner_grafs) => {
            let mut quoted = vec![
                Graf::Text("\u{201c}".to_string()), // left double quote
            ];
            quoted.extend(inner_grafs.clone());
            quoted.push(Graf::Text("\u{201d}".to_string())); // right double quote
            graf_to_tuna(quoted)
        }

        Graf::Link(text_grafs, url) => {
            let kids = graf_to_tuna(text_grafs.clone());
            vec![Tuna::Manx(Manx {
                g: Marx {
                    n: Mane::Tag("a".to_string()),
                    a: vec![(Mane::Tag("href".to_string()), vec![Beer::Char(string_to_atom(url.to_string()))])],
                },
                c: kids,
            })]
        }

        Graf::Mage(alt, url) => {
            let mut attrs = vec![
                (Mane::Tag("src".to_string()), vec![Beer::Char(string_to_atom(url.to_string()))]),
            ];
            if !alt.is_empty() {
                attrs.push((Mane::Tag("alt".to_string()), vec![Beer::Char(string_to_atom(alt.to_string()))]));
            }
            vec![Tuna::Manx(Manx {
                g: Marx {
                    n: Mane::Tag("img".to_string()),
                    a: attrs,
                },
                c: vec![],
            })]
        }
    }
}

fn graf_to_tuna(grafs: Vec<Graf>) -> Marl {
        if grafs.is_empty() {
            return vec![];
        }

    fn main(grafs: &[Graf]) -> Marl {
        if grafs.is_empty() {
            return vec![];
        }

        let first = &grafs[0];
        let rest = &grafs[1..];

        match first {
            Graf::Text(_) => {
                let mut fip = vec![first.text().unwrap().clone()];
                let mut remaining = rest;
                loop {
                    if remaining.is_empty() {
                        let full_text = fip.join("");
                        return vec![micfas(
                            full_text.bytes().map(|b| ParsedAtom::Small(b as u128)).collect()
                        )];
                    }

                    let next = &remaining[0];
                    if let Graf::Text(_) = next {
                        fip.push(next.text().unwrap().to_string());
                        remaining = &remaining[1..];
                    } else {
                        let full_text = fip.join("");
                        let text_node = micfas(
                            full_text.bytes().map(|b| ParsedAtom::Small(b as u128)).collect()
                        );
                        let tail = main(remaining);
                        return weld(vec![text_node], tail);
                    }
                }
            }
            _ => {
                let item_nodes = item(first);
                let tail = main(rest);
                weld(item_nodes, tail)
            }
        }
    }

    main(&grafs)
}

pub fn contents_to_id(tunas: &[Tuna]) -> String {
    fn collect_raw(tunas: &[Tuna]) -> String {
        if tunas.is_empty() {
            return String::new();
        }

        let first = &tunas[0];
        let rest = &tunas[1..];

        let head_text = match first {
            Tuna::Manx(manx) => {
                if manx.g.n == Mane::Tag("".to_string()) {
                    manx.g.a
                        .iter()
                        .flat_map(|(_, beers)| {
                            beers.iter().filter_map(|beer| match beer {
                                Beer::Char(atom) => {
                                    match atom {
                                        ParsedAtom::Small(n) => Some(*n as u8 as char),
                                        ParsedAtom::Big(big) => {
                                            if *big <= u32::MAX.into() {
                                                let code = big.clone().try_into().unwrap_or(0u32);
                                                char::from_u32(code)
                                            } else {
                                                None
                                            }
                                        }
                                    }
                                }
                                Beer::Hoon(_) => None,
                            })
                        })
                        .collect::<String>()
                } else {
                    collect_raw(&manx.c)
                }
            }
            _=> String::new(),
        };

        head_text + &collect_raw(rest)
    }

    let raw = collect_raw(tunas);
    raw.chars()
        .map(|c| {
            if c.is_ascii_lowercase() || c.is_ascii_digit() {
                c
            } else if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

// Markdown Parser

#[derive(Debug, Clone)]
struct CramState {
    err: Option<Hair>,
    ind: (u64, u64),           // (out, inr) ident level
    hac: Vec<Item>,            // stack of items
    cur: Item,                 // current item under construction
    par: Option<(Hair, Wall)>, // current paragraph being read
    loc: Hair,                 // current position
    txt: String,               // remaining input
}

impl<'src> CramState {

    fn new(txt: &'src str, loc: Hair) -> Self {
        Self {
            err: None,
            ind: (0, 0),
            hac: Vec::new(),
            cur: (Mite::Down, Marl::new()),
            par: None,
            loc: loc,
            txt: txt.to_string(),
        }
    }

    // Entry for the Markdown Parser
    fn resolve(mut self,
    ) -> (Hair, Option<(Marl, Hair, Tape)>) {

        self = self.line();

        if let Some(err) = self.err {
            return (err, None);
        }
        self = self.close_par();
        loop {
            if self.hac.is_empty() {
                let result = self.cur_to_tarp();

                return (self.loc, Some((result, self.loc, self.txt.to_string())));
            }
            self = self.close_item();
        }
    }

    fn line(mut self,
    ) -> Self {

        if self.err.is_some() {
            return self;
        }

        let saw = self.clone().look();

        if saw.is_none() {
            let (lin, err, line_loc, line_text) = self.read_line();
            self.loc = line_loc;
            self.txt = line_text;

            if let Some(e) = err {
                self.err = e;
                return self;
            };
            return self
                    .close_par()
                    .line();
        }

        let mut saw = saw.unwrap();

        if matches!(saw.sty, TrigStyle::End(_)) {
            self.loc.1 = saw.col;
            return self;
        }

        if self.ind.0 == 0 {
            self.ind = (saw.col, saw.col);
        }

        if self.par.is_none() ||
            (matches!(self.cur.0, Mite::Down | Mite::Lime | Mite::Bloc) &&
            (!matches!(saw.sty, TrigStyle::OldText) || saw.col > self.ind.1)) {

            self = self.close_par();

            self = self.back(saw.col);

            let (col_ok, new_sty) =
                match saw.col - self.ind.1 {
                    0 => (true, saw.sty),
                    8 => (true, TrigStyle::New(TrigNew::Poem)),
                    _ => (false, saw.sty),
                };

            saw.sty = new_sty;

            if  !col_ok {
                self.err = Some((self.loc.0, saw.col));
                return self;
            }

            self.ind.1 = saw.col;

            self = {
                let should_close =
                    (matches!(self.cur.0, Mite::Lunt) &&
                    !matches!(saw.sty, TrigStyle::New(TrigNew::Lint)))
                    ||
                    (matches!(self.cur.0, Mite::Lord) &&
                    !matches!(saw.sty, TrigStyle::New(TrigNew::Lite)));

                if should_close {
                    self.close_item()
                } else {
                    self
                }
            };
            self = match &saw.sty {
                TrigStyle::One(kind) => {
                    self.read_one(kind.clone())
                }
                TrigStyle::New(new_kind) => {
                    self.open_item(new_kind.clone())
                }
                _ =>  self,
            };
            self.par = Some((self.loc, Vec::new()));
            return self.line();
        }

        let par_is_empty = self
                            .par.as_ref()
                            .map_or(true, |(_, wall)| wall.is_empty());

        let first_line_is_legal = if par_is_empty {
            true
        } else {
            match self.cur.0 {
                Mite::Lord | Mite::Lunt => {
                    panic!()
                }
                Mite::Head => {
                    false
                }
                Mite::Poem => {
                    saw.col >= self.ind.1
                }
                Mite::Down | Mite::Lime | Mite::Bloc => {
                    saw.col == self.ind.1
                }
                _ => true,
            }
        };

        if !first_line_is_legal {
            let mut err_state = self.clone();
            err_state.err = Some((self.loc.0, saw.col));
            return err_state;
        }

        let (lin, err, new_loc, new_txt) = self.read_line();

        self.txt = new_txt;
        self.loc = new_loc;

        self.par = self.par.map(|(loc, mut wall)| {
            wall.push(lin);
            (loc, wall)
        });

        //  if End or error is found
        //  stop the recursion
        if let Some(e) = err {
            self.err = e;
            return self;
        }
        self.line()
    }

    fn look(mut self) -> Option<Trig> {
        let linemap = Arc::new(LineMap::new(&self.txt));
        match look_parse(linemap)
            .then_ignore(any().repeated().collect::<String>())
            .parse(&self.txt)
            .into_result() {
                Ok(Some(mut t)) => {
                    if !matches!(t.sty, TrigStyle::End(_))
                        && t.col < self.ind.0 {
                        t.sty = TrigStyle::End(EndType::Dent);
                    }
                    return Some(t);
                },
                _ => None,
            }
    }

    fn read_one(mut self,
                kind: OneType) -> Self {

        let input_owned = self.txt.clone();
        let input = input_owned.as_str();

        let (hoon, hoon_wide, linemap) = setup_hoon_parsers(&input);

        match kind {
            OneType::Expr =>
                self.parse_block(input,
                                 expr_parse(hoon.clone(),
                                            hoon_wide.clone(),
                                            linemap.clone())),
            OneType::Rule =>
                self.parse_block(input, hrul_parse()),
            OneType::Fens =>
                self.clone().parse_block(input, fens_parse(self.ind.1)),
        }
    }

    fn parse_block<P>(mut self, input: &'src str, parser: P) -> Self
    where
        P: chumsky::Parser<'src, &'src str, Marl, Err<'src>>,
    {
        match parser.then(any().repeated().collect::<String>())
                .parse(input)
                .into_result() {
            Ok((result, rest)) => {
                let consumed = input.len() - rest.len();
                self.txt = rest.to_string();
                self.loc.1 += consumed as u64;
                let mut flipped = result;
                flipped.reverse();
                self.cur.1 = weld(flipped, self.cur.1);
                self
            }
            Err(err) => {
                let first = err.into_iter().next().unwrap();
                let span = first.span().into_range();

                let linemap = LineMap::new(input);
                let (line, col) = linemap.line_col(span.start);

                self.err = Some((line, col));
                self
            }
        }
    }

    fn close_par(mut self,
    ) -> Self {
        let (par_loc, par_wall): (Hair, Wall) = match self.par.take() {
            Some((loc, wall)) => (loc, wall),
            None => return self,
        };

        if matches!(self.cur.0, Mite::Poem) {
            if !self.cur.1.is_empty() {
                let br_node = Tuna::Manx(Manx {
                    g: Marx {
                        n: Mane::Tag("br".to_string()),
                        a: vec![],
                    },
                    c: vec![],
                });
                self.cur.1.insert(0, br_node);
            }

            let stanza_nodes: Vec<Tuna> =
                par_wall
                .into_iter()
                .map(|line_str| {

                    let full_line = format!("{}\n", line_str);
                    let atoms: Vec<ParsedAtom> = full_line
                        .bytes()
                        .map(|b| ParsedAtom::Small(b as u128))
                        .collect();
                    let text_node = micfas(atoms);

                    Tuna::Manx(Manx {
                        g: Marx {
                            n: Mane::Tag("p".to_string()),
                            a: vec![],
                        },
                        c: vec![text_node],
                    })
                })
                .collect();

            self.par = None;
            self.cur.1 = weld(stanza_nodes, self.cur.1);
            self.ind.1 -= 8;
            return self.close_item();
        }

        let yex: String = par_wall
            .into_iter()
            .map(|line_str| {
                format!("{}{}\n", " ".repeat((self.ind.1 - 1) as usize), line_str)
            })
            .collect::<Vec<_>>()
            .join("");

        let (hoon, hoon_wide, linemap) = setup_hoon_parsers(yex.as_str());

        let (sail_tall, sail_wide) = setup_sail_parsers(hoon.clone(),
                                                        hoon_wide.clone(),
                                                        linemap.clone());

        let res = if matches!(self.cur.0, Mite::Head) {
            head_parse(hoon_wide.clone(),
                        sail_wide.clone(),
                        linemap.clone()).parse(&yex.as_str())
        } else {
            para_parse(hoon_wide.clone(),
                        sail_wide.clone(),
                            linemap.clone()).parse(&yex.as_str())
        };

        match res.into_result() {
            Err(err) => {
                self.err = Some((par_loc.0, par_loc.1));
                return self;
            }
            Ok(res) => {
                self.cur.1 = weld(res, self.cur.1);
                self.par = None;

                if matches!(self.cur.0, Mite::Head) {
                    self = self.close_item();
                }
                self
            }
        }
    }

    fn back(mut self, luc: u64) -> Self {
        let inr = self.ind.1;
        if luc >= inr {
            return self;
        }

        let nex = self.cur_indent();
        if nex > (inr - luc) {
            self.err = Some((self.loc.0, luc));
            return self;
        }

        self = self.close_item();
        self.ind.1 = inr - nex;
        self
    }

    fn cur_indent(&self) -> u64 {
        match self.cur.0 {
            Mite::Down => 2,
            Mite::Head => 0,
            Mite::Lunt => 0,
            Mite::Lime => 2,
            Mite::Lord => 0,
            Mite::Poem => 8,
            Mite::Bloc => 2,
        }
    }

    fn cur_to_tarp(&self) -> Marl {
        match self.cur.0 {
            Mite::Down | Mite::Head => {
                self.cur.1.iter().rev().cloned().collect()
            }
            _ => {
                let tag_name = match self.cur.0 {
                    Mite::Lunt => "ul",
                    Mite::Lord => "ol",
                    Mite::Lime => "li",
                    Mite::Poem => "div",
                    Mite::Bloc => "blockquote",
                    _ => unreachable!(),
                };
                vec![Tuna::Manx(Manx {
                    g: Marx {
                        n: Mane::Tag(tag_name.to_string()),
                        a: vec![],
                    },
                    c: self.cur.1.iter().rev().cloned().collect(),
                })]
            }
        }
    }

    fn close_item(mut self) -> Self {
        if self.hac.is_empty() {
            return self;
        }
        let top = self.hac.pop().unwrap();
        let merged_content = weld(self.cur_to_tarp(), top.1);
        self.cur = (top.0, merged_content);
        self
    }

    fn read_line(&self,
    ) -> (Tape, Option<Option<Hair>>, Hair, Tape) {  // [lin *(unit _err) loc txt]
        let mut lin = String::new();
        let mut txt: &str = &self.txt;
        let mut loc = self.loc;

        //  read until '\n' and check identation
        loop {
            //  if txt is empty returns error
            if txt.is_empty() {
                return ("".to_string(),
                        Some(Some(loc)),
                        loc,
                        txt.to_string());
            }

            let ch = txt.chars().next().unwrap();

            if ch != '\n' {
                if self.ind.1 > loc.1 {
                    if ch != ' ' {
                        //  identation mismatch
                        //  returns error
                        return ("".to_string(),
                                Some(Some(loc)),
                                loc,
                                txt.to_string());
                    }
                    // empty space continue
                    txt = &txt[1..];
                    loc.1 += 1;
                    continue;
                }

                // push char an continue
                txt = &txt[1..];
                lin.push(ch);
                loc.1 += 1;
                continue;
            }

            break;
        }

        lin = lin.trim_end().to_string();

        let eat_newline_loc = (loc.0 + 1, 1u64);
        let eat_newline_txt = (&txt[1..]).to_string();

        //  look at the rest of the string
        let saw = {
            let mut tmp = self.clone();
            tmp.loc = eat_newline_loc;
            tmp.txt = eat_newline_txt.clone();
            tmp.look()
        };

        //  check if the end (==) was found after the line
        if let Some(Trig {
            sty: TrigStyle::End(end_type),
            ..
        }) = saw {
            //  End was found, return Some to stop the recursion
            if matches!(end_type, EndType::Stet | EndType::Dent) {
                return (lin, Some(None), loc, txt.to_string());
            }
        }
        (lin, None, eat_newline_loc, eat_newline_txt.to_string())
    }

    fn open_item(mut self, saw: TrigNew) -> Self {
        match saw {
            TrigNew::Poem => self.open_item_push(Mite::Poem),
            TrigNew::Head => self.open_item_push(Mite::Head),
            TrigNew::Bloc => self.open_item_entr(Mite::Bloc),
            TrigNew::Lint => self.open_item_lent(Mite::Lunt),
            TrigNew::Lite => self.open_item_lent(Mite::Lord),
        }
    }

    fn open_item_push(mut self, mite: Mite) -> Self {
        self.hac.push(self.cur);
        self.cur = (mite, Vec::new());  
        self
    }

    fn open_item_entr(mut self, typ: Mite) -> Self {
        self.ind.1 += 2;

        let skip_chars = (self.ind.1 - self.loc.1) as usize;
        if skip_chars <= self.txt.len() {
            self.txt = self.txt[skip_chars..].to_string();
        } else {
            self.txt = String::new();
        }
        self.loc.1 = self.ind.1;

        self.open_item_push(typ)
    }

    fn open_item_lent(mut self, ord: Mite) -> Self {
        if self.cur.0 != ord {
            self = self.open_item_push(ord);
        }
        self.open_item_entr(Mite::Lime)
    }
}

fn look_parse<'src>(
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Option<Trig>, Err<'src>> {
    just(' ')
    .repeated()
    .map_with(move |_, extra| {
        let span: SimpleSpan = extra.span();
        let (_, col) = linemap.line_col(span.start);
        col as usize
    })
    .then(choice((
        newline().to(None),
        end().to(Some(TrigStyle::End(EndType::Done))),
        just("==").to(Some(TrigStyle::End(EndType::Stet))),
        just("---").to(Some(TrigStyle::One(OneType::Rule))),
        just("```").to(Some(TrigStyle::One(OneType::Fens))),
        just(";").to(Some(TrigStyle::One(OneType::Expr))),
        just('#')
            .repeated()
            .then_ignore(just(' '))
            .to(Some(TrigStyle::New(TrigNew::Head))),
        just("- ").to(Some(TrigStyle::New(TrigNew::Lint))),
        just("+ ").to(Some(TrigStyle::New(TrigNew::Lite))),
        just("> ").to(Some(TrigStyle::New(TrigNew::Bloc))),
        empty().to(Some(TrigStyle::OldText)),
    )))
    .map(|(col, style_opt)| style_opt.map(|sty| Trig { col: col as u64, sty: sty }))
}

fn calf_tic_parse<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    let bas = just('\\');
    let tic = just('`');
    choice((
        bas.ignore_then(tic).to('`'),
        tic.not().ignore_then(non_control_char()),
    ))
    .repeated()
    .collect::<String>()
}

fn cash_parse<'src>(
    tem: char,
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    let bas = just('\\');

    choice((
        whit().to(' '),
        bas.ignore_then(just(tem).clone()),
        just(tem).not().ignore_then(non_control_char()),
    ))
    .repeated()
    .at_least(1)
    .collect::<String>()
}

fn hrul_parse<'src>(
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    just(' ').repeated()
    .ignore_then(just("---"))
    .ignore_then(just('-').repeated())
    .ignore_then(newline())
    .ignored()
    .to(vec![Tuna::Manx(Manx {
        g: Marx { n: Mane::Tag("hr".to_string()), a: vec![] },
        c: vec![],
    })])
}

fn tics_parse<'src>()
-> impl Parser<'src, &'src str, (), Err<'src>>
{
    just('`')
        .then_ignore(just('`'))
        .then_ignore(just('`'))
        .then_ignore(newline())
        .ignored()
}

fn fens_parse<'src>(
    col: u64
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {

    let ace = just(' ');
    let prn = non_control_char();

    let indent_size = (col - 1) as usize;
    let ind = ace
        .repeated()
        .exactly(indent_size);

    let ind_tics = ind.clone().ignore_then(tics_parse());

    let content = choice((
        ind.clone()
            .ignore_then(newline())
            .ignore_then(
                tics_parse().not()
                    .ignore_then(prn.repeated().collect::<String>())
            ),
        ace.repeated()
            .ignore_then(newline())
            .to(String::new()),
    ))
    .repeated()
    .collect::<Vec<String>>();

    let opening = ace.repeated().ignore_then(tics_parse());
    let closing = ind_tics;

    opening
        .ignore_then(content)
        .then_ignore(closing)
        .map(|lines: Vec<String>| {
            let full_text = lines.join("\n");
            vec![Tuna::Manx(Manx {
                g: Marx {
                    n: Mane::Tag("pre".to_string()),
                    a: vec![],
                },
                c: vec![micfas(
                    full_text
                        .bytes()
                        .map(|b| ParsedAtom::Small(b as u128))
                        .collect()
                )],
            })]
        })
}

fn expr_parse<'src>(
    hoon: impl ParserExt<'src, Hoon> + Clone,
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {

    let (sail_tall, _sail_wide) = setup_sail_parsers(hoon, hoon_wide, linemap);

    just(';')
    .ignore_then(sail_tall.clone())
    .delimited_by(just(' ').repeated(), gap().rewind())
    .map(|t: Either<Manx, Marl>| {
        match t {
            Left(m) => drop_top(Left(Tuna::Manx(m))),
            Right(m) => drop_top(Right(m)),
        }
    })
}

pub fn whit<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>>
{
    just(' ')
    .or(just('\n'))
    .repeated()
    .at_least(1)
    .to(' '.to_string())
    .labelled("Whitespaces or Newlines")
}

fn para_parse<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {

    whit().or_not()
    .ignore_then(
    down_parse(hoon_wide, sail_wide, linemap)
        .map(|kids| {
            if kids.is_empty() {
                vec![]
            } else {
                vec![Tuna::Manx(Manx {
                    g: Marx {
                        n: Mane::Tag("p".to_string()),
                        a: vec![],
                    },
                    c: kids,
                })]
            }
        })).boxed()
}

fn werk_parse<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Vec<Graf>, Err<'src>> {
    recursive(|werk| {
        word_parse(hoon_wide, sail_wide.clone(), werk, linemap)
        .repeated()
        .collect::<Vec<Vec<Graf>>>()
        .map(|v| {
            v.into_iter().flatten().collect()
        })
        .boxed()
    })
}

fn reparse<'src, A, B>(
    a: impl Parser<'src, &'src str, A, Err<'src>>,
    b: impl Parser<'src, &'src str, B, Err<'src>> + Clone,
) -> impl Parser<'src, &'src str, B, Err<'src>> {
    a
    .to_slice()
    .try_map(move |slice: &'src str, span: SimpleSpan| {
        match b.clone().then_ignore(end()).parse(slice).into_result() {
            Ok(result) => Ok(result),
            Err(mut errs) => {
                let err = errs
                    .drain(..)
                    .next()
                    .unwrap()
                    .with_span(span.clone());
                let err = Rich::custom(span.clone(), err.reason().clone());
                Err(err)
            }
        }
    })
}

fn word_parse<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    werk:      impl ParserExt<'src, Vec<Graf>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Vec<Graf>, Err<'src>> {

    let ordinary = any().filter(|c: &char| c.is_ascii_alphabetic())
                    .then(any()
                            .filter(|c: &char| c.is_ascii_alphanumeric() || *c == '-')
                            .repeated()
                            .collect::<String>())
                        .map(|(first, rest)| {
                            let mut s = String::with_capacity(1 + rest.len());
                            s.push(first);
                            s.push_str(&rest);
                            vec![Graf::Text(s)]
                        });

    let escape = just("\\")
                    .ignore_then(
                        just(' ').not()
                        .ignore_then(non_control_char().map(|c| c.to_string()))
                    )
                    .map(|t| vec![Graf::Text(t)]);

    let br = just("\\").ignore_then(newline()).to({
        let br_node = Tuna::Manx(Manx {
            g: Marx { n: Mane::Tag("br".to_string()), a: vec![] },
            c: vec![],
        });
        vec![Graf::Expr(br_node)]
    });

    let bold = reparse(cash_parse('*'), werk.clone())
        .delimited_by(just('*'), just('*'))
        .map(|b| vec![Graf::Bold(b)]);

    let italic = reparse(cash_parse('_'), werk.clone()).boxed()
        .delimited_by(just('_'), just('_'))
        .map(|i| vec![Graf::Talc(i)]);

    let quoted = reparse(cash_parse('\"'), werk.clone()).boxed()
        .delimited_by(just("\""), just("\""))
        .map(|q| vec![Graf::Quod(q)]);

    let code_tick =
        calf_tic_parse()
        .delimited_by(just("`"), just("`"))
        .map(|c| vec![Graf::Code(c)]);

    let arm =
        just('+')
        .then(choice((just('+'), just('$'), just('*'))))
        .then(any().filter(|c: &char| c.is_ascii_lowercase()))
        .then(any().filter(|c: &char| c.is_ascii_alphanumeric()
                                    || *c == '-'
                                    || *c == ':')
                            .repeated()
                            .collect::<String>())
                .map(|(((a, b), c), rest)| {
                    let mut s = String::with_capacity(3 + rest.len());
                    s.push(a);
                    s.push(b);
                    s.push(c);
                    s.push_str(&rest);
                    vec![Graf::Code(s)]
                });

    let link = just('{')
        .ignore_then(reparse(cash_parse('}'), werk.clone()))
        .then_ignore(just('}'))
        .then_ignore(whit().or_not())
        .then(just('(')
                .ignore_then(cash_parse(')'))
                .then_ignore(just(')')))
        .map(|(text, url)| vec![Graf::Link(text, url)]);

    let mage = just('!')
        .ignore_then(
            just('{').ignore_then(cash_parse('}')).then_ignore(just('}'))
        )
        .then_ignore(whit().or_not())
        .then(
            just('(').ignore_then(cash_parse(')')).then_ignore(just(')'))
        )
        .map(|(alt, url)| vec![Graf::Mage(alt, url)]);

    let interpolated = just('{')
        .ignore_then(inline_embed(hoon_wide.clone(), sail_wide.clone(), linemap.clone()))
        .then_ignore(just('}'))
        .map(|e| vec![Graf::Expr(e)]);

    let lin = linemap.clone();
    let hoon_list = choice((whit(),
                            empty()
                                .try_map(move |_, span: SimpleSpan| {
                                    let (_line, col) = lin.line_col(span.start);
                                    if col == 1 {
                                        return Ok(String::new());
                                    }
                                    return Err(Rich::custom(span, "col is not 1"));
                                })
                            ))
                    .map(Graf::Text)
                    .then(just('#')
                            .ignore_then(hoon_wide.clone())
                            .to_slice()
                            .map(|s| Graf::Code(s.to_string()))
                        )
                    .then_ignore(whit().rewind())
                    .map(|(p, q)| {
                        vec![p, q]
                });

    let lin2 = linemap.clone();
    let hoon_constant_list =
                    choice((whit(),
                            empty()
                                .try_map(move |_, span: SimpleSpan| {
                                    let (_line, col) = lin2.line_col(span.start);
                                    if col == 1 {
                                        return Ok(String::new());
                                    }
                                    return Err(Rich::custom(span, "col is not 1"));
                                })
                            ))
                    .map(Graf::Text)
                    .then(
                        choice((
                            number().to_slice(),
                            just('.').ignore_then(perd()).to_slice(),
                            just('~').ignore_then(
                                choice((
                                    twid().to_slice(),
                                    empty().to(Coin::Dime("n".to_string(), ParsedAtom::Small(0))).to_slice(),
                                ))),
                            just('%').ignore_then(constant(linemap.clone())).to_slice(),
                        ))
                        .map(|s| Graf::Code(s.to_string()))
                    )
                    .then_ignore(whit().rewind())
                    .map(|(p, q)| {
                        vec![p, q]
                    });

    let whitespace = whit().map(|c| vec![Graf::Text(c)]);

    let byte = just(' ').not()
                .ignore_then(non_control_char().map(|c| c.to_string()))
                .map(|c| vec![Graf::Text(c)]);

    choice((
        ordinary,
        escape,
        br,
        bold,
        italic,
        quoted,
        code_tick,
        arm,
        link,
        mage,
        hoon_list,
        hoon_constant_list,
        whitespace,
        interpolated,
        byte,
    ))
}

fn head_parse<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {

    just(' ').repeated()
    .ignore_then(
        just('#')
        .repeated()
        .at_least(1)
        .at_most(6)
        .count()
    )
    .then(whit()
            .ignore_then(down_parse(hoon_wide, sail_wide, linemap)))
    .map(|(hashes, kids)| {
        let tag = format!("h{}", hashes);
        let id = contents_to_id(&kids);
        vec![Tuna::Manx(Manx {
            g: Marx {
                n: Mane::Tag(tag),
                a: vec![(Mane::Tag("id".to_string()), vec![Beer::Char(string_to_atom(id))])],
            },
            c: kids,
        })]
    })
}

fn down_parse<'src>(
    hoon_wide: impl ParserExt<'src, Hoon> + Clone,
    sail_wide: impl ParserExt<'src, Either<Manx, Marl>> + Clone,
    linemap: Arc<LineMap>,
) -> impl Parser<'src, &'src str, Marl, Err<'src>> {
    werk_parse(hoon_wide, sail_wide.clone(), linemap)
        .map(|grafs|{
            graf_to_tuna(grafs)
        })
}