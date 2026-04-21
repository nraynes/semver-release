pub struct Env {
    github_token: String,
    github_actor: String,
    github_repository: String,
}

impl Env {
    pub fn github_token(&self) -> &str {
        &self.github_token
    }

    pub fn github_actor(&self) -> &str {
        &self.github_actor
    }

    pub fn github_repository(&self) -> &str {
        &self.github_repository
    }

    pub fn new(github_token: &str, github_actor: &str, github_repository: &str) -> Self {
        Env {
            github_token: String::from(github_token),
            github_actor: String::from(github_actor),
            github_repository: String::from(github_repository),
        }
    }
}
