use crate::ast::hoon::*;
use crate::utils::*;
use std::collections::*;
use std::sync::Arc;
use chumsky::{
    input::{Stream, ValueInput},
    prelude::*,
};

pub fn parse_imports<'src>(
) -> impl Parser<'src, &'src str, Pile, Err<'src>>
{
    fashep_list().then_ignore(gap()).or_not()
    .then(faslus_list().then_ignore(gap()).or_not())
    .then(fastis_list().then_ignore(gap()).or_not())
    .then(fastar_list().then_ignore(gap()).or_not())
    .then(fashax_list().then_ignore(gap()).or_not())
    .map(|((((hep, lus), tis), tar), hax)| Pile {
        sur: hep.unwrap_or_default(),
        lib: lus.unwrap_or_default(),
        raw: tis.unwrap_or_default(),
        bar: tar.unwrap_or_default(),
        hax: hax.unwrap_or_default(),
        hoon: Hoon::ZapZap
    })
    .boxed()
}

// parses multiple /= imports
pub fn fastis_list<'src>(
) -> impl Parser<'src, &'src str, Vec<(Option<String>, Path)>, Err<'src>>
{
    just("/=")
    .ignore_then(gap())
    .ignore_then(choice((just('*').to(None),
                        symbol().map(|s| Some(s))
                        ))
                )
    .then_ignore(gap())
    .then(stap())
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<(Option<String>, Path)>>()
}

//
// parses multiple /* imports
//
pub fn fastar_list<'src>(
) -> impl Parser<'src, &'src str, Vec<(Term, Term, Path)>, Err<'src>>
{
    just("/*")
    .ignore_then(gap())
    .ignore_then(symbol())
    .then_ignore(gap())
    .then(just('%').ignore_then(symbol()))
    .then_ignore(gap())
    .then(stap())
    .map(|((a, b), path)| (a, b, path))
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<(String, String, Path)>>()
}

// parses multiple /#  imports
pub fn fashax_list<'src>(
) -> impl Parser<'src, &'src str, Vec<Taut>, Err<'src>>
{
    just("/#")
    .ignore_then(gap())
    .ignore_then(taut_rule()
                 .separated_by(just(',').ignore_then(gaw()))
                 .at_least(1)
                 .collect::<Vec<(Option<Term>, Term)>>())
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<Vec<(Option<Term>, Term)>>>()
    .map(|v: Vec<Vec<(Option<Term>, Term)>>| {
                v.into_iter().flatten().collect::<Vec<(Option<Term>, Term)>>()
                  })
}

// parses multiple /+  imports
pub fn faslus_list<'src>(
) -> impl Parser<'src, &'src str, Vec<Taut>, Err<'src>>
{
    just("/+")
    .ignore_then(gap())
    .ignore_then(taut_rule()
                 .separated_by(just(',').ignore_then(gaw()))
                 .at_least(1)
                 .collect::<Vec<(Option<Term>, Term)>>())
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<Vec<(Option<Term>, Term)>>>()
    .map(|v: Vec<Vec<(Option<Term>, Term)>>| {
        v.into_iter().flatten().collect::<Vec<(Option<Term>, Term)>>()
    })
}

// parses multiple /-  imports
pub fn fashep_list<'src>(
) -> impl Parser<'src, &'src str, Vec<Taut>, Err<'src>>
{
    just("/-")
    .ignore_then(gap())
    .ignore_then(taut_rule()
                 .separated_by(just(',').ignore_then(gaw()))
                 .at_least(1)
                 .collect::<Vec<(Option<Term>, Term)>>())
    .separated_by(gap())
    .at_least(1)
    .collect::<Vec<Vec<(Option<Term>, Term)>>>()
    .map(|v: Vec<Vec<(Option<Term>, Term)>>| {
            v.into_iter()
            .flatten()
            .collect::<Vec<(Option<Term>, Term)>>()
    })
}

//  $taut: file import from /lib or /sur
//  +$  taut  [face=(unit term) pax=term]
//
pub fn taut_rule<'src>(
) -> impl Parser<'src, &'src str, Taut, Err<'src>>
{
    choice((
        just('*').ignore_then(symbol()).map(|s| (None, s)),
        symbol().map(|s| Some(s))
            .then(just('=')
                    .ignore_then(symbol())),
        symbol().map(|s| (Some(s.clone()), s))
    )).labelled("Taut Rule")
}