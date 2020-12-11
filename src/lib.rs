pub mod error;
mod socket;

use error::SrtError;
use libsrt_sys as srt;
use socket::SrtSocket;

use std::{
    io::{self, Read, Write},
    iter::Iterator,
    net::{SocketAddr, ToSocketAddrs},
    ops::Drop,
    os::raw::{c_int, c_uint},
};

pub use socket::{SrtCongestionController, SrtKmState, SrtSocketStatus, SrtTransmissionType};

type Result<T> = std::result::Result<T, SrtError>;

pub fn startup() -> Result<()> {
    let result = unsafe { srt::srt_startup() };
    error::handle_result((), result)
}

pub fn cleanup() -> Result<()> {
    let result = unsafe { srt::srt_cleanup() };
    error::handle_result((), result)
}

pub fn builder() -> SrtBuilder {
    SrtBuilder {
        opt_vec: Vec::new(),
    }
}

pub fn async_builder() -> SrtAsyncBuilder {
    let opt_vec = [SrtPreConnectOpt::RcvSyn(false)].to_vec();
    SrtAsyncBuilder { opt_vec }
}

pub struct SrtListener {
    socket: SrtSocket,
}

impl SrtListener {
    pub fn accept(&self) -> Result<(SrtStream, SocketAddr)> {
        let (socket, addr) = self.socket.accept()?;
        Ok((SrtStream { socket }, addr))
    }
    pub fn close(self) -> Result<()> {
        self.socket.close()
    }
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.socket.local_addr()
    }
}

pub struct SrtStream {
    socket: SrtSocket,
}

