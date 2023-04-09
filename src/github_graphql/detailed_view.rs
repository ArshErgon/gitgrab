use ::reqwest::blocking::Client;
use anyhow::{Context, Result};
use colorful::{Color, Colorful};
use graphql_client::{reqwest::post_graphql_blocking as post_graphql, GraphQLQuery};
use std::collections::HashMap;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/graph/schema.graphql",
    query_path = "src/github_graphql/query.graphql",
    response_derives = "Debug"
)]

struct Kusa;
type URI = String;
type DateTime = String;

#[derive(Debug)]
pub struct RepositoriesInformation {
    pub key: String,
    pub stargazer_count: String,
    pub description: String,
    pub lang: String,
    pub fork_count: String,
    pub repo_url: String,
    pub updated_at: String,
    pub created_at: String,
    pub request: String,
}

pub fn get_graphql_info(
    username: String,
    secret_key: &str,
) -> (
    HashMap<String, String>,
    HashMap<String, u32>,
    HashMap<String, RepositoriesInformation>,
    String,
) {
    let data = user_authentication(username, secret_key);
    let error_msg = format!("
    {0}
    This error can happened because of the following
    1. User doesn't exists (recheck your username).
    2. Organization support is not available right now
    3. The token request is exceed (https://docs.github.com/en/apps/creating-github-apps/creating-github-apps/rate-limits-for-github-apps)
    ", "An Error Occuried".color(Color::Red));
    let return_data = match data {
        Ok(raw_data) => filter_out_data(raw_data),
        Err(_) => {
            eprint!("{error_msg}");
            std::process::exit(0);
        }
    };
    return_data
}

fn user_authentication(user_name: String, secret_key: &str) -> Result<kusa::ResponseData> {
    let variables = kusa::Variables { user_name };
    let client = Client::builder()
        .user_agent("graphql-rust/0.10.0")
        .default_headers(
            std::iter::once((
                reqwest::header::AUTHORIZATION,
                reqwest::header::HeaderValue::from_str(&format!("Bearer {}", secret_key)).unwrap(),
            ))
            .collect(),
        )
        .build()?;
    let response_body =
        post_graphql::<Kusa, _>(&client, "https://api.github.com/graphql", variables)?;
    response_body.data.context("failed to fetch data")
}

// getting the data out in hashmaps for easy retrival
fn filter_out_data(
    response_data: kusa::ResponseData,
) -> (
    HashMap<String, String>,
    HashMap<String, u32>,
    HashMap<String, RepositoriesInformation>,
    String,
) {
    const EMPTY: &str = "NA";
    let mut filter_data_map: HashMap<String, String> = HashMap::new();
    let mut languages: HashMap<String, u32> = HashMap::new();
    let mut fork_count = 0;
    let mut top_repositories: HashMap<String, RepositoriesInformation> = HashMap::new();
    let mut string_year = String::new();

    match response_data.user {
        Some(user) => {
            filter_data_map.insert(
                "bio".to_string(),
                user.bio.unwrap_or_else(|| EMPTY.to_string()),
            );
            filter_data_map.insert(
                "company".to_string(),
                user.company.unwrap_or_else(|| EMPTY.to_string()),
            );
            filter_data_map.insert("email".to_string(), user.email);

            filter_data_map.insert(
                "location".to_string(),
                user.location.unwrap_or_else(|| EMPTY.to_string()),
            );

            filter_data_map.insert("login".to_string(), user.login);

            filter_data_map.insert(
                "name".to_string(),
                user.name.unwrap_or_else(|| EMPTY.to_string()),
            );
            filter_data_map.insert(
                "twitter_username".to_string(),
                user.twitter_username.unwrap_or_else(|| EMPTY.to_string()),
            );
            filter_data_map.insert(
                "website_url".to_string(),
                user.website_url.unwrap_or_else(|| EMPTY.to_string()),
            );

            match user.followers {
                followers => {
                    filter_data_map
                        .insert("followers".to_string(), followers.total_count.to_string());
                }
            }

            match user.following {
                following => {
                    filter_data_map
                        .insert("following".to_string(), following.total_count.to_string());
                }
            }

            match user.pull_requests {
                requests => {
                    filter_data_map.insert("request".to_string(), requests.total_count.to_string());
                }
            }

            match user.watching {
                watcher => {
                    filter_data_map.insert("watcher".to_string(), watcher.total_count.to_string());
                }
            }

            match user.issues {
                issues => {
                    filter_data_map.insert("issues".to_string(), issues.total_count.to_string());
                }
            }

            match user.starred_repositories {
                stars => {
                    filter_data_map.insert("stars".to_string(), stars.total_count.to_string());
                }
            }

            match user.updated_at {
                update => {
                    filter_data_map.insert("update".to_string(), update);
                }
            }

            match user.contributions_collection {
                mut contribution => {
                    let years = contribution.contribution_years;
                    if years.len() > 1 {
                        string_year = years[years.len() - 1].to_string();
                    } else {
                        string_year = years[0].to_string();
                    }
                    // contribution_years.append(&mut contribution.contribution_years);
                    filter_data_map.insert(
                        "hasAnyContribution".to_string(),
                        contribution.has_any_contributions.to_string(),
                    );
                }
            }

            if let Some(edges) = user.top_repositories.edges.as_ref() {
                for node in edges
                    .iter()
                    .filter_map(|edge| edge.as_ref().map(|e| e.node.as_ref()).flatten())
                {
                    if node.is_fork {
                        continue;
                    }
                    let mut request_count = String::new();
                    let key = node.name.clone();
                    let description = node
                        .description
                        .clone()
                        .unwrap_or_else(|| "No description given".to_string());
                    let stargazer_count = node.stargazer_count;
                    let fork_count = node.fork_count;
                    let repo_url = node.projects_url.clone();
                    let created_at = node.created_at.clone();
                    let updated_at = node.updated_at.clone();
                    let request = node.clone().pull_requests.total_count.to_string();
                    if let Some(lang) = &node.primary_language {
                        let lang = lang.name.clone();
                        let data = RepositoriesInformation {
                            key: key.clone(),
                            stargazer_count: stargazer_count.to_string(),
                            description,
                            lang,
                            fork_count: fork_count.to_string(),
                            repo_url,
                            created_at,
                            updated_at,
                            request,
                        };
                        top_repositories.insert(key, (data));
                    }
                }
            } else {
                println!("No repos found");
            }
            match user.repositories {
                repo => {
                    filter_data_map.insert("repo".to_string(), repo.total_count.to_string());
                    if let Some(ref nodes) = repo.nodes {
                        nodes
                            .iter()
                            .filter_map(|node| if let Some(n) = node { Some(n) } else { None })
                            .filter_map(|node| node.primary_language.as_ref())
                            .for_each(|lang| {
                                *languages.entry(lang.name.to_string()).or_insert(0) += 1;
                            });
                    }

                    if let Some(nodes) = repo.nodes {
                        nodes
                            .iter()
                            .filter_map(|node| if let Some(n) = node { Some(n) } else { None })
                            .for_each(|node| {
                                fork_count += node.forks.total_count;
                            });
                    };
                }
            }
        }
        None => {
            let error_msg = format!(
                r"
Error, could not find information about the user
This error can happened because of the following
1. User doesn't exists (recheck your username).
2. Organization support is not available right now
3. The token request is exceed (https://docs.github.com/en/apps/creating-github-apps/creating-github-apps/rate-limits-for-github-apps)"
            );
            eprintln!("{error_msg}");

            std::process::exit(0)
        }
    }
    filter_data_map.insert("fork".to_string(), fork_count.to_string());
    let total_value: u32 = languages.values().sum();
    for (key, val) in languages.clone() {
        let mut percentage = (((val + 10) as f32 / total_value as f32) * 100.0) as u32;
        if percentage > 100 {
            percentage = 100
        }
        languages.insert(key.to_string(), percentage);
    }
    (filter_data_map, languages, top_repositories, string_year)
}
