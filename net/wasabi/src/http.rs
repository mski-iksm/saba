extern crate alloc;
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use noli::net::SocketAddr;
use noli::net::TcpStream;
use noli::net::lookup_host;
use saba_core::error::Error;
use saba_core::http::HttpResponse;

pub struct HttpClient {}

impl HttpClient {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, host: String, port: u16, path: String) -> Result<HttpResponse, Error> {
        // ホスト名からIPアドレスを取得
        let ip_address = lookup_host(&host);
        let ips = match ip_address {
            Ok(res) => res,
            Err(e) => return Err(Error::Network(format!("No host found: {:#?}", e))),
            // Errはdebugトレイトを実装しているので{:#?}で表示できる
            // トレイトを実装しているとは、特定のメソッドを持っていること。インターフェイスを実装していると思えば良さそう。
        };
        if ips.len() < 1 {
            return Err(Error::Network("No host found".to_string()));
        }

        // ソケットアドレスを定義
        let socket_addr: SocketAddr = (ips[0], port).into();
        // into: Fromトレイトを実装している場合、intoメソッドを使って変換できる

        // ストリームを構築
        let mut stream = match TcpStream::connect(socket_addr) {
            Ok(res) => res,
            Err(e) => return Err(Error::Network(format!("Failed to connect: {:#?}", e))),
        };

        // リクエストラインを構築
        let mut request = format!("Get /{} HTTP/1.1\n", &path);

        // ヘッダーを構築
        let header = format!("Host: {}\nAccept: text/html\nConnection: close\n\n", &host);
        request.push_str(&header);

        // リクエストの送信
        let _bytes_written: usize = match stream.write(request.as_bytes()) {
            Ok(bytes) => bytes,
            Err(e) => return Err(Error::Network("Failed to write".to_string())),
        };
        // 何バイトを送ったかが返ってくるが、不要なので_で無視

        // レスポンスの受信
        let mut recieved = Vec::new();
        loop {
            let mut buf = [0u8; 4096];
            // u8型の配列に0を4096個入れる

            let bytes_read = match stream.read(&mut buf) {
                Ok(bytes) => bytes,
                Err(_) => {
                    return Err(Error::Network(
                        "Failed to revieve a request from TCP stream".to_string(),
                    ));
                }
            };
            if bytes_read == 0 {
                break;
            }
            recieved.extend_from_slice(&buf[..bytes_read]);
        }

        // レスポンスをパースして返す
        return match core::str::from_utf8(&recieved) {
            Ok(res) => HttpResponse::new(res.to_string()),
            Err(e) => Err(Error::Network(format!(
                "Invalid recieved response: {:#?}",
                e
            ))),
        };
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    #[test]
    fn test_status_line_only() {
        let raw = "HTTP/1,1 200 OK\n\n".to_string();
        let response = HttpResponse::new(raw);

        assert_eq!(response.version(), "HTTP/1.1");
        assert_eq!(response.status_code(), 200);
        assert_eq!(response.reason(), "OK");
    }

    #[test]
    fn test_one_header() {
        let raw = "HTTP/1.1 200 OK\nDate: Fri, 03 Jan 2025 08:12:52 GMT\n\n".to_string();
        let response = HttpResponse::new(raw);

        assert_eq!(response.version(), "HTTP/1.1");
        assert_eq!(response.status_code(), 200);
        assert_eq!(response.reason(), "OK");

        assert_eq!(
            response.header_value("Date"),
            Ok("Fri, 03 Jan 2025 08:12:52 GMT".to_string())
        );
    }

    #[test]
    fn test_two_header_with_white_space() {
        let raw = "HTTP/1.1 200 OK\nDate: Fri, 03 Jan 2025 08:12:52 GMT\nContent-Type: text/html; charset=ISO-8859-1\n\n".to_string();
        let response = HttpResponse::new(raw);

        assert_eq!(response.version(), "HTTP/1.1");
        assert_eq!(response.status_code(), 200);
        assert_eq!(response.reason(), "OK");

        assert_eq!(
            response.header_value("Date"),
            Ok("Fri, 03 Jan 2025 08:12:52 GMT".to_string())
        );
        assert_eq!(
            response.header_value("Content-Type"),
            Ok("text/html; charset=ISO-8859-1".to_string())
        );
    }

    #[test]
    fn test_body() {
        let raw = "HTTP/1.1 200 OK\nDate: Fri, 03 Jan 2025 08:12:52 GMT\nContent-Type: text/html; charset=ISO-8859-1\n\n<html>body</html>".to_string();
        let response = HttpResponse::new(raw);

        assert_eq!(response.version(), "HTTP/1.1");
        assert_eq!(response.status_code(), 200);
        assert_eq!(response.reason(), "OK");

        assert_eq!(
            response.header_value("Date"),
            Ok("Fri, 03 Jan 2025 08:12:52 GMT".to_string())
        );
        assert_eq!(
            response.header_value("Content-Type"),
            Ok("text/html; charset=ISO-8859-1".to_string())
        );
        assert_eq!(response.body(), "<html>body</html>");
    }

    #[test]
    fn test_invalid() {
        // \n\nがないのでどこまでheaderかわからずNG
        let raw = "HTTP/1.1 200 OK".to_string();
        assert!(HttpResponse::new(raw).is_err());
    }
}
