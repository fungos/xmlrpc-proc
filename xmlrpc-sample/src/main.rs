extern crate xmlrpc_proc;

use xmlrpc::{Request, Value, Error};
use xmlrpc_proc::xmlrpc;

trait Endpoint {
    fn endpoint(&self) -> &str;
}

struct TestServer {
    endpoint: String,
}

impl Endpoint for TestServer {
    fn endpoint(&self) -> &str {
        &self.endpoint
    }
}

#[xmlrpc(TestServer)]
trait RemoteApi {
    #[xmlrpc(method="system.listMethods")]
    fn list(&self) -> Result<Value, Error>;
}

fn main() {
    let test = TestServer {
        endpoint: "http://rpc.pingomatic.com/".to_owned()
    };
    println!("{:?}", test.list());
}