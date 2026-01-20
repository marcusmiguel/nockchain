use parser::ast::hoon::*;
use num_bigint::BigUint;
use chumsky::prelude::*;
use parser::noun::*;
use parser::utils::*;
use ibig::UBig;
use nockvm::noun::{YES, NO, Atom, Noun};
use std::cmp;
use std::ops::BitAnd;
use num_traits::{One, Num, FromPrimitive, ToPrimitive};
use num_traits::identities::Zero;
use sha2::{Sha256, Digest};

//  This file contains functions and parsers
//  used to parse all the different Hoon Auras.
//

// @ud
pub fn decimal_number<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    let digit = any()
                    .filter(|c: &char| c.is_ascii_digit());

    let non_zero_digit = any()
                            .filter(|c: &char| matches!(c, '1'..='9'));

    let first =
        just('0').to("0".to_string())
            .or(
                non_zero_digit
                    .then(
                        digit
                            .repeated()
                            .at_most(2)
                            .collect::<Vec<char>>()
                    )
                    .map(|(h, t)| {
                        let mut s = String::with_capacity(3);
                        s.push(h);
                        s.extend(t);
                        s
                    }),
            );

    let three_digits =
        digit
            .repeated()
            .exactly(3)
            .collect::<String>();

    let rest =
        just('.')
            .ignore_then(gap().or_not())
            .ignore_then(three_digits)
            .repeated()
            .collect::<Vec<String>>();

    first
        .then(rest)
        .map(|(first_digits, rest_digits)| {
            let mut out = first_digits;
            for chunk in rest_digits {
                out.push_str(&chunk);
            }
            out
        })
        .labelled("Decimal Number")
}

// @ub
pub fn binary_number<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    let bit = any().filter(|c: &char| *c == '0' || *c == '1');

    let first_group =
        just('0').to("0".to_string())
            .or(
                just('1')
                    .then(bit.repeated().at_most(3).collect::<String>())
                    .map(|(h, t)| h.to_string() + &t)
            );

    let first = just("0b").ignore_then(first_group);

    let rest = just('.')
        .ignore_then(gap().or_not())
        .ignore_then(
            bit.repeated()
                .exactly(4)
                .collect::<String>(),
        );

    first
        .then(rest.repeated().collect::<Vec<String>>())
        .map(|(first, rest)| {
            if rest.is_empty() {
                first
            } else {
                let mut s = first;
                for r in rest {
                    s.push_str(&r);
                }
                s
            }
        })
        .labelled("Binary")
}

// @ux
pub fn hexadecimal_number<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    let hex = any().filter(|c: &char| c.is_ascii_hexdigit());

    let first_group = hex
        .then(hex.repeated().at_most(3).collect::<String>())
        .map(|(head, tail)| {
            if head == '0' && !tail.is_empty() {
                String::new()
            } else {
                let mut s = String::new();
                s.push(head);
                s.push_str(&tail);
                s
            }
        })
        .filter(|s| !s.is_empty());

    let first = just("0x").ignore_then(first_group);

    let rest = just('.')
        .ignore_then(gap().or_not())
        .ignore_then(
            hex.repeated()
                .exactly(4)
                .collect::<String>(),
        )
        .repeated()
        .collect::<Vec<String>>();

    first
        .then(rest)
        .map(|(first, rest)| {
            if rest.is_empty() {
                first
            } else {
                let mut s = first;
                for r in rest {
                    s.push_str(&r);
                }
                s
            }
        })
        .labelled("Hexadecimal")
}

// @if
pub fn ipv4_address<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    let octet = any().filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .at_most(3)
        .collect::<String>()
        .filter(|s: &String| {
            if s.is_empty() || s.starts_with('0') && s.len() > 1 { return false; }
            let n = s.parse::<u16>().unwrap_or(256);
            n <= 255
        });

    octet
        .separated_by(just('.').ignore_then(gap().or_not()))
        .exactly(4)
        .collect::<Vec<String>>()
        .map(|parts| parts.join("."))
        .labelled("IPv4-Address")
}

// @is
pub fn ipv6_address<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>>
{
    let rest = just('.').ignore_then(gap().or_not())
            .ignore_then(alphanumeric())
            .repeated()
            .exactly(7)
            .collect::<Vec<_>>();

    alphanumeric().then(rest)
        .map(|(first, mut rest)| {
            if rest.is_empty() {
                first.to_string()
            } else {
                let mut parts = vec![first];
                parts.append(&mut rest);
                parts.join(":").to_string()
            }
        })
        .labelled("Ipv6-Address")
}

//  @uv
pub fn base32_number<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>>
{
    let base32_digit =
        any().filter(|c: &char| c.is_ascii_digit() || ('a'..='v').contains(c));

    let first = just("0v")
                .ignore_then(
                    choice((just('0').to("0".to_string()),
                            any().filter(|c: &char| matches!(c, '1'..='9' | 'a'..='v'))
                                .then(base32_digit.repeated().at_most(4).collect::<String>())
                                .map(|(h, t)| h.to_string() + &t)
                    ))
                );

    let rest = just('.')
                .ignore_then(gap().or_not())
                .ignore_then(
                        base32_digit
                        .repeated()
                        .exactly(5)
                        .collect::<String>()
                    )
                .repeated()
                .collect::<Vec<String>>();

    first.then(rest)
        .map(|(first, mut rest)| {
            if rest.is_empty() {
                base32_to_atom(first.to_string())
            } else {
                let mut parts = vec![first];
                parts.append(&mut rest);
                base32_to_atom(parts.join(""))
            }
        })
        .labelled("Base32")
}

// @uw
pub fn base64_number<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>> {
    let digit = any().filter(|c: &char| matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '~'));

    let first = just("0w").ignore_then(
        just('0').to("0".to_string())
            .or(
                any().filter(|c: &char| matches!(c, '1'..='9' | 'a'..='z' | 'A'..='Z' | '-' | '~'))
                    .then(digit.repeated().at_most(4).collect::<String>())
                    .map(|(h, t)| h.to_string() + &t)
            )
    );

    let group = just('.')
        .ignore_then(gap().or_not())
        .ignore_then(digit.repeated().exactly(5).collect::<String>());

    first
        .then(group.repeated().collect::<Vec<String>>())
        .map(|(first, rest)| {
            if rest.is_empty() {
                base64_to_atom(first)
            } else {
                let mut parts = vec![first];
                parts.extend(rest);
                base64_to_atom(parts.join(""))
            }
        })
        .labelled("Base64")
}

// @da
pub fn absolute_date<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>>
{
    let era_year = decimal_without_leading_zero()
        .then(
            just('-')
                .to(false)
                .or_not()
                .map(|opt| opt.unwrap_or(true)),
        )
        .try_map(|(year_str, era), span| {
            let year: u64 = year_str.parse()
                .map_err(|_| Rich::custom(span, "invalid year number"))?;

            if year == 0 {
                return Err(Rich::custom(span, "year must be ≥ 1"));
            }

            Ok((era, year))
        });
        let month = just('.')
            .ignore_then(digits())
            .try_map(|s: String, span| {
                let m: u64 = s.parse().map_err(|_| Rich::custom(span, "invalid month"))?;
                if (1..=12).contains(&m) {
                    Ok(m)
                } else {
                    Err(Rich::custom(span, "month out of range (1–12)"))
                }
            });
        let day = just('.')
            .ignore_then(digits())
            .try_map(|s, span| {
                let d: u64 = s.parse().map_err(|_| Rich::custom(span, "invalid day"))?;
                if (1..=31).contains(&d) {
                    Ok(d)
                } else {
                    Err(Rich::custom(span, "day out of range (1–31)"))
                }
            });
    let hour_min_secs_fractions =
        just("..")
            .ignore_then(
                digits()
                    .try_map(|s, span| {
                        let h: u64 = s
                                        .parse::<u64>()
                                        .map_err(|_| Rich::custom(span, "invalid hour"))?;
                        if h < 24 { Ok(h) } else { Err(Rich::custom(span, "hour out of range (0–23)")) }
                    })
                    .then_ignore(just("."))
                    .then(
                        digits()
                        .try_map(|s, span| {
                            let m: u64 = s
                                .parse::<u64>()
                                .map_err(|_| Rich::custom(span, "invalid minute"))?;
                            if m < 60 {
                                Ok(m)
                            } else {
                                Err(Rich::custom(span, "minute out of range (0–59)"))
                            }
                        }))
                    .then_ignore(just("."))
                    .then(digits()
                          .try_map(|s, span| {
                                let s: u64 = s
                                    .parse::<u64>()
                                    .map_err(|_| Rich::custom(span, "invalid second"))?;
                                if s < 60 {
                                    Ok(s)
                                } else {
                                    Err(Rich::custom(span, "second out of range (0–59)"))
                                }
                            })))
            .then(
                just("..")
                    .ignore_then(
                        alphanumeric()
                            .separated_by(just("."))
                            .at_least(1)
                            .collect::<Vec<String>>(),
                    )
                    .or_not()
                    .map(|opt| opt.unwrap_or_default()),
            )
            .try_map(|(((h, m), s), frags), span| {
                let mut fractions = Vec::new();

                for f in frags {
                        let val = u16::from_str_radix(&f, 16)
                            .map_err(|_| Rich::custom(span, "invalid fraction digits"))?;
                        fractions.push(val);
                    }

                Ok((h, m, s, fractions))
            })
            .or_not()
            .map(|opt| opt.unwrap_or((0, 0, 0, Vec::new())));

    era_year
    .then(month)
    .then(day)
    .then(hour_min_secs_fractions)
    .map(|((((era, y), m), d), (hour, min, sec, f))| {
        ParsedAtom::Small(year(era,
                        y,
                        m,
                        d,
                        hour,
                        min,
                        sec,
                        &f
                    ))
    })
}

// @dr
pub fn relative_date<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>> {
    let time_part = relative_date_pair()
        .separated_by(just('.'))
        .at_least(1)
        .collect::<Vec<(char, u64)>>();

    let hex_part = just("..")
        .ignore_then(
            any().filter(|c: &char| c.is_ascii_hexdigit())
                .repeated()
                .exactly(4)
                .collect::<String>()
                .map(|s| u16::from_str_radix(&s, 16).unwrap_or(0))
                .separated_by(just('.'))
                .at_least(1)
                .collect::<Vec<u16>>()
        )
        .or_not()
        .map(|v| v.unwrap_or_default());

    time_part
        .then(hex_part)
        .map(|(pairs, hex_vec):  (Vec<(char, u64)>, Vec<u16>)| {
            let mut days = 0u64;
            let mut hours = 0u64;
            let mut minutes = 0u64;
            let mut seconds = 0u64;

            for (unit, value) in pairs {
                match unit {
                    'd' => days += value,
                    'h' => hours += value,
                    'm' => minutes += value,
                    's' => seconds += value,
                    _ => {},
                }
            }

            ParsedAtom::Small(yule(days, hours, minutes, seconds, &hex_vec))
        })
}

//  @uc
pub fn bitcoin_address<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    just("0c")
    .ignore_then(alphanumeric())
    .labelled("Bitcoin Address")
}

// @r
pub fn float<'src>(
) -> impl Parser<'src, &'src str, (String, ParsedAtom), Err<'src>>
{
    let floats =
            just('-').or_not()
            .then(decimal_without_leading_zero())
            .then(choice((
                    just('.')
                        .ignore_then(digits())
                        .map(|frac| {
                            (frac.len(),
                            frac.parse::<BigUint>().expect("float: invalid fraction"))
                        }),
                    empty().to((0, BigUint::zero())))))
            .then(choice((
                    just('e')
                        .ignore_then(just('-').or_not())
                        .then(decimal_without_leading_zero())
                        .map(|(maybe_hep, expo)| {
                            (!maybe_hep.is_some(), expo.parse::<u128>().expect("float: invalid exponent"))
                        }),
                    empty().to((true, 0)))))
            .map(|(((maybe_hep, p), (len_mant, mant)), (sign_expo, expo))| {
                let term1 = new_si(sign_expo, expo);
                let term2 = sun_si(len_mant as u128);
                let h = dif_si(term1, term2);
                let po = BigUint::from(10u32).pow(len_mant.try_into().unwrap());
                let integer_part = p.parse::<BigUint>().expect("float: invalid decimal");
                let a = integer_part * po + mant;
                DecimalFloat::Finite { sign: !maybe_hep.is_some(), exp: h, mant: a }
            });

    let inf =  just('-').or_not()   //  -inf or inf
                    .then(just("inf"))
                    .map(|(maybe_hep, inf)| DecimalFloat::Infinity{ sign: !maybe_hep.is_some() })
                    .boxed();

    let nan = just("nan")
                .to(DecimalFloat::NaN)
                .boxed();  //  nan

    let royl_rn
             =  choice((
                  floats,  ///  1.10 or 1e10
                  inf,
                  nan,
                )).boxed();

    let rh = just("~~").ignore_then(royl_rn.clone());
    let rq = just("~~~").ignore_then(royl_rn.clone());
    let rd = just('~').ignore_then(royl_rn.clone());
    let rs = royl_rn;

    choice((
        rh.map(|dn| ("rh".to_string(), rylh(dn))),
        rq.map(|dn| ("rq".to_string(), rylq(dn))),
        rd.map(|dn| ("rd".to_string(), ryld(dn))),
        rs.map(|dn| ("rs".to_string(), ryls(dn))),
    )).labelled("Float")
}

//  String -> ParsedAtom conversion

pub fn string_to_atom(s: String) -> ParsedAtom {
    let vec_u128: Vec<u128> = s.chars().map(|c| c as u128).collect();

    rap(3, &vec_u128)
}

pub fn ta_to_atom(s: String) -> ParsedAtom {
    if s == "~.".to_string() {
        return ParsedAtom::Small(0);
    }
    let vec_u128: Vec<u128> = s.chars().map(|c| c as u128).collect();

    rap(3, &vec_u128)
}

pub fn term_to_atom(s: String) -> ParsedAtom {
    if s == "$".to_string() {
        return ParsedAtom::Small(0);
    }
    let vec_u128: Vec<u128> = s.chars().map(|c| c as u128).collect();

    rap(3, &vec_u128)
}

//  @ud to @
pub fn decimal_to_atom(s: String) -> ParsedAtom {
    ParsedAtom::Small(s.parse::<u128>().expect("decimal_to_atom failed"))
}

//  @ux to @
pub fn hex_to_atom(s: String) -> ParsedAtom {
    let clean = s.strip_prefix("0x").unwrap_or(&s);

    if clean.len() <= 32 {
        if let Ok(n) = u128::from_str_radix(clean, 16) {
            return ParsedAtom::Small(n);
        }
    }

    let big = BigUint::parse_bytes(clean.as_bytes(), 16)
        .expect("invalid hex in big atom");

    ParsedAtom::Big(big)
}

//  @ub to @
pub fn binary_to_atom(s: String) -> ParsedAtom {
    ParsedAtom::Small(u128::from_str_radix(&s, 2).expect("binary_to_atom failed"))
}

//  @t to @
pub fn cord_chars_to_atom(chars: Vec<char>) -> ParsedAtom {

    let mut atom = BigUint::zero();
    let mut power = BigUint::from(1u32);
    let base = BigUint::from(256u32);

    for &c in &chars {
        let byte = BigUint::from(c as u32 & 0xFF);
        atom += &byte * &power;
        power *= &base;
    }

    ParsedAtom::Big(atom)

}

const ALPH64: &str =
    "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ-~";

//  @uw to @
pub fn base64_to_atom(s: String) -> ParsedAtom {
    let mut n: u128 = 0;

    for ch in s.chars() {
        let v = match ALPH64.find(ch) {
            Some(i) => i as u128,
            None => panic!("invalid digit '{ch}' in base64"),
        };

        n = n
            .checked_mul(64)
            .expect("value exceeds u128 range (mul)");

        n = n
            .checked_add(v)
            .expect("value exceeds u128 range (add)");
    }

    ParsedAtom::Small(n)
}

const ALPH32: &str = "0123456789abcdefghijklmnopqrstuv";

