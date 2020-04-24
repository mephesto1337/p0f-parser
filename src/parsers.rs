use nom::bytes::complete::{is_not, tag};
use nom::character::complete::char;
use nom::combinator::map_res;
use nom::sequence::{delimited, preceded, tuple};

use crate::error::Error;
use crate::structs::{DateTime, P0f, P0fModule};

type IResult<I, O> = nom::IResult<I, O, Error<I>>;

fn parse_date(i: &str) -> IResult<&str, DateTime> {
    map_res(delimited(char('['), is_not("]"), char(']')), |s| {
        DateTime::parse_from_str(s, "%Y/%m/%d %H:%M:%S")
    })(i)
}

fn parse_mod(i: &str) -> IResult<&str, &str> {
    preceded(tag("mod="), is_not("|"))(i.trim())
}

fn parse_cli(i: &str) -> IResult<&str, &str> {
    preceded(tag("|cli="), is_not("|"))(i)
}

fn parse_srv(i: &str) -> IResult<&str, &str> {
    preceded(tag("|srv="), is_not("|"))(i)
}

fn parse_subj(i: &str) -> IResult<&str, &str> {
    preceded(tag("|subj="), is_not("|"))(i)
}

fn parse_os(i: &str) -> IResult<&str, &str> {
    preceded(tag("|os="), is_not("|"))(i)
}

fn parse_dist(i: &str) -> IResult<&str, &str> {
    preceded(tag("|dist="), is_not("|"))(i)
}

fn parse_params(i: &str) -> IResult<&str, &str> {
    preceded(tag("|params="), is_not("|"))(i)
}

fn parse_uptime(i: &str) -> IResult<&str, &str> {
    preceded(tag("|uptime="), is_not("|"))(i)
}

fn parse_link(i: &str) -> IResult<&str, &str> {
    preceded(tag("|link="), is_not("|"))(i)
}

fn parse_reason(i: &str) -> IResult<&str, &str> {
    preceded(tag("|reason="), is_not("|"))(i)
}

fn parse_app(i: &str) -> IResult<&str, &str> {
    preceded(tag("|app="), is_not("|"))(i)
}

fn parse_lang(i: &str) -> IResult<&str, &str> {
    preceded(tag("|lang="), is_not("|"))(i)
}

fn parse_raw_mtu(i: &str) -> IResult<&str, usize> {
    map_res(preceded(tag("|raw_mtu="), is_not("")), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_raw_freq(i: &str) -> IResult<&str, &str> {
    preceded(tag("|raw_freq="), is_not(""))(i)
}

fn parse_raw_sig(i: &str) -> IResult<&str, &str> {
    preceded(tag("|raw_sig="), is_not(""))(i)
}

fn parse_raw_hits(i: &str) -> IResult<&str, &str> {
    preceded(tag("|raw_hits="), is_not(""))(i)
}

fn parse_p0f_uptime(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, uptime, raw_freq)) = tuple((
        parse_cli,
        parse_srv,
        parse_subj,
        parse_uptime,
        parse_raw_freq,
    ))(i)?;

    Ok((
        rest,
        P0fModule::Uptime {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            uptime: String::from(uptime),
            raw_freq: String::from(raw_freq),
        },
    ))
}

pub fn parse_p0f_mtu(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, link, raw_mtu)) =
        tuple((parse_cli, parse_srv, parse_subj, parse_link, parse_raw_mtu))(i)?;

    Ok((
        rest,
        P0fModule::Mtu {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            link: String::from(link),
            raw_mtu,
        },
    ))
}

pub fn parse_p0f_syn(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, os, dist, params, raw_sig)) = tuple((
        parse_cli,
        parse_srv,
        parse_subj,
        parse_os,
        parse_dist,
        parse_params,
        parse_raw_sig,
    ))(i)?;

    Ok((
        rest,
        P0fModule::Syn {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            os: String::from(os),
            dist: String::from(dist),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

pub fn parse_p0f_synack(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, os, dist, params, raw_sig)) = tuple((
        parse_cli,
        parse_srv,
        parse_subj,
        parse_os,
        parse_dist,
        parse_params,
        parse_raw_sig,
    ))(i)?;

    Ok((
        rest,
        P0fModule::SynAck {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            os: String::from(os),
            dist: String::from(dist),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

pub fn parse_p0f_host_change(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, reason, raw_hits)) = tuple((
        parse_cli,
        parse_srv,
        parse_subj,
        parse_reason,
        parse_raw_hits,
    ))(i)?;

    Ok((
        rest,
        P0fModule::HostChange {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            reason: String::from(reason),
            raw_hits: String::from(raw_hits),
        },
    ))
}

pub fn parse_p0f_http_request(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, app, lang, params, raw_sig)) = tuple((
        parse_cli,
        parse_srv,
        parse_subj,
        parse_app,
        parse_lang,
        parse_params,
        parse_raw_sig,
    ))(i)?;

    Ok((
        rest,
        P0fModule::HttpResponse {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            app: String::from(app),
            lang: String::from(lang),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

pub fn parse_p0f_http_response(i: &str) -> IResult<&str, P0fModule> {
    let (rest, (client, server, subject, app, lang, params, raw_sig)) = tuple((
        parse_cli,
        parse_srv,
        parse_subj,
        parse_app,
        parse_lang,
        parse_params,
        parse_raw_sig,
    ))(i)?;

    Ok((
        rest,
        P0fModule::HttpResponse {
            client: String::from(client),
            server: String::from(server),
            subject: String::from(subject),
            app: String::from(app),
            lang: String::from(lang),
            params: String::from(params),
            raw_sig: String::from(raw_sig),
        },
    ))
}

pub fn parse_line(i: &str) -> IResult<&str, P0f> {
    let (rest, date) = parse_date(i)?;
    let (rest, module) = match parse_mod(rest)? {
        (remain, "uptime") => parse_p0f_uptime(remain)?,
        (remain, "mtu") => parse_p0f_mtu(remain)?,
        (remain, "syn") => parse_p0f_syn(remain)?,
        (remain, "syn+ack") => parse_p0f_synack(remain)?,
        (remain, "host change") => parse_p0f_host_change(remain)?,
        (remain, "http request") => parse_p0f_http_request(remain)?,
        (remain, "http response") => parse_p0f_http_response(remain)?,
        (remain, module) => (
            "",
            P0fModule::Unparsed {
                module: String::from(module),
                remain: String::from(remain),
            },
        ),
    };

    Ok((rest, P0f { date, module }))
}
