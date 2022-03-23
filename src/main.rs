use octocrab::{Octocrab, models};

#[tokio::main]
async fn main() -> octocrab::Result<()> {
    let token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env variable is required");
    let gh_url = std::env::var("GITHUB_URL").expect("GITHUB_URL env variable is required");

    let repo_owner =
        std::env::var("GITHUB_REPO_OWNER").expect("GITHUB_REPO_OWNER env variable is required");
    let repo_name =
        std::env::var("GITHUB_REPO_NAME").expect("GITHUB_REPO_NAME env variable is required");

    let octocrab = Octocrab::builder()
        .base_url(gh_url)
        .unwrap()
        .personal_token(token)
        .build()?;
        

    let repo = octocrab.repos(&repo_owner, &repo_name);
    let repo_info = repo.get().await?;

    println!(
        "{} has {} stars",
        repo_info.full_name.unwrap(),
        repo_info.stargazers_count.unwrap_or(0),
    );


    let mut pr_pages = octocrab
    .pulls(repo_owner, repo_name)
    .list()
    .state(octocrab::params::State::Open)
    .send()
    .await?;

    loop {
        for pr in &pr_pages {

            let  title: String;
            match &pr.title {
                Some(p) => title =  p.to_owned(),
                None => title = pr.id.to_string(),
            };


            println!("{}", title)
        }
        pr_pages = match octocrab
            .get_page::<models::pulls::PullRequest>(&pr_pages.next)
            .await?
    {
        Some(next_page) => next_page,
        None => break,
    }
    }


    Ok(())
}
