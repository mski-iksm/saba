use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(), // ""だと&strになっちゃう。Stringにすることで所有権を持つし、可変にできる。
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    pub fn parse(&mut self) -> Result<Self, String> {
        // Results Ok(T) Err(E) にくるまった値を返す
        // T: 成功時の戻り値の型
        // E: エラー時の戻り値の型
        // Result<T,E> は、TかEをそれぞれ設定
        // 実際に返ってくるのは、Ok(T)かErr(E)のいずれか
        if !self.is_http() {
            return Err("http:// から始まるURLを指定してください".to_string());
        }

        self.host = self.extract_host();
        self.port = self.extract_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        return Ok(self.clone());
    }

    fn is_http(&self) -> bool {
        if self.url.starts_with("http://") {
            return true;
        }
        return false;
    }

    fn extract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/') // 2つに分割
            .collect(); // Vec<&str>になる

        if let Some(index) = url_parts[0].find(':') {
            return url_parts[0][..index].to_string();
            // port番号が含まれる場合はそこを抜く
        } else {
            return url_parts[0].to_string();
        }
    }

    fn extract_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/') // 2つに分割
            .collect(); // Vec<&str>になる

        if let Some(index) = url_parts[0].find(':') {
            return url_parts[0][index + 1..].to_string();
        } else {
            return "80".to_string();
        }
    }
    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/') // 2つに分割
            .collect(); // Vec<&str>になる

        if url_parts.len() < 2 {
            return "".to_string();
        }
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        return path_and_searchpart[0].to_string();
    }
    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, '/') // 2つに分割
            .collect(); // Vec<&str>になる

        if url_parts.len() < 2 {
            return "".to_string();
        }
        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, '?').collect();
        if path_and_searchpart.len() < 2 {
            return "".to_string();
        }
        return path_and_searchpart[1].to_string();
    }

    pub fn host(&self) -> String {
        return self.host.clone(); // clone()でコピーを返す
    }
    pub fn port(&self) -> String {
        return self.port.clone();
    }
    pub fn path(&self) -> String {
        return self.path.clone();
    }
    pub fn searchpart(&self) -> String {
        return self.searchpart.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*; // 親モジュールの関数を使う

    #[test] // テスト関数であることを示すアトリビュート
    fn test_url_host() {
        let url = "http://example.com".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(resulted, expeted);
    }

    #[test]
    fn test_url_host_port() {
        let url = "http://example.com:8080".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(resulted, expeted);
    }

    #[test]
    fn test_url_host_port_path() {
        let url = "http://example.com:8080/path".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "8080".to_string(),
            path: "path".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(resulted, expeted);
    }

    #[test]
    fn test_url_host_path() {
        let url = "http://example.com/path".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "path".to_string(),
            searchpart: "".to_string(),
        });
        assert_eq!(resulted, expeted);
    }

    #[test]
    fn test_url_hostpath_searchpath() {
        let url = "http://example.com/path?search=hoge&key=piyo".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Ok(Url {
            url: url.clone(),
            host: "example.com".to_string(),
            port: "80".to_string(),
            path: "path".to_string(),
            searchpart: "search=hoge&key=piyo".to_string(),
        });
        assert_eq!(resulted, expeted);
    }

    #[test]
    fn test_no_scheme() {
        let url = "example.com".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Err("http:// から始まるURLを指定してください".to_string());
        assert_eq!(resulted, expeted);
    }

    #[test]
    fn test_unsupported_scheme() {
        let url = "ftp://example.com".to_string();
        let resulted = Url::new(url.clone()).parse();
        let expeted = Err("http:// から始まるURLを指定してください".to_string());
        assert_eq!(resulted, expeted);
    }
}
