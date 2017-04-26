extern crate iron;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::Handler;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

pub trait Decoder {
    type T: Sized;
    fn decode(&self, bytes: &[u8]) -> IronResult<Self::T>;
}

pub trait Encoder {
    type T;
    fn decode(&self, target: &Self::T) -> IronResult<Vec<u8>>;
}

pub trait Service {
    type Req: Sized;
    type Res;

    fn process(&self, req: Self::Req) -> IronResult<Self::Res>;
}

pub struct IronService<S, D, E> where S: 'static + Send + Sync + Service,
    D: 'static + Send + Sync + Decoder<T=<S as Service>::Req>,
    E: 'static + Send + Sync + Encoder<T=<S as Service>::Res> {
    decoder: D,
    encoder: E,
    service: S,
}

impl<S, D, E> IronService<S, D, E> where S: 'static + Send + Sync + Service,
    D: 'static + Send + Sync + Decoder<T=<S as Service>::Req>,
    E: 'static + Send + Sync + Encoder<T=<S as Service>::Res> {
        pub fn new_with_json(service: S) {

        }
    }

impl<S, D, E> Handler for IronService<S, D, E> where S: 'static + Send + Sync + Service,
    D: 'static + Send + Sync + Decoder<T=<S as Service>::Req>,
    E: 'static + Send + Sync + Encoder<T=<S as Service>::Res> {

    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
