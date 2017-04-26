extern crate iron;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::Handler;
use iron::status;

use serde::{Serialize, Serializer, Deserialize, Deserializer};

use std::io::Read;

pub trait Decoder {
    type T: Sized;
    fn decode(&self, bytes: &[u8]) -> IronResult<Self::T>;
}

pub trait Encoder {
    type T;
    fn encode(&self, target: &Self::T) -> IronResult<Vec<u8>>;
}

pub trait Service {
    type Req: Sized;
    type Res;

    fn process(&self, req: Self::Req) -> IronResult<Self::Res>;
}

pub struct IronService<S, D, E>
    where S: 'static + Send + Sync + Service,
          D: 'static + Send + Sync + Decoder<T = <S as Service>::Req>,
          E: 'static + Send + Sync + Encoder<T = <S as Service>::Res>
{
    decoder: D,
    encoder: E,
    service: S,
}

impl<S, D, E> IronService<S, D, E>
    where S: 'static + Send + Sync + Service,
          D: 'static + Send + Sync + Decoder<T = <S as Service>::Req>,
          E: 'static + Send + Sync + Encoder<T = <S as Service>::Res>
{
    pub fn new_with_json(service: S) -> IronService<S, D, E> {
        unimplemented!()
    }
}

impl<S, D, E> Handler for IronService<S, D, E>
    where S: 'static + Send + Sync + Service,
          D: 'static + Send + Sync + Decoder<T = <S as Service>::Req>,
          E: 'static + Send + Sync + Encoder<T = <S as Service>::Res>
{
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut body_data = Vec::new();
        if let Err(e) = req.body.read_to_end(&mut body_data) {
            return Err(IronError::new(e, (status::InternalServerError, "IO Error")));
        };

        let s_req = self.decoder.decode(&body_data)?;
        let s_res = self.service.process(s_req)?;
        let encoded_res = self.encoder.encode(&s_res)?;
        Ok(Response::with((status::Ok, encoded_res)))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
