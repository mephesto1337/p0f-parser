//lib.rs
mod structs;
use structs::*;

// Parsers
pub mod parsers {
    use super::*;
    use nom::IResult;
    use nom::bytes::complete::*;
    use nom::combinator::*;
    use nom::character::complete::*;
    use nom::sequence::*;

    pub fn parse_date(i: &str) -> IResult<&str, &str> {
            delimited(char('['), is_not("]"), char(']'))(i)
    }

    pub fn parse_mod(i: &str) -> IResult<&str, &str> {
        preceded(tag("mod="), is_not("|")) (i.trim())
    }

    pub fn parse_cli(i: &str) -> IResult<&str, &str>{
        preceded(tag("|cli="), is_not("|")) (i)
    }

    pub fn parse_srv(i: &str) -> IResult<&str, &str>{
        preceded(tag("|srv="), is_not("|")) (i)
    }

    pub fn parse_subj(i: &str) -> IResult<&str, &str>{
        preceded(tag("|subj="), is_not("|")) (i)
    }

    pub fn parse_os(i: &str) -> IResult<&str, &str>{
        preceded(tag("|os="), is_not("|")) (i)
    }

    pub fn parse_dist(i: &str) -> IResult<&str, &str>{
        preceded(tag("|dist="), is_not("|")) (i)
    }

    pub fn parse_params(i: &str) -> IResult<&str, &str>{
        preceded(tag("|params="), is_not("|")) (i)
    }

    pub fn parse_uptime(i: &str) -> IResult<&str, &str>{
        preceded(tag("|uptime="), is_not("|")) (i)
    }

    pub fn parse_link(i: &str) -> IResult<&str, &str>{
        preceded(tag("|link="), is_not("|")) (i)
    }

    pub fn parse_reason(i: &str) -> IResult<&str, &str>{
        preceded(tag("|reason="), is_not("|")) (i)
    }

    pub fn parse_app(i: &str) -> IResult<&str, &str>{
        preceded(tag("|app="), is_not("|")) (i)
    }

    pub fn parse_lang(i: &str) -> IResult<&str, &str>{
        preceded(tag("|lang="), is_not("|")) (i)
    }

    pub fn parse_raw_mtu(i: &str) -> IResult<&str, &str>{
        preceded(tag("|raw_mtu="), is_not("")) (i)
    }

    pub fn parse_raw_freq(i: &str) -> IResult<&str, &str>{
        preceded(tag("|raw_freq="), is_not("")) (i)
    }

    pub fn parse_raw_sig(i: &str) -> IResult<&str, &str>{
        preceded(tag("|raw_sig="), is_not("")) (i)
    }

    pub fn parse_raw_hits(i: &str) -> IResult<&str, &str>{
        preceded(tag("|raw_hits="), is_not("")) (i)
    }

    pub fn unparsed(i: &str) -> IResult<&str, &str> {
        rest(i)
    }

    pub fn parse_common(i: &str) -> IResult<&str, (&str, &str)> {
        match tuple((
            parse_date,
            parse_mod
        )) (i) {
            Ok(( remain,(date, module) )) => Ok(( remain, ( date, module ) )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_uptime(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_uptime,
            parse_raw_freq
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, uptime, raw_freq) ))
                => Ok(("", P0f::Uptime{ date: String::from(date),
                                        module: String::from(module),
                                        client: String::from(client),
                                        server: String::from(server),
                                        subject: String::from(subject),
                                        uptime: String::from(uptime),
                                        raw_freq: String::from(raw_freq),
                                    } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_mtu(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_link,
            parse_raw_mtu
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, link, raw_mtu) ))
                => Ok(("", P0f::Mtu{ date: String::from(date),
                                        module: String::from(module),
                                        client: String::from(client),
                                        server: String::from(server),
                                        subject: String::from(subject),
                                        link: String::from(link),
                                        raw_mtu: String::from(raw_mtu),
                                    } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_syn(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_os,
            parse_dist,
            parse_params,
            parse_raw_sig
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, os, dist, params, raw_sig) ))
                => Ok(("", P0f::Syn{ date: String::from(date),
                                        module: String::from(module),
                                        client: String::from(client),
                                        server: String::from(server),
                                        subject: String::from(subject),
                                        os: String::from(os),
                                        dist: String::from(dist),
                                        params: String::from(params),
                                        raw_sig: String::from(raw_sig),
                                    } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_synack(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_os,
            parse_dist,
            parse_params,
            parse_raw_sig
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, os, dist, params, raw_sig) ))
                => Ok(("", P0f::SynAck{ date: String::from(date),
                                        module: String::from(module),
                                        client: String::from(client),
                                        server: String::from(server),
                                        subject: String::from(subject),
                                        os: String::from(os),
                                        dist: String::from(dist),
                                        params: String::from(params),
                                        raw_sig: String::from(raw_sig),
                                    } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_host_change(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_reason,
            parse_raw_hits
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, reason, raw_hits) ))
                => Ok(("", P0f::HostChange{ date: String::from(date),
                                            module: String::from(module),
                                            client: String::from(client),
                                            server: String::from(server),
                                            subject: String::from(subject),
                                            reason: String::from(reason),
                                            raw_hits: String::from(raw_hits),
                                        } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_http_request(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_app,
            parse_lang,
            parse_params,
            parse_raw_sig
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, app, lang, params, raw_sig) ))
                => Ok(("", P0f::HttpRequest{ date: String::from(date),
                                            module: String::from(module),
                                            client: String::from(client),
                                            server: String::from(server),
                                            subject: String::from(subject),
                                            app: String::from(app),
                                            lang: String::from(lang),
                                            params: String::from(params),
                                            raw_sig: String::from(raw_sig)
                                        } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_p0f_http_response(i: &str) -> IResult<&str, P0f> {
        match tuple((
            parse_date,
            parse_mod,
            parse_cli,
            parse_srv,
            parse_subj,
            parse_app,
            parse_lang,
            parse_params,
            parse_raw_sig
        )) (i) {
            Ok(( _remain, (date, module, client, server, subject, app, lang, params, raw_sig) ))
                => Ok(("", P0f::HttpResponse{ date: String::from(date),
                                            module: String::from(module),
                                            client: String::from(client),
                                            server: String::from(server),
                                            subject: String::from(subject),
                                            app: String::from(app),
                                            lang: String::from(lang),
                                            params: String::from(params),
                                            raw_sig: String::from(raw_sig)
                                        } )),
            Err(e) => Err(e)
        }
    }

    pub fn parse_line(i: &str) -> IResult<&str, P0f> {
        match parse_common(i) {
            Ok(( _remain, (_date, "uptime"))) => parse_p0f_uptime(i),
            Ok(( _remain, (_date, "mtu"))) => parse_p0f_mtu(i),
            Ok(( _remain, (_date, "syn"))) => parse_p0f_syn(i),
            Ok(( _remain, (_date, "syn+ack"))) => parse_p0f_synack(i),
            Ok(( _remain, (_date, "host change"))) => parse_p0f_host_change(i),
            Ok(( _remain, (_date, "http request"))) => parse_p0f_http_request(i),
            Ok(( _remain, (_date, "http response"))) => parse_p0f_http_response(i),
            Ok(( remain, (date, module))) => Ok(("", P0f::Unparsed{date: String::from(date),
                                                                   module: String::from(module),
                                                                   remain: String::from(remain)})),
            Err(e) => Err(e),
        }
    }
}
