use std::net::IpAddr;

pub type DateTime = chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Endpoint {
    pub addr: IpAddr,
    pub port: u16,
}

#[derive(Debug)]
pub enum Subject {
    Client,
    Server,
}

#[derive(Debug)]
pub enum P0fModule {
    Uptime {
        uptime: String,
        raw_freq: String,
    },
    Mtu {
        link: String,
        raw_mtu: usize,
    },
    Syn {
        os: String,
        dist: String,
        params: String,
        raw_sig: String,
    },
    SynAck {
        os: String,
        dist: String,
        params: String,
        raw_sig: String,
    },
    HostChange {
        reason: String,
        raw_hits: String,
    },
    HttpRequest {
        app: String,
        lang: String,
        params: String,
        raw_sig: String,
    },
    HttpResponse {
        app: String,
        lang: String,
        params: String,
        raw_sig: String,
    },
    Unparsed {
        module: String,
        remain: String,
    },
}

#[derive(Debug)]
pub struct P0f {
    pub date: DateTime,
    pub module: P0fModule,
    pub client: Endpoint,
    pub server: Endpoint,
    pub subject: Subject,
}
