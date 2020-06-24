# xmlrpc-proc

A procedural macro to generate typed method calls for a remote rpc endpoint using xmlrpc.


## Why

A rpc call using the crate [xmlrpc](https://crates.io/crates/xmlrpc) is required to use the builder pattern with "untyped" arguments, like this from [their sample](https://github.com/jonas-schievink/xml-rpc-rs/blob/master/examples/client.rs):

```rust
let pow_request = Request::new("pow").arg(2).arg(8); // Compute 2**8
let request_result = pow_request.call_url("http://127.0.0.1:8000");
```

With this procedural macro, you can define the remote methods as a Rust trait,
and take advantage of compiler help to identify and avoid passing wrong arguments to the remote call. We could then define the remote interface this way:

```rust
#[xmlrpc(RemoteServer)]
trait RemoteApi {
    fn pow(&self, val: i32, power: i32) -> Result<Value, Error>;
}
```

which will generate:
```rust
impl RemoteApi for RemoteServer {
    fn pow(&self, val: i32, power: i32) -> Result<Value, Error> {
        let req = Request::new("pow").arg(val).arg(power);
        req.call_url(&self.endpoint)
    }
}
```

And using it would become as simple as a rust method call:
```rust
let request_result = server.pow(2, 8);
```

For a complete example, see the [xmlrpc-sample](https://github.com/fungos/xmlrpc-proc/blob/master/xmlrpc-sample/src/main.rs)

### TODO

- Automatic result conversion to primitives or user types
  - i.e. `Result<Value, Error>` to `Result<User, Error>` where `User` is a local struct.