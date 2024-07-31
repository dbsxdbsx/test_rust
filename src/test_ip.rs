use std::error::Error;
use std::fmt;
use std::net::IpAddr;
use std::time::Duration;

use regex::Regex;
use reqwest::Client;
use serde_json::Value;
use tokio::time::timeout;

#[derive(Debug)]
struct IpInfo {
    ip: IpAddr,
    country: String,
    region: String,
    city: String,
}

impl fmt::Display for IpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, 国家: {}, 地区: {}, 城市: {}",
            self.ip, self.country, self.region, self.city
        )
    }
}

impl IpInfo {
    async fn new(ip: IpAddr) -> Result<Self, Box<dyn Error>> {
        let client = Client::new();
        let url = format!("https://ipwhois.app/json/{}?lang=zh-CN", ip);
        let response = client.get(&url).send().await?.json::<Value>().await?;

        let country = response["country"].as_str().unwrap_or("未知").to_string();
        let region = response["region"].as_str().unwrap_or("未知").to_string();
        let city = response["city"].as_str().unwrap_or("未知").to_string();

        Ok(IpInfo {
            ip,
            country,
            region,
            city,
        })
    }
}

async fn addr_v4(url: &str) -> Result<IpInfo, Box<dyn Error>> {
    let client = Client::new();

    let secs = 3;
    let response = match timeout(Duration::from_secs(secs), client.get(url).send()).await {
        Ok(resp) => resp?,
        Err(_) => return Err(format!("请求超时({secs}秒)，请检查网络连接").into()),
    };

    let response_text = match timeout(Duration::from_secs(secs), response.text()).await {
        Ok(text) => text?,
        Err(_) => return Err(format!("获取响应文本超时({secs}秒)，请检查网络连接").into()),
    };

    let ip_str = match extract_ip_from_fetched_content(&response_text) {
        Ok(ip) => ip,
        Err(_) => return Err("无法从响应文本中提取IP地址".into()),
    };

    let ip: IpAddr = match ip_str.parse() {
        Ok(ip) => ip,
        Err(_) => return Err("解析IP地址失败".into()),
    };

    let ip_info = IpInfo::new(ip).await?;
    Ok(ip_info)
}

fn extract_ip_from_fetched_content(html: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(
        r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
    )?;
    if let Some(captures) = re.find(html) {
        Ok(captures.as_str().to_string())
    } else {
        Err("无法从响应文本中提取IP地址".into())
    }
}

pub async fn test() -> Result<(), Box<dyn Error>> {
    let services = vec![
        ("国内网站", "http://ip.3322.net"),
        ("国外网站", "https://api.ipify.org"),
    ];

    for (description, url) in services {
        match addr_v4(url).await {
            Ok(info) => {
                println!("通过{}{}获取的公网IP信息: {}", description, url, info.ip);
                println!("IP 位置信息: {}", info);
            }
            Err(e) => println!("无法从{}{}获取公网IP: {}", description, url, e),
        }
    }

    Ok(())
}
