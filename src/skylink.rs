use std::str::from_utf8;

pub type Skylink = [u8; 46];

pub trait Encode: Sized {
    /// The encoded size of the type.
    const SIZE: usize;
    /// Encodes the type in the provided `bytes` buffer. Must fit exactly.
    fn encode(&self, bytes: &mut [u8]);
}

impl Encode for Skylink {
    const SIZE: usize = 46;

    fn encode(&self, bytes: &mut [u8]) {
        bytes.copy_from_slice(self);
    }
}

pub fn go() {
    let mut s: Skylink = [0; 46];
    let b = "AABANrVr6zYiEAhf0ltfDhUx3sGA5V15AvFkciX32eRwwg".as_bytes();
    s.copy_from_slice(&b[..46]);

    let mut bytes = [0; 46];
    s.encode(&mut bytes);
    println!("bytes {:?}", from_utf8(&bytes).unwrap());
}