//  @uv to @
pub fn base32_to_atom(s: String) -> ParsedAtom {
    let mut n: u128 = 0;

    for ch in s.chars() {
        let v = match ALPH32.find(ch) {
            Some(i) => i as u128,
            None => panic!("invalid digit '{ch}' in base32"),
        };

        n = n
            .checked_mul(32)
            .expect("value exceeds u128 range (mul)");

        n = n
            .checked_add(v)
            .expect("value exceeds u128 range (add)");
    }

    ParsedAtom::Small(n)
}

// +fim
pub fn base58_to_atom(s: String) -> Option<ParsedAtom> {
    let yek = build_yek();

    let digits: Vec<u8> = s.chars()
        .map(|ch| cha_fa(&yek, ch))
        .collect::<Option<_>>()?;

    let a = ParsedAtom::Big(bass_58(&digits));
    den_fa(&a)
}

pub fn ipv4_to_atom(s: String) ->  Option<ParsedAtom>{
    let addr = s
        .parse::<std::net::Ipv4Addr>().ok()?;

    let ip_num = u32::from_be_bytes(addr.octets());

    Some(ParsedAtom::Small(ip_num.into()))
}

pub fn ipv6_to_atom(s: String) -> Option<ParsedAtom> {
    let addr = s.parse::<std::net::Ipv6Addr>().ok()?;
    let num = u128::from_be_bytes(addr.octets());
    Some(ParsedAtom::Small(num))
}

//
//  Atom manipulation
//

fn bloq_bits(bloq: u32) -> u32 {
    if bloq >= 7 {
        panic!("bloq must be < 7 (max 64-bit chunks for u128)");
    }
    1 << bloq
}

pub fn met(bloq: usize, atom: &ParsedAtom) -> usize {
    let bits_per_block: usize = 1usize << bloq;

    match atom {
        ParsedAtom::Small(n) => {
            if *n == 0 {
                1
            } else {
                let atom_bits: usize = 128 - n.leading_zeros() as usize;
                (atom_bits + bits_per_block - 1) / bits_per_block
            }
        }
        ParsedAtom::Big(b) => {
            if b.is_zero() {
                1
            } else {
                let atom_bits: usize = b.bits() as usize;
                (atom_bits + bits_per_block - 1) / bits_per_block
            }
        }
    }
}

/// rep: assemble list of ParsedAtoms into one ParsedAtom using bite spec
///
/// - `bloq`: block size exponent
/// - `step_opt`: number of bloqs to take from each atom; if `None`, defaults to 1 (per Hoon ?^(a a [a *step]))
/// - `list`: slice of ParsedAtoms (representing Hoon `(list @)`)
///
/// Semantics:
///   result = Σ_i ( (atom_i & mask) << (i * chunk_bits) )
///   where mask = (1 << chunk_bits) - 1
pub fn rep(bloq: usize, step_opt: Option<usize>, list: &[ParsedAtom]) -> ParsedAtom {
    let step = step_opt.unwrap_or(1); // default step = 1

    let bloq_size = 1usize << bloq;        // 2^bloq
    let chunk_bits = step * bloq_size;     // bits per item

    if list.is_empty() || chunk_bits == 0 {
        return ParsedAtom::Small(0);
    }

    let mut result = BigUint::from(0u32);

    for (i, atom) in list.iter().enumerate() {
        let atom_bu = atom.to_biguint();

        let truncated = if chunk_bits < 128 {
            let mask = (1u128 << chunk_bits) - 1;
            let mask_bu = BigUint::from(mask);
            atom_bu & mask_bu
        } else {
            if atom_bu.bits() as usize <= chunk_bits {
                atom_bu
            } else {
                let mask = ( BigUint::from(1u32) << chunk_bits) - 1u8;
                &atom_bu & mask
            }
        };

        let shifted = if i == 0 {
            truncated
        } else {
            truncated << (i * chunk_bits)
        };

        result += shifted;
    }

    ParsedAtom::Big(result)
}

pub fn rap(bloq: usize, chunks: &[u128]) -> ParsedAtom {
    if chunks.is_empty() {
        return ParsedAtom::Small(0);
    }

    let bits_per_bloq = bloq_bits(bloq as u32) as u64;
    let mut result = BigUint::zero();
    let mut shift = 0u64;

    for &chunk in chunks {
        let width_bloqs = met(bloq, &ParsedAtom::Small(chunk)) as u64;
        let width_bits = width_bloqs * bits_per_bloq;

        let mask = if width_bits >= 128 {
            u128::MAX
        } else {
            (1u128 << width_bits) - 1
        };
        if chunk & !mask != 0 {
            panic!("atom {:#x} too large for bloq {}", chunk, bloq);
        }

        let chunk_big = BigUint::from(chunk);
        result |= chunk_big << shift;

        shift += width_bits;

        if shift > 128 {
        }
    }

    if shift <= 128 {
        let value = result.to_u128().expect("logic error: shift <=128 but not u128");
        ParsedAtom::Small(value)
    } else {
        ParsedAtom::Big(result)
    }
}

pub fn right_child(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        (2 * right_child(n - 1)) + 1
    }
}

pub fn left_child(n: u64) -> u64 {
    if n == 0 {
        0
    } else {
        2 * (left_child(n - 1) + 1)
    }
}

pub fn peg(a: u64, b: u64) -> Result<u64, &'static str> {
    if a == 0 || b == 0 {
        return Err("peg: a and b must be non-zero");
    }

    let k = b.ilog2();
    let offset = b & ((1u64 << k) - 1);
    Ok((a << k) + offset)
}

fn cut_u(v: u128, shift: usize, bits: usize) -> u8 {
    ((v >> shift) & ((1 << bits) - 1)) as u8
}

/// Extract `run` bloqs starting at bloq `start`, where each bloq is `2^bloq` bits.
pub fn cut(bloq: usize, start: usize, run: usize, atom: &ParsedAtom) -> ParsedAtom {
    if run == 0 {
        return ParsedAtom::Small(0);
    }

    let bloq_bits = match 1usize.checked_shl(bloq as u32) {
        Some(b) => b,
        None => return ParsedAtom::Small(0),
    };

    let bit_start = match start.checked_mul(bloq_bits) {
        Some(s) => s,
        None => return ParsedAtom::Small(0),
    };

    let bit_len = match run.checked_mul(bloq_bits) {
        Some(l) => l,
        None => return ParsedAtom::Small(0),
    };

    let src_bits = match atom {
        ParsedAtom::Small(0) => 0,
        ParsedAtom::Small(n) => (128 - n.leading_zeros()) as usize,
        ParsedAtom::Big(b) => b.bits() as usize,
    };

    if bit_start >= src_bits {
        return ParsedAtom::Small(0);
    }

    let bit_len = cmp::min(bit_len, src_bits - bit_start);
    if bit_len == 0 {
        return ParsedAtom::Small(0);
    }

    let shifted = match atom {
        ParsedAtom::Small(n) => {
            if bit_start >= 128 {
                ParsedAtom::Small(0)
            } else {
                ParsedAtom::Small(n >> bit_start)
            }
        }
        ParsedAtom::Big(b) => {
            if bit_start == 0 {
                atom.clone()
            } else {
                ParsedAtom::from_biguint(b >> bit_start)
            }
        }
    };

    match &shifted {
        ParsedAtom::Small(n) => {
            if bit_len >= 128 {
                shifted
            } else {
                let mask = (1u128 << bit_len) - 1;
                ParsedAtom::Small(*n & mask)
            }
        }
        ParsedAtom::Big(b) => {
            if bit_len <= 128 {
                let low_u128 = {
                    let mut limbs = b.iter_u64_digits();
                    let lo = limbs.next().unwrap_or(0);
                    let hi = limbs.next().unwrap_or(0);
                    ((hi as u128) << 64) | (lo as u128)
                };
                let mask = if bit_len == 128 {
                    u128::MAX
                } else {
                    (1u128 << bit_len) - 1
                };
                ParsedAtom::Small(low_u128 & mask)
            } else {
                let mask = (BigUint::one() << bit_len) - BigUint::one();
                let masked = b & &mask;
                ParsedAtom::from_biguint(masked)
            }
        }
    }
}

pub fn lsh(bloq: usize, step: usize, atom: &ParsedAtom) -> ParsedAtom {
    let bits = match step.checked_mul(1usize << bloq) {
        Some(b) => b,
        None => return ParsedAtom::Small(0),
    };
    atom_shl(atom, bits)
}

pub fn rsh(bloq: usize, step: usize, atom: &ParsedAtom) -> ParsedAtom {
    let bits = match step.checked_mul(1usize << bloq) {
        Some(b) => b,
        None => return ParsedAtom::Small(0),
    };
    atom_shr(atom, bits)
}

fn lsh_u128(bloq: usize, step: usize, atom: u128) -> u128 {
    let bits = step.checked_mul(1 << bloq).unwrap_or(128);
    if bits >= 128 { 0 } else { atom << bits }
}

fn rsh_u128(bloq: usize, step: usize, atom: u128) -> u128 {
    let bits = step.checked_mul(1 << bloq).unwrap_or(128);
    if bits >= 128 { 0 } else { atom >> bits }
}

fn lsh_big(bloq: usize, step: usize, atom: &BigUint) -> BigUint {
    let bits = step.checked_mul(1 << bloq).unwrap_or(usize::MAX);
    if bits == 0 { atom.clone() } else { atom << bits }
}

fn rsh_big(bloq: usize, step: usize, atom: &BigUint) -> BigUint {
    let bits = step.checked_mul(1 << bloq).unwrap_or(usize::MAX);
    if bits == 0 { atom.clone() } else { atom >> bits }
}

fn end(bloq: usize, step: usize, atom: &ParsedAtom) -> ParsedAtom {
    let total_bits = match step.checked_mul(1usize << bloq) {
        Some(b) => b,
        None => return ParsedAtom::Small(0),
    };
    atom_mask_low_bits(atom, total_bits)
}

fn end_big(bloq: usize, step: usize, atom: &BigUint) -> BigUint {
    let total_bits = match step.checked_mul(1usize << bloq) {
        Some(b) => b as u128,
        None => return BigUint::zero(),
    };
    if total_bits == 0 {
        return BigUint::zero();
    }
    let mask = (BigUint::one() << total_bits) - BigUint::one();
    atom & &mask
}

fn end_u128(bloq: usize, step: usize, atom: u128) -> u128 {
    let total_bits = match step.checked_mul(1usize << bloq) {
        Some(b) => b as u128,
        None => return 0,
    };
    if total_bits >= 128 {
        atom
    } else {
        let mask = (1u128 << total_bits) - 1;
        atom & mask
    }
}

fn atom_shl(a: &ParsedAtom, bits: usize) -> ParsedAtom {
    if bits == 0 { return a.clone(); }
    match a {
        ParsedAtom::Small(n) => {
            if bits >= 128 { ParsedAtom::from_biguint(BigUint::from(*n) << bits) }
            else { ParsedAtom::Small(n << bits) }
        }
        ParsedAtom::Big(b) => ParsedAtom::from_biguint(b << bits),
    }
}

fn atom_shr(atom: &ParsedAtom, bits: usize) -> ParsedAtom {
    if bits == 0 {
        return atom.clone();
    }
    match atom {
        ParsedAtom::Small(n) => {
            if bits >= 128 {
                ParsedAtom::Small(0)
            } else {
                ParsedAtom::Small(n >> bits)
            }
        }
        ParsedAtom::Big(b) => {
            ParsedAtom::from_biguint(b >> bits)
        }
    }
}

fn atom_mask_low_bits(atom: &ParsedAtom, bits: usize) -> ParsedAtom {
    if bits == 0 {
        return ParsedAtom::Small(0);
    }
    match atom {
        ParsedAtom::Small(n) => {
            if bits >= 128 {
                ParsedAtom::Small(*n)
            } else {
                let mask = (1u128 << bits) - 1;
                ParsedAtom::Small(*n & mask)
            }
        }
        ParsedAtom::Big(b) => {
            if bits <= 128 {
                let mask: u128 = (1u128 << bits) - 1;
                let mut limbs = b.iter_u64_digits();
                let lo = limbs.next().unwrap_or(0);
                let hi = limbs.skip(1).next().unwrap_or(0);
                let low_u128 = ((hi as u128) << 64) | (lo as u128);
                ParsedAtom::Small(low_u128 & mask)
            } else {
                let mask = (BigUint::one() << bits) - BigUint::one();
                ParsedAtom::from_biguint(b & &mask)
            }
        }
    }
}

fn dis_big(x: &BigUint, mask: &BigUint) -> BigUint {
    x & mask
}

fn dis<T: Copy + BitAnd<Output = T>>(x: T, mask: T) -> T {
    x & mask
}

fn con(hi: u64, lo: u64) -> u64 {
    hi | lo
}

fn con_atoms(hi: ParsedAtom, lo: ParsedAtom) -> ParsedAtom {
    match (hi, lo) {
        (ParsedAtom::Small(a), ParsedAtom::Small(b)) => ParsedAtom::Small(a | b),
        (a, b) => {
            let x = a.to_biguint();
            let y = b.to_biguint();
            ParsedAtom::from_biguint(x | y)
        }
    }
}

fn mix(x: u64, y: u64) -> u64 {
    x ^ y
}

fn mix_big(x: &BigUint, y: &BigUint) -> BigUint {
    x ^ y
}
pub fn pow(base: u128, exp: u128) -> BigUint {
    if exp == 0 {
        return BigUint::from(1u8);
    }

    let mut result = BigUint::from(1u8);
    let mut base = BigUint::from(base);
    let mut exp = exp;

    while exp > 0 {
        if exp & 1 == 1 {
            result *= &base;
        }
        base *= base.clone();
        exp >>= 1;
    }

    result
}

pub fn fil(a: u32, b: u32, c: u128) -> ParsedAtom {
    if b == 0 {
        return ParsedAtom::Small(0);
    }

    let bloq_bits = 1u32 << a; // 2^a bits per block
    let mask = if bloq_bits >= 128 {
        u128::MAX
    } else {
        (1u128 << bloq_bits) - 1
    };
    let c_masked = c & mask;

    if bloq_bits as u64 * b as u64 <= 128 && c_masked != 0 {
        let mut result = 0u128;
        for i in 0..b {
            let shift = (b - 1 - i) as u32 * bloq_bits;
            if shift >= 128 { break; }
            result |= c_masked << shift;
        }
        ParsedAtom::Small(result)
    } else {
        let c_big = BigUint::from(c_masked);
        let mut result = BigUint::from(0u8);
        for i in 0..b {
            let shift = (b - 1 - i) as usize * bloq_bits as usize;
            result |= &c_big << shift;
        }
        ParsedAtom::Big(result)
    }
}

pub fn atom_less_than(a: Atom, b: Atom) -> Noun {
    if atom_less_than_b(&a, &b) {
        YES
    } else {
        NO
    }
}

fn atom_less_than_b(a: &Atom, b: &Atom) -> bool {
    if let (Some(a_direct), Some(b_direct)) = (a.direct(), b.direct()) {
        return unsafe { a_direct.data() < b_direct.data() };
    }

    let a_bits = a.bit_size();
    let b_bits = b.bit_size();

    if a_bits != b_bits {
        return a_bits < b_bits;
    }

    if a_bits <= 64 && b_bits <= 64 {
        if let (Ok(a_u64), Ok(b_u64)) = (a.as_u64(), b.as_u64()) {
            return a_u64 < b_u64;
        }
    }

    let a_limbs = if a.is_direct() {
        &[unsafe { a.direct().unwrap().data() }]
    } else {
        unsafe { std::slice::from_raw_parts(a.data_pointer(), a.size()) }
    };

    let b_limbs = if b.is_direct() {
        &[unsafe { b.direct().unwrap().data() }]
    } else {
        unsafe { std::slice::from_raw_parts(b.data_pointer(), b.size()) }
    };

    let max_limbs = a_limbs.len().max(b_limbs.len());
    for i in 0..max_limbs {
        let a_idx = a_limbs.len().wrapping_sub(1).wrapping_sub(i);
        let b_idx = b_limbs.len().wrapping_sub(1).wrapping_sub(i);

        let a_limb = if a_idx < a_limbs.len() { a_limbs[a_idx] } else { 0 };
        let b_limb = if b_idx < b_limbs.len() { b_limbs[b_idx] } else { 0 };

        if a_limb != b_limb {
            return a_limb < b_limb;
        }
    }

    false
}


