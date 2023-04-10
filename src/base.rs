pub const BASE: &str = r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <title>TexCreate</title>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <link rel="stylesheet" href="https://www.w3schools.com/w3css/4/w3.css">
</head>
<style>
    h1,
    p,
    td,
    th {
        color: #3c675e;
        font-weight: bold;
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
</style>

<body>

    <!-- Header -->
    <header class="w3-display-container w3-content w3-center" style="max-width:1500px">
        <img class="w3-image" src="/assets/logo.png" alt="Logo" width="1500" height="600">

        <!-- Navbar (placed at the bottom of the header image) -->
        <div class="w3-bar w3-light-grey w3-round w3-display-bottommiddle w3-hide-small" style="bottom:-16px">
            <a href="/create" class="w3-bar-item w3-button">TexCreate Web</a>
            <a href="https://github.com/MKProj/texcreate" class="w3-bar-item w3-button">Github</a>
            <a href="https://crates.io/crates/texcreate" class="w3-bar-item w3-button">Crates.io</a>
            <a href="https://mkproj.com" class="w3-bar-item w3-button">MKProj</a>
        </div>
    </header>
    <!-- Navbar on small screens -->
    <div class="w3-center w3-light-grey w3-padding-16 w3-hide-large w3-hide-medium">
        <div class="w3-bar w3-light-grey">
            <a href="/create" class="w3-bar-item w3-button">TexCreate Web</a>
            <a href="https://github.com/MKProj/texcreate" class="w3-bar-item w3-button">Github</a>
            <a href="https://crates.io/crates/texcreate" class="w3-bar-item w3-button">Crates.io</a>
            <a href="https://mkproj.com" class="w3-bar-item w3-button">MKProj</a>
        </div>
    </div>
    <!-- Page content -->
    <div class="w3-content w3-padding-large w3-margin-top">
        <div class="w3-center">
            <h1>Guides</h1>
            <hr>
            <p><a href="https://mkproj.github.io/texcreate/">Getting Started with TexCreate!</a></p>
            <p><a href="">Creating Templates with TexCGen!</a></p>
        </div>
        <br>
        <div class="w3-center">
            <h1>Documentation</h1>
            <hr>
            <p><a href="https://docs.rs/texcore/latest/texcore/">TexCore Docs.rs</a></p>
            <p><a href="https://docs.rs/texcreate_repo/latest/texcreate_repo/">TexCreate Repo Docs.rs</a></p>
        </div>
    </div>
    <div class="w3-content w3-padding-large w3-margin-top">
        <div class="w3-center">
            <h1>Current Repo Version: v{}</h1>
            <input class="w3-input w3-border w3-padding" type="text" placeholder="> Search for templates.." id="myInput"
                onkeyup="myFunction()">

            <table class="w3-table-all w3-margin-top" id="myTable">
                <tr>
                    <th style="width:60%;">Template Name</th>
                    <th style="width:40%;">Description</th>
                </tr>
                {templates}
            </table>
        </div>
        <br>
        <div class="w3-center">
            <h1>All Releases</h1>
            <table class="w3-table-all">
                <tr>
                    <th>Version</th>
                    <th># of Templates</th>
                    <th>View Release</th>
                </tr>
                {releases}
            </table>
        </div>
        <br/>
        <div class="w3-center">
            <h1>TexCreate Blog</h1>
            <input class="w3-input w3-border w3-padding" type="text" placeholder="> Search for a particular issue..." id="newsInput"
                onkeyup="newsLookup()">
            <table class="w3-table-all w3-margin-top" id="newsTable">
                <tr>
                    <th style="width:40%;">Post Issue</th>
                    <th style="width:60%;">Post Title</th>
                </tr>
                {posts}
            </table>
        </div>


        <!-- Contact -->
        <div class="w3-light-grey w3-padding-large w3-padding-32 w3-margin-top" id="contact">
            <footer class="w3-center">
                <p>Powered by <a href="https://mkproj.com" target="_blank" class="w3-hover-text-green">MKProjects</a>
                </p>
            </footer>

        </div>

        <!-- End page content -->
    </div>
    <script>
        function myFunction() {
            var input, filter, table, tr, td, i;
            input = document.getElementById("myInput");
            filter = input.value.toUpperCase();
            table = document.getElementById("myTable");
            tr = table.getElementsByTagName("tr");
            for (i = 0; i < tr.length; i++) {
                td = tr[i].getElementsByTagName("td")[0];
                if (td) {
                    txtValue = td.textContent || td.innerText;
                    if (txtValue.toUpperCase().indexOf(filter) > -1) {
                        tr[i].style.display = "";
                    } else {
                        tr[i].style.display = "none";
                    }
                }
            }
        }
        function newsLookup() {
            var input, filter, table, tr, td, i;
            input = document.getElementById("newsInput");
            filter = input.value.toUpperCase();
            table = document.getElementById("newsTable");
            tr = table.getElementsByTagName("tr");
            for (i = 0; i < tr.length; i++) {
                td = tr[i].getElementsByTagName("td")[0];
                if (td) {
                    txtValue = td.textContent || td.innerText;
                    if (txtValue.toUpperCase().indexOf(filter) > -1) {
                        tr[i].style.display = "";
                    } else {
                        tr[i].style.display = "none";
                    }
                }
            }
        }
    </script>
</body>

</html>"#;
