use multiplayer::{Client, Server};
use game::StateInputEvent;

#[test]
#[ignore]
fn test() {
    const IP: &str = "localhost:15926";
    let server = std::thread::spawn(|| {
        let mut server = Server::start(IP).unwrap();
        assert_eq!(server.new_events().len(), 0);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert_eq!(server.num_clients(), 1);
        let new_events = server.new_events();
        assert_eq!(new_events.len(), 1);
        server.send_events(new_events);
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert_eq!(server.num_clients(), 0);
    });

    let client = std::thread::spawn(|| {
        let mut client = Client::start(IP).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1000));
        let event = StateInputEvent::Jump;
        client.send_events(vec![event]);
        assert!(client.next_tick_events().is_none());
        std::thread::sleep(std::time::Duration::from_millis(2000));
        assert_eq!(client.next_tick_events().unwrap().len(), 1);
        client.disconnect();
    });

    server.join().unwrap();
    client.join().unwrap();
}