fn sub_or_panic(mut a: u128, b: u128) -> u128 {
    a = a.checked_sub(b).expect("subtraction underflow");
    a
}

fn sub_or_panic_big(a: &BigUint, b: &BigUint) -> BigUint {
    if a < b {
        panic!("subtraction underflow");
    }
    a - b
}

fn dvr_big(a: &BigUint, b: &BigUint) -> (BigUint, BigUint) {
    let quot = a / b;
    let rem  = a % b;
    (quot, rem)
}

pub fn bex(a: u128) -> u128 {
    if a == 0 {
        1
    } else {
        assert!(a < 128, "bex: exponent too large for u128");
        1u128 << a
    }
}

fn biguint_to_ubig(b: &BigUint) -> UBig {
    UBig::from_le_bytes(&b.to_bytes_le())
}

//  Phonetic Parsing  ~foobar
//  @p and @q

pub const SIS: [[u8; 3]; 256] = [
    *b"doz", *b"mar", *b"bin", *b"wan", *b"sam", *b"lit", *b"sig", *b"hid",
    *b"fid", *b"lis", *b"sog", *b"dir", *b"wac", *b"sab", *b"wis", *b"sib",
    *b"rig", *b"sol", *b"dop", *b"mod", *b"fog", *b"lid", *b"hop", *b"dar",
    *b"dor", *b"lor", *b"hod", *b"fol", *b"rin", *b"tog", *b"sil", *b"mir",
    *b"hol", *b"pas", *b"lac", *b"rov", *b"liv", *b"dal", *b"sat", *b"lib",
    *b"tab", *b"han", *b"tic", *b"pid", *b"tor", *b"bol", *b"fos", *b"dot",
    *b"los", *b"dil", *b"for", *b"pil", *b"ram", *b"tir", *b"win", *b"tad",
    *b"bic", *b"dif", *b"roc", *b"wid", *b"bis", *b"das", *b"mid", *b"lop",
    *b"ril", *b"nar", *b"dap", *b"mol", *b"san", *b"loc", *b"nov", *b"sit",
    *b"nid", *b"tip", *b"sic", *b"rop", *b"wit", *b"nat", *b"pan", *b"min",
    *b"rit", *b"pod", *b"mot", *b"tam", *b"tol", *b"sav", *b"pos", *b"nap",
    *b"nop", *b"som", *b"fin", *b"fon", *b"ban", *b"mor", *b"wor", *b"sip",
    *b"ron", *b"nor", *b"bot", *b"wic", *b"soc", *b"wat", *b"dol", *b"mag",
    *b"pic", *b"dav", *b"bid", *b"bal", *b"tim", *b"tas", *b"mal", *b"lig",
    *b"siv", *b"tag", *b"pad", *b"sal", *b"div", *b"dac", *b"tan", *b"sid",
    *b"fab", *b"tar", *b"mon", *b"ran", *b"nis", *b"wol", *b"mis", *b"pal",
    *b"las", *b"dis", *b"map", *b"rab", *b"tob", *b"rol", *b"lat", *b"lon",
    *b"nod", *b"nav", *b"fig", *b"nom", *b"nib", *b"pag", *b"sop", *b"ral",
    *b"bil", *b"had", *b"doc", *b"rid", *b"moc", *b"pac", *b"rav", *b"rip",
    *b"fal", *b"tod", *b"til", *b"tin", *b"hap", *b"mic", *b"fan", *b"pat",
    *b"tac", *b"lab", *b"mog", *b"sim", *b"son", *b"pin", *b"lom", *b"ric",
    *b"tap", *b"fir", *b"has", *b"bos", *b"bat", *b"poc", *b"hac", *b"tid",
    *b"hav", *b"sap", *b"lin", *b"dib", *b"hos", *b"dab", *b"bit", *b"bar",
    *b"rac", *b"par", *b"lod", *b"dos", *b"bor", *b"toc", *b"hil", *b"mac",
    *b"tom", *b"dig", *b"fil", *b"fas", *b"mit", *b"hob", *b"har", *b"mig",
    *b"hin", *b"rad", *b"mas", *b"hal", *b"rag", *b"lag", *b"fad", *b"top",
    *b"mop", *b"hab", *b"nil", *b"nos", *b"mil", *b"fop", *b"fam", *b"dat",
    *b"nol", *b"din", *b"hat", *b"nac", *b"ris", *b"fot", *b"rib", *b"hoc",
    *b"nim", *b"lar", *b"fit", *b"wal", *b"rap", *b"sar", *b"nal", *b"mos",
    *b"lan", *b"don", *b"dan", *b"lad", *b"dov", *b"riv", *b"bac", *b"pol",
    *b"lap", *b"tal", *b"pit", *b"nam", *b"bon", *b"ros", *b"ton", *b"fod",
    *b"pon", *b"sov", *b"noc", *b"sor", *b"lav", *b"mat", *b"mip", *b"fip",
];

pub const DEX: [[u8; 3]; 256] = [
    *b"zod", *b"nec", *b"bud", *b"wes", *b"sev", *b"per", *b"sut", *b"let", *b"ful", *b"pen", *b"syt", *b"dur", *b"wep", *b"ser", *b"wyl", *b"sun", 
    *b"ryp", *b"syx", *b"dyr", *b"nup", *b"heb", *b"peg", *b"lup", *b"dep", *b"dys", *b"put", *b"lug", *b"hec", *b"ryt", *b"tyv", *b"syd", *b"nex", 
    *b"lun", *b"mep", *b"lut", *b"sep", *b"pes", *b"del", *b"sul", *b"ped", *b"tem", *b"led", *b"tul", *b"met", *b"wen", *b"byn", *b"hex", *b"feb", 
    *b"pyl", *b"dul", *b"het", *b"mev", *b"rut", *b"tyl", *b"wyd", *b"tep", *b"bes", *b"dex", *b"sef", *b"wyc", *b"bur", *b"der", *b"nep", *b"pur", 
    *b"rys", *b"reb", *b"den", *b"nut", *b"sub", *b"pet", *b"rul", *b"syn", *b"reg", *b"tyd", *b"sup", *b"sem", *b"wyn", *b"rec", *b"meg", *b"net", 
    *b"sec", *b"mul", *b"nym", *b"tev", *b"web", *b"sum", *b"mut", *b"nyx", *b"rex", *b"teb", *b"fus", *b"hep", *b"ben", *b"mus", *b"wyx", *b"sym", 
    *b"sel", *b"ruc", *b"dec", *b"wex", *b"syr", *b"wet", *b"dyl", *b"myn", *b"mes", *b"det", *b"bet", *b"bel", *b"tux", *b"tug", *b"myr", *b"pel", 
    *b"syp", *b"ter", *b"meb", *b"set", *b"dut", *b"deg", *b"tex", *b"sur", *b"fel", *b"tud", *b"nux", *b"rux", *b"ren", *b"wyt", *b"nub", *b"med", 
    *b"lyt", *b"dus", *b"neb", *b"rum", *b"tyn", *b"seg", *b"lyx", *b"pun", *b"res", *b"red", *b"fun", *b"rev", *b"ref", *b"mec", *b"ted", *b"rus", 
    *b"bex", *b"leb", *b"dux", *b"ryn", *b"num", *b"pyx", *b"ryg", *b"ryx", *b"fep", *b"tyr", *b"tus", *b"tyc", *b"leg", *b"nem", *b"fer", *b"mer", 
    *b"ten", *b"lus", *b"nus", *b"syl", *b"tec", *b"mex", *b"pub", *b"rym", *b"tuc", *b"fyl", *b"lep", *b"deb", *b"ber", *b"mug", *b"hut", *b"tun", 
    *b"byl", *b"sud", *b"pem", *b"dev", *b"lur", *b"def", *b"bus", *b"bep", *b"run", *b"mel", *b"pex", *b"dyt", *b"byt", *b"typ", *b"lev", *b"myl", 
    *b"wed", *b"duc", *b"fur", *b"fex", *b"nul", *b"luc", *b"len", *b"ner", *b"lex", *b"rup", *b"ned", *b"lec", *b"ryd", *b"lyd", *b"fen", *b"wel", 
    *b"nyd", *b"hus", *b"rel", *b"rud", *b"nes", *b"hes", *b"fet", *b"des", *b"ret", *b"dun", *b"ler", *b"nyr", *b"seb", *b"hul", *b"ryl", *b"lud", 
    *b"rem", *b"lys", *b"fyn", *b"wer", *b"ryc", *b"sug", *b"nys", *b"nyl", *b"lyn", *b"dyn", *b"dem", *b"lux", *b"fed", *b"sed", *b"bec", *b"mun", 
    *b"lyr", *b"tes", *b"mud", *b"nyt", *b"byr", *b"sen", *b"weg", *b"fyr", *b"mur", *b"tel", *b"rep", *b"teg", *b"pec", *b"nel", *b"nev", *b"fes"
];

/// Fetch prefix syllable (Hoon ++tos)
pub fn tos_po(i: u8) -> ParsedAtom {
    let b = SIS[i as usize];
    ParsedAtom::Small((b[0] as u128)
        | ((b[1] as u128) << 8)
        | ((b[2] as u128) << 16 ))
}

/// Fetch suffix syllable (Hoon ++tod)
pub fn tod_po(i: u8) -> ParsedAtom {
    let b = DEX[i as usize];
    ParsedAtom::Small((b[0] as u128)
        | ((b[1] as u128) << 8)
        | ((b[2] as u128) << 16))
}

/// Linear prefix search (Hoon ++ins)
pub fn ins(a: &[u8]) -> Option<u8> {
    if a.len() != 3 {
        return None;
    }

    let key = [a[0], a[1], a[2]];

    for (i, entry) in SIS.iter().enumerate() {
        if *entry == key {
            return Some(i as u8);
        }
    }

    None
}

/// Linear suffix search (Hoon ++ind)
pub fn ind(a: &[u8]) -> Option<u8> {
    if a.len() != 3 {
        return None;
    }

    let key = [a[0], a[1], a[2]];

    for (i, entry) in DEX.iter().enumerate() {
        if *entry == key {
            return Some(i as u8);
        }
    }

    None
}

// +tip:ab
pub fn tip<'src>(
) -> impl Parser<'src, &'src str, u8, Err<'src>>
{
    any()
        .filter(|c: &char| c.is_ascii_lowercase())
        .repeated()
        .exactly(3)
        .collect::<String>()
        .try_map(|s, span| {
            match ins(s.as_bytes()) {
                Some(i) => Ok(i),
                None => Err(Rich::custom(span, format!("invalid prefix syllable '{s}'"))),
            }
        }).labelled("Phonetic Prefix")
}

// +tiq:ab
pub fn tiq<'src>(
) -> impl Parser<'src, &'src str, u8, Err<'src>>
{
    any()
    .filter(|c: &char| c.is_ascii_lowercase())
    .repeated()
    .exactly(3)
    .collect::<String>()
    .try_map(|s, span| {
        match ind(s.as_bytes()) {
            Some(i) => Ok(i),
            None => Err(Rich::custom(span, format!("invalid suffix syllable '{s}'"))),
        }
    }).labelled("Phonetic Suffix")
}

// +hif:ab
pub fn hif<'src>(
) -> impl Parser<'src, &'src str, u16, Err<'src>>
{
    tip()
    .then(tiq())
    .map(|(p, q)| {
        (p as u16) * 256 + (q as u16)
    })
}

// @p
pub fn phonemic_name<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>>
{
    let tep =  any()
            .filter(|c: &char| c.is_ascii_lowercase())
            .repeated()
            .exactly(3)
            .to_slice()
            .try_map(|s: &str, span| {
                if s == "doz" {
                    return Err(Rich::custom(span, "prefix 'doz' is forbidden"));
                }
                match ins(s.as_bytes()) {
                    Some(i) => Ok(i),
                    None => Err(Rich::custom(span, format!("invalid prefix syllable '{s}'"))),
                }
            }).labelled("Phonetic Prefix");
    let hef = tip()
                .then(tiq())
                .try_map(|(p, q), span| {
                    let val = (p as u16) * 256 + (q as u16);
                    if val == 0 {
                        Err(Rich::custom(span, format!("phonetic is zero")))
                    } else {
                        Ok(val)
                    }
                }).boxed();
    let huf =  hef.clone() // u16
                .then(just('-')
                    .ignore_then(hif())  // u16
                    .repeated()
                    .at_most(3)
                    .collect::<Vec<_>>())
                    .map(|(first, rest)| {
                        std::iter::once(first).chain(rest).collect::<Vec<_>>()
                    })
                    .map(|hefs: Vec<u16>| {
                        let mut acc = BigUint::from(0u32);
                        for &digit in &hefs {
                            acc = (acc << 16) + BigUint::from(digit);
                        }
                        acc
                    });
    let hyf =  hif()
                .separated_by(just('-'))
                .exactly(4)
                .collect::<Vec<_>>()
                .map(|hefs: Vec<u16>| {
                    let mut acc = BigUint::from(0u32);
                    for &digit in &hefs {
                        acc = (acc << 16) + BigUint::from(digit);
                    }
                    acc
                });
    let other = huf
                .then(just("--").ignore_then(gap().or_not())
                        .ignore_then(hyf)
                        .repeated()
                        .at_least(1)
                        .collect::<Vec<_>>())
                .map(|(first, rest)| {
                    std::iter::once(first).chain(rest).collect::<Vec<_>>()
                })
                .map(|hefs: Vec<BigUint>| {
                    let acc = hefs
                                .iter()
                                .fold(BigUint::from(0u32), |acc, d| (acc << 64) + d);
                    ParsedAtom::Big(fynd_big(&acc))
                });
    let planet_moon = hef
                    .then(
                        just('-')
                        .ignore_then(hif())
                        .repeated()
                        .at_least(1)
                        .at_most(3)
                        .collect::<Vec<_>>())
                    .map(|(first, rest)| {
                        std::iter::once(first).chain(rest).collect::<Vec<_>>()
                    })
                    .map(|hefs: Vec<u16>| {
                        let mut acc = BigUint::from_u32(0).unwrap();
                        for &digit in &hefs {
                            acc = (acc << 16) + BigUint::from_u32(digit as u32).unwrap();
                        }
                        ParsedAtom::Big(fynd_big(&acc))
                    });
    let star = tep
                .then(tiq())
                .map(|(p, q)| {
                    let x = (p as u16) * 256 + (q as u16);
                    ParsedAtom::Small(x as u128)
                });
    let galaxy = tiq().map(|p| ParsedAtom::Small(p.into()));

    choice((
            other.labelled("Long Phonemic"),
            planet_moon.labelled("Planet or Moon"),
            star.labelled("Star"),
            galaxy.labelled("Galaxy"),
        ))
}

// @q
pub fn phonemic_name_unscrambled<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>>
{
    hif().or(tiq().map(|i| i as u16))
    .then(just('-')
            .ignore_then(gap().or_not())
            .ignore_then(hif())
            .repeated()
            .collect::<Vec<_>>())
    .map(|(first, rest)| {
        std::iter::once(first)
            .chain(rest)
            .map(ParsedAtom::from)
            .collect::<Vec<ParsedAtom>>()
    })
    .map(|mut hifs| {
        hifs.reverse();
        rep(4, None, &hifs)
    })
}

fn mix_atoms(a: ParsedAtom, b: ParsedAtom) -> ParsedAtom {
    match (a, b) {
        (ParsedAtom::Small(x), ParsedAtom::Small(y)) => ParsedAtom::Small(x ^ y),
        (a, b) => {
            let x = a.to_biguint();
            let y = b.to_biguint();
            ParsedAtom::from_biguint(&x ^ &y)
        }
    }
}

const RAKU: [u32; 4] = [
    0xb76d_5eed,
    0xee28_1300,
    0x85bc_ae01,
    0x4b38_7af7,
];

fn rol32(x: u32, r: u32) -> u32 {
    x.rotate_left(r)
}

fn fmix32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85eb_ca6b);
    h ^= h >> 13;
    h = h.wrapping_mul(0xc2b2_ae35);
    h ^= h >> 16;
    h
}

