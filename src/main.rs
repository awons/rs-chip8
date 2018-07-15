extern crate mio;
extern crate termios;
extern crate libc;

mod emulator;

use termios::{Termios, TCSANOW, ICANON, ECHO, tcsetattr};
use mio::*;
use mio::unix::EventedFd;

use libc::*;

fn unbuffer_stdin() {
    let termios = Termios::from_fd(0).unwrap();
    let mut new_termios = termios.clone();
    new_termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(0, TCSANOW, &mut new_termios).unwrap();
}

fn main() {
    unbuffer_stdin();

    let poll = Poll::new().unwrap();
    const STDIN: Token = Token(0);
    let ev_fd = EventedFd(&0);
    
    poll.register(&ev_fd, STDIN, Ready::readable(), PollOpt::edge()).unwrap();

    let mut events = Events::with_capacity(8);
    let mut key_buffer = [0; 1];
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            if let STDIN = event.token() {
                read_key(&mut key_buffer);
                println!("pressed key: {:?}", key_buffer[0]);
            }
        }
    }
}

fn read_key(buffer: &mut [u8; 1]) {
    unsafe {
        read(0, buffer.as_mut_ptr() as *mut c_void, 1);
    }
}
