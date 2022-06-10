use std::{
    env::{self},
    fs::File,
    io::{self, Write},
    process,
};

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

pub fn dump_sql(sql_info: SqlDumpInfo) -> io::Result<()> {
    let output = process::Command::new("mysqldump")
        .arg(sql_info.name)
        .arg(sql_info.password)
        .arg(sql_info.database)
        .output()
        .unwrap();

    let mut file = File::create(sql_info.save_path)?;

    file.write_all(&output.stdout)?;

    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok().unwrap();

    let sql_dump_info = SqlDumpInfo::new(
        env::var("name")?,
        env::var("password")?,
        env::var("database")?,
        env::var("save_path")?,
    );

    dump_sql(sql_dump_info)?;

    println!("=>文件保存成功!");

    Ok(())
}
