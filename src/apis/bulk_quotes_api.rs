/* 
 * Open API for FSP Interoperability (FSPIOP)
 *
 * Based on API Definition.docx updated on 2020-05-19 Version 1.1. Note - The API supports a maximum size of 65536 bytes (64 Kilobytes) in the HTTP header.
 *
 * OpenAPI spec version: 1.1
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::collections::HashMap;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use hyper::header::UserAgent;

use super::{Error, configuration};

pub struct BulkQuotesApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> BulkQuotesApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> BulkQuotesApiClient<C> {
        BulkQuotesApiClient {
            configuration: configuration,
        }
    }
}

pub trait BulkQuotesApi {
    fn bulk_quotes(&self, body: ::models::BulkQuotesPostRequest, accept: &str, content_type: &str, date: &str, fspiop_source: &str, content_length: i32, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn bulk_quotes_by_id(&self, ID: &str, content_type: &str, date: &str, fspiop_source: &str, accept: &str, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn bulk_quotes_by_id1(&self, ID: &str, content_type: &str, date: &str, fspiop_source: &str, body: ::models::BulkQuotesIdPutResponse, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str, content_length: i32) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
    fn bulk_quotes_error_by_id(&self, ID: &str, body: ::models::ErrorInformationObject, content_type: &str, date: &str, fspiop_source: &str, content_length: i32, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>>;
}


impl<C: hyper::client::Connect>BulkQuotesApi for BulkQuotesApiClient<C> {
    fn bulk_quotes(&self, body: ::models::BulkQuotesPostRequest, accept: &str, content_type: &str, date: &str, fspiop_source: &str, content_length: i32, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/bulkQuotes?{}", configuration.base_path, query_string);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("Accept", accept);
            headers.set_raw("Content-Length", content_length);
            headers.set_raw("Content-Type", content_type);
            headers.set_raw("Date", date);
            headers.set_raw("X-Forwarded-For", x_forwarded_for);
            headers.set_raw("FSPIOP-Source", fspiop_source);
            headers.set_raw("FSPIOP-Destination", fspiop_destination);
            headers.set_raw("FSPIOP-Encryption", fspiop_encryption);
            headers.set_raw("FSPIOP-Signature", fspiop_signature);
            headers.set_raw("FSPIOP-URI", FSPIOP_URI);
            headers.set_raw("FSPIOP-HTTP-Method", fspiop_http_method);
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn bulk_quotes_by_id(&self, ID: &str, content_type: &str, date: &str, fspiop_source: &str, accept: &str, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Get;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/bulkQuotes/{ID}?{}", configuration.base_path, query_string, ID=ID);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("Content-Type", content_type);
            headers.set_raw("Date", date);
            headers.set_raw("X-Forwarded-For", x_forwarded_for);
            headers.set_raw("FSPIOP-Source", fspiop_source);
            headers.set_raw("FSPIOP-Destination", fspiop_destination);
            headers.set_raw("FSPIOP-Encryption", fspiop_encryption);
            headers.set_raw("FSPIOP-Signature", fspiop_signature);
            headers.set_raw("FSPIOP-URI", FSPIOP_URI);
            headers.set_raw("FSPIOP-HTTP-Method", fspiop_http_method);
            headers.set_raw("Accept", accept);
        }



        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn bulk_quotes_by_id1(&self, ID: &str, content_type: &str, date: &str, fspiop_source: &str, body: ::models::BulkQuotesIdPutResponse, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str, content_length: i32) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/bulkQuotes/{ID}?{}", configuration.base_path, query_string, ID=ID);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("Content-Type", content_type);
            headers.set_raw("Date", date);
            headers.set_raw("X-Forwarded-For", x_forwarded_for);
            headers.set_raw("FSPIOP-Source", fspiop_source);
            headers.set_raw("FSPIOP-Destination", fspiop_destination);
            headers.set_raw("FSPIOP-Encryption", fspiop_encryption);
            headers.set_raw("FSPIOP-Signature", fspiop_signature);
            headers.set_raw("FSPIOP-URI", FSPIOP_URI);
            headers.set_raw("FSPIOP-HTTP-Method", fspiop_http_method);
            headers.set_raw("Content-Length", content_length);
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

    fn bulk_quotes_error_by_id(&self, ID: &str, body: ::models::ErrorInformationObject, content_type: &str, date: &str, fspiop_source: &str, content_length: i32, x_forwarded_for: &str, fspiop_destination: &str, fspiop_encryption: &str, fspiop_signature: &str, FSPIOP_URI: &str, fspiop_http_method: &str) -> Box<Future<Item = (), Error = Error<serde_json::Value>>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Put;

        let query_string = {
            let mut query = ::url::form_urlencoded::Serializer::new(String::new());
            query.finish()
        };
        let uri_str = format!("{}/bulkQuotes/{ID}/error?{}", configuration.base_path, query_string, ID=ID);

        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut uri: hyper::Uri = uri_str.parse().unwrap();

        let mut req = hyper::Request::new(method, uri);

        if let Some(ref user_agent) = configuration.user_agent {
            req.headers_mut().set(UserAgent::new(Cow::Owned(user_agent.clone())));
        }

        {
            let mut headers = req.headers_mut();
            headers.set_raw("Content-Length", content_length);
            headers.set_raw("Content-Type", content_type);
            headers.set_raw("Date", date);
            headers.set_raw("X-Forwarded-For", x_forwarded_for);
            headers.set_raw("FSPIOP-Source", fspiop_source);
            headers.set_raw("FSPIOP-Destination", fspiop_destination);
            headers.set_raw("FSPIOP-Encryption", fspiop_encryption);
            headers.set_raw("FSPIOP-Signature", fspiop_signature);
            headers.set_raw("FSPIOP-URI", FSPIOP_URI);
            headers.set_raw("FSPIOP-HTTP-Method", fspiop_http_method);
        }


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

}
