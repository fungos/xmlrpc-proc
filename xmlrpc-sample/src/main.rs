extern crate xmlrpc_proc;

use xmlrpc::{Request, Value, Error};
use xmlrpc_proc::xmlrpc;

trait Endpoint {
    fn endpoint(&self) -> &str;
}

struct Gravatar {
    endpoint: String,
}

impl Endpoint for Gravatar {
    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[xmlrpc(Gravatar)]
trait GravatarApi {
    // want this: #[xmlrpc(method="grav.exists")]
    fn exists(&self, user_id: i32) -> Result<Value, Error>;
}

fn main() {
    let gravatar = Gravatar {
        endpoint: "https://secure.gravatar.com/xmlrpc?user=39f5fae18c0830f540136d805209066e".to_owned()
    };
    gravatar.exists(123).unwrap();
}