use native_tls::TlsStream;
use std::io;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use self::Stream::{Tcp, Tls};

#[derive(Debug)]
pub enum Stream {
    Tcp(TcpStream),
    Tls(WrappedTlsStream),
}

impl Stream {
    pub fn try_clone(&self) -> io::Result<Stream> {
        match *self {
            Tcp(ref s) => Ok(Tcp(s.try_clone()?)),
            Tls(ref s) => Ok(Tls(s.clone())),
        }
    }

    pub fn as_tcp(&self) -> io::Result<TcpStream> {
        match *self {
            Tcp(ref s) => s.try_clone(),
            Tls(ref s) => s.as_tcp(),
        }
    }
}

impl io::Read for Stream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Tcp(ref mut s) => s.read(buf),
            Tls(ref mut s) => s.read(buf),
        }
    }
}

impl io::Write for Stream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Tcp(ref mut s) => s.write(buf),
            Tls(ref mut s) => s.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Tcp(ref mut s) => s.flush(),
            Tls(ref mut s) => s.flush(),
        }
    }
}

// Clonable TLS Stream
#[derive(Debug, Clone)]
pub struct WrappedTlsStream(Arc<Mutex<TlsStream<TcpStream>>>);

impl WrappedTlsStream {
    pub fn new(stream: TlsStream<TcpStream>) -> Self {
        Self(Arc::new(Mutex::new(stream)))
    }

    pub fn as_tcp(&self) -> io::Result<TcpStream> {
        self.0.lock().unwrap().get_ref().try_clone()
    }
}

impl io::Read for WrappedTlsStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.lock().unwrap().read(buf)
    }
}

impl io::Write for WrappedTlsStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.lock().unwrap().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.lock().unwrap().flush()
    }
}
