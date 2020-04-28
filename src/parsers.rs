use std::net::IpAddr;
use std::time::Duration;

use nom::bytes::complete::{is_not, tag, take_until};
use nom::character::complete::{char, digit1, space0};
use nom::combinator::map_res;
use nom::sequence::{delimited, preceded, tuple};

use crate::error::Error;
use crate::structs::{DateTime, Endpoint, P0f, P0fModule, Subject, Uptime};

type IResult<I, O> = nom::IResult<I, O, Error<I>>;

fn parse_date(i: &str) -> IResult<&str, DateTime> {
    map_res(delimited(char('['), is_not("]"), char(']')), |s| {
        DateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S")
    })(i)
}

fn parse_mod(i: &str) -> IResult<&str, &str> {
    preceded(tag("mod="), is_not("|"))(i.trim())
}

fn parse_tag<'i>(i: &'i str, stag: &str) -> IResult<&'i str, &'i str> {
    let separator = "|";
    preceded(
        tuple((tag(separator), tag(stag), tag("="))),
        is_not(separator),
    )(i)
}

fn parse_cli(i: &str) -> IResult<&str, Endpoint> {
    let (rest, cli) = parse_tag(i, "cli")?;
    match parse_endpoint(cli) {
        Ok((_, ep)) => Ok((rest, ep)),
        Err(e) => Err(e),
    }
}

fn parse_srv(i: &str) -> IResult<&str, Endpoint> {
    let (rest, cli) = preceded(tag("|srv="), is_not("|"))(i)?;
    match parse_endpoint(cli) {
        Ok((_, ep)) => Ok((rest, ep)),
        Err(e) => Err(e),
    }
}

fn parse_subj(i: &str) -> IResult<&str, Subject> {
    let (rest, subj) = parse_tag(i, "subj")?;
    match subj {
        "cli" => Ok((rest, Subject::Client)),
        "srv" => Ok((rest, Subject::Server)),
        _ => Err(Error::make_nom_error(
            &i[6..],
            nom::error::ErrorKind::NoneOf,
        )),
    }
}

fn parse_os(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "os")
}

fn parse_dist(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "dist")
}

fn parse_params(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "params")
}

fn parse_uptime(i: &str) -> IResult<&str, Uptime> {
    let space_tag = |t: &'static str| tuple((space0, tag(t), space0));
    let mut duration = Duration::from_secs(0);

    let (rest, uptime) = parse_tag(i, "uptime")?;
    let (_, (days, _, hours, _, minutes, _, modulo, _)) = tuple((
        map_res(digit1, |s: &str| s.parse::<u64>()),
        space_tag("days"),
        map_res(digit1, |s: &str| s.parse::<u64>()),
        space_tag("hrs"),
        map_res(digit1, |s: &str| s.parse::<u64>()),
        space_tag("min (modulo"),
        map_res(digit1, |s: &str| s.parse::<u64>()),
        space_tag("days)"),
    ))(uptime)?;

    duration += Duration::from_secs(minutes * 60);
    duration += Duration::from_secs(hours * 3600);
    duration += Duration::from_secs(days * 86400);

    Ok((
        rest,
        Uptime {
            duration,
            modulo: Duration::from_secs(modulo * 86400),
        },
    ))
}

fn parse_link(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "link")
}

fn parse_reason(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "reason")
}

fn parse_app(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "app")
}

fn parse_lang(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "lang")
}

fn parse_raw_mtu(i: &str) -> IResult<&str, usize> {
    map_res(|i| parse_tag(i, "raw_mtu"), |s: &str| s.parse::<usize>())(i)
}

fn parse_raw_freq(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "raw_freq")
}

fn parse_raw_sig(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "raw_sig")
}

fn parse_raw_hits(i: &str) -> IResult<&str, &str> {
    parse_tag(i, "raw_hits")
}

fn parse_endpoint(i: &str) -> IResult<&str, Endpoint> {
    let (rest, (addr, _sep, port)) = tuple((
        map_res(take_until("/"), |s: &str| s.parse::<IpAddr>()),
        tag("/"),
        map_res(digit1, |s: &str| s.parse::<u16>()),
    ))(i)?;

    Ok((rest, Endpoint { addr, port }))
}

fn parse_p0f_uptime(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (uptime, raw_freq)) = tuple((parse_uptime, parse_raw_freq))(i)?;

    Ok((
        rest,
        P0fModule::Uptime {
            uptime,
            raw_freq: String::from(raw_freq),
        },
    ))
}

fn parse_p0f_mtu(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (link, raw_mtu)) = tuple((parse_link, parse_raw_mtu))(i)?;

    Ok((
        rest,
        P0fModule::Mtu {
            link: String::from(link),
            raw_mtu,
        },
    ))
}

fn parse_p0f_syn(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (os, dist, params, raw_sig)) =
        tuple((parse_os, parse_dist, parse_params, parse_raw_sig))(i)?;

    Ok((
        rest,
        P0fModule::Syn {
            os: String::from(os),
            dist: String::from(dist),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

fn parse_p0f_synack(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (os, dist, params, raw_sig)) =
        tuple((parse_os, parse_dist, parse_params, parse_raw_sig))(i)?;

    Ok((
        rest,
        P0fModule::SynAck {
            os: String::from(os),
            dist: String::from(dist),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

fn parse_p0f_host_change(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (reason, raw_hits)) = tuple((parse_reason, parse_raw_hits))(i)?;

    Ok((
        rest,
        P0fModule::HostChange {
            reason: String::from(reason),
            raw_hits: String::from(raw_hits),
        },
    ))
}

fn parse_p0f_http_request(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (app, lang, params, raw_sig)) =
        tuple((parse_app, parse_lang, parse_params, parse_raw_sig))(i)?;

    Ok((
        rest,
        P0fModule::HttpResponse {
            app: String::from(app),
            lang: String::from(lang),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

fn parse_p0f_http_response(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (app, lang, params, raw_sig)) =
        tuple((parse_app, parse_lang, parse_params, parse_raw_sig))(i)?;

    Ok((
        rest,
        P0fModule::HttpResponse {
            app: String::from(app),
            lang: String::from(lang),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

pub fn parse_line(i: &str) -> IResult<&str, P0f> {
    let (remain, (date, module, client, server, subject)) =
        tuple((parse_date, parse_mod, parse_cli, parse_srv, parse_subj))(i)?;

    let (rest, module) = match module {
        "uptime" => parse_p0f_uptime(remain)?,
        "mtu" => parse_p0f_mtu(remain)?,
        "syn" => parse_p0f_syn(remain)?,
        "syn+ack" => parse_p0f_synack(remain)?,
        "host change" => parse_p0f_host_change(remain)?,
        "http request" => parse_p0f_http_request(remain)?,
        "http response" => parse_p0f_http_response(remain)?,
        _ => {
            return Err(Error::make_nom_error(remain, nom::error::ErrorKind::NoneOf));
        }
    };

    Ok((
        rest,
        P0f {
            date,
            client,
            server,
            subject,
            module,
        },
    ))
}
