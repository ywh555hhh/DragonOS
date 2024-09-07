#![allow(unused_variables)]

use core::fmt::Debug;
use alloc::sync::Arc;
use system_error::SystemError::{self, *};
use crate::net::socket::*;
use crate::net::syscall_util::MsgHdr;

/// # `Socket` methods
/// ## Reference
/// - [Posix standard](https://pubs.opengroup.org/onlinepubs/9699919799/)
pub trait Socket: Sync + Send + Debug {
    /// # `epoll_items`
    /// socket的epoll事件集
    fn epoll_items(&self) -> EPollItems;

    /// # `wait_queue`
    /// 获取socket的wait queue
    fn wait_queue(&self) -> WaitQueue;
    
    /// # `update_io_events`
    /// 更新socket的事件。
    fn update_io_events(&self) -> Result<EPollEventType, SystemError>;

    /// # `accept`
    /// 接受连接，仅用于listening stream socket
    /// ## Block
    /// 如果没有连接到来，会阻塞
    fn accept(&self) -> Result<(Arc<Inode>, Endpoint), SystemError> {
        Err(ENOSYS)
    }

    /// # `bind` 
    /// 对应于POSIX的bind函数，用于绑定到本机指定的端点
    fn bind(&self, endpoint: Endpoint) -> Result<(), SystemError> {
        Err(ENOSYS)
    }

    /// # `close`
    /// 关闭socket
    fn close(&self) -> Result<(), SystemError> {
        Err(ENOSYS)
    }

    /// # `connect` 
    /// 对应于POSIX的connect函数，用于连接到指定的远程服务器端点
    fn connect(&self, endpoint: Endpoint) -> Result<(), SystemError> {
        Err(ENOSYS)
    }

    // fnctl
    // freeaddrinfo
    // getaddrinfo
    // getnameinfo
    
    /// # `get_peer_name`
    /// 获取对端的地址
    fn get_peer_name(&self) -> Result<Endpoint, SystemError> {
        Err(ENOSYS)
    }

    /// # `get_name`
    /// 获取socket的地址
    fn get_name(&self) -> Result<Endpoint, SystemError> {
        Err(ENOSYS)
    }

    /// # `get_option`
    /// 对应于 Posix `getsockopt` ，获取socket选项
    fn get_option(
        &self,
        level: OptionsLevel,
        name: usize,
        value: &mut [u8],
    ) -> Result<usize, SystemError> {
        log::warn!("getsockopt is not implemented");
        Ok(0)
    }

    /// # `listen`
    /// 监听socket，仅用于stream socket
    fn listen(&self, backlog: usize) -> Result<(), SystemError> {
        Err(ENOSYS)
    }

    // poll
    // pselect

    /// # `read`
    fn read(&self, buffer: &mut [u8]) -> Result<usize, SystemError> {
        self.recv(buffer, MessageFlag::empty())
    }

    /// # `recv`
    /// 接收数据，`read` = `recv` with flags = 0
    fn recv(&self, buffer: &mut [u8], flags: MessageFlag) -> Result<usize, SystemError> {
        Err(ENOSYS)
    }

    /// # `recv_from`
    fn recv_from(
        &self, 
        buffer: &mut [u8],
        flags: MessageFlag,
        address: Option<Endpoint>,
    ) -> Result<(usize, Endpoint), SystemError> 
    { 
        Err(ENOSYS) 
    }

    /// # `recv_msg`
    fn recv_msg(&self, msg: &mut MsgHdr, flags: MessageFlag) -> Result<usize, SystemError> {
        Err(ENOSYS)
    }

    // select

    /// # `send`
    fn send(&self, buffer: &[u8], flags: MessageFlag) -> Result<usize, SystemError> {
        Err(ENOSYS)
    }

    /// # `send_msg`
    fn send_msg(&self, msg: &MsgHdr, flags: MessageFlag) -> Result<usize, SystemError> {
        Err(ENOSYS)
    }

    /// # `send_to`
    fn send_to(&self, buffer: &[u8], flags: MessageFlag, address: Endpoint) -> Result<usize, SystemError> {
        Err(ENOSYS)
    }

    /// # `set_option`
    /// Posix `setsockopt` ，设置socket选项
    /// ## Parameters
    /// - level 选项的层次
    /// - name 选项的名称
    /// - value 选项的值
    /// ## Reference
    /// https://code.dragonos.org.cn/s?refs=sk_setsockopt&project=linux-6.6.21
    fn set_option(
        &self,
        level: OptionsLevel,
        name: usize,
        val: &[u8],
    ) -> Result<(), SystemError> {
        log::warn!("setsockopt is not implemented");
        Ok(())
    }

    /// # `shutdown`
    fn shutdown(&self, how: Shutdown) -> Result<(), SystemError> {
        Err(ENOSYS)
    }

    // sockatmark
    // socket
    // socketpair

    /// # `write`
    fn write(&self, buffer: &[u8]) -> Result<usize, SystemError> {
        self.send(buffer, MessageFlag::empty())
    }

    // /// # `endpoint`
    // fn endpoint(&self) -> Option<Endpoint> {
    //     None
    // }

    // /// # `peer_endpoint`
    // /// 获取对端的端点
    // fn peer_endpoint(&self) -> Option<Endpoint> {
    //     None
    // }

    // fn write_buffer(&self, _buf: &[u8]) -> Result<usize, SystemError> {
    //     todo!()
    // }
}