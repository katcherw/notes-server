/*
 * Entry point and bulk of application.
 */

use std::fs;
use clap::Parser;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Server, Response};
use orgize::Org;
use pulldown_cmark::{Parser as MdParser, Options as MdOptions, html::push_html};

mod base_html;
mod css;

const NOTES_URL_PREFIX: &str = "/notes/";

#[derive(Parser, Debug)]
#[command(name = "notes-server")]
#[command(about = "View your org and md files in a web browser")]
struct Args {
    /// Directory containing .org and .md files
    dir: String,
}

// walks given directory tree and finds all supported files. Returns relative
// file names.
fn get_fnames(base_dir: &Path) -> Vec<PathBuf> {
    let mut fnames: Vec<PathBuf> = Vec::new();
    let walker = walkdir::WalkDir::new(&base_dir);
    for entry in walker.into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() &&
            entry.path().extension()
            .and_then(|s| s.to_str())
            .map(|ext| ["org", "md"].contains(&ext))
            == Some(true) {
                if let Ok(suffix) = entry.path().strip_prefix(base_dir) {
                    fnames.push(suffix.to_path_buf());
                }
        }
    }
    fnames
}

// returns html string for main page showing list of files
fn render_index_page(files: &Vec<PathBuf>) -> String {
    let mut html = String::from(css::CSS);
    html += base_html::BASE_HTML;
    html += "<body>";
    html.push_str(r#"<div class="index-panel">"#);
    html.push_str(r#"<div class="search-bar">"#);
      html.push_str(r#"<input type="text" placeholder="Search..." />"#);
      html.push_str(r#"<button>Search</button>"#);
    html.push_str("</div>");
    html.push_str(r#"<div class="index-list">"#);
    html.push_str("<h1>Index</h1>");
    html.push_str("<ul>");
    for key in files {
        html.push_str(&format!("<li><a href=\"{}{}\">{}</a></li>", NOTES_URL_PREFIX, key.to_string_lossy(), key.to_string_lossy()));
    }
    html.push_str("</ul>");
    html.push_str("</div>");
    html.push_str("</div>");
    // html.push_str("<form method=\"get\" action=\"/search\">Search: <input type=\"text\" name=\"q\"><input type=\"submit\" value=\"Search\"></form>");
    html.push_str("</body></html>");
    html
}

// parses org file and returns html string
fn parse_note_org(base_path: &Path, note_name: &str) -> String {
    if let Ok(content) = fs::read_to_string(base_path.join(note_name)) {
        let mut writer = Vec::new();
        Org::parse(&content).write_html(&mut writer).unwrap();
        return String::from_utf8(writer).unwrap();
    }
    String::new()
}

// parses md file and returns html string
fn parse_note_md(base_path: &Path, note_name: &str) -> String {
    if let Ok(content) = fs::read_to_string(base_path.join(note_name)) {
        let parser = MdParser::new_ext(&content, MdOptions::all());
        let mut html_output = String::new();
        push_html(&mut html_output, parser);
        return html_output;
    }
    String::new()
}

// figures out what type of note it is and parses appropriate one
fn parse_note(base_path: &Path, note_name: &str) -> String {
    if note_name.ends_with(".org") {
        return parse_note_org(base_path, note_name);
    }
    else if note_name.ends_with(".md") {
        return parse_note_md(base_path, note_name);
    }
    String::new()
}

// reads given note and returns formatted html for the page
fn render_note(base_path: &Path, note_name: &str) -> String {
    let mut html = String::from(css::CSS);
    html += base_html::BASE_HTML;
    html += "<body>";
    html.push_str(r#"<div class="note-titlebar">"#);
    html.push_str(note_name);
    html.push_str(r#"</div>"#);
    html.push_str(r#"<div class="notes-panel">"#);

    let html_note = parse_note(&base_path, &note_name);
    html.push_str(&html_note);

    html.push_str(r#"</div>"#);
    html.push_str("</body></html>");
    html
}

// runs the web server, displaying links for given file names. Never returns.
fn run_server(base_path: &Path, fnames: &Vec<PathBuf>, port: u16) {
    let server = Server::http(("0.0.0.0", port)).unwrap();
    println!("Serving Org/Markdown files on http:))//localhost:{}", port);

    for request in server.incoming_requests() {
        let url = request.url();
        let header = Header::from_bytes(&b"Content-Type"[..], &b"text/html"[..]).unwrap();
        //let map = org_map.read().unwrap();
        println!("url: {}", url);

        if url == "/" {
            let html_index = render_index_page(&fnames);
            let response = Response::from_string(html_index).with_header(header);
            let _ = request.respond(response);
        }
        else if url.starts_with(NOTES_URL_PREFIX) {
            let note_name = url.strip_prefix(NOTES_URL_PREFIX).unwrap();
            let html = render_note(&base_path, &note_name);
            let response = Response::from_string(html).with_header(header);
            let _ = request.respond(response);
        }
        else {
            let response = Response::from_string("404 Not Found").with_status_code(404).with_header(header);
            let _ = request.respond(response);
        }
    }
}

fn main() {
    let args = Args::parse();
    let dir = Path::new(&args.dir);
    
    let fnames = get_fnames(&dir);

    run_server(dir, &fnames, 8001);
}
