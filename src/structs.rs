pub type DateTime = chrono::NaiveDateTime;

#[derive(Debug)]
pub enum P0fModule {
    Uptime {
        client: String,
        server: String,
        subject: String,
        uptime: String,
        raw_freq: String,
    },
    Mtu {
        client: String,
        server: String,
        subject: String,
        link: String,
        raw_mtu: usize,
    },
    Syn {
        client: String,
        server: String,
        subject: String,
        os: String,
        dist: String,
        params: String,
        raw_sig: String,
    },
    SynAck {
        client: String,
        server: String,
        subject: String,
        os: String,
        dist: String,
        params: String,
        raw_sig: String,
    },
    HostChange {
        client: String,
        server: String,
        subject: String,
        reason: String,
        raw_hits: String,
    },
    HttpRequest {
        client: String,
        server: String,
        subject: String,
        app: String,
        lang: String,
        params: String,
        raw_sig: String,
    },
    HttpResponse {
        client: String,
        server: String,
        subject: String,
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
}