fn muk(seed: u32, len: u32, key: u64) -> u32 {
    let c1: u32 = 0xcc9e_2d51;
    let c2: u32 = 0x1b87_3593;

    let mut data = vec![0u8; len as usize];
    let mut k = key;
    for i in 0..len as usize {
        data[i] = (k & 0xff) as u8;
        k >>= 8;
    }

    let nblocks = (len / 4) as usize; // intentionally off-by-one
    let mut h1 = seed;

    let mut blocks = Vec::new();
    for i in 0..nblocks {
        let mut v = 0u32;
        for j in 0..4 {
            let idx = i * 4 + j;
            if idx < data.len() {
                v |= (data[idx] as u32) << (8 * j);
            }
        }
        blocks.push(v);
    }

    let mut i = nblocks;
    while i > 0 {
        let mut k1 = blocks[nblocks - i];
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe654_6b64);
        i -= 1;
    }

    let tail = &data[(nblocks * 4)..];
    let mut k1 = 0u32;

    match len & 3 {
        3 => {
            k1 ^= (tail[2] as u32) << 16;
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        2 => {
            k1 ^= (tail[1] as u32) << 8;
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        1 => {
            k1 ^= tail[0] as u32;
            k1 = k1.wrapping_mul(c1);
            k1 = k1.rotate_left(15);
            k1 = k1.wrapping_mul(c2);
            h1 ^= k1;
        }
        _ => {}
    }

    h1 ^= len;
    fmix32(h1)
}

fn eff(j: u64, r: u64) -> u64 {
    let seed = RAKU[(j as usize) & 3];
    muk(seed, 2, r) as u64
}

fn dvr(a: &ParsedAtom, b: &ParsedAtom) -> (ParsedAtom, ParsedAtom) {
    match (a, b) {
        (ParsedAtom::Small(x), ParsedAtom::Small(y)) => {
            let (q, r) = (x / y, x % y);
            (ParsedAtom::Small(q), ParsedAtom::Small(r))
        }
        _ => {
            let a_big = a.to_biguint();
            let b_big = b.to_biguint();
            let (q, r) = dvr_big(&a_big, &b_big);
            (ParsedAtom::Big(q), ParsedAtom::Big(r))
        }
    }
}

fn dvr_u64(a: u64, b: u64) -> (u64, u64) {
    (a / b, a % b)
}

fn fen(
    r: u64,
    a: u64,
    b: u64,
    m: u64,
) -> u64 {
    let mut j = r;

    let (ahh, ale) = if r % 2 == 0 {
        (m % a, m / a)
    } else {
        (m / a, m % a)
    };

    let (mut ell, mut arr) = if ale == a {
        (ahh, ale)
    } else {
        (ale, ahh)
    };

    while j >= 1 {
        let f = eff(j - 1, ell);

        let tmp = if j % 2 != 0 {
            (arr + a - (f % a)) % a
        } else {
            (arr + b - (f % b)) % b
        };

        j -= 1;
        arr = ell;
        ell = tmp;
    }

    &arr * a + ell
}

pub fn feis(m: ParsedAtom) -> ParsedAtom {
    debug_assert!(m.lt(&ParsedAtom::Small(0xffff_0000)));
    let m_u64 = m.to_u64_lossy();
    let a = 0xffffu64;
    let b = 0x1_0000u64;
    let k = a * b; // 0xffff_0000

    let mut c = fe_u64(4, a, b, |j, r| eff(j, r), m_u64);
    while c >= k {
        c = fe_u64(4, a, b, |j, r| eff(j, r), c);
    }
    ParsedAtom::Small(c as u128)
}

fn fe_u64(r: u64, a: u64, b: u64, prf: impl Fn(u64, u64) -> u64, m: u64) -> u64 {
    let mut j = 1u64;
    let mut ell = m % a;
    let mut arr = m / a;

    loop {
        if j > r {
            return if r % 2 == 1 {
                arr * a + ell
            } else if arr == a {
                arr * a + ell
            } else {
                ell * a + arr
            };
        }

        let f = prf(j - 1, arr);
        let tmp = if j % 2 == 1 {
            (f + ell) % a
        } else {
            (f + ell) % b
        };

        ell = arr;
        arr = tmp;
        j += 1;
    }
}

fn feen(r: u64, a: u64, b: u64, k: u64, m: u64) -> u64 {
    let c = fen(r, a, b, m);
    if c < k.into() {
        c
    } else {
        fen(r, a, b, c)
    }
}

pub fn fein(pyn: ParsedAtom) -> ParsedAtom {
    let lower_16 = ParsedAtom::Small(0x1_0000);
    let upper_16 = ParsedAtom::Small(0xffff_ffff);
    let lower_32 = ParsedAtom::Small(0x1_0000_0000);
    let upper_32 = ParsedAtom::Small(0xffff_ffff_ffff_ffff);

    if pyn.ge(&lower_16) && pyn.le(&upper_16) {
        let offset = match (&pyn, &lower_16) {
            (ParsedAtom::Small(x), ParsedAtom::Small(y)) => ParsedAtom::Small(x - y),
            _ => ParsedAtom::Big(&pyn.to_biguint() - &lower_16.to_biguint()),
        };
        let feised = feis(offset);
        match (&feised, &lower_16) {
            (ParsedAtom::Small(x), ParsedAtom::Small(y)) => ParsedAtom::Small(x + y),
            _ => ParsedAtom::Big(&feised.to_biguint() + &lower_16.to_biguint()),
        }
    }
    else if pyn.ge(&lower_32) && pyn.le(&upper_32) {
        let mask_lo = ParsedAtom::Small(0xffff_ffff);
        let lo = match (&pyn, &mask_lo) {
            (ParsedAtom::Small(x), ParsedAtom::Small(m)) => ParsedAtom::Small(dis(*x, *m)),
            _ => ParsedAtom::Big(dis_big(&pyn.to_biguint(), &mask_lo.to_biguint())),
        };

        let mask_hi = ParsedAtom::Small(0xffff_ffff_0000_0000);
        let hi = match (&pyn, &mask_hi) {
            (ParsedAtom::Small(x), ParsedAtom::Small(m)) => ParsedAtom::Small(dis(*x, *m)),
            _ => ParsedAtom::Big(dis_big(&pyn.to_biguint(), &mask_hi.to_biguint())),
        };

        let feined_lo = fein(lo);
        con_atoms(hi, feined_lo)
    }
    else {
        pyn
    }
}

fn tail(m: u64) -> u64 {
    feen(
        4,
        0xffff,
        0x1_0000,
        0xffff * 0x1_0000,
        m,
    )
}

fn fynd_big(cry: &BigUint) -> BigUint {
    let one_16 = BigUint::from(0x1_0000u32);
    let max_32 = BigUint::from(0xffff_ffffu32);
    let one_32 = BigUint::from(0x1_0000_0000u64);
    let max_64 = BigUint::from(u64::MAX);

    if cry >= &one_16 && cry <= &max_32 {
        let x = cry.to_u64().unwrap();
        return BigUint::from(fynd_u64(x));
    }

    if cry >= &one_32 && cry <= &max_64 {
        let lo = cry & &max_32;
        let hi = cry - &lo;
        let lo_f = BigUint::from(fynd_u64(lo.to_u64().unwrap()));
        return hi + lo_f;
    }

    cry.clone()
}

pub fn fynd_u64(cry: u64) -> u64 {
    if cry >= 0x1_0000 && cry <= 0xffff_ffff {
        return 0x1_0000 + tail(cry - 0x1_0000);
    }

    if cry >= 0x1_0000_0000 {
    // && cry <= 0xffff_ffff_ffff_ffff
        let lo = dis(cry, 0xffff_ffff);
        let hi = dis(cry, 0xffff_ffff_0000_0000);
        return con(hi, fynd_u64(lo));
    }

    cry
}

//  Coin Parsers (atoms encoded in strings)
//

// Top-level Coin parser  (atoms encoded in strings)
pub fn nuck<'src>(
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    choice((
        symbol().map(|s| Coin::Dime("tas".to_string(), string_to_atom(s))),
        number().map(|(p, q)| Coin::Dime(p, q)),
        just('.').ignore_then(perd()),
        just('~').ignore_then(
            choice((
                twid(),
                empty().to(Coin::Dime("n".to_string(), ParsedAtom::Small(0))),
            ))),
    )).boxed()
}

// Parses Coin after a leading sig, ~.
pub fn twid<'src>(
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    choice((
        just('0')
            .ignore_then(base32())
            .validate(|s, extra, emit| {
                let atom = base32_to_atom(s);
                let cued = cue_simple(atom);
                match cued {
                    Ok(c) => Coin::Blob(c),
                    Err(_e) => {
                        emit.emit(Rich::custom(extra.span(), format!("Failed to cue.")));
                        Coin::Blob(NounExpr::ParsedAtom(ParsedAtom::Small(0)))
                    }
                }
             }),
        crub(),
    ))
}

// Parse @da, @dr, @p, @t.
pub fn crub<'src>(
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    choice((
            absolute_date().map(|d| Coin::Dime("da".to_string(), d)),
            relative_date().map(|d| Coin::Dime("dr".to_string(), d)),
            phonemic_name().map(|p| Coin::Dime("p".to_string(), p)),
            just('.')
                .ignore_then(urs())
                .map(|atom| Coin::Dime("ta".to_string(), string_to_atom(atom))),
            just('~')
                .ignore_then(urx())
                .map(|atom| Coin::Dime("t".to_string(), atom)),
            just('-')
                .ignore_then(urx())
                .map(|atom| Coin::Dime("c".to_string(), taft(&atom))),
    ))
}

//  Parse Coin literal with escapes.
//
pub fn nusk<'src>(
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    urt()
    .validate(|s, extra, emit| {
        let wicked = wick(s);
        match wicked {
            Some(w) => w,
            None => {
                emit.emit(Rich::custom(extra.span(), format!("Invalid Knot Escape in '{}'.", s)));
                "".to_string()
            }
        }})
        .try_map(|unescaped: String, span| {
            let parsed = nuck().parse(&unescaped);
            match parsed.into_result() {
                Ok(output) => Ok(output),
                Err(_errors) => {
                    Err(Rich::custom(span, "Literal parse failed."))
                }
            }
        })
}

// Wraps Coin into Rock/Sand or Coltar
//
pub fn jock(rad: bool, lot: &Coin) -> Hoon {
    match lot {
        Coin::Dime(tag, atom) => {
            if rad {
                Hoon::Rock(tag.clone(), NounExpr::ParsedAtom(atom.clone()))
            } else {
                Hoon::Sand(tag.clone(), NounExpr::ParsedAtom(atom.clone()))
            }
        }

        Coin::Blob(noun) => {
            if rad {
                Hoon::Rock("$".to_string(), noun.clone())
            } else {
                match noun {
                    NounExpr::ParsedAtom(atom) => Hoon::Sand("$".to_string(), NounExpr::ParsedAtom(atom.clone())),
                    NounExpr::Cell(head, tail) => {
                        Hoon::Pair(
                            Box::new(jock(rad, &Coin::Blob(*head.clone()))),
                            Box::new(jock(rad, &Coin::Blob(*tail.clone()))),
                        )
                    }
                }
            }
        }

        Coin::Many(coins) => {
            Hoon::ColTar(coins.iter().map(|c| jock(rad, c)).collect())
        }
    }
}

// Parses a $coin literal without their respective standard prefixes.
//
pub fn perd<'src>(
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    choice((
        zust(),
        nusk()
            .separated_by(just('_'))
            .at_least(1)
            .collect::<Vec<_>>()
            .delimited_by(just('_'), just("__"))
            .map(|t| Coin::Many(t))
    ))
}

//  Parses @if, @is, @f, @r or @q.
//
pub fn zust<'src>(
) -> impl Parser<'src, &'src str, Coin, Err<'src>>
{
    choice((
        ipv6_address()
        .validate(|s, extra, emit| {
            let maybe_ipv6 = ipv6_to_atom(s.clone());
            match maybe_ipv6 {
                None => {
                    emit.emit(Rich::custom(
                        extra.span(),
                        "Invalid IPv6 Address",
                    ));
                    Coin::Dime("is".to_string(), ParsedAtom::Small(0))
                },
                Some(atom) => Coin::Dime("is".to_string(), atom),
            }
        }),
        ipv4_address()
        .validate(|s, extra, emit| {
            let maybe_ipv4 = ipv4_to_atom(s);
            match maybe_ipv4 {
                None => {
                    emit.emit(Rich::custom(
                        extra.span(),
                        "invalid IPv4 address",
                    ));
                    return Coin::Dime("if".to_string(), ParsedAtom::Small(0));
                },
                Some(atom) => Coin::Dime("if".to_string(), atom),
            }
        }),
        float().map(|(p, q)| Coin::Dime(p, q)),
        just("y").to(Coin::Dime("f".to_string(), ParsedAtom::Small(0))),
        just("n").to(Coin::Dime("f".to_string(), ParsedAtom::Small(1))),
        just('~')
            .ignore_then(phonemic_name_unscrambled())
            .map(|s| Coin::Dime("q".to_string(), s)),
    ))
}


pub fn urs<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    any()
        .filter(|c: &char| matches!(c, '0'..='9' | 'a'..='z' | '.' | '_' | '~' | '-'))
        .repeated()
        .collect::<String>()
}

pub fn urt<'src>(
) -> impl Parser<'src, &'src str, &'src str, Err<'src>> {
    any()
        .filter(|c: &char| matches!(c, '0'..='9' | 'a'..='z' | '.' | '~' | '-'))
        .repeated()
        .at_least(1)
        .to_slice()
}

fn wick(s: &str) -> Option<String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '~' {
            match chars.next() {
                Some('~') => out.push('~'),           // ~~ -> ~
                Some('-') => out.push('_'),           // ~- -> _
                Some(_) | None => return None,        // invalid escape
            }
        } else {
            // Only allow valid @ta characters: [a-z0-9._-]
            if c.is_ascii_lowercase()
                || c.is_ascii_digit()
                || c == '.'
                || c == '_'
                || c == '-'
            {
                out.push(c);
            } else {
                return None; // invalid char in atom
            }
        }
    }

    Some(out)
}

pub fn urx<'src>(
) -> impl Parser<'src, &'src str, ParsedAtom, Err<'src>> {
    let hex_escape =
        any().filter(|c: &char| c.is_ascii_hexdigit())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .delimited_by(just('~'), just('.'))
        .map(|hex_str: String| {
                let big = BigUint::from_str_radix(&hex_str, 16).unwrap_or_default();
                let value_32 = big.iter_u32_digits().next().unwrap_or(0); // low 32 bits

                let tuft_result = tuft(&ParsedAtom::Small(value_32 as u128));

                match tuft_result {
                    ParsedAtom::Small(n) => n,
                    ParsedAtom::Big(_) => panic!("tuft overflow"),
                }
            });

    let special = choice((
        just("~~").to(b'~' as u128),
        just("~.").to(b'.' as u128),
        just('.').to(b' ' as u128),
    ));

    let ascii = any().filter(|c: &char| {
        c.is_ascii_digit() || c.is_ascii_lowercase() || *c == '-' || *c == '_'
    })
    .map(|c| c as u128);

    let token = choice((
        hex_escape,
        special,
        ascii,
    ));

    token
    .repeated()
    .at_least(1)
    .collect::<Vec<u128>>()
    .map(|chars: Vec<u128>| rap(3, &chars))
}

