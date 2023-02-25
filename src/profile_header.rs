use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
// Using Option<String> because sometimes the values are null, and it gets panic.
struct User {
    login: String,
    name: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    location: Option<String>,
    bio: Option<String>,
    twitter_username: Option<String>,
    email: Option<String>,
    public_repos: Option<String>,
    public_gists: Option<String>,
    followers: Option<String>,
    following: Option<String>,
    created_at: String,
    updated_at: String,
}

// it solves the None part of the panic when I will do the .unwrap() for Option<String>
impl User {
    fn new(
        login: String,
        name: Option<String>,
        company: Option<String>,
        blog: Option<String>,
        location: Option<String>,
        bio: Option<String>,
        twitter_username: Option<String>,
        email: Option<String>,
        public_repos: Option<String>,
        public_gists: Option<String>,
        followers: Option<String>,
        following: Option<String>,
        created_at: String,
        updated_at: String,
    ) -> Self {
        User {
            login,
            name: Some(name.unwrap_or_else(|| "NA".to_string())),
            company: Some(company.unwrap_or_else(|| "NA".to_string())),
            blog: Some(blog.unwrap_or_else(|| "NA".to_string())),
            location: Some(location.unwrap_or_else(|| "NA".to_string())),
            bio: Some(bio.unwrap_or_else(|| "NA".to_string())),
            twitter_username: Some(twitter_username.unwrap_or_else(|| "NA".to_string())),
            email: Some(email.unwrap_or_else(|| "NA".to_string())),
            public_repos: Some(public_repos.unwrap_or_else(|| "0".to_string())),
            public_gists: Some(public_gists.unwrap_or_else(|| "0".to_string())),
            followers: Some(followers.unwrap_or_else(|| "0".to_string())),
            following: Some(following.unwrap_or_else(|| "0".to_string())),
            created_at,
            updated_at,
        }
    }
}

#[tokio::main]
async fn main_info(
    user: &str,
    secret_key: String,
) -> Result<
    &[String],
    Box<dyn std::error::Error>,
> {
    let client = Client::new();
    let request_url = format!("https://api.github.com/users/{user}");

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(reqwest::header::USER_AGENT, "{secret_key}".parse().unwrap());

    let response = client.get(&request_url).headers(headers).send().await?;

    let body = response.text().await?;

    let github_data = match serde_json::from_str::<User>(&body) {
        Ok(user) => User::new(
            user.login,
            user.name,
            user.company,
            user.blog,
            user.location,
            user.bio,
            user.twitter_username,
            user.email,
            user.public_repos,
            user.public_gists,
            user.followers,
            user.following,
            user.created_at,
            user.updated_at,
        ),
        Err(e) => return Err(e.into()),
    };

    let data_vec = vec![
        github_data.login,
        github_data.name.unwrap(),
        github_data.company.unwrap(),
        github_data.blog.unwrap(),
        github_data.location.unwrap(),
        github_data.bio.unwrap(),
        github_data.twitter_username.unwrap(),
        github_data.email.unwrap(),
        github_data.public_repos.unwrap(),
        github_data.public_gists.unwrap(),
        github_data.followers.unwrap(),
        github_data.following.unwrap(),
        github_data.created_at,
        github_data.updated_at,
    ];

    Ok(data_vec)
}

pub fn start_header_info(
    user: &str,
    secret_key: String,
) -> Vec<
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
> {
    let data = main_info(user, secret_key).unwrap();
    data
}