impl SrtStream {
    pub fn peer_addr(&self) -> Result<SocketAddr> {
        self.socket.peer_addr()
    }
    pub fn close(self) -> Result<()> {
        self.socket.close()
    }
    pub fn set_time_drift_tracer(&self, enable: bool) -> Result<()> {
        self.socket.set_time_drift_tracer(enable)
    }
    pub fn set_input_bandwith(&self, bytes_per_sec: i64) -> Result<()> {
        self.socket.set_input_bandwith(bytes_per_sec)
    }
    pub fn set_recovery_bandwidth_overhead(&self, per_cent: i32) -> Result<()> {
        self.socket.set_recovery_bandwidth_overhead(per_cent)
    }
    pub fn set_receive_timeout(&self, msecs: i32) -> Result<()> {
        self.socket.set_receive_timeout(msecs)
    }
    pub fn set_send_timeout(&self, msecs: i32) -> Result<()> {
        self.socket.set_send_timeout(msecs)
    }
    pub fn get_flight_flag_size(&self) -> Result<i32> {
        self.socket.get_flight_flag_size()
    }
    pub fn get_input_bandwith(&self) -> Result<i64> {
        self.socket.get_input_bandwith()
    }
    pub fn get_ip_type_of_service(&self) -> Result<i32> {
        self.socket.get_ip_type_of_service()
    }
    pub fn get_initial_sequence_number(&self) -> Result<i32> {
        self.socket.get_initial_sequence_number()
    }
    pub fn get_ip_time_to_live(&self) -> Result<i32> {
        self.socket.get_ip_time_to_live()
    }
    pub fn get_ipv6_only(&self) -> Result<i32> {
        self.socket.get_ipv6_only()
    }
    pub fn get_km_refresh_rate(&self) -> Result<i32> {
        self.socket.get_km_refresh_rate()
    }
    pub fn get_km_preannounce(&self) -> Result<i32> {
        self.socket.get_km_preannounce()
    }
    pub fn get_linger(&self) -> Result<i32> {
        self.socket.get_linger()
    }
    pub fn get_max_reorder_tolerance(&self) -> Result<i32> {
        self.socket.get_max_reorder_tolerance()
    }
    pub fn get_max_bandwith(&self) -> Result<i64> {
        self.socket.get_max_bandwith()
    }
    pub fn get_mss(&self) -> Result<i32> {
        self.socket.get_mss()
    }
    pub fn get_nak_report(&self) -> Result<bool> {
        self.socket.get_nak_report()
    }
    pub fn get_encryption_key_length(&self) -> Result<i32> {
        self.socket.get_encryption_key_length()
    }
    pub fn get_peer_latency(&self) -> Result<i32> {
        self.socket.get_peer_latency()
    }
    pub fn get_peer_version(&self) -> Result<i32> {
        self.socket.get_peer_version()
    }
    pub fn get_receive_buffer(&self) -> Result<i32> {
        self.socket.get_receive_buffer()
    }
    pub fn get_receive_data(&self) -> Result<i32> {
        self.socket.get_receive_data()
    }
    pub fn get_receive_km_state(&self) -> Result<SrtKmState> {
        self.socket.get_receive_km_state()
    }
    pub fn get_receive_latency(&self) -> Result<i32> {
        self.socket.get_receive_latency()
    }
    pub fn get_receive_blocking(&self) -> Result<bool> {
        self.socket.get_receive_blocking()
    }
    pub fn get_receive_timeout(&self) -> Result<i32> {
        self.socket.get_receive_timeout()
    }
    pub fn get_rendezvous(&self) -> Result<bool> {
        self.socket.get_rendezvous()
    }
    pub fn get_reuse_address(&self) -> Result<bool> {
        self.socket.get_reuse_address()
    }
    pub fn get_send_buffer(&self) -> Result<i32> {
        self.socket.get_send_buffer()
    }
    pub fn get_send_data(&self) -> Result<i32> {
        self.socket.get_send_data()
    }
    pub fn get_send_km_state(&self) -> Result<SrtKmState> {
        self.socket.get_send_km_state()
    }
    pub fn get_send_blocking(&self) -> Result<bool> {
        self.socket.get_send_blocking()
    }
    pub fn get_send_timeout(&self) -> Result<i32> {
        self.socket.get_send_timeout()
    }
    pub fn get_connection_state(&self) -> Result<SrtSocketStatus> {
        self.socket.get_connection_state()
    }
    pub fn get_stream_id(&self) -> Result<String> {
        self.socket.get_stream_id()
    }
    pub fn get_too_late_packet_drop(&self) -> Result<bool> {
        self.socket.get_too_late_packet_drop()
    }
    pub fn get_timestamp_based_packet_delivery_mode(&self) -> Result<bool> {
        self.socket.get_timestamp_based_packet_delivery_mode()
    }
    pub fn get_udp_receive_buffer(&self) -> Result<i32> {
        self.socket.get_udp_receive_buffer()
    }
    pub fn get_udp_send_buffer(&self) -> Result<i32> {
        self.socket.get_udp_send_buffer()
    }
    pub fn get_srt_version(&self) -> Result<i32> {
        self.socket.get_srt_version()
    }
}

impl Read for SrtStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(self.socket.recv(buf)?)
    }
}

impl Write for SrtStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        Ok(self.socket.send(buf)?)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

pub struct SrtBuilder {
    opt_vec: Vec<SrtPreConnectOpt>,
}

impl SrtBuilder {
    pub fn connect<A: ToSocketAddrs>(self, remote: A) -> Result<SrtStream> {
        let socket = SrtSocket::new()?;
        self.config_socket(&socket)?;
        socket.connect(remote)?;
        Ok(SrtStream { socket })
    }
    pub fn listen<A: ToSocketAddrs>(self, addr: A, backlog: i32) -> Result<SrtListener> {
        let socket = SrtSocket::new()?;
        self.config_socket(&socket)?;
        let socket = socket.bind(addr)?;
        socket.listen(backlog)?;
        Ok(SrtListener { socket })
    }
    pub fn rendezvous<A: ToSocketAddrs, B: ToSocketAddrs>(
        self,
        local: A,
        remote: B,
    ) -> Result<SrtStream> {
        let socket = SrtSocket::new()?;
        self.config_socket(&socket)?;
        socket.rendezvous(local, remote)?;
        Ok(SrtStream { socket })
    }
}

