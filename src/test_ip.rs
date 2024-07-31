use anyhow::{anyhow, Result};
use regex::Regex;
use reqwest::Client;
use serde_json::Value;
use std::fmt;
use std::net::IpAddr;
use std::time::{Duration, Instant};
use tokio::time::timeout;

#[derive(Debug)]
struct IpInfo {
    ip: IpAddr,
    country: String,
    region: String,
    city: String,
    latency: Duration,
    url: String,
}

impl fmt::Display for IpInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}, 国家: {}, 地区: {}, 城市: {}, 延时: {:?}, 测试链接: {}",
            self.ip, self.country, self.region, self.city, self.latency, self.url
        )
    }
}

impl IpInfo {
    async fn new(ip_url: &str) -> Result<Self> {
        let (ip, latency) = Self::fetch_ip_info(ip_url).await?;
        let url = ip_url.to_string();
        let mut ip_info = Self::new_with_complete_ip_info(ip, latency).await?;
        ip_info.url = url;
        Ok(ip_info)
    }

    async fn fetch_ip_info(url: &str) -> Result<(IpAddr, Duration)> {
        let client = Client::new();

        let start_time = Instant::now();
        let secs = 3;
        let response = match timeout(Duration::from_secs(secs), client.get(url).send()).await {
            Ok(resp) => resp?,
            Err(_) => return Err(anyhow!("请求超时({secs}秒)，请检查网络连接")),
        };
        let latency = start_time.elapsed();

        let response_text = match timeout(Duration::from_secs(secs), response.text()).await {
            Ok(text) => text?,
            Err(_) => return Err(anyhow!("获取响应文本超时({secs}秒)，请检查网络连接")),
        };

        let ip_str = Self::extract_ip_from_fetched_content(&response_text)?;
        let ip: IpAddr = ip_str.parse().map_err(|_| anyhow!("解析IP地址失败"))?;

        Ok((ip, latency))
    }

    fn extract_ip_from_fetched_content(html: &str) -> Result<String> {
        let re = Regex::new(
            r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
        )?;
        if let Some(captures) = re.find(html) {
            Ok(captures.as_str().to_string())
        } else {
            Err(anyhow!("无法从响应文本中提取IP地址"))
        }
    }

    async fn new_with_complete_ip_info(ip: IpAddr, latency: Duration) -> Result<Self> {
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
            latency,
            url: String::new(),
        })
    }
}

pub async fn test() -> Result<()> {
    let ip_urls = vec!["http://ip.3322.net", "https://api.ipify.org"];

    for ip_url in ip_urls {
        match IpInfo::new(ip_url).await {
            Ok(info) => {
                println!("IP信息: {}", info);
            }
            Err(e) => println!("无法获取公网IP: {}", e),
        }
    }

    Ok(())
}
