use anyhow::Error;
use reqwest::Url;
use std::collections::HashMap;
use term_table::{row::Row, table_cell::TableCell, Table};

#[derive(Debug)]
struct LocStruct {
    language: String,
    files: u64,
    lines: u64,
    blanks: u64,
    comments: u64,
    lines_of_code: u64,
}

fn find_lines(pair: (&str, &str)) -> Result<(HashMap<String, LocStruct>), Error> {
    let (username, repo_name) = pair;
    let mut loc_map: HashMap<String, LocStruct> = HashMap::new();
    let input = format!("https://api.codetabs.com/v1/loc/?github={username}/{repo_name}");
    let url = Url::parse(&input)?;
    let response = reqwest::blocking::get(url).map_err(|err| reqwest::Error::from(err))?;

    if response.status() != 200 {
        eprintln!("Could not find the data associated with {username}/{repo_name}");
        std::process::exit(0)
    }

    crate::get_detailed_view::ascii_text("LoC".to_string());
    let body = response.text()?;
    let data: Vec<HashMap<String, serde_json::Value>> = serde_json::from_str(&body)?;
    for loc_data in data {
        let language = loc_data
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let files = loc_data.get("files").and_then(|v| v.as_u64()).unwrap_or(0);
        let lines_of_code = loc_data
            .get("linesOfCode")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let blanks = loc_data.get("blanks").and_then(|v| v.as_u64()).unwrap_or(0);
        let comments = loc_data
            .get("comments")
            .and_then(|v| v.as_u64())
            .unwrap_or(0);
        let lines = loc_data.get("lines").and_then(|v| v.as_u64()).unwrap_or(0);

        loc_map.insert(
            language.clone(),
            LocStruct {
                language,
                files,
                lines_of_code,
                blanks,
                comments,
                lines,
            },
        );
    }

    Ok(loc_map)
}

fn create_loc_table(data: HashMap<String, LocStruct>, url: String) {
    let mut table = Table::new();
    table.max_column_width = 110;
    table.style = term_table::TableStyle::thin();
    table.add_row(Row::new(vec![
        TableCell::new("Language"),
        TableCell::new_with_alignment("Files", 1, term_table::table_cell::Alignment::Right),
        TableCell::new_with_alignment("Lines", 2, term_table::table_cell::Alignment::Right),
        TableCell::new_with_alignment("Blanks", 3, term_table::table_cell::Alignment::Right),
        TableCell::new_with_alignment("Comments", 3, term_table::table_cell::Alignment::Right),
        TableCell::new_with_alignment("LOC", 3, term_table::table_cell::Alignment::Right),
    ]));

    for (_, loc_data) in data {
        let lang = loc_data.language;
        let file = loc_data.files;
        let lines_of_code = loc_data.lines_of_code;
        let blanks = loc_data.blanks;
        let comments = loc_data.comments;
        let lines = loc_data.lines;
        table.add_row(Row::new(vec![
            TableCell::new(lang),
            TableCell::new_with_alignment(file, 1, term_table::table_cell::Alignment::Right),
            TableCell::new_with_alignment(lines, 2, term_table::table_cell::Alignment::Right),
            TableCell::new_with_alignment(blanks, 3, term_table::table_cell::Alignment::Right),
            TableCell::new_with_alignment(comments, 3, term_table::table_cell::Alignment::Right),
            TableCell::new_with_alignment(
                lines_of_code,
                3,
                term_table::table_cell::Alignment::Right,
            ),
        ]));
    }
    println!("The details of the repo: {url}\n");
    print!("{}", table.render());
}

pub fn start_lines(para_url: String) {
    let url: Vec<&str> = para_url
        .split(|c| c == '/')
        .filter(|s| !s.is_empty())
        .collect();
    if url.len() < 1 {
        eprintln!("invalid url");
        std::process::exit(0);
    }

    let username = url[url.len() - 2];
    let project = url[url.len() - 1];
    match find_lines((username, project)) {
        Ok(data) => {
            create_loc_table(data, para_url);
            std::process::exit(0)
        }
        Err(_) => {
            eprint!("Error");
            std::process::exit(0);
        }
    };
}
