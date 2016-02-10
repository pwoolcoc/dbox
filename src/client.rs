use super::{ApiError, Result, Response, Endpoint, DropboxClient};
use hyper::client as hyper_client;
use hyper::error as hyper_error;
use hyper::header::{Headers, Authorization, Bearer, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use hyper::http::RawStatus;
use std::io::Read;
use std::collections::BTreeMap;
use std::fmt;
use rustc_serialize;
use rustc_serialize::json;

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    token: String,
    user_agent: String,
    proxies: Vec<String>,
    max_retries: u32,
    max_connections: u32,
}

impl Client {
    pub fn new(token: &str) -> Result<Client> {
        if token.len() < 62 {
            Err(ApiError::TokenError)
        } else {
            Ok(Client {
                token: token.to_owned(),
                proxies: vec![],
                user_agent: "Dropbox SDK/Rust".to_owned(),
                max_retries: 4,
                max_connections: 8,
            })
        }
    }

    pub fn user_agent(self, user_agent: &str) -> Client {
        Client {
            user_agent: user_agent.to_owned(),
            .. self
        }
    }

    pub fn max_retries(self, max_retries: u32) -> Client {
        Client {
            max_retries: max_retries,
            .. self
        }
    }

    pub fn max_connections(self, max_connections: u32) -> Client {
        Client {
            max_connections: max_connections,
            .. self
        }
    }

    pub fn proxies(self, proxies: Vec<String>) -> Client {
        Client {
            proxies: proxies,
            .. self
        }
    }
}

impl DropboxClient for Client {
    fn access_token(&self) -> &str {
        self.token.as_ref()
    }

    fn request<T>(&self, endpoint: Endpoint, url: &str, headers: &mut BTreeMap<String, String>, body: Option<T>) -> Result<Response>
            where T: rustc_serialize::Encodable + Clone
    {
        let endpoint = format!("{}", endpoint);
        let url = format!("https://{}.dropboxapi.com/2/{}", endpoint, url);
        let sbody = {
            if body.is_some() {
                let body = body.clone();
                json::encode(&body.unwrap()).unwrap()
            } else {
                "".to_owned()
            }
        };

        let mut hheaders = Headers::new();

        for (key, value) in headers.iter() {
            hheaders.set_raw(key.to_owned(), vec![value.to_owned().into_bytes()]);
        }

        hheaders.set(
            Authorization(
                Bearer {
                    token: self.access_token().to_owned(),
                }
            )
        );
        let hclient = hyper_client::Client::new();
        let mut builder = hclient.post(&url).headers(hheaders);
        if body.is_some() {
            builder = builder.body(&sbody);
        }
        match builder.send() {
            Ok(mut res) => {
                match res.status {
                    StatusCode::Ok => {
                        let mut _body = String::new();
                        res.read_to_string(&mut _body);
                        let status_raw = res.status_raw();
                        let dropbox_result = res.headers.get_raw("dropbox-api-result");
                        let dropbox_result = dropbox_result.map(|s| {
                                String::from_utf8(s[0].clone()).unwrap()
                        });
                        // println!("Body: {:?}", &_body);
                        Ok(Response {
                            _status: status_raw.0,
                            _api_result: dropbox_result,
                            _body: _body,
                        })
                    },
                    _ => {
                        let mut _body = String::new();
                        res.read_to_string(&mut _body);
                        println!("{:?}", _body);
                        Err(ApiError::ClientError)
                    }
                }

            },
            Err(e) => {
                println!("{:?}", e);
                Err(ApiError::ClientError)
            }
        }
    }
}
