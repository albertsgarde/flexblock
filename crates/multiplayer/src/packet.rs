use std::convert::TryInto;

use bytes::BytesMut;
use serde::{de::DeserializeOwned, Serialize};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, BufReader, BufWriter, Result};

const PACKET_SIZE_SIZE: usize = std::mem::size_of::<u64>();

#[derive(Debug)]
pub enum NextObjectError {
    DeserializeError(bincode::Error),
    TokioIo(tokio::io::Error),
}

impl From<bincode::Error> for NextObjectError {
    fn from(e: bincode::Error) -> Self {
        NextObjectError::DeserializeError(e)
    }
}

impl From<tokio::io::Error> for NextObjectError {
    fn from(e: tokio::io::Error) -> Self {
        NextObjectError::TokioIo(e)
    }
}

impl std::fmt::Display for NextObjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            NextObjectError::DeserializeError(err) => {
                write!(f, "Failed to deserialize packet. Error: {}", err)
            }
            NextObjectError::TokioIo(err) => write!(f, "Error reading next packet. Error: {}", err),
        }
    }
}

impl std::error::Error for NextObjectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            NextObjectError::DeserializeError(err) => Some(err),
            NextObjectError::TokioIo(err) => Some(err),
        }
    }
}

pub struct Receiver<R: AsyncRead + Unpin> {
    reader: BufReader<R>,
    cur_packet_size: Option<u64>,
    cur_packet: BytesMut,
}

impl<R: AsyncRead + Unpin> Receiver<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader: BufReader::new(reader),
            cur_packet_size: None,
            cur_packet: BytesMut::new(),
        }
    }

    async fn next_bytes(&mut self) -> Result<Vec<u8>> {
        if self.cur_packet_size.is_none() {
            self.cur_packet.reserve(PACKET_SIZE_SIZE);
            while self.cur_packet.len() < PACKET_SIZE_SIZE {
                self.reader.read_buf(&mut self.cur_packet).await?;
            }
            let packet_size_buffer = self.cur_packet.split_to(PACKET_SIZE_SIZE);
            self.cur_packet_size = Some(u64::from_be_bytes(
                packet_size_buffer[..].try_into().unwrap(),
            ));
        }

        let packet_size: usize = self
            .cur_packet_size
            .unwrap()
            .try_into()
            .expect("Packet too large for receiving system.");
        self.cur_packet.reserve(packet_size);

        while self.cur_packet.len() < packet_size {
            self.reader.read_buf(&mut self.cur_packet).await?;
        }

        self.cur_packet_size = None;
        let packet = self.cur_packet.split_to(packet_size);
        Ok(packet.to_vec())
    }

    pub async fn next_object<T: Serialize + DeserializeOwned>(
        &mut self,
    ) -> std::result::Result<T, NextObjectError> {
        let packet = self.next_bytes().await?;
        Ok(bincode::deserialize(&packet)?)
    }
}

pub struct Transmitter<W: AsyncWrite + Unpin> {
    writer: BufWriter<W>,
}

impl<W: AsyncWrite + Unpin> Transmitter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer: BufWriter::new(writer),
        }
    }

    async fn write_bytes(&mut self, bytes: &[u8]) -> Result<()> {
        let packet_size: u64 = bytes.len().try_into().expect("Packet too large to send.");
        let packet_size_bytes = packet_size.to_be_bytes();
        self.writer.write_all(&packet_size_bytes).await?;
        self.writer.write_all(bytes).await?;
        self.writer.flush().await?;
        Ok(())
    }

    pub async fn write_object<T: Serialize + DeserializeOwned>(
        &mut self,
        object: &T,
    ) -> Result<()> {
        let packet = bincode::serialize(object).expect("Failed to serialize object.");
        self.write_bytes(&packet).await
    }
}
