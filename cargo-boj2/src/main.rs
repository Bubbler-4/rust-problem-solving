use std::error::Error;
use thirtyfour::prelude::*;

const BOJAUTOLOGIN: &str = "7c1ed0c92b456bb8a257d94649893c4ba4939bb6";

const ONLINEJUDGE: &str = "i25hvcqfbbbkjimtsgk80mde62";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless()?;
    let driver = WebDriver::new("http://localhost:4444", caps).await?;
    if let Err(error) = test_boj(&driver).await {
        eprintln!("Error: {:?}", error);
    }
    // Navigate to https://www.acmicpc.net.
    // driver.goto("https://www.acmicpc.net").await?;
    // boj_cookie(&driver).await?;
    // driver.refresh().await?;
    // let source = driver.source().await?;
    // println!("{}", source);
    // let elem = driver.query(By::Id("challenge-container")).first().await?;
    // elem.wait_until().not_displayed().await?;
    // let source = driver.source().await?;
    // println!("{}", source);
    // let _elem_username = driver.query(By::ClassName("username")).first().await?;
    // let username_text = elem_username.text().await?;
    // assert_eq!(username_text, "bubbler");

    driver.quit().await?;

    Ok(())
}

async fn test_boj(driver: &WebDriver) -> WebDriverResult<()> {
    driver.goto("https://www.acmicpc.net").await?;
    // boj_cookie(&driver).await?;
    // driver.refresh().await?;
    let source = driver.source().await?;
    println!("{}", source);
    let elem = driver.query(By::Id("challenge-container")).first().await?;
    elem.wait_until().not_displayed().await?;
    let source = driver.source().await?;
    println!("{}", source);
    Ok(())
}

async fn boj_cookie(driver: &WebDriver) -> WebDriverResult<()> {
    let mut cookie = Cookie::new("bojautologin", BOJAUTOLOGIN);
    cookie.set_domain(".acmicpc.net");
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    driver.add_cookie(cookie.clone()).await?;
    let mut cookie = Cookie::new("OnlineJudge", ONLINEJUDGE);
    cookie.set_domain(".acmicpc.net");
    cookie.set_path("/");
    cookie.set_same_site(SameSite::Lax);
    driver.add_cookie(cookie.clone()).await
}