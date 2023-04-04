use anyhow;
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest::async_http_client;
use url::Url;

use dotenv;
use std::cmp::Ordering;

fn get_token_from_env() -> (String,String,String,String) {

    let mut client_id:String = "hoge".to_string();
    let mut project_id:String = "hoge".to_string();
    let mut auth_url:String = "hoge".to_string();
    let mut token_url:String = "hoge".to_string();
    let mut client_secret:String = "hoge".to_string();


    dotenv::from_filename("api.env").ok();
    let result: Vec<(String, String)> = dotenv::vars().collect();
    for item in result{
        let client_comparison = item.0.cmp(&"client_id".to_string());
        let project_comparison = item.0.cmp(&"project_id".to_string());
        let auth_comparison = item.0.cmp(&"auth_url".to_string());
        let token_comparison = item.0.cmp(&"token_url".to_string());
        let cs_comparison = item.0.cmp(&"token_url".to_string());
        if client_comparison ==  Ordering::Equal {
            client_id = item.1.to_string();
        }else if project_comparison == Ordering::Equal{
            project_id = item.1.to_string();
        }else if auth_comparison == Ordering::Equal{
            auth_url = item.1.to_string();
        }else if token_comparison == Ordering::Equal{
            token_url = item.1.to_string();
        }else if client_secret == Ordering::Equal{
            cs_comparison = item.i.to_string();
        }
    }
    return (client_id.to_string(), project_id.to_string(), auth_url.to_string(), token_url.to_string(), cs_comparison.to_string());
}

fn main(){
    let token = get_token_from_env();
    // println!("{}: {}: {}: {}", token.0, token.1, token.2, token.3);
    let client_id = token.0;
    let client_secret = token.1;
    let url_oauth = token.2;
    let token_url = token.3;
    // let client_secret = token.4;
    // Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
    // token URL.
    let client =
        BasicClient::new(
            ClientId::new(client_id.to_string()),
            Some(ClientSecret::new(client_secret.to_string())),
            AuthUrl::new(url_oauth.to_string())?,
            Some(TokenUrl::new(token_url.to_string())?)
        )
        // Set the URL the user will be redirected to after the authorization process.
        .set_redirect_uri(RedirectUrl::new("http://localhost:8080/auth/google/callback".to_string())?);

    // Generate a PKCE challenge.
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    // Generate the full authorization URL.
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        // Set the desired scopes.
        .add_scope(Scope::new("read".to_string()))
        .add_scope(Scope::new("write".to_string()))
        // Set the PKCE code challenge.
        .set_pkce_challenge(pkce_challenge)
        .url();

    // This is the URL you should redirect the user to, in order to trigger the authorization
    // process.
    println!("Browse to: {}", auth_url);

    // Once the user has been redirected to the redirect URL, you'll have access to the
    // authorization code. For security reasons, your code should verify that the `state`
    // parameter returned by the server matches `csrf_state`.

    // Now you can trade it for an access token.
    let token_result = client
        .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request_async(async_http_client)
        .await?;

    // Unwrapping token_result will either produce a Token or a RequestTokenError.
}
