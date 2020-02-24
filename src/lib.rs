use chrono::Local;
use base64::encode;
use reqwest;
use reqwest::header::*;
use rust_md5::md5::hash;

fn string_to_hash(input: &str) -> String {
    let vec = input.as_bytes().to_vec();
    let output = &mut Vec::new();
    hash(vec, output);
    hex_to_string(output)
}
fn hex_to_string(bytes: &Vec<u8>) -> String {
    return bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<Vec<_>>()
        .join("");
}

pub fn get_as(device_id:&str) -> String {
    let token = "token://com.coolapk.market/c67ef5943784d09750dcfbb31020f0ab?";
    let package_name = "com.coolapk.market";

    let timestamp = Local::now().timestamp();
    //let timestamp :i64 = 1582379396;
    let timestamp_str= timestamp.to_string();
    let timestamp_md5 = string_to_hash(timestamp_str.as_str());
    let salt = token.to_string() + timestamp_md5.as_str() + "$" + device_id + "&" + package_name;
    let salt_encoded = encode(salt.as_bytes());
    let salt_md5 = string_to_hash(salt_encoded.as_str());
    let token = salt_md5 + device_id + "0x" + format!("{:x}", timestamp).as_str();
    token
}
pub fn test_token(token: String) {
    let client = reqwest::Client::new();
    let mut res = client
        .get("https://api.coolapk.com/v6/main/init")
        .header(USER_AGENT, "Dalvik/2.1.0 (Linux; U; Android 7.1.2; VirtualBox Build/N2G48H) (#Build; Android-x86; VirtualBox; android_x86-userdebug 7.1.2 N2G48H eng.cwhuan.20180502.160334 test-keys; 7.1.2) +CoolMarket/9.0.2")
        .header("X-Requested-With", "XMLHttpRequest")
        .header("X-Sdk-Int", "26")
        .header("X-Sdk-Locale", "zh-CN")
        .header("X-App-Id", "com.coolapk.market")
        .header("X-App-Version", "10.0.1")
        .header("X-App-Code", "2001201")
        .header("X-App-Token", token.as_str())
        .send()
        .unwrap();
    println!("{}", res.text().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        let token =  get_as("00000000-0000-0000-0000-000000000000");
        test_token(token);
    }
}
