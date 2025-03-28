use quinn::{Endpoint, ServerConfig, TransportConfig, VarInt, ClientConfig};
use rustls::{ClientConfig as RustlsClientConfig, ServerConfig as RustlsServerConfig, RootCertStore, Certificate as RustlsCertificate, PrivateKey, ServerName};
use rcgen::{Certificate, CertificateParams};
use serde::{Deserialize, Serialize};
use socket2::{Socket, Domain, Type}; // ✅ `socket2` 사용
use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub order_id: String,
    pub order_type: String,
    pub amount: f64,
    pub price: f64,
}

pub fn build_client_config() -> Result<ClientConfig, Box<dyn Error>> {
    let mut roots = RootCertStore::empty(); // ✅ 루트 인증서 저장소 설정

    let mut rustls_config = RustlsClientConfig::builder()
        .with_safe_defaults() // ✅ 기본 암호화 설정 적용
        .with_custom_certificate_verifier(Arc::new(NoCertificateVerification)) // ✅ `dangerous()` 없이 직접 호출
        .with_no_client_auth(); // ✅ 클라이언트 인증서 비활성화

    // ✅ 직접 루트 인증서 적용
    // rustls_config.root_store = roots;

    Ok(ClientConfig::new(Arc::new(rustls_config)))
}



/// ✅ **모든 인증서를 허용하는 커스텀 인증서 검증기**
struct NoCertificateVerification;


impl rustls::client::ServerCertVerifier for NoCertificateVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate, // ✅ 수정됨
        _intermediates: &[rustls::Certificate], // ✅ 수정됨
        _server_name: &rustls::ServerName,
        _ocsp_response: &mut dyn Iterator<Item = &[u8]>,
        _signature_schemes: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        println!("⚠️ Warning: 인증서 검증 비활성화됨 (테스트 모드)");
        Ok(rustls::client::ServerCertVerified::assertion())
    }
}


/// ✅ **클라이언트 엔드포인트 생성**
pub async fn create_client_endpoint(bind_addr: &str) -> Result<Endpoint, Box<dyn Error>> {
    let addr: SocketAddr = bind_addr.parse()?;

    // ✅ UDP 소켓 생성 및 포트 재사용 설정
    let socket = Socket::new(Domain::IPV4, Type::DGRAM, None)?;
    socket.set_reuse_address(true)?;
    socket.set_nonblocking(true)?;
    socket.bind(&addr.into())?;

    let udp_socket = UdpSocket::from_std(socket.into())?;
    let endpoint = Endpoint::client(udp_socket.local_addr()?)?;
    Ok(endpoint)
}

/// ✅ **QUIC 서버 시작**
pub async fn start_server(addr: &str, tx: mpsc::UnboundedSender<String>) -> Result<(), Box<dyn Error>> {
    let socket_addr: SocketAddr = "[::]:5000".parse()?; // ✅ IPv6 지원
    let server_config = build_server_config()?;
    let endpoint = Endpoint::server(server_config, socket_addr)?;

    println!("✅ QUIC 서버 실행 중: {}", addr);

    loop {
        if let Some(connecting) = endpoint.accept().await {
            let tx = tx.clone();
            tokio::spawn(async move {
                match connecting.await {
                    Ok(connection) => {
                        println!("🔗 새 클라이언트 연결됨: {:?}", connection.remote_address());
                        tokio::spawn(async move {
                            if let Err(e) = handle_connection(connection, tx).await {
                                eprintln!("❌ 연결 처리 중 오류 발생: {:?}", e);
                            }
                        });
                    }
                    Err(e) => eprintln!("❌ 클라이언트 연결 실패: {:?}", e),
                }
            });
        }
    }
}


pub async fn send_order(addr: &str, order: &Order) -> Result<(), Box<dyn Error>> {
    let socket_addr: SocketAddr = addr.parse()?;
    println!("send_order() - target socket_addr: {:?}", socket_addr);

    // ✅ 클라이언트 QUIC 설정 (TLS 포함)
    let client_config = build_client_config()?; // 🔥 최신 클라이언트 설정 사용

    // ✅ 클라이언트 QUIC 엔드포인트 생성
    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    endpoint.set_default_client_config(client_config);

    // ✅ 서버에 연결
    let connection = endpoint.connect(socket_addr, "server")?.await?;
    println!("✅ 서버 연결 성공: {:?}", connection.remote_address());

    // ✅ 주문 데이터 전송
    let mut send_stream = connection.open_uni().await?;
    let order_json = serde_json::to_string(order)?;
    send_stream.write_all(order_json.as_bytes()).await?;

    println!("✅ 주문 전송 성공!");
    
    Ok(())
}




/// ✅ **클라이언트 요청 처리**
async fn handle_connection(connection: quinn::Connection, tx: mpsc::UnboundedSender<String>) -> Result<(), Box<dyn Error>> {
    let mut recv_stream = match connection.accept_uni().await {
        Ok(recv_stream) => recv_stream,
        Err(e) => {
            eprintln!("❌ 스트림 수락 실패: {:?}", e);
            return Err(e.into());
        }
    };

    let mut buf = vec![0; 1024];
    match recv_stream.read(&mut buf).await {
        Ok(Some(n)) => {
            let received_data = String::from_utf8_lossy(&buf[..n]).to_string();
            println!("📩 주문 수신 완료: {}", received_data);
            if let Err(e) = tx.send(received_data) {
                eprintln!("❌ 주문 데이터 전송 실패: {}", e);
            }
        }
        Ok(None) => eprintln!("❌ 클라이언트가 데이터를 전송하지 않음."),
        Err(e) => eprintln!("❌ 데이터 수신 중 오류 발생: {:?}", e),
    }
    Ok(())
}

/// ✅ **QUIC 서버 설정 생성 함수 (TLS 인증서 포함)**
fn build_server_config() -> Result<ServerConfig, Box<dyn Error>> {
    let cert_params = CertificateParams::default();
    let cert = Certificate::from_params(cert_params)?;
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();

    let certs = vec![RustlsCertificate(cert_der)];
    let key = PrivateKey(key_der);

    let rustls_config = RustlsServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(certs, key)?;

    let mut server_config = ServerConfig::with_crypto(Arc::new(rustls_config));
    
    let mut transport = TransportConfig::default();
    transport.keep_alive_interval(Some(std::time::Duration::from_secs(10)));

    if let Some(transport_config) = Arc::get_mut(&mut server_config.transport) {
        transport_config.max_concurrent_uni_streams(VarInt::from_u32(100));
    } else {
        eprintln!("❌ `Arc::get_mut()` 실패: `transport` 수정 불가능");
    }

    Ok(server_config)
}
