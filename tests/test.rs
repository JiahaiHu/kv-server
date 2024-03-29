extern crate kv_server;
extern crate futures;

use futures::Future;

use kv_server::client::Client;
use kv_server::server::Server;

#[test]
fn kv_test() {
    let host = String::from("127.0.0.1");
    let port = 34567;
    let mut server = Server::new(host.clone(), port);
    let client = Client::new(host.clone(), port);
    
    server.start();

    let test_data = vec![("A", "Alex"), ("B", "Bob"), ("T", "Tom"), ("M", "Mike")];
    
    // delete if exists
    for (key, _) in test_data.iter() {
        client.delete(key.to_string());
    }

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

    let _ = server.stop().wait();
}

#[test]
fn recovery_test() {
    let host = String::from("127.0.0.1");
    let port = 34567;
    let key = String::from("A");
    let value = String::from("Alex");
    
    // insert data into database
    {
        let mut server = Server::new(host.clone(), port);
        server.start();
    
        let client = Client::new(host.clone(), port);    
        
        client.put(key.clone(), value.clone());
    
        let _ = server.stop().wait();
    }

    // test recovery
    {
        let mut server = Server::new(host.clone(), port);
        server.start();

        let client = Client::new(host.clone(), port);
        
        let ret = client.get(key.clone());
        assert_eq!(Some(value.clone()), ret);
    
        let _ = server.stop().wait();
    }
}
