use mio::{Events, Poll, Interest, Token, unix::SourceFd};
use std::os::unix::io::AsRawFd;
use udev::Event;
use anyhow::Error;

const TOKEN_SOCKET: Token = Token(0);

pub fn poll(f: fn(Event)) -> Result<(), Error> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(120);

    let socket = udev::MonitorBuilder::new()?
        .match_subsystem("block")?
        .listen()?;

    poll.registry().register(&mut SourceFd(&socket.as_raw_fd()), TOKEN_SOCKET, Interest::READABLE)?;

    loop {
        let poll_timeout = None;
        poll.poll(&mut events, poll_timeout)?;

        for event in &events {
            match event.token() {
                TOKEN_SOCKET => {
                    socket.clone().for_each(|event| {
                        f(event);
                        //println!("{:?}", event.event_type());
                    });
                },
                _ => unimplemented!("other token responsibilities")
            }
        };
    }
}
