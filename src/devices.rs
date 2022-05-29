use mio::{Events, Poll, Interest, Token, unix::SourceFd};
use std::os::unix::io::AsRawFd;
use udev::Event;

const TOKEN_SOCKET: Token = Token(0);

pub fn poll<P>(f: P)
where P: Fn(&Event) {
    let mut poll = Poll::new().unwrap();
    let mut events = Events::with_capacity(120);

    let socket = udev::MonitorBuilder::new().unwrap()
        .match_subsystem("block").unwrap()
        .listen().unwrap();

    poll.registry()
        .register(
            &mut SourceFd(&socket.as_raw_fd()),
            TOKEN_SOCKET,
            Interest::READABLE)
        .unwrap();

    loop {
        let poll_timeout = None;
        poll.poll(&mut events, poll_timeout).unwrap();

        for event in &events {
            match event.token() {
                TOKEN_SOCKET => {
                    socket.clone().for_each(|event| f(&event));
                },
                _ => unimplemented!("other token responsibilities")
            }
        };
    }
}
