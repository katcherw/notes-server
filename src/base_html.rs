pub const BASE_HTML: &str = r#"
<!DOCTYPE html>
<html lang="en">
	<head>
		<meta charset="utf-8">
		<meta http-equiv="X-UA-Compatible" content="IE=edge">
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <link rel="preconnect" href="https://fonts.googleapis.com">
        <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
        <link href="https://fonts.googleapis.com/css2?family=Inconsolata:wght@200..900&family=Nunito:ital,wght@0,200..1000;1,200..1000&display=swap" rel="stylesheet">
		<title>Note Server</title>
	</head>
	<body>
        <div class="headerbar">
            <div class="productname">
              <a href=/ class="productname-link-style" > Note Server</a>
            </div>
        </div>
	</body>
</html>
"#;