impl SrtBuilder {
    #[cfg(target_os = "linux")]
    pub fn set_bind_to_device(mut self, device: &str) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::BindToDevice(device.to_string()));
        self
    }
    pub fn set_connection_timeout(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::ConnTimeO(msecs));
        self
    }
    pub fn set_flight_flag_size(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::FC(packets));
        self
    }
    pub fn set_ip_type_of_service(mut self, tos: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::IpTos(tos));
        self
    }
    pub fn set_ipv4_time_to_live(mut self, hops: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::IpTtl(hops));
        self
    }
    pub fn set_ipv6_only(mut self, value: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Ipv6Only(value));
        self
    }
    pub fn set_km_refresh_rate(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::KmRefreshRate(packets));
        self
    }
    pub fn set_km_preannounce(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::KmPreAnnounce(packets));
        self
    }
    pub fn set_linger(mut self, secs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Linger(secs));
        self
    }
    pub fn set_max_reorder_tolerance(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::LossMaxTtl(packets));
        self
    }
    pub fn set_max_bandwith(mut self, bytes_per_sec: i64) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::MaxBW(bytes_per_sec));
        self
    }
    pub fn set_message_api(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::MessageApi(enable));
        self
    }
    pub fn set_min_version(mut self, version: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::MinVersion(version));
        self
    }
    pub fn set_mss(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Mss(bytes));
        self
    }
    pub fn set_nak_report(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::NakReport(enable));
        self
    }
    pub fn set_packet_filter(mut self, filter: String) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PacketFilter(filter));
        self
    }
    pub fn set_passphrase(mut self, passphrase: String) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Passphrase(passphrase));
        self
    }
    pub fn set_payload_size(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PayloadSize(bytes));
        self
    }
    pub fn set_encryption_key_length(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PBKeyLen(bytes));
        self
    }
    pub fn set_peer_idle_timeout(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PeerIdleTimeO(msecs));
        self
    }
    pub fn set_peer_latency(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PeerLatency(msecs));
        self
    }
    pub fn set_receive_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::RcvBuf(bytes));
        self
    }
    pub fn set_receive_latency(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::RcvLatency(msecs));
        self
    }
    pub fn set_rendezvous(mut self, rendezvous: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Rendezvous(rendezvous));
        self
    }
    pub fn set_retransmission_algorithm(mut self, reduced: bool) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::RetrainsmitAlgo(reduced));
        self
    }
    pub fn set_reuse_address(mut self, reuse_address: bool) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::ReuseAddr(reuse_address));
        self
    }
    pub fn set_live_congestion_controller(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::Congestion(SrtCongestionController::Live));
        self
    }
    pub fn set_file_congestion_controller(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::Congestion(SrtCongestionController::File));
        self
    }
    pub fn set_send_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::SndBuf(bytes));
        self
    }
    pub fn set_send_drop_delay(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::SndDropDelay(msecs));
        self
    }
    pub fn set_stream_id(mut self, id: String) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::StreamId(id));
        self
    }
    pub fn set_enforced_encryption(mut self, enforced: bool) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::EnforcedEncryption(enforced));
        self
    }
    pub fn set_too_late_packet_drop(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::TlPktDrop(enable));
        self
    }
    pub fn set_live_transmission_type(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::TransType(SrtTransmissionType::Live));
        self
    }
    pub fn set_file_transmission_type(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::TransType(SrtTransmissionType::File));
        self
    }
    pub fn set_timestamp_based_packet_delivery_mode(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::TsbPdMode(enable));
        self
    }
    pub fn set_udp_send_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::UdpSndBuf(bytes));
        self
    }
    pub fn set_udp_receive_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::UdpRcvBuf(bytes));
        self
    }
    fn config_socket(self, socket: &SrtSocket) -> Result<()> {
        for opt in self.opt_vec {
            match opt {
                SrtPreConnectOpt::BindToDevice(value) => socket.set_bind_to_device(value)?,
                SrtPreConnectOpt::ConnTimeO(value) => socket.set_connection_timeout(value)?,
                SrtPreConnectOpt::FC(value) => socket.set_flight_flag_size(value)?,
                SrtPreConnectOpt::IpTos(value) => socket.set_ip_type_of_service(value)?,
                SrtPreConnectOpt::IpTtl(value) => socket.set_ipv4_time_to_live(value)?,
                SrtPreConnectOpt::Ipv6Only(value) => socket.set_ipv6_only(value)?,
                SrtPreConnectOpt::KmRefreshRate(value) => socket.set_km_refresh_rate(value)?,
                SrtPreConnectOpt::KmPreAnnounce(value) => socket.set_km_preannounce(value)?,
                SrtPreConnectOpt::Linger(value) => socket.set_linger(value)?,
                SrtPreConnectOpt::LossMaxTtl(value) => socket.set_max_reorder_tolerance(value)?,
                SrtPreConnectOpt::MaxBW(value) => socket.set_max_bandwith(value)?,
                SrtPreConnectOpt::MessageApi(value) => socket.set_message_api(value)?,
                SrtPreConnectOpt::MinVersion(value) => socket.set_min_version(value)?,
                SrtPreConnectOpt::Mss(value) => socket.set_mss(value)?,
                SrtPreConnectOpt::NakReport(value) => socket.set_nak_report(value)?,
                SrtPreConnectOpt::PacketFilter(value) => socket.set_packet_filter(&value)?,
                SrtPreConnectOpt::Passphrase(value) => socket.set_passphrase(&value)?,
                SrtPreConnectOpt::PayloadSize(value) => socket.set_payload_size(value)?,
                SrtPreConnectOpt::PBKeyLen(value) => socket.set_encryption_key_length(value)?,
                SrtPreConnectOpt::PeerIdleTimeO(value) => socket.set_peer_idle_timeout(value)?,
                SrtPreConnectOpt::PeerLatency(value) => socket.set_peer_latency(value)?,
                SrtPreConnectOpt::RcvBuf(value) => socket.set_receive_buffer(value)?,
                SrtPreConnectOpt::RcvLatency(value) => socket.set_receive_latency(value)?,
                SrtPreConnectOpt::RcvSyn(value) => socket.set_receive_blocking(value)?,
                SrtPreConnectOpt::Rendezvous(value) => socket.set_rendezvous(value)?,
                SrtPreConnectOpt::RetrainsmitAlgo(value) => {
                    socket.set_retransmission_algorithm(value)?
                }
                SrtPreConnectOpt::ReuseAddr(value) => socket.set_reuse_address(value)?,
                SrtPreConnectOpt::Congestion(value) => socket.set_congestion_controller(value)?,
                SrtPreConnectOpt::SndBuf(value) => socket.set_send_buffer(value)?,
                SrtPreConnectOpt::SndDropDelay(value) => socket.set_send_drop_delay(value)?,
                SrtPreConnectOpt::StreamId(value) => socket.set_stream_id(&value)?,
                SrtPreConnectOpt::EnforcedEncryption(value) => {
                    socket.set_enforced_encryption(value)?
                }
                SrtPreConnectOpt::TlPktDrop(value) => socket.set_too_late_packet_drop(value)?,
                SrtPreConnectOpt::TransType(value) => socket.set_transmission_type(value)?,
                SrtPreConnectOpt::TsbPdMode(value) => {
                    socket.set_timestamp_based_packet_delivery_mode(value)?
                }
                SrtPreConnectOpt::UdpSndBuf(value) => socket.set_udp_receive_buffer(value)?,
                SrtPreConnectOpt::UdpRcvBuf(value) => socket.set_udp_send_buffer(value)?,
            }
        }
        Ok(())
    }
}

