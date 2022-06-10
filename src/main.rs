use std::{env, process};

use chrono::Utc;
use reqwest::{Client, StatusCode};

pub static API_URL: &str = "http://v0.api.upyun.com/";
pub struct SqlDumpInfo {
    pub name: String,
    pub password: String,
    pub database: String,
    pub save_path: String,
}

impl SqlDumpInfo {
    pub fn new(name: String, password: String, database: String, save_path: String) -> Self {
        let mut new_name = String::from("-u");
        new_name.push_str(name.as_str());

        let mut new_password = String::from("-p");
        new_password.push_str(password.as_str());

        Self {
            name: new_name,
            password: new_password,
            database,
            save_path,
        }
    }
}

pub async fn init() {
    dotenv::dotenv().ok().unwrap();
}

pub async fn dump_sql() -> Result<String, anyhow::Error> {
    let sql_dump_info = SqlDumpInfo::new(
        env::var("name")?,
        env::var("password")?,
        env::var("database")?,
        env::var("save_path")?,
    );

    let output = process::Command::new("mysqldump")
        .arg(sql_dump_info.name)
        .arg(sql_dump_info.password)
        .arg(sql_dump_info.database)
        .output()
        .unwrap();

    Ok(String::from_utf8(output.stdout)?)
}

async fn remote_upload_file(file_content: String) -> Result<(), anyhow::Error> {
    let mut file_name = String::from("/sql_dump/dump_");
    file_name.push_str(Utc::now().to_string().as_str());
    file_name.push_str(".sql");

    let request_url = format!("{}{}/{}", API_URL, "smile-uyun", file_name);

    let resp = Client::new()
        .post(request_url.as_str())
        .basic_auth(env::var("operator")?, Some(env::var("operator_password")?))
        .body(file_content.as_bytes().to_vec())
        .send()
        .await?;

    if resp.status() == StatusCode::OK {
        println!("成功!");
    } else {
        println!("失败, resp text {}", resp.text().await?);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init().await;

    remote_upload_file(dump_sql().await?).await?;

    Ok(())
}
