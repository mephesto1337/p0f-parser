//lib.rs

// Struct
#[derive(Clone, Default, Debug)]
pub struct P0f {
    pub date: std::string::String,
    pub log: std::string::String,
}

// Parsers
pub mod parsers {
    use nom::error::ErrorKind;
    use nom::bytes::complete::*;
    use nom::combinator::{rest};
    use nom::character::complete::{char};
    use nom::sequence::{delimited,tuple};

    // parsing functions
    // syn+ack : [2020/04/17 11:39:16] mod=syn+ack|cli=192.168.0.34/35822|srv=173.194.76.189/443|subj=srv|os=???|dist=22|params=none|raw_sig=4:106+22:0:1430:mss*44,8:mss,sok,ts,nop,ws::0

     pub fn date_content(i: &str) -> nom::IResult<&str, &str> {
         delimited(char('['), is_not("]"), char(']'))(i)
     }

    pub fn unparsed(i: &str) -> nom::IResult<&str, &str> {
        //take_until("\n")(i)
        rest::<_,(_, ErrorKind)>(i)
    }

    pub fn parse_line(i: &str) -> nom::IResult<&str, (&str, &str)> {
        tuple((
            date_content,
            unparsed,
        ))(i)
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
