use base64;
use reqwest;
use std::str;

/// 获取订阅链接内容
async fn fetch_subscription(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let bytes = resp.bytes().await?;
    let content = parse_from_base64(bytes);
    Ok(content)
}

fn parse_from_base64<T>(bytes: T) -> String
where
    T: AsRef<[u8]>,
{
    let content = base64::decode(&bytes).unwrap();
    str::from_utf8(&content).unwrap().into()
}

/// 解析服务器配置列表
fn parse_servers(content: &str) -> Result<Vec<String>, base64::DecodeError> {
    content
        .lines()
        .filter(|line| line.len() % 4 == 0) // 只解码长度是 4 的倍数的行
        .map(|line| {
            let line = remove_known_prefixes(line);
            println!("{}", line);
            base64::decode(line)
                .map_err(|e| e)
                .and_then(|config_bytes| {
                    str::from_utf8(&config_bytes)
                        .map(|config| config.to_string())
                        .map_err(|_| base64::DecodeError::InvalidByte(0, 0)) // 这里只是为了类型匹配，实际应该返回正确的错误
                })
        })
        .collect()
}

fn remove_known_prefixes(line: &str) -> &str {
    let prefixes = ["vmess://", "trojan://", "ss://", "ssr://"];
    let mut result = line;

    for prefix in &prefixes {
        result = result.strip_prefix(prefix).unwrap_or(result);
    }

    result
}

pub async fn test_proxy() {
    let url = "https://feed.iggv5.com/c/0c36e375-7374-4f05-af03-eff1f140b06d";
    let fetch_subscription = fetch_subscription(url).await;
    println!("fetched subscription: {:?}", fetch_subscription);
    match fetch_subscription {
        Ok(content) => match parse_servers(&content) {
            Ok(servers) => {
                for server in servers {
                    println!("{}", server);
                }
            }
            Err(e) => eprintln!("解析服务器配置时出错: {}", e),
        },
        Err(e) => eprintln!("请求订阅链接时出错: {}", e),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_base64_str() {
        // parse out this base 64 str:vmess://eyJwb3J0IjozMjgwMCwicHMiOiJpR0ctSEsgLSBcdTk5OTlcdTZlMmZcdTdlZmNcdTU0MDggLSAwMS0gXHU1MDBkXHU3Mzg3IDEiLCJ0bHMiOiIiLCJpZCI6IjBjMzZlMzc1LTczNzQtNGYwNS1hZjAzLWVmZjFmMTQwYjA2ZCIsImFpZCI6IjIiLCJ2IjoiMiIsImhvc3QiOiJicm9hZGNhc3Rsdi5jaGF0LmJpbGliaWxpLmNvbSIsInR5cGUiOiJub25lIiwicGF0aCI6IlwvIiwibmV0Ijoid3MiLCJhZGQiOiJzbWFydC5pZmFzdGNkbmNhY2hlLmNvbSJ9
        let base64_str = "vmess://eyJwb3J0IjozMjgwMCwicHMiOiJpR0ctSEsgLSBcdTk5OTlcdTZlMmZcdTdlZmNcdTU0MDggLSAwMS0gXHU1MDBkXHU3Mzg3IDEiLCJ0bHMiOiIiLCJpZCI6IjBjMzZlMzc1LTczNzQtNGYwNS1hZjAzLWVmZjFmMTQwYjA2ZCIsImFpZCI6IjIiLCJ2IjoiMiIsImhvc3QiOiJicm9hZGNhc3Rsdi5jaGF0LmJpbGliaWxpLmNvbSIsInR5cGUiOiJub25lIiwicGF0aCI6IlwvIiwibmV0Ijoid3MiLCJhZGQiOiJzbWFydC5pZmFzdGNkbmNhY2hlLmNvbSJ9";
        let base64_str = remove_known_prefixes(base64_str);
        println!("{}", base64_str);
        let config = parse_from_base64(base64_str);
        println!("{}", config);
    }
}
