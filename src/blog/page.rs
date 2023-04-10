pub fn get_page(title: &str, date: &str, content: &str) -> String{
    let mut page = PAGE.replace("{title}", title);
    page = page.replace("{date}", date);
    page = page.replace("{content}", content);
    page
}


pub const PAGE: &str = r#"<!DOCTYPE html>
<html lang="en">

<head>
    <title>TexCreate</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
    <link href='https://fonts.googleapis.com/css?family=Dekko' rel='stylesheet'>
    <link href='https://fonts.googleapis.com/css?family=Allerta Stencil' rel='stylesheet'>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/highlight.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/languages/toml.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/languages/bash.min.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.3.1/languages/rust.min.js"></script>
    <script>hljs.highlightAll();</script>
</head>
<style>
    h1,
    h2,
    h3,
    td,
    th {
        color: #3c675e;
        font-weight: bold;
    }
    code {
        color: #3c675e;
        font-size: 12pt;
    }
    h4 {
        text-align: right;
        color: grey;
        font-style: italic;
    }
    p, li {
    font-family: 'Dekko';
    font-size: 14pt;
    }
    .w3-button {
        color: #3c675e;
        background: #e2ecec;
    }

    td.template,
    th.template {
        background: #e2ecec;
    }

    hr {
        width: 50%;
        height: 5px;
        margin-left: auto;
        margin-right: auto;
        background-color: #3c675e;
    }

    pre {
        font-size: 14px;
        border: 2.5px solid #3c675e;
        background-color: #f5f5f5;
        color: #333;
        font-family: monospace;
        white-space: pre-wrap;
        word-wrap: break-word;
    }
</style>

<body>

    <!-- Header -->
    <header class="w3-display-container w3-content w3-center" style="max-width:1500px">
        <img class="w3-image" src="/assets/logo.png" alt="Logo" width="1500" height="600">

        <!-- Navbar (placed at the bottom of the header image) -->
        <div class="w3-bar w3-light-grey w3-round w3-display-bottommiddle w3-hide-small" style="bottom:-16px">
            <a href="/" class="w3-bar-item w3-button">Home</a>
        <a href="/create" class="w3-bar-item w3-button">TexCreate Web</a>
            <a href="https://github.com/MKProj/texcreate" class="w3-bar-item w3-button">Github</a>
            <a href="https://crates.io/crates/texcreate" class="w3-bar-item w3-button">Crates.io</a>
            <a href="https://mkproj.com" class="w3-bar-item w3-button">MKProj</a>
        </div>
    </header>
    <!-- Navbar on small screens -->
    <div class="w3-center w3-light-grey w3-padding-16 w3-hide-large w3-hide-medium">
        <div class="w3-bar w3-light-grey">
        <a href="/" class="w3-bar-item w3-button">Home</a>
        <a href="/create" class="w3-bar-item w3-button">TexCreate Web</a>
            <a href="https://github.com/MKProj/texcreate" class="w3-bar-item w3-button">Github</a>
            <a href="https://crates.io/crates/texcreate" class="w3-bar-item w3-button">Crates.io</a>
            <a href="https://mkproj.com" class="w3-bar-item w3-button">MKProj</a>
        </div>
    </div>

    <div class="w3-content w3-padding-large w3-margin-top">
        <div class="w3-center">
            <!-- Content -->
            <h1>{title}</h1>
            <hr />
        </div>
        <h4>Published: {date}</h2>

            {content}


            <div class="w3-light-grey w3-padding-large w3-padding-32 w3-margin-top" id="contact">
                <footer class="w3-center">
                    <p>Powered by <a href="https://mkproj.com" target="_blank"
                            class="w3-hover-text-green">MKProjects</a>
                    </p>
                </footer>
            </div>

            <!-- End page content -->
    </div>
</body>

</html>
"#;