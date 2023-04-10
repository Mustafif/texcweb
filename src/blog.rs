mod page;

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use toml::{to_string_pretty, from_str};
use chrono::NaiveDate;
use std::fs::read_dir;
use tokio::fs::{File, write};
use tokio::io::AsyncWriteExt;
use tokio::io::Result;
use tokio::spawn;

/// A post's file will be in the following structure:
/// posts/
///     {issue}~{title}~YYYY-MM-DD.md
/// So we can get the following fields by first splitting the string by `~`
/// We will get the title from the left, and date to the right
/// Reading the file and converting to html, we get the content.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Post{
    issue: usize,
    title: String,
    date: String,
    content: String,
}

impl Post{
    pub fn new(path: PathBuf, file_name: String) -> Self{
        // remove extension
        let file_name = file_name.trim().replace(".md", "");
        // split the string
        let v: Vec<&str> = file_name.split("~").collect();
        // get the issue number
        let issue: usize = v[0].trim().parse().unwrap();
        // get the title
        let title = v[1].to_string();
        // get the date
        let date = v[2].to_string();
        // turn the markdown to html
        let content = markdown::file_to_html(path.as_path()).unwrap();
        // create a new post
        Self{
            issue,
            title,
            date,
            content
        }
    }
    pub fn stage(&self) -> (PathBuf, String){
        let file_name = format!("{}.html", self.title);
        let path = PathBuf::from("blog_views").join(&file_name);
        let page = page::get_page(&self.title, &self.date, &self.content);
        (path, page)
    }
    pub fn row(&self) -> String{
        let title = &self.title;
        let issue = if self.issue < 10{
            format!("0{}", &self.issue)
        } else{
            self.issue.to_string()
        };
        format!(r"<tr>
        <td><a href='/blog/{}'>Issue {}</a></td>
        <td> {} </td>
        </tr>
        ", title, issue, title)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Posts{
    posts: Vec<Post>
}

impl Posts{
    pub fn new() -> Result<Self>{
        let mut posts = Vec::new();
        for entry in read_dir("posts")?{
            let entry = entry?;
            let path = entry.path();
            let filename = entry.file_name().into_string().unwrap();
            let post = Post::new(path, filename);
            posts.push(post);
        }
        Ok(Self{posts})
    }
    pub async fn build(&self) -> Result<String>{
        let path = PathBuf::from("posts.toml");
        let data = to_string_pretty(&self).unwrap();
        let mut toml = File::create(path).await?;
        toml.write_all(data.as_bytes()).await?;
        let mut tasks = Vec::new();
        let mut rows = Vec::new();
        for post in &self.posts{
            rows.push(post.row());
            let (path, content) = post.stage();
            let task = spawn(async move {
                write(path, content.as_bytes()).await
            });
            tasks.push(task)
        }
        for t in tasks{
            t.await??;
        }
        Ok(rows.join("\n"))
    }
}