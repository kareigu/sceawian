pub fn repo_build<'a>() -> git2::build::RepoBuilder<'a> {
    let mut repo_build = git2::build::RepoBuilder::new();
    repo_build.fetch_options(fetch_opts());
    return repo_build;
}

pub fn remote_callbacks<'a>() -> git2::RemoteCallbacks<'a> {
    let mut callbacks = git2::RemoteCallbacks::default();
    callbacks.credentials(|_, username, _| git2::Cred::ssh_key_from_agent(username.unwrap()));
    return callbacks;
}

pub fn fetch_opts<'a>() -> git2::FetchOptions<'a> {
    let mut fetch_opts = git2::FetchOptions::default();
    fetch_opts.remote_callbacks(remote_callbacks());
    return fetch_opts;
}