// tuft: ParsedAtom (codepoint) -> ParsedAtom (UTF-8 bytes, @t)
pub fn tuft(atom: &ParsedAtom) -> ParsedAtom {
    let mut bytes: Vec<u8> = Vec::new();
    let mut a = atom.clone();

    loop {
        if a.is_zero() {
            break;
        }

        let b_atom = end(5, 1, &a);
        let b = b_atom.to_u128().unwrap();

        a = rsh(5, 1, &a);

        if b <= 0x7f {
            bytes.push(b as u8);
            continue;
        }

        if b <= 0x7ff {
            bytes.push(
                (0b1100_0000 | cut_u(b, 6, 5)) as u8
            );
            bytes.push(
                (0b1000_0000 | (b & 0x3f)) as u8
            );
            continue;
        }

        if b <= 0xffff {
            bytes.push(
                (0b1110_0000 | cut_u(b, 12, 4)) as u8
            );
            bytes.push(
                (0b1000_0000 | cut_u(b, 6, 6)) as u8
            );
            bytes.push(
                (0b1000_0000 | (b & 0x3f)) as u8
            );
            continue;
        }

        bytes.push(
            (0b1111_0000 | cut_u(b, 18, 3)) as u8
        );
        bytes.push(
            (0b1000_0000 | cut_u(b, 12, 6)) as u8
        );
        bytes.push(
            (0b1000_0000 | cut_u(b, 6, 6)) as u8
        );
        bytes.push(
            (0b1000_0000 | (b & 0x3f)) as u8
        );
    }

    let mut acc: u128 = 0;
    for (i, byte) in bytes.iter().enumerate() {
        acc |= (*byte as u128) << (i * 8);
    }

    ParsedAtom::Small(acc)
}
fn atom_to_u8(atom: &ParsedAtom) -> u8 {
    match end(3, 1, atom) {
        ParsedAtom::Small(n) => n as u8,
        ParsedAtom::Big(_) => 0,
    }
}

// --- UTF-8 continuation byte check ---
fn is_continuation(b: u8) -> bool {
    b & 0xC0 == 0x80
}

// --- teff: UTF-8 leading byte → length (1–4) ---
fn teff(atom: &ParsedAtom) -> usize {
    let b = atom_to_u8(atom);
    if b == 0 {
        return 0;
    }
    if b <= 0x7F { 1 }
    else if b <= 0xDF { 2 }
    else if b <= 0xEF { 3 }
    else if b <= 0xF4 { 4 }
    else { 1 }
}

// --- Decode one UTF-8 codepoint ---
fn decode_one_utf8(atom: &ParsedAtom, len: usize) -> u32 {
    match len {
        1 => atom_to_u8(atom) as u32,
        2 => {
            let b0 = atom_to_u8(atom);
            let b1 = atom_to_u8(&rsh(3, 1, atom));
            if !is_continuation(b1) { return 0xFFFD; }
            let cp = ((b0 & 0x1F) as u32) << 6 | (b1 & 0x3F) as u32;
            if cp < 0x80 { 0xFFFD } else { cp }
        }
        3 => {
            let b0 = atom_to_u8(atom);
            let b1 = atom_to_u8(&rsh(3, 1, atom));
            let b2 = atom_to_u8(&rsh(3, 2, atom));
            if !is_continuation(b1) || !is_continuation(b2) { return 0xFFFD; }
            let cp = ((b0 & 0x0F) as u32) << 12 | ((b1 & 0x3F) as u32) << 6 | (b2 & 0x3F) as u32;
            if cp < 0x800 || (0xD800..=0xDFFF).contains(&cp) { 0xFFFD } else { cp }
        }
        4 => {
            let b0 = atom_to_u8(atom);
            let b1 = atom_to_u8(&rsh(3, 1, atom));
            let b2 = atom_to_u8(&rsh(3, 2, atom));
            let b3 = atom_to_u8(&rsh(3, 3, atom));
            if !is_continuation(b1) || !is_continuation(b2) || !is_continuation(b3) { return 0xFFFD; }
            let cp = ((b0 & 0x07) as u32) << 18 | ((b1 & 0x3F) as u32) << 12
                   | ((b2 & 0x3F) as u32) << 6  | (b3 & 0x3F) as u32;
            if !(0x1_0000..=0x10_FFFF).contains(&cp) { 0xFFFD } else { cp }
        }
        _ => 0xFFFD,
    }
}

// @t (UTF-8 atom) -> @c (UTF-32 packed atom)
pub fn taft(atom: &ParsedAtom) -> ParsedAtom {
    let mut codepoints = Vec::new();
    let mut current = atom.clone();

    loop {
        let len = teff(&current);
        if len == 0 {
            break;
        }
        let cp = decode_one_utf8(&current, len);
        codepoints.push(cp);
        current = rsh(3, len, &current); // shift by `len` bytes
    }

    // Pack into @c: each u32 in 32-bit lane, LSB-first (rap 5)
    if codepoints.is_empty() {
        ParsedAtom::Small(0)
    } else if codepoints.len() <= 4 {
        let mut acc: u128 = 0;
        for (i, &cp) in codepoints.iter().enumerate() {
            acc |= (cp as u128) << (i * 32);
        }
        ParsedAtom::Small(acc)
    } else {
        let mut acc = BigUint::zero();
        for (i, &cp) in codepoints.iter().enumerate() {
            acc |= BigUint::from(cp) << (i * 32);
        }
        ParsedAtom::from_biguint(acc)
    }
}

pub fn base32<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>> {
    any()
        .filter(|c: &char| c.is_ascii_alphanumeric() && *c <= 'v')
        .repeated()
        .at_least(1)
        .collect::<String>()
}


const BTC_BASE58: &str = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

fn build_yek() -> [u8; 256] {
    let mut yek = [0xFFu8; 256];
    for (i, ch) in BTC_BASE58.chars().enumerate() {
        let idx = ch as u8 as usize;
        if idx < 256 {
            yek[idx] = i as u8;
        }
    }
    yek
}

fn cha_fa(yek: &[u8; 256], ch: char) -> Option<u8> {
    let idx = ch as u32;
    if idx > 255 { return None; }
    let val = yek[idx as usize];
    if val == 0xFF { None } else { Some(val) }
}

fn bass_58(digits: &[u8]) -> BigUint {
    digits.iter().fold(BigUint::from(0u32), |acc, &d| {
        &acc * 58u32 + d as u32
    })
}

fn tok(a: &ParsedAtom) -> ParsedAtom {
    let b = pad_fa(&a);

    let swapped = swp(3, a);

    let padded = lsh(3, b, &swapped);

    let len = b + met(3, a);

    let hashed = shay(len as u64, &padded.to_biguint());

    let double_hashed = &ParsedAtom::Big(shay(32, &hashed));
    let truncated = end(3, 4, double_hashed);

    let n = net(5, &truncated);
    n
}

pub fn shay(len: u64, ruz: &BigUint) -> BigUint {
    let len = len as usize;

    let ruz_bytes = ruz.to_bytes_le();
    let msg_len = ruz_bytes.len();

    let mut msg = vec![0u8; len];

    if len == 0 {
    } else if msg_len >= len {
        msg.copy_from_slice(&ruz_bytes[..len]);
    } else {
        msg[..msg_len].copy_from_slice(&ruz_bytes);
    }

    let mut hasher = Sha256::new();
    hasher.update(&msg);
    let digest = hasher.finalize();

    BigUint::from_bytes_le(&digest)
}

fn swp(bloq: usize, b: &ParsedAtom) -> ParsedAtom {
    let blocks = rip(bloq, b);
    let rev = flop(&blocks);
    rep(bloq, None, &rev)
}

fn rip(bloq: usize, b: &ParsedAtom) -> Vec<ParsedAtom> {
    if b.is_zero() {
        return Vec::new();
    }

    let mut out = Vec::new();
    let mut cur = b.clone();

    while !cur.is_zero() {
        out.push(end(bloq, 1, &cur));
        cur = rsh(bloq, 1, &cur);
    }

    out
}

pub fn den_fa(a: &ParsedAtom) -> Option<ParsedAtom> {
    let b = rsh(3, 4, a);

    if tok(&b) == end(3, 4, a) {
        Some(b)
    } else {
        None
    }
}

fn sit(a: usize, b: &ParsedAtom) -> ParsedAtom {
    end(a, 1, b)
}

//  flip byte endianness
fn net(a: usize, b: &ParsedAtom)
-> ParsedAtom {
    let b = sit(a, b);

    if a <= 3 {
        return b;
    }

    let c: usize = a - 1;

    let hi_bit = cut(c, 0, 1, &b);
    let hi = net(c, &hi_bit);

    let lo_bit = cut(c, 1, 1, &b);
    let lo = net(c, &lo_bit);

    let res = con_atoms(lsh(c, 1, &hi), lo);
    res
}

fn met_big(bloq: u32, atom: &BigUint) -> u32 {
    let bits = 1u32 << bloq; // bloq_bits
    if atom.is_zero() {
        return 1;
    }
    let atom_bits = atom.bits() as u32;
    (atom_bits + bits - 1) / bits
}

/// pad(a): number of zero bytes needed to pad `a` to 21 bytes
fn pad_fa_big(a: &BigUint) -> usize {
    let b = met(3, &ParsedAtom::Big(a.clone()));
    if b >= 21 {
        0
    } else {
        21 - b as usize
    }
}

pub fn pad_fa(atom: &ParsedAtom) -> usize {
    21usize.saturating_sub(met(3, atom))
}

pub fn enc_fa(atom: &ParsedAtom) -> ParsedAtom {
    let a = atom;

    let shifted = lsh(3, 4, a).to_biguint();
    let checksum = tok(atom).to_biguint();

    ParsedAtom::from_biguint(shifted ^ checksum)
}

pub fn trip(mut atom: ParsedAtom) -> Tape {
    let mut out = Vec::new();

    while atom != ParsedAtom::Small(0) {
        let byte_atom = end(3, 1, &atom);

        let byte = match byte_atom {
            ParsedAtom::Small(x) => x as u8,
            ParsedAtom::Big(b) => b.try_into().unwrap_or(0),
        };

        out.push((byte as char).to_string());
        atom = rsh(3, 1, &atom);
    }

    out
}

pub fn wack(a: &str) -> String {
    a.chars()
        .flat_map(|c| match c {
            '~' => vec!['~', '~'],
            '_' => vec!['~', '-'],
            _ => vec![c],
        })
        .collect()
}

pub fn rent_co(lot: &Coin) -> ParsedAtom {
    let rend_res = rend_co(lot);
    let bytes: Vec<u128> = rend_res
        .into_iter()
        .flat_map(|s: String| s.chars().map(|c| c as u128).collect::<Vec<_>>())
        .collect();
   let rap_res = rap(3 as usize, &bytes);
   rap_res
}

pub fn rend_co(lot: &Coin) -> Tape {
    rend_with_rep(lot, vec![])
}

fn rend_many(coins: &[Coin], rep: Tape) -> Tape {
    if coins.is_empty() {
        return vec!["_".to_string(), "_".to_string()].into_iter().chain(rep).collect();
    }
    let first = &coins[0];
    let rest = &coins[1..];

    let mut res = vec!["_".to_string()];
    let rendered_first = rend_co(first);
    let escaped_knot = wack(&rendered_first.concat(
    ));
    let taped_escaped = trip(string_to_atom(escaped_knot));
    res.extend(taped_escaped);
    res.extend(rend_many(rest, rep));
    res
}

