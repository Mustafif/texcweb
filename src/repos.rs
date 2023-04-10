use crate::base::BASE;
use crate::current;
use std::collections::HashMap;
use texcreate_repo::Repo;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt, Result};
use crate::blog::Posts;

fn link(version: u64) -> String {
    format!("https://github.com/MKProj/mkproj_texcgen/releases/tag/v{version}")
}

fn repo_link(version: u64) -> String {
    format!("https://github.com/MKProj/mkproj_texcgen/releases/download/v{version}/repo.toml")
}

#[derive(Debug, Clone)]
pub struct CurrentRelease {
    version: u64,
    info: HashMap<String, String>,
}

impl CurrentRelease {
    pub fn to_html(&self) -> String {
        let mut parts = Vec::new();

        for (name, desc) in self.info.iter() {
            parts.push("<tr>".to_string());
            parts.push(format!("<td>{name}</td>"));
            parts.push(format!("<td>{desc}</td>"));
            parts.push("</tr>".to_string());
        }

        parts.join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct Release {
    version: u64,
    num: u64,
    info: HashMap<String, String>,
    link: String,
}

impl Release {
    pub fn new(repo: &Repo) -> Self {
        Self {
            version: repo.version(),
            num: repo.num(),
            info: repo.info(),
            link: link(repo.version()),
        }
    }
    pub fn as_current(&self) -> CurrentRelease {
        CurrentRelease {
            version: self.version,
            info: self.info.clone(),
        }
    }
    pub fn to_html(&self) -> String {
        let mut parts = Vec::new();
        parts.push("<tr>".to_string());
        parts.push(format!("<td>{}</td>", self.version));
        parts.push(format!("<td>{}</td>", self.num));
        parts.push(format!("<td><a href='{}'>View Here</a></td>", &self.link));
        parts.push("</tr>".to_string());
        parts.join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct Releases {
    rels: Vec<Release>,
}

impl Releases {
    pub async fn new() -> Self {
        let mut rels = Vec::new();

        let curr = current();
        for i in 0..=curr {
            let repo = Repo::get_repo(&repo_link(i)).await;
            let release = Release::new(&repo);
            rels.push(release);
        }
        Self { rels }
    }
    pub fn get_current(&self) -> CurrentRelease {
        let mut releases = self.clone();
        let release = releases.rels.pop().expect("Couldn't get current release");
        release.as_current()
    }
    pub fn to_html(&self) -> String {
        let mut parts = Vec::new();
        for r in &self.rels {
            parts.push(r.to_html());
        }
        parts.join("\n")
    }
}

pub async fn build_index() -> Result<()> {
    let mut base = BASE.to_string();
    let releases = Releases::new().await;
    let curr = releases.get_current();
    base = base.replace("{}", &curr.version.to_string());
    base = base.replace("{releases}", releases.to_html().as_str());
    base = base.replace("{templates}", curr.to_html().as_str());
    let posts = Posts::new()?;
    let posts_data = posts.build().await?;
    base = base.replace("{posts}", &posts_data);
    let mut file = File::create("index.html").await?;
    file.write_all(base.as_bytes()).await?;
    Ok(())
}
