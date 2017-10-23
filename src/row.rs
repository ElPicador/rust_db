use std::io;
use std::io::Error;
use std::io::ErrorKind;
use std::ops::Deref;
use std::io::BufWriter;
use std::io::BufReader;
use std::io::Write;
use std::io::Read;


pub struct Row {
    pub id: u32,
    pub username: String,
    pub email: String,
}

impl Row {
    pub fn read(mut input: BufReader<u8>) -> io::Result<Row> {
        let mut id_buffer: [u8; 4] = [0; 4];
        input.read(&id_buffer);
        let mut username_len: [u8; 8] = [0; 8];
        input.read(&username_len);
        Err(Error::new(ErrorKind::Other, "oh no!"))
    }

    pub fn write(&self, mut output: &BufWriter<Vec<u8>>) -> io::Result<()> {
        output
            .write(&Row::u32_array_u8(self.id))
            .and(output.write(&Row::usize_array_u8(self.username.len())))
            .and(output.write(self.username.as_bytes()))
            .and(output.write(&Row::usize_array_u8(self.email.len())))
            .and(output.write(self.email.as_bytes()))
            .map(|_| ())
    }

    fn u32_array_u8(x: u32) -> [u8; 4] {
        let b1: u8 = ((x >> 24) & 0xff) as u8;
        let b2: u8 = ((x >> 16) & 0xff) as u8;
        let b3: u8 = ((x >> 8) & 0xff) as u8;
        let b4: u8 = (x & 0xff) as u8;
        return [b1, b2, b3, b4];
    }

    fn usize_array_u8(x: usize) -> [u8; 8] {
        let b1: u8 = ((x >> 56) & 0xffff) as u8;
        let b2: u8 = ((x >> 48) & 0xffff) as u8;
        let b3: u8 = ((x >> 40) & 0xffff) as u8;
        let b4: u8 = ((x >> 32) & 0xffff) as u8;
        let b5: u8 = ((x >> 24) & 0xffff) as u8;
        let b6: u8 = ((x >> 16) & 0xffff) as u8;
        let b7: u8 = ((x >> 8) & 0xffff) as u8;
        let b8: u8 = (x & 0xffff) as u8;
        return [b1, b2, b3, b4, b5, b6, b7, b8];
    }
}

pub fn insert_to_row(input: &String) -> Option<Row> {
    let res: Vec<&str> = input.trim().split(" ").collect();
    if res.len() != 4 {
        return None;
    }

    let username = String::from(res.get(2).unwrap().deref());
    let email = String::from(res.get(3).unwrap().deref());

    match String::from(res.get(1).unwrap().deref()).parse::<u32>() {
        Ok(id) => Some(Row { id, username, email }),
        Err(_) => None
    }
}