fn rend_with_rep(lot: &Coin, mut rep: Tape) -> Tape {
    match lot {
        Coin::Blob(noun) => {
            let jammed = jam_simple(noun.clone());
            let mut res = vec!["~".to_string(), "0".to_string()];
            res.extend(v_co(1, &jammed));
            res
        }

        Coin::Many(coins) => {
            let mut res = vec![".".to_string()];
            res.extend(rend_many(coins, rep));
            res
        }

        Coin::Dime(prefix, q) => {
            let yed = end(3, 1, &string_to_atom(prefix.to_string())); // first char of prefix
            let hay = cut(3, 1, 1, &string_to_atom(prefix.to_string())); // second char

            let yed_char = match &yed {
                ParsedAtom::Small(x) => *x as u8 as char,
                ParsedAtom::Big(_) => unreachable!(), // prefix is short
            };

            let hay_char = match &hay {
                ParsedAtom::Small(x) => *x as u8 as char,
                ParsedAtom::Big(_) => unreachable!(),
            };

            match yed_char {
                'c' => {
                    let mut res = vec!['~'.to_string(), '-'.to_string()];
                    let wood_res = wood(&tuft(q));
                    let rip_res = rip(3, &wood_res);
                    let qtape: Vec<_> = rip_res.into_iter().flat_map(|a| trip(a)).collect();
                    res.extend(qtape);
                    res.extend(rep);
                    res
                }

                'd' => match hay_char {
                    'a' => {
                        let yod = yore(q);
                        let mut rep = rep;
                        if !yod.t.f.is_empty() {
                            let frac_tape = s_co(&yod.t.f);
                            let mut new_rep = vec![".".to_string()];
                            new_rep.extend(frac_tape);
                            new_rep.extend(rep);
                            rep = new_rep;
                        }

                        let t = &yod.t;
                        if !(yod.t.f.is_empty() && t.h == 0 && t.m == 0 && t.s == 0) {
                            let s_atom = ParsedAtom::Small(t.s as u128);
                            let mut new_rep = vec![".".to_string()];
                            new_rep.extend(y_co(&s_atom));
                            let m_atom = ParsedAtom::Small(t.m as u128);
                            let mut newer_rep = vec![".".to_string()];
                            newer_rep.extend(y_co(&m_atom));
                            newer_rep.extend(new_rep);
                            let h_atom = ParsedAtom::Small(t.h as u128);
                            let mut newest_rep = vec![".".to_string(), ".".to_string()];
                            newest_rep.extend(y_co(&h_atom));
                            newest_rep.extend(newer_rep);
                            newest_rep.extend(rep);
                            rep = newest_rep
                        }

                        let d_atom = ParsedAtom::Small(t.d as u128);
                        let mut new_rep = vec![".".to_string()];
                        new_rep.extend(a_co(&d_atom));
                        new_rep.extend(rep);
                        rep = new_rep;

                        let m_atom = ParsedAtom::Small(yod.m as u128);
                        let mut newer_rep = vec![".".to_string()];
                        newer_rep.extend(a_co(&m_atom));
                        newer_rep.extend(rep);
                        rep = newer_rep;

                        if !yod.era {
                            let mut newest_rep = vec!["-".to_string()];
                            newest_rep.extend(rep);
                            rep = newest_rep;
                        }

                        let y_atom = ParsedAtom::Small(yod.y as u128);
                        let mut res = vec!["~".to_string()];
                        res.extend(a_co(&y_atom));
                        res.extend(rep);
                        res
                    }

                    'r' => {
                        let yug = yell(q);

                        let mut rep = rep;

                        if !yug.f.is_empty() {
                            let frac_tape = s_co(&yug.f);
                            let mut new_rep = vec![".".to_string()];
                            new_rep.extend(frac_tape);
                            new_rep.extend(rep);
                            rep = new_rep;
                        }

                        let mut res = vec!["~".to_string()];

                        if yug.d == 0 && yug.m == 0 && yug.h == 0 && yug.s == 0 {
                            res.extend(vec!["s".to_string(), "0".to_string()]);
                            res.extend(rep);
                            return res;
                        }

                        if yug.s != 0 {
                            let s_atom = ParsedAtom::Small(yug.s as u128);
                            let mut new_rep = vec![".".to_string(), "s".to_string()];
                            new_rep.extend(a_co(&s_atom));
                            new_rep.extend(rep);
                            rep = new_rep;
                        }

                        if yug.m != 0 {
                            let m_atom = ParsedAtom::Small(yug.m as u128);
                            let mut new_rep = vec![".".to_string(), "m".to_string()];
                            new_rep.extend(a_co(&m_atom));
                            new_rep.extend(rep);
                            rep = new_rep;
                        }

                        if yug.h != 0 {
                            let h_atom = ParsedAtom::Small(yug.h as u128);
                            let mut new_rep = vec![".".to_string(), "h".to_string()];
                            new_rep.extend(a_co(&h_atom));
                            new_rep.extend(rep);
                            rep = new_rep;
                        }

                        if yug.d != 0 {
                            let d_atom = ParsedAtom::Small(yug.d as u128);
                            let mut new_rep = vec![".".to_string(), "d".to_string()];
                            new_rep.extend(a_co(&d_atom));
                            new_rep.extend(rep);
                            rep = new_rep;
                        }

                        res.extend(rep.iter().skip(1).cloned());
                        res
                    }

                    _ => z_co(q),
                },

                'f' => {
                    match q {
                        ParsedAtom::Small(0) => vec!['.'.to_string(), 'y'.to_string()],
                        ParsedAtom::Small(1) => vec!['.'.to_string(), 'n'.to_string()],
                        _ => z_co(q),
                    }
                    .into_iter()
                    .chain(rep.into_iter())
                    .collect()
                }

                'n' => {
                    let mut res = vec!['~'.to_string()];
                    res.extend(rep);
                    res
                }

                'i' => match hay_char {
                    'f' => ro_co([3, 10, 4], &|x| d_ne(x), q),
                    's' => ro_co([4, 16, 8], &|x| x_ne(x), q),
                    _ => z_co(q),
                },

               'p' => {
                    let sxz = fein(q.clone());
                    let dyx = met(3, &sxz);

                    let mut out: Tape = vec!['~'.to_string()];

                    if dyx <= 1 {
                        let byte = sxz.to_u8_lossy();
                        let syl = tod_po(byte);
                        out.extend(trip(syl));
                        out.extend(rep);
                        return out;
                    }

                    let dyy = met(4, &sxz);
                    let mut chunks = Vec::with_capacity(dyy);

                    for imp in 0..dyy {
                        let log = cut(4, imp, 1, &sxz);

                        let hi_atom = rsh(3, 1, &log);
                        let hi = hi_atom.to_u8_lossy();

                        let lo_atom = end(3, 1, &log);
                        let lo = lo_atom.to_u8_lossy();

                        let prefix = trip(tos_po(hi));
                        let suffix = trip(tod_po(lo));

                        let mut chunk = weld(&prefix, &suffix);

                        let sep = if imp % 4 == 0 {
                            if imp == 0 {
                                    vec![]
                                } else {
                                    vec!['-'.to_string(), '-'.to_string()]
                                }
                        } else {
                            vec!['-'.to_string()]
                        };
                        chunk.extend(sep);

                        chunks.push(chunk);
                    }

                    chunks.reverse();
                    for chunk in chunks {
                        out.extend(chunk);
                    }
                    out.extend(rep);
                    out
                }

                'q' => {
                    let head = vec![".".to_string(), "~".to_string()];

                    let lot: Vec<ParsedAtom> = if q.is_zero() {
                        vec![ParsedAtom::Small(0)]
                    } else {
                        rip(3, q)
                    };

                    let mut r: Tape = Vec::new();
                    let mut s = true;

                    for atom in lot.into_iter() {
                        let q_atom = atom.to_u8().expect("byte");

                        let mut rendered = if s {
                            trip(tod_po(q_atom))
                        } else {
                            trip(tos_po(q_atom))
                        };

                        let tail = if s && !r.is_empty() {
                            let mut t = vec!["-".to_string()];
                            t.extend(r);
                            t
                        } else {
                            r
                        };

                        s = !s;
                        r = weld(rendered, tail);
                    }

                    let mut res = head;
                    res = weld(res, r);
                    res = weld(res, rep);
                    res
                }

                'r' => match hay_char {
                    'd' => {
                        let val = q.to_u128().unwrap();
                        let df = rlyd(val);
                        let rc = r_co(&df, rep.clone());
                        let mut res = vec![".".to_string(), "~".to_string()];
                        res.extend(rc);
                        res.extend(rep);
                        res
                    }
                    'h' => {
                        let val = q.to_u128().unwrap();
                        let df = rlyh(val);
                        let rc = r_co(&df, rep.clone());
                        let mut res = vec![".".to_string(), "~".to_string(), "~".to_string()];
                        res.extend(rc);
                        res.extend(rep);
                        res
                    }
                    'q' => {
                        let val = q.to_u128().unwrap();
                        let df = rlyq(val);
                        let rc = r_co(&df, rep.clone());
                        let mut res = vec![".".to_string(), "~".to_string(), "~".to_string(), "~".to_string()];
                        res.extend(rc);
                        res.extend(rep);
                        res
                    }
                    's' => {
                        let val = q.to_u128().unwrap();
                        let df = rlys(val);
                        let rc = r_co(&df, rep.clone());
                        let mut res = vec![".".to_string()];
                        res.extend(rc);
                        res.extend(rep);
                        res
                    }
                    _ => {
                        let mut res = z_co(q);
                        res.extend(rep);
                        res
                    }
                },

                'u' => {
                    match hay_char {
                        'c' => {
                            // base58check with padding
                            let encoded = enc_fa(q);
                            let padded_ones = reap(pad_fa(&q), '1'.to_string());
                            let mut res = vec!['0'.to_string(), 'c'.to_string()];
                            res.extend(padded_ones);
                            res.extend(c_co(&encoded));
                            res.extend(rep);
                            res
                        }
                        'b' => with_prefix("0b", &ox_co([2, 4], &|x| d_ne(x), q), rep),
                        'i' => with_prefix("0i", &d_co(1, q), rep),
                        'x' => with_prefix("0x", &ox_co([16, 4], &|x| x_ne(x), q), rep),
                        'v' => with_prefix("0v", &ox_co([32, 5], &|x| x_ne(x), q), rep),
                        'w' => with_prefix("0w", &ox_co([64, 5], &|x| w_ne(x), q), rep),
                        _ => {
                            vec![ox_co([10, 3], &|x| d_ne(x), q).into_iter().chain(rep).collect()]
                        }
                    }
                }

                's' => {
                    let q = q.to_u128().expect("signed number is bigger than 128 bits");
                    let sign_prefix_chars = if syn_si(q) {
                            vec!['-'.to_string(), '-'.to_string()]
                        } else {
                            vec!['-'.to_string()]
                        };
                    let abs_val = abs_si(q);
                    let mut res: Tape = sign_prefix_chars.into_iter().collect();
                    res.extend(rend_with_rep(&Coin::Dime("u".into(), ParsedAtom::Small(abs_val)), rep));
                    res
                }

                't' => {
                    if hay_char == 'a' {
                        let third = cut(3, 2, 1, &string_to_atom(prefix.to_string()));
                        let third_char = match &third {
                            ParsedAtom::Small(x) => *x as u8 as char,
                            ParsedAtom::Big(_) => '\0',
                        };
                        if third_char == 's' {
                            let mut res: Vec<_> = rip(3, q).into_iter().flat_map(|a| trip(a)).collect();
                            res.extend(rep);
                            res
                        } else {
                            let mut res = vec!['~'.to_string(), '.'.to_string()];
                            res.extend(rip(3, q).into_iter().flat_map(|a| trip(a)));
                            res.extend(rep);
                            res
                        }
                    } else {
                        let mut res = vec!['~'.to_string(), '~'.to_string()];
                        let wooded = wood(q);
                        res.extend(rip(3, &ParsedAtom::from(wooded)).into_iter().flat_map(|a| trip(a)));
                        res.extend(rep);
                        res
                    }
                }

                _ => z_co(q),
            }
        }
    }
}

fn r_co(df: &DecimalFloat, mut rep: Tape) -> Tape {
    match df {
        DecimalFloat::Infinity { sign } => {
            let prefix = if *sign { "inf" } else { "-inf" };
            prefix.chars().map(|c| c.to_string()).chain(rep.into_iter()).collect()
        }
        DecimalFloat::NaN => {
            "nan".chars().map(|c| c.to_string()).chain(rep.into_iter()).collect()
        }
        DecimalFloat::Finite { sign, exp, mant } => {

            let f: Tape = d_co(1, &ParsedAtom::Big(mant.clone()));

            let (e, exp): (u128, u128) = {
                let e = sun_si(f.len() as u128);

                let sci = sum_si(*exp, sum_si(e, 1));

                if syn_si(dif_si(*exp, 6)) {
                    (2, sci)
                }
                else if !syn_si(dif_si(sci, 3)) {
                    (2, sci)
                }
                else {
                    (sum_si(sci, 2), 0)
                }
            };

            if exp != 0u128 {
                let exp_mark = if syn_si(exp) { "e" } else { "e-" };
                rep = weld(
                    vec![exp_mark.to_string()],
                    d_co(1, &ParsedAtom::Small(abs_si(exp))),
                );
            }

            let mut out = weld(ed_co(&e, &f), rep);

            if !sign {
                out = weld(vec!["-".to_string()], out);
            }

            out
        }
    }
}

fn ed_co(exp: &u128, int: &Tape) -> Tape {
    let cmp = cmp_si(*exp, 0);
    let pos = cmp == 2;
    let dig = abs_si(*exp) as usize;

    if !pos {
        let mut out = reap(dig + 1, "0".to_string());
        out.extend(int.clone());
        return into(out, 1, ".");
    }

    let len = int.len();

    if dig < len {
        return into(int.clone(), dig, ".");
    }

    let mut out = int.clone();
    out.extend(reap(dig - len, "0".to_string()));
    out
}

fn wood_go(a: &ParsedAtom) -> Vec<u128> {
    if a.is_zero() {
        return Vec::new();
    }

    let b = teff(a);
    let c_atom = taft(&end(3, b, a));
    let c = c_atom.to_u32().unwrap();
    let mut d = wood_go(&rsh(3, b, a));

    // alnum or '-'
    if (c >= b'a' as u32 && c <= b'z' as u32)
        || (c >= b'0' as u32 && c <= b'9' as u32)
        || c == b'-' as u32
    {
        d.insert(0, c as u128);
        return d;
    }

    match c as u8 {
        b' ' => {
            d.insert(0, b'.' as u128);
        }
        b'.' => {
            d.insert(0, b'.' as u128);
            d.insert(0, b'~' as u128);
        }
        b'~' => {
            d.insert(0, b'~' as u128);
            d.insert(0, b'~' as u128);
        }
        _ => {
            d = wood_hex(c, d);
        }
    }

    d
}

fn wood_hex(c: u32, mut d: Vec<u128>) -> Vec<u128> {
    let e = met(2, &ParsedAtom::Small(c as u128));

    d.insert(0, b'.' as u128);

    for i in 0..e {
        let shift = i * 4;
        let f = (c >> shift) & 0xF;
        let ch = if f <= 9 { 48 + f } else { 87 + f };
        d.insert(0, ch as u128);
    }

    d.insert(0, b'~' as u128);
    d
}

pub fn wood(a: &ParsedAtom) -> ParsedAtom {
    let bytes = wood_go(a);
    rap(3, &bytes)
}

fn into(mut tape: Tape, idx: usize, ch: &str) -> Tape {
    tape.insert(idx, ch.to_string());
    tape
}

// fn atom_to_char(atom: &ParsedAtom) -> char {
//     let code = match atom {
//         ParsedAtom::Small(x) => *x as u32,
//         ParsedAtom::Big(b) => {
//             if *b > BigUint::from(u32::MAX) {
//                 0xFFFD //  replacement
//             } else {
//                 b.clone().try_into().unwrap_or(0xFFFD)
//             }
//         }
//     };
//     std::char::from_u32(code).unwrap_or('\u{FFFD}')
// }

fn d_ne(tig: u128) -> char {
    (tig as u8 + b'0') as char
}

fn x_ne(tig: u128) -> char {
    if tig < 10 {
        (b'0' + tig as u8) as char
    } else {
        (b'a' + (tig - 10) as u8) as char
    }
}

fn v_ne(tig: u128) -> char {
    if tig >= 10 {
        (tig + 87) as u8 as char
    } else {
        (tig + 48) as u8 as char
    }
}


fn w_ne(tig: u128) -> char {
    // base64 with - and ~ for 62/63
    if tig == 62 {
        '-'
    } else if tig == 63 {
        '~'
    } else if tig < 26 {
        (b'A' + tig as u8) as char
    } else if tig < 52 {
        (b'a' + (tig - 26) as u8) as char
    } else if tig < 62 {
        (b'0' + (tig - 52) as u8) as char
    } else {
        unreachable!()
    }
}

fn c_ne(tig: u128) -> char {
    // base58: skips 0, O, I, l
    const CHARS: &[u8] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";
    CHARS[tig as usize] as char
}

fn with_prefix(prefix: &str, body: &Tape, rep: Tape) -> Tape {
    let mut res: Tape = prefix.chars().map(|c| c.to_string()).collect();
    res.extend(body.iter().cloned());
    res.extend(rep);
    res
}

fn s_co(frac: &[u64]) -> Tape {
    if frac.is_empty() {
        return vec![];
    }
    let mut res = vec![".".to_string()];
    let first = ParsedAtom::Small(frac[0] as u128);
    res.extend(x_co(4, &first));
    res.extend(s_co(&frac[1..]));
    res
}

fn em_co<F>(
    bas: u128,
    min: usize,
    mut par: F,
    hol: &ParsedAtom,
    rep: Tape,
) -> Tape
where
    F: FnMut(bool, u128, Tape) -> Tape,
{
    if hol.is_zero() && min == 0 {
        return rep;
    }
    let (dar, rad) = dvr(hol, &ParsedAtom::Small(bas));
    let next_min = min.saturating_sub(1);
    let rad_u128 = rad.to_u128().unwrap_or(0);
    let next_rep = par(dar.is_zero(), rad_u128, rep);
    em_co(bas, next_min, par, &dar, next_rep)
}


fn d_co(min: usize, dat: &ParsedAtom) -> Tape {
    em_co(
        10,
        min,
        |_, b, c: Tape| {
            let ch = d_ne(b);
            std::iter::once(ch.to_string()).chain(c).collect()
        },
        dat,
        vec![],
    )
}

fn x_co(min: usize, dat: &ParsedAtom) -> Tape {
    em_co(
        16,
        min,
        |_, b, c| {
            let ch = x_ne(b).to_string();
            std::iter::once(ch).chain(c).collect::<Vec<String>>()
        },
        dat,
        vec![],
    )
}

fn v_co(min: usize, dat: &ParsedAtom) -> Tape {
    em_co(
        32,
        min,
        |_, b, c| {
            let ch = v_ne(b).to_string();
            std::iter::once(ch).chain(c).collect::<Vec<String>>()
        },
        dat,
        vec![],
    )
}

// fn w_co(min: usize, dat: &ParsedAtom) -> Tape {
//     em_co(
//         64,
//         min,
//         |_, b, c| {
//             let ch = w_ne(b).to_string();
//             std::iter::once(ch).chain(c).collect::<Vec<String>>()
//         },
//         dat,
//         vec![],
//     )
// }

fn c_co(dat: &ParsedAtom) -> Tape {
    em_co(
        58,
        1,
        |_, b, c| {
            let ch = c_ne(b).to_string();
            std::iter::once(ch).chain(c).collect::<Vec<String>>()
        },
        dat,
        vec![],
    )
}

fn a_co(dat: &ParsedAtom) -> Tape {
    d_co(1, dat)
}

fn y_co(dat: &ParsedAtom) -> Tape {
    d_co(2, dat)
}

fn z_co(dat: &ParsedAtom) -> Tape {
    let mut res = vec!["0".to_string(), "x".to_string()];
    res.extend(x_co(1, dat));
    res
}

fn ox_co<F>(
    [bas, gop]: [u128; 2],
    dug: &F,
    hol: &ParsedAtom,
) -> Tape
where
    F: Fn(u128) -> char,
{
    let pow_bas_gop = pow(bas, gop)
                        .to_u128()
                        .expect("base does not fit in u128");
    em_co(
        pow_bas_gop,
        0,
        |top, seg, res| {
            let prefix: Tape = if top { vec![] } else { vec!['.'.to_string()] };
            let inner = em_co(
                bas,
                if top { 0 } else { gop as usize },
                |_, b, c| std::iter::once(dug(b).to_string()).chain(c).collect::<Vec<String>>(),
                &ParsedAtom::Small(seg),
                res,
            );
            prefix.into_iter().chain(inner).collect()
        },
        hol,
        vec![],
    )
}

fn ro_co<F>(
    [buz, bas, mut dop]: [usize; 3],
    dug: &F,
    hol: &ParsedAtom,
) -> Tape
where
    F: Fn(u128) -> char,
{
    if dop == 0 {
        return vec![];
    }
    let pod = dop - 1;
    let seg = cut(buz, pod, 1, hol); // bloq = buz, start = pod, run = 1
    let mut res = vec!['.'.to_string()];
    res.extend(em_co(
        bas as u128,
        1,
        |_, b, c| std::iter::once(dug(b).to_string()).chain(c).collect::<Vec<String>>(),
        &seg,
        ro_co([buz, bas, pod], dug, hol),
    ));
    res
}

