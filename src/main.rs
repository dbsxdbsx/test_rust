use std::error::Error;
use std::net::IpAddr;

use regex::Regex;
use reqwest::Client;

async fn addr_v4(url: &str) -> Result<Option<IpAddr>, Box<dyn Error>> {
    let client = Client::new();
    let response = client.get(url).send().await?.text().await?;
    let ip_str = extract_ip_from_fetched_content(&response)?;
    let ip: IpAddr = ip_str.parse()?;
    Ok(Some(ip))
}

fn extract_ip_from_fetched_content(html: &str) -> Result<String, Box<dyn Error>> {
    let re = Regex::new(
        r"\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\b",
    )?;
    if let Some(captures) = re.find(html) {
        Ok(captures.as_str().to_string())
    } else {
        Err("无法从response中提取 IP 地址".into())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let services = vec![
        ("国内网站", "http://ip.3322.net"),
        ("国外未被封锁网站", "https://api.ipify.org"),
        // ("国外被封锁的网站", "https://www.google.com/search?q=how+to+get+my+ip++from+google&newwindow=1&sca_esv=661c50c18604ea69&sca_upv=1&sxsrf=ADLYWIKqZ6Ukk241018NL0bEARnpeotEjQ%3A1722396922707&source=hp&ei=-rCpZt-wKYKpwPAP5uP2mQw&iflsig=AL9hbdgAAAAAZqm_CkEp6x7FY1QudTm1UgzbUupNgMbY&ved=0ahUKEwjflfTFrNCHAxWCFBAIHeaxPcMQ4dUDCBU&uact=5&oq=how+to+get+my+ip++from+google&gs_lp=Egdnd3Mtd2l6Ih1ob3cgdG8gZ2V0IG15IGlwICBmcm9tIGdvb2dsZTIGEAAYFhgeMgYQABgWGB4yBhAAGBYYHjILEAAYgAQYhgMYigUyCxAAGIAEGIYDGIoFMgsQABiABBiGAxiKBTILEAAYgAQYhgMYigUyCxAAGIAEGIYDGIoFMggQABiABBiiBDIIEAAYgAQYogRIwidQAFjrJXAAeACQAQOYAeYHoAGiPKoBCzItMi4yLjIuNS4zuAEDyAEA-AEBmAILoALCKsICCxAAGIAEGJECGIoFwgIFEAAYgATCAgUQLhiABMICCBAuGIAEGNQCmAMAkgcLMi0yLjIuNS4wLjKgB7Fp&sclient=gws-wiz"),
    ];

    for (description, url) in services {
        match addr_v4(url).await {
            Ok(Some(ip)) => println!("通过 {} 获取的公网IP: {}", description, ip),
            Ok(None) => println!("无法从 {} 获取公网IP", description),
            Err(e) => println!("无法从 {} 获取公网IP: {}", description, e),
        }
    }

    Ok(())
}
