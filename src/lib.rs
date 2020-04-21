//lib.rs

// Struct
#[derive(Clone, Default, Debug)]
pub struct P0f {
    pub date: String,
    pub module: String,
    pub unparsed: String,
}

// Parsers
pub mod parsers {
    use super::*;
    use nom::error::ErrorKind;
    use nom::bytes::complete::{is_not,tag,take_until};
    use nom::branch::{alt};
    use nom::combinator::{rest,all_consuming};
    use nom::character::complete::{char,alpha1};
    use nom::sequence::{preceded,delimited,tuple};

    // parsing functions
    // syn+ack : [2020/04/17 11:39:16] mod=syn+ack|cli=192.168.0.34/35822|srv=173.194.76.189/443|subj=srv|os=???|dist=22|params=none|raw_sig=4:106+22:0:1430:mss*44,8:mss,sok,ts,nop,ws::0

    pub fn date_content(i: &str) -> nom::IResult<&str, &str> {
        delimited(char('['), is_not("]"), char(']'))(i)
    }

    // pub fn kv(i: &str) -> nom::IResult<&str, (&str, &str)> {
    //     tuple((
    //         tag(alpha1),
    //         tag("=")
    //     ))(i)
    // }

    pub fn parse_syn(i: &str) -> nom::IResult<&str, &str> {
            tag("syn")(i)
        }

    pub fn parse_synack(i: &str) -> nom::IResult<&str, &str> {
        tag("syn+ack")(i)
    }

    pub fn parse_mtu(i: &str) -> nom::IResult<&str, &str> {
        tag("mtu")(i)
    }

    pub fn parse_uptime(i: &str) -> nom::IResult<&str, &str> {
        tag("uptime")(i)
    }

    pub fn parse_host_change(i: &str) -> nom::IResult<&str, &str> {
        tag("host change")(i)
    }

    pub fn parse_http_request(i: &str) -> nom::IResult<&str, &str> {
        tag("http request")(i)
    }

    pub fn parse_http_response(i: &str) -> nom::IResult<&str, &str> {
        tag("http response")(i)
    }

    pub fn module(i: &str) -> nom::IResult<&str, &str> {
        preceded(tag("mod="), alt(( parse_syn,
                                    parse_synack,
                                    parse_mtu,
                                    parse_uptime,
                                    parse_host_change,
                                    parse_http_request,
                                    parse_http_response
                                )))(i.trim())
    }

    pub fn unparsed(i: &str) -> nom::IResult<&str, &str> {
        rest::<_,(_, ErrorKind)>(i)
    }

    pub fn parse_line(i: &str) -> nom::IResult<&str, P0f> {
        match all_consuming(
            tuple((
                date_content,
                module,
                unparsed,
            ))) (i) { Ok(( remaining_input, (
                           date,
                           module,
                           unparsed,
            ))) => { Ok(( remaining_input,
                           P0f {
                               date: date.to_string(),
                               module: module.to_string(),
                               unparsed: unparsed.to_string(),
                           }))
            }
            Err(e) => Err(e)
        }
    }

    // tests
    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
         fn test_date_content() {
            assert_eq!(date_content("[2020/04/17 11:39:16] mod=syn+ack"), Ok((" mod=syn+ack", "2020/04/17 11:39:16")))
         }

        #[test]
        fn test_unparsed() {
            assert_eq!(unparsed(" mod=syn+ack|cli=192.168.0.34/35822\n"), Ok(("\n", (" mod=syn+ack|cli=192.168.0.34/35822"))))
        }
    }
}