//  Date parsing @da @dr
//

fn relative_date_pair<'src>(
)-> impl Parser<'src, &'src str, (char, u64), Err<'src>> {
    any().filter(|&c| c == 'd' || c == 'h' || c == 'm' || c == 's')
        .then(
            decimal_without_leading_zero()
            .try_map(|s, span| {
                s.parse::<u64>().map_err(|_| Rich::custom(span, "Invalid Number"))
            })
        )
}

// decimal without leading 0 and without dots.
//
pub fn decimal_without_leading_zero<'src>(
) -> impl Parser<'src, &'src str, String, Err<'src>>
{
    just('0')
    .to("0".to_string())
    .or(
        any().filter(|c: &char| matches!(c, '1'..='9'))
            .then(any().filter(|c: &char| c.is_ascii_digit()).repeated().collect::<String>())
            .map(|(h, t)| format!("{h}{t}"))
    )
}

// ++year: date -> @da
pub fn year(a: bool, y: u64, m: u64, d: u64, h: u64, min: u64, s: u64, f: &[u16]) -> u128 {
    let yer = if a {
        YEAR_OFFSET + y
    } else {
        // (sub 292.277.024.400 (dec y))
        YEAR_OFFSET - (y - 1)
    };

    let day_count = yawn(yer, m, d);

    yule(day_count, h, min, s, f)
}

pub fn yell(now: &ParsedAtom) -> Tarp {
    let sec_atom = rsh(6, 1, now);

    let raw = end(6, 1, now);

    let mut fan = Vec::new();
    let mut muc = 4;
    let mut current_raw = raw.clone();

    while muc > 0 && !current_raw.is_zero() {
        muc -= 1;
        let digit_atom = cut(4, muc, 1, &current_raw);
        let digit:  u64 = match &digit_atom {
            ParsedAtom::Small(x) => *x as u64,
            ParsedAtom::Big(b) => b.clone().try_into().unwrap_or(0),
        };
        fan.push(digit);

        current_raw = end(4, muc, &current_raw);
    }

    let sec_u64:  u64 = match &sec_atom {
        ParsedAtom::Small(x) => *x as u64,
        ParsedAtom::Big(b) => b.clone().try_into().expect("yell: sec too large"),
    };

    let day = (sec_u64 / DAY) as u64;
    let sec = (sec_u64 % DAY) as u64;
    let hor = (sec / HOR) as u64;
    let sec = (sec % HOR) as u64;
    let mit = (sec / MIT) as u64;
    let sec = (sec % MIT) as u64;

    Tarp {
        d: day,
        h: hor,
        m: mit,
        s: sec,
        f: fan,
    }
}

pub fn yore(now: &ParsedAtom) -> Date {
    let rip: Tarp = yell(now);
    let (y_ger, m_ger, d_ger) = yall(rip.d);

    const PIVOT: u64 = 292_277_024_400;

    let (era, y_out) = if y_ger > PIVOT {
        (true, y_ger - PIVOT)
    } else {
        (false, PIVOT - y_ger)
    };

    Date {
        era,
        y: y_out,
        m: m_ger,
        t: Tarp {
            d: d_ger,
            h: rip.h,
            m: rip.m,
            s: rip.s,
            f: rip.f,
        },
    }
}

