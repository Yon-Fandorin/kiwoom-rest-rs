use super::*;

// 명시적인 로컬 연결은 LOGIN 텍스트를 교환하고 미지원 서비스는 전송 전에 거부한다.
#[tokio::test]
async fn local_websocket_mock_connects_and_exchanges_frames() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(socket).await.unwrap();
        let text = ws.next().await.unwrap().unwrap().into_text().unwrap();
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["trnm"], "LOGIN");
        assert_eq!(v["token"], "synthetic");
        ws.send(Message::Text(
            "{\"trnm\":\"LOGIN\",\"return_code\":0}".into(),
        ))
        .await
        .unwrap();
        ws.next().await;
    });
    let mut transport =
        TungsteniteTransport::connect(&format!("ws://{address}"), 1024, Duration::from_secs(2))
            .await
            .unwrap();
    assert!(transport.send("{\"trnm\":\"ORDER\"}".into()).await.is_err());
    transport
        .send("{\"trnm\":\"LOGIN\",\"token\":\"synthetic\"}".into())
        .await
        .unwrap();
    assert_eq!(
        transport.receive().await.unwrap(),
        "{\"trnm\":\"LOGIN\",\"return_code\":0}"
    );
    transport.close().await.unwrap();
    server.await.unwrap();
}

// 오버플로 기한은 연결 시도 전에 거부하며 팩토리에서도 같은 한도를 적용한다.
#[tokio::test]
async fn overflowing_connection_timeout_is_rejected_before_network() {
    assert!(matches!(
        TungsteniteFactory::with_explicit_network_opt_in(Environment::Sandbox, 1024, Duration::MAX),
        Err(ClientError::InvalidRequest(_))
    ));
    assert!(matches!(
        TungsteniteTransport::connect("ws://127.0.0.1:0", 1024, Duration::MAX).await,
        Err(ClientError::InvalidRequest(_))
    ));
}

// 코덱 제어 Ping에는 Pong으로 응답하고 바이너리 데이터는 프로토콜 오류로 거부한다.
#[tokio::test]
async fn local_codec_handles_ping_pong_and_rejects_binary() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(socket).await.unwrap();
        ws.send(Message::Ping(vec![1, 2, 3].into())).await.unwrap();
        ws.send(Message::Pong(vec![4].into())).await.unwrap();
        ws.send(Message::Text("synthetic-event".into()))
            .await
            .unwrap();
        let reply = ws.next().await.unwrap().unwrap();
        assert!(matches!(reply, Message::Pong(payload) if payload.as_ref() == [1, 2, 3]));
        ws.send(Message::Binary(vec![0, 1].into())).await.unwrap();
    });
    let mut transport =
        TungsteniteTransport::connect(&format!("ws://{address}"), 1024, Duration::from_secs(2))
            .await
            .unwrap();
    assert_eq!(transport.receive().await.unwrap(), "synthetic-event");
    assert!(matches!(
        transport.receive().await,
        Err(ClientError::Protocol(_))
    ));
    server.await.unwrap();
}

// 종료 프레임·큰 단일 프레임·작은 조각을 합쳐 상한을 넘긴 메시지는 이벤트로 반환하지 않는다.
#[tokio::test]
async fn local_codec_close_and_oversized_message_fail() {
    for case in ["close", "frame", "message"] {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let mut ws = tokio_tungstenite::accept_async(socket).await.unwrap();
            match case {
                "frame" => ws.send(Message::Text("x".repeat(65).into())).await.unwrap(),
                "message" => {
                    use tokio_tungstenite::tungstenite::protocol::frame::{
                        Frame,
                        coding::{Data, OpCode},
                    };
                    for (opcode, last) in [(Data::Text, false), (Data::Continue, true)] {
                        ws.send(Message::Frame(Frame::message(
                            vec![b'x'; 40],
                            OpCode::Data(opcode),
                            last,
                        )))
                        .await
                        .unwrap();
                    }
                }
                _ => ws.close(None).await.unwrap(),
            }
        });
        let mut transport =
            TungsteniteTransport::connect(&format!("ws://{address}"), 64, Duration::from_secs(2))
                .await
                .unwrap();
        assert!(matches!(
            transport.receive().await,
            Err(ClientError::Transport)
        ));
        server.await.unwrap();
    }
}

// 프로토콜 Pong 홍수도 별도의 유한한 코덱 예산에서 거부하며 텍스트를 무한히 기다리지 않는다.
#[tokio::test]
async fn protocol_control_flood_is_bounded_before_text_delivery() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (socket, _) = listener.accept().await.unwrap();
        let mut ws = tokio_tungstenite::accept_async(socket).await.unwrap();
        for _ in 0..=MAX_PROTOCOL_CONTROLS_PER_RECEIVE {
            if ws.send(Message::Pong(Vec::new().into())).await.is_err() {
                return;
            }
        }
        let _ = ws.send(Message::Text("must-not-arrive".into())).await;
    });
    let mut transport =
        TungsteniteTransport::connect(&format!("ws://{address}"), 1024, Duration::from_secs(2))
            .await
            .unwrap();
    assert!(matches!(
        tokio::time::timeout(Duration::from_secs(2), transport.receive())
            .await
            .unwrap(),
        Err(ClientError::Protocol(
            "WebSocket protocol control frame limit reached"
        ))
    ));
    server.await.unwrap();
}