pub struct SrtAsyncStream {
    socket: SrtSocket,
    epoll: Epoll,
}

impl SrtAsyncStream {
    pub fn close(self) -> Result<()> {
        self.socket.close()
    }
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.socket.local_addr()
    }
}

pub struct SrtAsyncBuilder {
    opt_vec: Vec<SrtPreConnectOpt>,
}

pub struct SrtAsyncListener {
    socket: SrtSocket,
    epoll: Epoll,
}

impl SrtAsyncListener {
    pub fn accept(&self) -> Result<(SrtStream, SocketAddr)> {
        let (socket, addr) = self.socket.accept()?;
        Ok((SrtStream { socket }, addr))
    }
    pub fn close(self) -> Result<()> {
        self.socket.close()
    }
    pub fn local_addr(&self) -> Result<SocketAddr> {
        self.socket.local_addr()
    }
}

impl SrtAsyncBuilder {
    pub fn connect<A: ToSocketAddrs>(self, remote: A) -> Result<SrtAsyncStream> {
        let socket = SrtSocket::new()?;
        self.config_socket(&socket)?;
        socket.set_send_blocking(false)?;
        socket.connect(remote)?;
        let mut epoll = Epoll::new()?;
        let events = srt::SRT_EPOLL_OPT::SRT_EPOLL_IN | srt::SRT_EPOLL_OPT::SRT_EPOLL_OUT;
        epoll.add(&socket, &events)?;
        Ok(SrtAsyncStream { socket, epoll })
    }
    pub fn listen<A: ToSocketAddrs>(self, addr: A, backlog: i32) -> Result<SrtAsyncListener> {
        let socket = SrtSocket::new()?;
        self.config_socket(&socket)?;
        let socket = socket.bind(addr)?;
        socket.listen(backlog)?;
        let mut epoll = Epoll::new()?;
        let events = srt::SRT_EPOLL_OPT::SRT_EPOLL_IN;
        epoll.add(&socket, &events)?;
        Ok(SrtAsyncListener { socket, epoll })
    }
    pub fn rendezvous<A: ToSocketAddrs, B: ToSocketAddrs>(
        self,
        local: A,
        remote: B,
    ) -> Result<SrtAsyncStream> {
        let socket = SrtSocket::new()?;
        self.config_socket(&socket)?;
        socket.rendezvous(local, remote)?;
        socket.set_send_blocking(false)?;
        let mut epoll = Epoll::new()?;
        let events = srt::SRT_EPOLL_OPT::SRT_EPOLL_IN | srt::SRT_EPOLL_OPT::SRT_EPOLL_OUT;
        epoll.add(&socket, &events)?;
        Ok(SrtAsyncStream { socket, epoll })
    }
}

