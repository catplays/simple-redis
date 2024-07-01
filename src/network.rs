use std::fmt::Debug;

use crate::{
    cmd::{Command, CommandExecutor}, Backend, RespDecode, RespEncode, RespError, RespFrame
};
use anyhow::Result;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::{Decoder, Encoder, Framed};
use tracing::info;
use futures::SinkExt;

#[derive(Debug)]
struct RespFrameCodec;

#[derive(Debug)]
struct RedisRequest {
    frame: RespFrame,
    backend: Backend,
}

struct RedisResp {
    frame: RespFrame,
}

pub async fn stream_handler(stream: TcpStream, backend: Backend) -> Result<()> {
    // how to get a frame from the stream?
    let mut framed = Framed::new(stream, RespFrameCodec);
    loop {
        match framed.next().await {
            Some(Ok(frame)) => {
                info!("Receive frame:{:?}", frame);
                let request = RedisRequest {
                    frame,
                    backend: backend.clone(),
                };
                let resp = request_handler(request).await?;
                info!("Sending response: {:?}", resp.frame);
                framed.send(resp.frame).await?
            }
            Some(Err(e)) => return Err(e),
            None => return Ok(()),
        }
    }
}

async fn request_handler(request: RedisRequest) -> Result<RedisResp> {
    let (frame, backend) = (request.frame, request.backend);
    let cmd = Command::try_from(frame)?;
    info!("Executing command: {:?}", cmd);
    let frame = cmd.execute(&backend);
    Ok(RedisResp { frame })
}

impl Encoder<RespFrame> for RespFrameCodec {
    type Error = anyhow::Error;

    fn encode(&mut self, item: RespFrame, dst: &mut bytes::BytesMut) -> anyhow::Result<()> {
        let encoder = item.encode();
        dst.extend_from_slice(&encoder);
        Ok(())
    }
}

impl Decoder for RespFrameCodec {
    type Item = RespFrame;

    type Error = anyhow::Error;

    fn decode(&mut self, src: &mut bytes::BytesMut) -> Result<Option<RespFrame>> {
        match RespFrame::decode(src) {
            Ok(frame) => Ok(Some(frame)),
            Err(RespError::NotComplete) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}
