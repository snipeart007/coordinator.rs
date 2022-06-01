extern crate futures_core;
extern crate tokio_stream;
extern crate kiddo;
extern crate async_trait;
extern crate scc;

pub mod core;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