impl SrtAsyncBuilder {
    #[cfg(target_os = "linux")]
    pub fn set_bind_to_device(mut self, device: &str) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::BindToDevice(device.to_string()));
        self
    }
    pub fn set_connection_timeout(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::ConnTimeO(msecs));
        self
    }
    pub fn set_flight_flag_size(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::FC(packets));
        self
    }
    pub fn set_ip_type_of_service(mut self, tos: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::IpTos(tos));
        self
    }
    pub fn set_ipv4_time_to_live(mut self, hops: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::IpTtl(hops));
        self
    }
    pub fn set_ipv6_only(mut self, value: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Ipv6Only(value));
        self
    }
    pub fn set_km_refresh_rate(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::KmRefreshRate(packets));
        self
    }
    pub fn set_km_preannounce(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::KmPreAnnounce(packets));
        self
    }
    pub fn set_linger(mut self, secs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Linger(secs));
        self
    }
    pub fn set_max_reorder_tolerance(mut self, packets: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::LossMaxTtl(packets));
        self
    }
    pub fn set_max_bandwith(mut self, bytes_per_sec: i64) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::MaxBW(bytes_per_sec));
        self
    }
    pub fn set_message_api(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::MessageApi(enable));
        self
    }
    pub fn set_min_version(mut self, version: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::MinVersion(version));
        self
    }
    pub fn set_mss(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Mss(bytes));
        self
    }
    pub fn set_nak_report(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::NakReport(enable));
        self
    }
    pub fn set_packet_filter(mut self, filter: String) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PacketFilter(filter));
        self
    }
    pub fn set_passphrase(mut self, passphrase: String) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Passphrase(passphrase));
        self
    }
    pub fn set_payload_size(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PayloadSize(bytes));
        self
    }
    pub fn set_encryption_key_length(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PBKeyLen(bytes));
        self
    }
    pub fn set_peer_idle_timeout(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PeerIdleTimeO(msecs));
        self
    }
    pub fn set_peer_latency(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::PeerLatency(msecs));
        self
    }
    pub fn set_receive_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::RcvBuf(bytes));
        self
    }
    pub fn set_receive_latency(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::RcvLatency(msecs));
        self
    }
    pub fn set_rendezvous(mut self, rendezvous: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::Rendezvous(rendezvous));
        self
    }
    pub fn set_retransmission_algorithm(mut self, reduced: bool) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::RetrainsmitAlgo(reduced));
        self
    }
    pub fn set_reuse_address(mut self, reuse_address: bool) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::ReuseAddr(reuse_address));
        self
    }
    pub fn set_live_congestion_controller(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::Congestion(SrtCongestionController::Live));
        self
    }
    pub fn set_file_congestion_controller(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::Congestion(SrtCongestionController::File));
        self
    }
    pub fn set_send_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::SndBuf(bytes));
        self
    }
    pub fn set_send_drop_delay(mut self, msecs: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::SndDropDelay(msecs));
        self
    }
    pub fn set_stream_id(mut self, id: String) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::StreamId(id));
        self
    }
    pub fn set_enforced_encryption(mut self, enforced: bool) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::EnforcedEncryption(enforced));
        self
    }
    pub fn set_too_late_packet_drop(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::TlPktDrop(enable));
        self
    }
    pub fn set_live_transmission_type(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::TransType(SrtTransmissionType::Live));
        self
    }
    pub fn set_file_transmission_type(mut self) -> Self {
        self.opt_vec
            .push(SrtPreConnectOpt::TransType(SrtTransmissionType::File));
        self
    }
    pub fn set_timestamp_based_packet_delivery_mode(mut self, enable: bool) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::TsbPdMode(enable));
        self
    }
    pub fn set_udp_send_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::UdpSndBuf(bytes));
        self
    }
    pub fn set_udp_receive_buffer(mut self, bytes: i32) -> Self {
        self.opt_vec.push(SrtPreConnectOpt::UdpRcvBuf(bytes));
        self
    }
    fn config_socket(self, socket: &SrtSocket) -> Result<()> {
        for opt in self.opt_vec {
            match opt {
                SrtPreConnectOpt::BindToDevice(value) => socket.set_bind_to_device(value)?,
                SrtPreConnectOpt::ConnTimeO(value) => socket.set_connection_timeout(value)?,
                SrtPreConnectOpt::FC(value) => socket.set_flight_flag_size(value)?,
                SrtPreConnectOpt::IpTos(value) => socket.set_ip_type_of_service(value)?,
                SrtPreConnectOpt::IpTtl(value) => socket.set_ipv4_time_to_live(value)?,
                SrtPreConnectOpt::Ipv6Only(value) => socket.set_ipv6_only(value)?,
                SrtPreConnectOpt::KmRefreshRate(value) => socket.set_km_refresh_rate(value)?,
                SrtPreConnectOpt::KmPreAnnounce(value) => socket.set_km_preannounce(value)?,
                SrtPreConnectOpt::Linger(value) => socket.set_linger(value)?,
                SrtPreConnectOpt::LossMaxTtl(value) => socket.set_max_reorder_tolerance(value)?,
                SrtPreConnectOpt::MaxBW(value) => socket.set_max_bandwith(value)?,
                SrtPreConnectOpt::MessageApi(value) => socket.set_message_api(value)?,
                SrtPreConnectOpt::MinVersion(value) => socket.set_min_version(value)?,
                SrtPreConnectOpt::Mss(value) => socket.set_mss(value)?,
                SrtPreConnectOpt::NakReport(value) => socket.set_nak_report(value)?,
                SrtPreConnectOpt::PacketFilter(value) => socket.set_packet_filter(&value)?,
                SrtPreConnectOpt::Passphrase(value) => socket.set_passphrase(&value)?,
                SrtPreConnectOpt::PayloadSize(value) => socket.set_payload_size(value)?,
                SrtPreConnectOpt::PBKeyLen(value) => socket.set_encryption_key_length(value)?,
                SrtPreConnectOpt::PeerIdleTimeO(value) => socket.set_peer_idle_timeout(value)?,
                SrtPreConnectOpt::PeerLatency(value) => socket.set_peer_latency(value)?,
                SrtPreConnectOpt::RcvBuf(value) => socket.set_receive_buffer(value)?,
                SrtPreConnectOpt::RcvLatency(value) => socket.set_receive_latency(value)?,
                SrtPreConnectOpt::RcvSyn(value) => socket.set_receive_blocking(value)?,
                SrtPreConnectOpt::Rendezvous(value) => socket.set_rendezvous(value)?,
                SrtPreConnectOpt::RetrainsmitAlgo(value) => {
                    socket.set_retransmission_algorithm(value)?
                }
                SrtPreConnectOpt::ReuseAddr(value) => socket.set_reuse_address(value)?,
                SrtPreConnectOpt::Congestion(value) => socket.set_congestion_controller(value)?,
                SrtPreConnectOpt::SndBuf(value) => socket.set_send_buffer(value)?,
                SrtPreConnectOpt::SndDropDelay(value) => socket.set_send_drop_delay(value)?,
                SrtPreConnectOpt::StreamId(value) => socket.set_stream_id(&value)?,
                SrtPreConnectOpt::EnforcedEncryption(value) => {
                    socket.set_enforced_encryption(value)?
                }
                SrtPreConnectOpt::TlPktDrop(value) => socket.set_too_late_packet_drop(value)?,
                SrtPreConnectOpt::TransType(value) => socket.set_transmission_type(value)?,
                SrtPreConnectOpt::TsbPdMode(value) => {
                    socket.set_timestamp_based_packet_delivery_mode(value)?
                }
                SrtPreConnectOpt::UdpSndBuf(value) => socket.set_udp_receive_buffer(value)?,
                SrtPreConnectOpt::UdpRcvBuf(value) => socket.set_udp_send_buffer(value)?,
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
enum SrtPreConnectOpt {
    BindToDevice(String),
    ConnTimeO(i32),
    FC(i32),
    IpTos(i32),
    IpTtl(i32),
    Ipv6Only(i32),
    KmRefreshRate(i32),
    KmPreAnnounce(i32),
    Linger(i32),
    LossMaxTtl(i32),
    MaxBW(i64),
    MessageApi(bool),
    MinVersion(i32),
    Mss(i32),
    NakReport(bool),
    PacketFilter(String),
    Passphrase(String),
    PayloadSize(i32),
    PBKeyLen(i32),
    PeerIdleTimeO(i32),
    PeerLatency(i32),
    RcvBuf(i32),
    RcvLatency(i32),
    RcvSyn(bool),
    Rendezvous(bool),
    RetrainsmitAlgo(bool),
    ReuseAddr(bool),
    Congestion(SrtCongestionController),
    SndBuf(i32),
    SndDropDelay(i32),
    StreamId(String),
    EnforcedEncryption(bool),
    TlPktDrop(bool),
    TransType(SrtTransmissionType),
    TsbPdMode(bool),
    UdpSndBuf(i32),
    UdpRcvBuf(i32),
}

struct Epoll {
    id: i32,
    num_sock: usize,
}

impl Epoll {
    fn new() -> Result<Self> {
        let result = unsafe { srt::srt_epoll_create() };
        if result == -1 {
            error::handle_result(Self { id: 0, num_sock: 0 }, result)
        } else {
            Ok(Self {
                id: result,
                num_sock: 0,
            })
        }
    }
    fn add(&mut self, socket: &SrtSocket, event: &srt::SRT_EPOLL_OPT) -> Result<()> {
        let result = unsafe {
            srt::srt_epoll_add_usock(
                self.id,
                socket.id,
                event as *const srt::SRT_EPOLL_OPT as *const i32,
            )
        };
        if result == 0 {
            self.num_sock += 1;
        }
        error::handle_result((), result)
    }
    fn remove(&mut self, socket: &SrtSocket) -> Result<()> {
        let result = unsafe { srt::srt_epoll_remove_usock(self.id, socket.id) };
        if result == 0 {
            self.num_sock -= 1;
        }
        error::handle_result((), result)
    }
    fn update(&self, socket: &SrtSocket, event: &srt::SRT_EPOLL_OPT) -> Result<()> {
        let result = unsafe {
            srt::srt_epoll_update_usock(
                self.id,
                socket.id,
                event as *const srt::SRT_EPOLL_OPT as *const i32,
            )
        };
        error::handle_result((), result)
    }
    fn wait(&self, timeout: i64) -> Result<Vec<(SrtSocket, srt::SRT_EPOLL_OPT)>> {
        let mut array = vec![srt::SRT_EPOLL_EVENT { fd: 0, events: 0 }; self.num_sock];
        let result = unsafe {
            srt::srt_epoll_uwait(
                self.id,
                array[..].as_mut_ptr() as *mut srt::SRT_EPOLL_EVENT,
                array.len() as c_int,
                timeout,
            )
        };
        let num_changes = error::handle_result(result, result)? as usize;
        array.truncate(num_changes);
        Ok(array
            .iter()
            .map(|event| {
                (
                    SrtSocket { id: event.fd },
                    srt::SRT_EPOLL_OPT(event.events as c_uint),
                )
            })
            .collect())
    }
    fn clear(&mut self) -> Result<()> {
        let result = unsafe { srt::srt_epoll_clear_usocks(self.id) };
        if result == 0 {
            self.num_sock = 0;
        }
        error::handle_result((), result)
    }
}

impl Drop for Epoll {
    fn drop(&mut self) {
        unsafe {
            srt::srt_epoll_release(self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate as srt;
    use std::{
        io::{Read, Write},
        net::SocketAddr,
        sync::mpsc,
        thread,
    };

    #[test]
    fn test_connect_accept() {
        srt::startup().expect("failed startup");
        let (tx, rx) = mpsc::channel::<SocketAddr>();
        thread::spawn(move || {
            let listen = srt::builder()
                .set_file_transmission_type()
                .listen("127.0.0.1:0", 1)
                .expect("fail listen()");
            let local = listen.local_addr().expect("fail local_addr()");
            tx.send(local).expect("fail send through mpsc channel");
            let (mut peer, _peer_addr) = listen.accept().expect("fail accep()");
            peer.write_all(b"testing").expect("fail write()");
            assert!(peer.close().is_ok());
            assert!(listen.close().is_ok());
        });
        let addr = rx.recv().expect("fail recv through mpsc channel");
        let mut connect = srt::builder()
            .set_file_transmission_type()
            .connect(addr)
            .expect("fail connect()");
        let mut buf = Vec::new();
        connect.read_to_end(&mut buf).expect("fail read()");
        assert_eq!(
            std::str::from_utf8(&buf).expect("malformed message"),
            "testing"
        );
        assert!(connect.close().is_ok());
        srt::cleanup().expect("failed cleanup()");
    }
    #[test]
    fn test_rendezvous() {
        srt::startup().expect("failed startup");
        thread::spawn(move || {
            let mut one = srt::builder()
                .set_file_transmission_type()
                .rendezvous("127.0.0.1:123", "127.0.0.1:234")
                .expect("fail rendezvous()");
            one.write_all(b"testing").expect("fail write()");
            assert!(one.close().is_ok());
        });
        let mut two = srt::builder()
            .set_file_transmission_type()
            .rendezvous("127.0.0.1:234", "127.0.0.1:123")
            .expect("fail rendezvous()");
        let mut buf = Vec::new();
        two.read_to_end(&mut buf).expect("fail read()");
        assert_eq!(
            std::str::from_utf8(&buf).expect("malformed message"),
            "testing"
        );
        srt::cleanup().expect("failed cleanup");
    }
}
