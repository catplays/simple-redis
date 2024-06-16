
use crate::{Backend, RespFrame, RespNull};

use super::{CommandExecutor, HGet, HGetAll, HSet, RESP_OK};



impl CommandExecutor for HGet {
    fn execute(self,backend: &Backend) -> RespFrame {
        todo!()
    }
}

impl CommandExecutor for HSet {
    fn execute(self,backend: &Backend) -> RespFrame {
        todo!()
    }
}

impl CommandExecutor for HGetAll {
    fn execute(self,backend: &Backend) -> RespFrame {
        todo!()
    }
}