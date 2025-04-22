/*
 * Contains the css styles as a constant to be added to the html string.
 */

pub const CSS: &str = r#"
<style>
body {
    margin: 0;
    font-family: 'Nunito', serif;
    background-color: #e6eff7;
}

.headerbar {
	color: inherit;
    display: flex;
    justify-content: space-between;
	background-color: #8fcfe3;
    padding: 15px 20px;
    align-items: center;
}

.productname {
	color: #060c4f;
	vertical-align: middle;
	font-size: 2em;
    font-weight: 800;
}

.productname-link-style, .productname-link-style:visited {
	color: #060c4f;
    text-decoration: none;
}

.index-panel {
    max-width: 600px;
    margin: 2rem auto;
    background-color: #e6eff7;
}   

.search-bar {
    display: flex;
    align-items: center;
}

.search-bar input {
    flex: 1;
    padding: 0.5rem;
    font-size: 1rem;
    height: 2.5 rem;
}

.search-bar button {
    padding: 0.5rem 1rem;
    height: 2.5 rem;
    font-size: 1rem;
    background-color: #007bff;
    color: white;
    border: none;
    cursor: pointer;
}

.index-list {
    background-color: #e6eff7;
    color: black;
    padding: 5px 40px;
}

.index-list ul {
    list-style-type: none;
    padding: 0;
}

.index-list li {
    margin-bottom: 10px;
}

.index-list a {
    color: inherit;
    text-decoration: none;
}

.notes-panel {
    max-width: 800px;
    margin: 2rem auto;
    padding: 0 1rem;
    background-color: #e6eff7;
}   

.note-titlebar {
	color: black;
	background-color: #d8e8f2;
    padding: 15px 20px;
    text-align: center;
    align-items: center;
	vertical-align: middle;
	font-size: 2em;
    font-weight: 800;
}

</style>
"#;
