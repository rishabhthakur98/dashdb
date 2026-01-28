use std::{
    io::{Read, Write},
    net::TcpStream,
};
use dashmap::DashMap;
use tracing::instrument;
use crate::printerror;

#[instrument(skip(stream, shared_map))]
pub fn set_key_value(stream: &mut TcpStream, shared_map: &DashMap<Vec<u8>, Vec<u8>>) -> bool {
    let mut request_key_value_length: [u8; 4] = [0, 0, 0, 0];
    printerror!(stream.read_exact(&mut request_key_value_length));

    let request_key_length =
        u16::from_be_bytes([request_key_value_length[0], request_key_value_length[1]]);
    let request_value_length =
        u16::from_be_bytes([request_key_value_length[2], request_key_value_length[3]]);
    let request_body_length: usize =
        (request_key_length as usize) + (request_value_length as usize);

    let mut request_body: Vec<u8> = vec![0; request_body_length];
    printerror!(stream.read_exact(&mut request_body));

    let request_value: Vec<u8> = request_body.split_off(request_key_length as usize);
    let request_key: Vec<u8> = request_body;

    shared_map.insert(request_key, request_value);

    let response: [u8; 1] = [1];
    printerror!(stream.write_all(&response));

    printerror!(stream.flush());

    true
}

#[instrument(skip(stream, shared_map))]
pub fn get_value(stream: &mut TcpStream, shared_map: &DashMap<Vec<u8>, Vec<u8>>) -> bool {
    let mut request_key_length: [u8; 2] = [0, 0];
    printerror!(stream.read_exact(&mut request_key_length));

    let request_key_length = u16::from_be_bytes([request_key_length[0], request_key_length[1]]);

    let mut request_key: Vec<u8> = vec![0; request_key_length as usize];
    printerror!(stream.read_exact(&mut request_key));

    let mut response: Vec<u8> = Vec::with_capacity(1);
    match shared_map.get(&request_key) {
        Some(key_value) => {
            
            let value = key_value.value();
            let value_length: usize = value.len();
            let value_length_u16: u16 = value_length as u16;
            let value_length_array = value_length_u16.to_be_bytes();
            
            &response.push(1);
            &response.extend(&value_length_array);
            &response.extend(value);
            
        }
        None => {
            response.push(2);
        }
    }
    printerror!(stream.write_all(&response));

    printerror!(stream.flush());

    true
}

#[instrument(skip(stream, shared_map))]
pub fn delete_key_value(stream: &mut TcpStream, shared_map: &DashMap<Vec<u8>, Vec<u8>>) -> bool {
    let mut request_key_length: [u8; 2] = [0, 0];
    printerror!(stream.read_exact(&mut request_key_length));

    let request_key_length = u16::from_be_bytes([request_key_length[0], request_key_length[1]]);

    let mut request_key: Vec<u8> = vec![0; request_key_length as usize];
    printerror!(stream.read_exact(&mut request_key));

    let mut response: [u8; 1] = [0];
    match shared_map.remove(&request_key) {
        Some(_) => {
            response[0] = 1;
        }
        None => {
            response[0] = 2;
        }
    }
    printerror!(stream.write_all(&response));

    printerror!(stream.flush());

    true
}
