extern crate kv_server;

use std::thread;
use std::time::Duration;

use kv_server::client::Client;
use kv_server::server::Server;

#[test]
fn kvtest() {
    let host = String::from("127.0.0.1");
    let port = 34567;
    let mut server = Server::new(host.clone(), port);
    let client = Client::new(host.clone(), port);
    
    server.start();

    let test_data = vec![("A", "Alex"), ("B", "Bob"), ("T", "Tom"), ("M", "Mike")];
    
    // test get
    let key = String::from("A");
    let ret = client.get(key.clone());
    assert_eq!(None, ret);

    // test put
    let value = String::from("Alex");
    client.put(key.clone(), value.clone());
    let ret = client.get(key.clone());
    assert_eq!(Some(value.clone()), ret);

    // test delete
    client.delete(key.clone());
    let ret = client.get(key.clone());
    assert_eq!(None, ret);

    // test scan
    let key_start = String::from("A");
    let key_end = String::from("C");
    let ret = client.scan(key_start.clone(), key_end.clone());
    assert_eq!(None, ret);
    for (key, value) in test_data.iter() {
        client.put(key.to_string().clone(), value.to_string().clone());
    }
    let ret = client.scan(key_start, key_end).unwrap();
    let v = ret.get("A").unwrap();
    assert_eq!(v.clone(), "Alex".to_string());
    let v = ret.get("B").unwrap();
    assert_eq!(v.clone(), "Bob".to_string());
    let v = ret.get("M");
    assert_eq!(v, None);
    let v = ret.get("T");
    assert_eq!(v, None);

    thread::sleep(Duration::from_millis(100));

    server.stop();
}