pub fn yall(day: u64) -> (u64, u64, u64) {
    let mut day = day;
    let mut era = 0;
    let mut cet = 0;
    let mut lep = false;

    era = day / ERA;
    day %= ERA;

    if day < CETY + 1 {
        lep = true;
        cet = 0;
    } else {
        lep = false;
        day = day - (CETY + 1);
        cet = 1 + (day / CETY);
        day %= CETY;
    }

    let mut yer = 400 * era + 100 * cet;

    loop {
        let dis = if lep { 366 } else { 365 };
        if day < dis {
            break;
        }
        let ner = yer + 1;
        day = day - dis;
        lep = (ner & 3) == 0; // faster than atom ops
        yer = ner;
    }

    let cah = if lep { &MOY } else { &MOH };
    let mut mot = 0;
    loop {
        let zis = cah[mot as usize];
        if day < zis {
            return (yer, mot + 1, day + 1); // 1-based month/day
        }
        day -= zis;
        mot += 1;
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && (year % 100 != 0 || year % 400 == 0)
}

pub fn yule(d: u64, h: u64, m: u64, s: u64, f: &[u16]) -> u128 {
    let sec = d * DAY + h * HOR + m * MIT + s;

    let mut fac: u64 = 0;
    let mut muc = 4i32; // starts at 4
    for &val in f.iter().take(4) {
        muc -= 1; // decrement *before* shift
        fac += (val as u64) << (muc as u32 * 16);
    }

    ((sec as u128) << 64) | (fac as u128)
}


const YEAR_OFFSET: u64 = 292_277_024_400;

fn yelp(yer: u64) -> bool {
    (yer % 4 == 0) && ((yer % 100 != 0) || (yer % 400 == 0))
}

// Constants from ++yo
const CETY: u64 = 36_524;   // days in 100 years (non-leap century)
const DAY: u64 = 86_400;    // seconds/day
const ERA: u64 = 146_097;   // days in 400 years
const HOR: u64 = 3_600;     // seconds/hour
const MIT: u64 = 60;        // seconds/minute
const MOH: [u64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; // normal
const MOY: [u64; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]; // leap

// ++yawn: days since "Jesus" (proleptic Gregorian)
fn yawn(mut yer: u64, mut mot: u64, mut day: u64) -> u64 {
    // => .(mot (dec mot), day (dec day))
    mot = mot.saturating_sub(1);
    day = day.saturating_sub(1);

    let cah = if yelp(yer) { &MOY } else { &MOH };
    for i in 0..mot as usize {
        day += cah[i];
    }

    loop {
        if yer % 4 != 0 {
            if yer == 0 { break; }
            yer -= 1;
            day += if yelp(yer) { 366 } else { 365 };
            continue;
        }
        if yer % 100 != 0 {
            if yer < 4 { break; }
            yer -= 4;
            day += if yelp(yer) { 1_461 } else { 1_460 };
            continue;
        }
        if yer % 400 != 0 {
            if yer < 100 { break; }
            yer -= 100;
            day += if yelp(yer) { 36_525 } else { 36_524 };
            continue;
        }
        // divisible by 400
        day += (yer / 400) * (1 + 4 * CETY); // 1 + 4*36524 = 146097 = ERA
        break;
    }
    day
}

// @s numbers
pub fn apply_sign(a: bool, b: ParsedAtom) -> ParsedAtom {
    match b {
        ParsedAtom::Small(n) => {
            let out = if a {
                2 * n
            } else if n == 0 {
                0
            } else {
                2 * (n - 1) + 1
            };
            ParsedAtom::Small(out)
        }
        ParsedAtom::Big(n) => {
            let out = if a {
                &n << 1
            } else if n.is_zero() {
                num_bigint::BigUint::from(0u32)
            } else {
                ((&n - 1u32) << 1) + 1u32
            };
            ParsedAtom::Big(out)
        }
    }
}

// ++  si                                                  ::  signed integer
pub fn syn_si(a: u128) -> bool {
    end_u128(0, 1, a) == 0
}

pub fn abs_si(a: u128) -> u128 {
    let rsh_res = rsh_u128(0, 1, a);
    let end_res = end_u128(0, 1, a.clone());
    end_res + rsh_res
}

pub fn old_si(a: u128) -> (bool, u128) {
    (syn_si(a), abs_si(a))
}
pub fn new_si(sign: bool, mag: u128) -> u128 {
    if mag == 0 { 0 } else if sign { mag << 1 } else { (mag << 1) - 1 }
}
fn sun_si(a: u128) -> u128 {
    a << 1
}

pub fn sum_si(a: u128, b: u128) -> u128 {
    let (c_sign, c_mag) = old_si(a);
    let (d_sign, d_mag) = old_si(b);
    match (c_sign, d_sign) {
        (false, false) => new_si(false, c_mag.wrapping_add(d_mag)),
        (false, true) => {
            if c_mag >= d_mag { new_si(false, c_mag - d_mag) }
            else { new_si(true, d_mag - c_mag) }
        },
        (true, false) => {
            if c_mag >= d_mag { new_si(true, c_mag - d_mag) }
            else { new_si(false, d_mag - c_mag) }
        },
        (true, true) => new_si(true, c_mag.wrapping_add(d_mag)),
    }
}

pub fn dif_si(a: u128, b: u128) -> u128 {
    let (b_sign, b_mag) = old_si(b);
    let neg_b = new_si(!b_sign, b_mag);
    sum_si(a, neg_b)
}

pub fn me(b: u128, p: u128) -> u128 {
    let t = dif_si(2, b);
    let p_si = sun_si(p);
    dif_si(t, p_si)
}

pub fn sig(p: usize, w: usize, a: &ParsedAtom) -> bool {
    let bit = cut(0, p + w, 1, a);
    match bit {
        ParsedAtom::Small(0) => true,
        ParsedAtom::Small(1) => false,
        _ => unreachable!(),
    }
}

// @r aux functions

pub fn bif(a: BinaryFloat, w: u128, p: u128, b: u128, r: char) -> ParsedAtom {
    match a {
        BinaryFloat::Infinity { sign } => {
            let fill_val = fil(0, w as u32, 1);
            let q = lsh(0, p as usize, &fill_val);
            if sign {
                q
            } else {
                let q_u128 = q.to_u128()
                            .expect("float bigger than 128 bits");
                ParsedAtom::Small(q_u128.wrapping_add(bex(w + p)))
            }
        }

        BinaryFloat::NaN => {
            let fill_val = fil(0, (w + 1) as u32, 1);
            let shift = sub_or_panic(p, 1) as usize;
            if shift >= 128 { panic!("bif: shift too large"); }
            lsh(0, shift, &fill_val)
        }

        BinaryFloat::Finite { sign, exp: e, mant: a_a } => {
            if a_a.is_zero() {
                return if sign {
                    ParsedAtom::Small(0)
                } else {
                    ParsedAtom::Small(bex(w + p))
                };
            }

            let ma = met_big(0, &a_a) as u128;

            if ma != p + 1 {
                assert!(e == dif_si(dif_si(2, b), sun_si(p)), "bif: subnormal exponent != me");
                assert!(ma < p + 1, "bif: subnormal mantissa too large");

                let a_small = if a_a.bits() > 128 {
                    panic!("bif: mantissa too large for Small");
                } else {
                    a_a.to_u128().unwrap()
                };

                return if sign {
                    ParsedAtom::Small(a_small)
                } else {
                    ParsedAtom::Small(a_small.wrapping_add(bex(w + p)))
                };
            }

            let diff = dif_si(e, dif_si(dif_si(2, b), sun_si(p)));
            let q = sum_si(diff, 2);

            let abs_q = abs_si(q);
            let shifted = (abs_q as u128) << p;
            let a_small = if a_a.bits() > 128 {
                panic!("bif: mantissa too large");
            } else {
                a_a.to_u128().unwrap()
            };
            let low_p = a_small & ((1u128 << p) - 1);
            let r = shifted.wrapping_add(low_p);

            if sign {
                ParsedAtom::Small(r)
            } else {
                ParsedAtom::Small(r.wrapping_add(bex(w + p)))
            }
        }
    }
}

pub fn grd_fl(a: DecimalFloat, b: u128, p: u128, w: u128, mut r: char) -> BinaryFloat {

    //  +pa:ff arm will set these configs before calling +grd:fl
    let v = me(b, p);
    let p = p + 1;
    let w = bex(w) - 3;
    let d = 'd';

    match a {
        DecimalFloat::NaN => BinaryFloat::NaN,
        DecimalFloat::Infinity { sign } => BinaryFloat::Infinity { sign },
        DecimalFloat::Finite { sign, exp: e, mant } => {
            r = 'n';
            let q = abs_si(e);
            let pow5 = pow(5, q);

            let left = BinaryFloat::Finite {
                sign,
                exp: 0,
                mant: BigUint::from(mant),
            };
            if syn_si(e) {
                let right = BinaryFloat::Finite {
                    sign: true,
                    exp: e,
                    mant: pow5,
                };
                binaryfloat_mul(left, right, p, v, w, r, d)
            } else {
                let divisor = BinaryFloat::Finite {
                    sign: true,
                    exp: sun_si(q),
                    mant: pow5,
                };
                binaryfloat_div(left.clone(), divisor.clone(), p, v, w, r, d)
            }
        }
    }
}

//  finish parsing @rh
//  rylh -> grd:rh -> grd:ff -> grd:fl
pub fn rylh(a: DecimalFloat) -> ParsedAtom {
    let w = 5;
    let p = 10;
    let b = 30; // --15
    let r = 'z';
    let grd_res = grd_fl(a, b,  p, w, r);
    bif(grd_res, w, p, b, r)
}

//  prep @rh for print
pub fn rlyh(a: u128) -> DecimalFloat {
    let w = 5;
    let p = 10;
    let b = 30; // --15
    let r = 'z';
    let sea_res = sea(w, p, b, &ParsedAtom::Small(a));
    drg_fl(sea_res, p, w, b)
}

//  finish parsing @rq
pub fn rylq(a: DecimalFloat) -> ParsedAtom {
    let w = 15;
    let p = 112;
    let b = 32766; // --16.383
    let r = 'z';
    let grd_res = grd_fl(a, b,  p, w, r);
    bif(grd_res, w, p, b, r)
}

//  prep @rq for print
pub fn rlyq(a: u128) -> DecimalFloat {
    let w = 15;
    let p = 112;
    let b = 32766; // --16.383
    let r = 'z';
    let sea_res = sea(w, p, b, &ParsedAtom::Small(a));
    drg_fl(sea_res, p, w, b)
}

//  finish parsing @rd
pub fn ryld(a: DecimalFloat) -> ParsedAtom {
    let w = 11;
    let p = 52;
    let b = 2046; // --1.023
    let r = 'z';
    let grd_res = grd_fl(a, b,  p, w, r);
    bif(grd_res, w, p, b, r)
}

//  prep @rd for print
pub fn rlyd(a: u128) -> DecimalFloat {
    let w = 11;
    let p = 52;
    let b = 2046; // --1.023
    let r = 'z';
    let sea_res = sea(w, p, b, &ParsedAtom::Small(a));
    drg_fl(sea_res, p, w, b)
}

//  finish parsing @rs
pub fn ryls(a: DecimalFloat) -> ParsedAtom {
    let w = 8;
    let p = 23;
    let b = 254; // --127
    let r = 'z';
    let grd_res = grd_fl(a, b,  p, w, r);
    bif(grd_res, w, p, b, r)
}

// prep @rs for print
pub fn rlys(a: u128) -> DecimalFloat {
    let w = 8;
    let p = 23;
    let b = 254; // --127
    let r = 'z';
    let sea_res = sea(w, p, b, &ParsedAtom::Small(a));
    drg_fl(sea_res, p, w, b)
}


fn prc(p: u128) -> u128 {
    assert!(p > 1, "precision should be >= 2");
    p
}

fn lug(mode: LugMode, mut e: u128, mut a: BigUint, s: bool, p: u128, v: u128, w: u128, r: char, d: char) -> BinaryFloat {

    use BinaryFloat::*;
    use LugMode::*;

    if a == BigUint::zero() { panic!("lug: mantissa zero"); }

    let m = met(0, &ParsedAtom::Big(a.clone())) as u128;
    let prc_res = prc(p);
    assert!(s | (m > prc_res), "lug: stick bit is false or precision is invalid");

    let max_p = if m > prc_res {
        sub_or_panic(m as u128, prc_res)
    } else {
        0
    };

    let max_q = {
        let abs_arg = if d == 'i' {
            0
        } else if cmp_si(e, v) == 1  {
            dif_si(v, e)
        } else {
            0
        };
        abs_si(abs_arg)
    };

    let q = max_p.max(max_q);

    let b = end_big(0, q as usize, &a).to_u128().expect("value too large for u128");

    a = rsh(0, q as usize, &ParsedAtom::Big(a)).to_biguint();

    e = sum_si(e, sun_si(q));

    if a == BigUint::zero() {
        assert!(d != 'i', "lug: d == %i");
        return match mode {
            Floor | Smaller => Finite { sign: true, exp: 0, mant: BigUint::zero() },
            Ceiling | Larger => {
                Finite { sign: true, exp: v, mant: BigUint::one() }
            },
            Nearest | NearestTowards => {
                let half = bex(q.saturating_sub(1));
                if s {
                    if b <= half {
                       return Finite { sign: true, exp: 0, mant: BigUint::zero() };
                    }
                    return Finite { sign: true, exp: v, mant: BigUint::one() };
                }
                if b < half {
                    return Finite { sign: true, exp: 0, mant: BigUint::zero() };
                }
                return Finite { sign: true, exp: v, mant: BigUint::one() };
            },
            NearestAway => {
                let half = bex(q.saturating_sub(1));
                if b < half {
                    return Finite { sign: true, exp: 0, mant: BigUint::zero() };
                }
                return Finite { sign: true, exp: v, mant: BigUint::one() };
            }
        };
    }

    (e, a) = xpd(e, a, d, p, v);

    match mode {
        Floor => { /* no change */ }
        Larger => a = a + BigUint::one(),
        Smaller => {
            if b == 0 && s {
                if e == v && d != 'i' {
                    a = sub_or_panic_big(&a, &BigUint::one());
                } else {
                    let y = sub_or_panic_big(&(a.clone() * BigUint::from(2 as u128)), &BigUint::one());
                    if met_big(0, &y) as u128 <= prc_res {
                        a = y;
                        e = dif_si(e, 2);
                    } else {
                        a = sub_or_panic_big(&a, &BigUint::one());
                    }
                }
            }
        },
        Ceiling => { if !(b == 0 && !s) { a = a + BigUint::one(); } },
        Nearest => {
            if b != 0 {
                let y = bex(sub_or_panic(q, 1));
                if b == y && s {
                    if dis_big(&a, &BigUint::one()) != BigUint::zero() {
                        a = a + BigUint::one();
                    }
                } else if b < y {
                } else {
                    a = a + BigUint::one();
                }
            }
        }
        NearestAway => {
            if b != 0 {
                let y = bex(sub_or_panic(q, 1));
                if !(b < y) {
                    a = a + BigUint::one();
                }
            }
        }
        NearestTowards => {
            if b != 0 {
                let y = bex(sub_or_panic(q, 1));
                if b == y {
                    if !s {
                        a = a + BigUint::one();
                    }
                }
                if !(b < y) {
                    a = a + BigUint::one();
                }
            }
        }
    };

    (e, a) = if (met_big(0, &a.clone()) as u128) != (prc_res + 1) {
        (e, a)
    } else {
        a = rsh(0, 1, &ParsedAtom::Big(a)).to_u128().expect("lug: cast failled").into();
        e = sum_si(e, 2);
        (e, a)
    };

    if a == BigUint::zero() {
        return Finite { sign: true, exp: 0, mant: BigUint::zero() };
    }

    let res = if d == 'i' {
        Finite { sign: true, exp: e, mant: BigUint::from(a) }
    } else if cmp_si(emx(v, w), e) == 1 {
        Infinity { sign: true }
    } else {
        Finite { sign: true, exp: e, mant: BigUint::from(a) }
    };

    if !(d == 'f') {
        return res;
    }

    match res {
        Finite { sign, exp, ref mant } => {
            if  met_big(0, &mant.clone()) as u128 == prc(p) {
               return Finite { sign: true, exp: 0, mant: BigUint::zero() };
            }
            res
        },
        _ => res,
    }
}

fn emx(v: u128, w: u128) -> u128 {
    sum_si(v, sun_si(w))
}

fn rou(e: u128, a: BigUint, p: u128, v: u128, w: u128, r: char, d: char) -> BinaryFloat {
    rau(e, a, true, p, v, w, r, d)
}

pub fn binaryfloat_mul_internal(a_e: u128, a_a: BigUint, b_e: u128, b_a: BigUint, p: u128, v: u128, w: u128, r: char, d: char) -> BinaryFloat {
    let e = sum_si(a_e, b_e);
    let a = a_a * b_a;
    rou(e, a, p, v, w, r, d)
}

pub fn binaryfloat_div_internal(
    a_e: u128,
    a_a: BigUint,
    b_e: u128,
    b_a: BigUint,
    p: u128,
    v_min: u128,
    w: u128,
    r: char,
    d: char,
) -> BinaryFloat {
    let ma = met_big(0, &a_a) as u128;
    let mb = met_big(0, &b_a) as u128;

    let rhs = sun_si(mb + prc(p) + 1);
    let v = dif_si(sun_si(ma), rhs);

    let (a_e_shifted, a_a_shifted) = if syn_si(v) {
        (a_e, a_a)
    } else {
        let shift = abs_si(v) as usize;
        let new_e = sum_si(v, a_e);
        let new_a = lsh(0, shift, &ParsedAtom::Big(a_a.clone())).to_biguint();
        (new_e, new_a)
    };

    let j = dif_si(a_e_shifted, b_e);
    let (quot, rem) = dvr_big(&a_a_shifted, &b_a);

    rau(j, quot, rem.is_zero(), p, v_min, w, r, d)
}

fn xpd(e: u128, a: BigUint, d: char, p: u128, v: u128) -> (u128, BigUint) {
    let ma = met_big(0, &a.clone()) as u128;

    if ma >= prc(p) {
        return (e, a);
    }

    let shift = if d == 'i' {
        sub_or_panic(prc(p), ma as u128)
    } else {
        let w = dif_si(e, v);
        let q = if syn_si(w) { abs_si(w) } else { 0 };
        let needed = sub_or_panic(prc(p), ma as u128);
        q.min(needed)
    };

    let e_new = dif_si(e, sun_si(shift));
    let a_new = lsh_big(0, shift as usize, &a);

    (e_new, a_new)
}

pub fn binaryfloat_mul(a: BinaryFloat, b: BinaryFloat, p: u128, v: u128, w: u128, mut r: char, d: char) -> BinaryFloat {
    use BinaryFloat::*;

    if matches!(a, NaN) || matches!(b, NaN) {
        return NaN;
    }

    if let Infinity { sign: sa } = a {
        if let Infinity { sign: sb } = b {
            return Infinity { sign: sa == sb };
        }

        let b_mant = if let Finite { ref mant, .. } = b { mant.clone() } else { BigUint::zero() };
        if b_mant == BigUint::zero() {
            return NaN;
        }
        return Infinity { sign: sa == b.sign() };
    }

    if let Infinity { sign: sb } = b {
        let a_mant = if let Finite { ref mant, .. } = a { mant.clone() } else { BigUint::zero() };
        if a_mant == BigUint::zero() {
            return NaN;
        }
        return Infinity { sign: a.sign() == sb };
    }

    let (sa, ea, ma) = if let Finite { sign, exp, mant } = a { (sign, exp, mant) } else { (false, 0, BigUint::zero()) };
    let (sb, eb, mb) = if let Finite { sign, exp, mant } = b { (sign, exp, mant) } else { (false, 0, BigUint::zero()) };

    if ma == BigUint::zero() || mb == BigUint::zero() {
        return Finite {
            sign: sa == sb, // =(s.a s.b)
            exp: 0, // zer = [e=--0 a=0]
            mant:  BigUint::zero(),
        };
    }

    if ma == BigUint::zero() || mb == BigUint::zero() {
        return binaryfloat_mul_internal(ea, ma, eb, mb, p, v, w, r, d);
    }
    r = swr(r);
    fli(binaryfloat_mul_internal(ea, ma, eb, mb, p, v, w, r, d))
}

pub fn binaryfloat_div(a: BinaryFloat, b: BinaryFloat, p: u128, v: u128, w: u128, mut r: char, d: char) -> BinaryFloat {
    use BinaryFloat::*;

    if matches!(a, NaN) || matches!(b, NaN) {
        return NaN;
    }

    if let Infinity { sign: sa } = a {
        if let Infinity { sign: sb } = b {
            return NaN;
        }
        return Infinity { sign: sa == b.sign() };
    }

    if let Infinity { sign: sb } = b {
        return Finite {
            sign: a.sign() == sb,
            exp: 0,     // zer = [e=--0 a=0]
            mant: BigUint::zero(),
        };
    }

    let (sa, ea, ma) = if let Finite { sign, exp, mant } = a { (sign, exp, mant) } else { (false, 0, BigUint::zero()) };
    let (sb, eb, mb) = if let Finite { sign, exp, mant } = b { (sign, exp, mant) } else { (false, 0, BigUint::zero()) };

    if ma == BigUint::zero() {
        if mb == BigUint::zero() {
            return NaN;
        }
        return Finite {
            sign: sa == sb,
            exp: 0,
            mant: BigUint::zero(),
        };
    }

    if mb == BigUint::zero() {
        return Infinity { sign: sa == sb };
    }

    if sa == sb {
        return binaryfloat_div_internal(ea, ma, eb, mb, p, v, w, r, d);
    }
    r = swr(r);
    fli(binaryfloat_div_internal(ea, ma, eb, mb, p, v, w, r, d))
}


#[derive(Debug, Clone, Copy)]
enum LugMode {
    Floor,     // %fl
    Ceiling,   // %ce
    Smaller,   // %sm
    Larger,    // %lg
    Nearest,   // %ne  (ties to even)
    NearestAway,
    NearestTowards,
}


pub fn sea(w: u128, p: u128, b: u128, a: &ParsedAtom) -> BinaryFloat {
    let f = cut(0, 0, p as usize, a);
    let e_atom = cut(0, p as usize, w as usize, a);
    let s = sig(p as usize, w as usize, a);

    let e = match e_atom {
        ParsedAtom::Small(x) => x,
        ParsedAtom::Big(_) => panic!("exponent field >128 bits"),
    };
    let f_u128 = match f {
        ParsedAtom::Small(x) => x,
        ParsedAtom::Big(_) => panic!("mantissa field >128 bits"),
    };

    let max_exp_field = sub_or_panic(bex(w), 1); // bex(w) >= 1

    if e == 0 {
        if f_u128 == 0 {
            BinaryFloat::Finite { sign: s, exp: 0, mant: BigUint::zero() }
        } else {
            let me_val = me(b, p);
            BinaryFloat::Finite { sign: s, exp: me_val, mant: BigUint::from(f_u128) }
        }
    } else if e == max_exp_field {
        if f_u128 == 0 {
            BinaryFloat::Infinity { sign: s }
        } else {
            BinaryFloat::NaN
        }
    } else {
        let me_val = me(b, p);
        let q = sum_si(sum_si(sun_si(e), me_val), 1); // e + me + (-1)

        let r = f_u128.wrapping_add(bex(p));

        BinaryFloat::Finite { sign: s, exp: q, mant: BigUint::from(r) }
    }
}

//  inner function for drg_fl
pub fn drg(
    e: u128,
    a: BigUint,
    p: u128,
    v: u128,
    w: u128,
    d: char,
) -> (u128, BigUint) {
    assert!(!a.is_zero(), "drg: mantissa must be nonzero");
    println!("drg caleed e {} a {} p {} v {} w {} d {}", e, a, p, v, w, d);
    // drg caleed e 43 a 13176795 p 24 v 299 w 253 d d
    //  it should return (13, 31.415.927)
    //  but it returns 0 and 13176795

    let (e, a) = xpd(e, a, d, p, v);
    println!("xpd result: e:{} a:{}", e, a);
    assert!(!a.is_zero(), "xpd must not produce zero in drg");

    let (mut r, mut s, mut mn, mut mp) = {
        if syn_si(e) {
            let shift = abs_si(e) as usize;
            let r = lsh_big(0, shift, &a.clone());
            let s = BigUint::one();
            let mn = BigUint::one();
            let mp = BigUint::one();
            (r, s, mn, mp)
        } else {
            let shift = abs_si(e) as usize;
            let s = lsh_big(0, shift, &BigUint::one());
            let r = a.clone();
            let mn = BigUint::one();
            let mp = BigUint::one();
            (r, s, mn, mp)
        }
    };

    println!("r: {} s: {} mn: {} mp: {}", r, s, mn, mp);

    let a_orig = BigUint::from(1u128) << sub_or_panic(prc(p), 1); // 2^(p-1)
    let halfway = a == a_orig;
    let cond2 = e != v || d == 'i';
    if halfway && cond2 {
        r = lsh_big(0, 1, &r);
        s = lsh_big(0, 1, &s);
        mp = lsh_big(0, 1, &mp);
    }

    let mut k = 0u128; // --0 = 0 (@s zero)
    let ten = BigUint::from(10u32);
    let nine = BigUint::from(9u32);
    let q = (&s + &nine) / &ten;
    loop {
        if r >= q {
            break;
        }
        k = dif_si(k, 2);
        r *= &ten;
        mn *= &ten;
        mp *= &ten;
    }
    loop {
        let two_r = &r * 2u32;
        let left = &two_r + &mp;
        let right = &s * 2u32;
        if left < right {
            break;
        }
        s *= &ten;
        k = sum_si(k, 2);
    }

    let mut o = BigUint::zero();
    let mut u = BigUint::zero();

    loop {
        let (u_big, rem) = dvr_big(&(&r * &ten), &s);

        k = dif_si(k, 2);

        u = (u_big.to_u64().expect("digit ≥10") as u32).into();

        r = rem;
        mn *= &ten;
        mp *= &ten;

        let l = &r * 2u32 < mn;

        let two_s = &s * 2u32;
        let h = two_s < mp || (&r * 2u32 > sub_or_panic_big(&two_s, &mp));

        if !l && !h {
            o = o * &ten + u;
            continue;
        }

        let q = h && (!l || &r * 2u32 > s);
        let digit = if q { u + BigUint::one() } else { u };
        o = o * &ten + digit;
        break;
    }
    println!("drg returning {} {}", k, o);
    (k, o)
}

//  @rs to decimal float.
pub fn drg_fl(
    a: BinaryFloat,
    p: u128,
    w: u128,
    b: u128,
) -> DecimalFloat {
    match a {
        BinaryFloat::Finite { sign, exp, mant } => {
            if mant.is_zero() {
                DecimalFloat::Finite { sign, exp: 0, mant: BigUint::zero() }
            } else {
                let p = p + 1;
                let v = me(b, p);
                let w = bex(w) - 3;
                let d = 'd';
                let (k, digits) = drg(exp, mant, p, v, w, d);
                DecimalFloat::Finite { sign, exp: k, mant: digits }
            }
        }
        BinaryFloat::Infinity { sign } => DecimalFloat::Infinity { sign },
        BinaryFloat::NaN => DecimalFloat::NaN,
    }
}

// swr: swap rounding direction for negative numbers
pub fn swr(r: char) -> char {
    match r {
        'd' => 'u',
        'u' => 'd',
        _ => r,
    }
}

// fli: flip sign of BinaryFloat
pub fn fli(a: BinaryFloat) -> BinaryFloat {
    match a {
        BinaryFloat::Finite { sign, exp, mant } => BinaryFloat::Finite { sign: !sign, exp, mant },
        BinaryFloat::Infinity { sign } => BinaryFloat::Infinity { sign: !sign },
        BinaryFloat::NaN => BinaryFloat::NaN,
    }
}

// zer: zero float node
pub fn zer() -> BinaryFloat {
    BinaryFloat::Finite {
        sign: false,
        exp: 0, // si-encoding of 0 is 0
        mant: BigUint::from(0u8),
    }
}

fn rau(e: u128, a: BigUint, t: bool, p: u128, v: u128, w: u128, r: char, d: char) -> BinaryFloat {
    let mode = match r {
        'z' | 'd' => LugMode::Floor,
        'a' | 'u' => LugMode::Ceiling,
        'n'       => LugMode::Nearest,
        _         => LugMode::Nearest,
    };

    lug(mode, e, a, t, p, v, w, r, d)
}

pub fn cmp_si(a: u128, b: u128) -> u128 {
    if a == b {
        0
    } else if syn_si(a) {
        if syn_si(b) {
            if a > b { 2 } else { 1 }
        } else {
            2
        }
    } else if syn_si(b) {
        1
    } else {
        if a  >  b { 1 } else { 2 }
    }
}