//! Screenshot utility for SWE Cloud UI
//! Run with: cargo run --example take_screenshots

use rsc_browser::{Browser, LaunchConfig, Viewport};
use std::path::Path;

const BASE_URL: &str = "http://localhost:3000";
const SCREENSHOT_DIR: &str = "screenshots";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create screenshot directory
    std::fs::create_dir_all(SCREENSHOT_DIR)?;

    println!("Launching headless Chrome...");
    let browser = Browser::launch(
        LaunchConfig::default()
            .headless(true)
            .viewport(Viewport {
                width: 1920,
                height: 1080,
                device_scale_factor: 1.0,
                is_mobile: false,
                has_touch: false,
            })
    ).await?;

    let page = browser.new_page().await?;

    // Screenshot 1: Landing page
    println!("Taking screenshot: Landing page");
    page.goto(BASE_URL).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/01-landing.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/01-landing.png", SCREENSHOT_DIR);

    // Screenshot 2: CloudEmu
    println!("Taking screenshot: CloudEmu");
    page.goto(&format!("{}/cloudemu", BASE_URL)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/02-cloudemu.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/02-cloudemu.png", SCREENSHOT_DIR);

    // Screenshot 3: CloudKit
    println!("Taking screenshot: CloudKit");
    page.goto(&format!("{}/cloudkit", BASE_URL)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/03-cloudkit.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/03-cloudkit.png", SCREENSHOT_DIR);

    // Screenshot 4: IAC
    println!("Taking screenshot: IAC");
    page.goto(&format!("{}/iac", BASE_URL)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/04-iac.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/04-iac.png", SCREENSHOT_DIR);

    // Screenshot 5: Settings
    println!("Taking screenshot: Settings");
    page.goto(&format!("{}/settings", BASE_URL)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/05-settings.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/05-settings.png", SCREENSHOT_DIR);

    // Screenshot 6: 404 page
    println!("Taking screenshot: 404 page");
    page.goto(&format!("{}/nonexistent", BASE_URL)).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/06-not-found.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/06-not-found.png", SCREENSHOT_DIR);

    // Screenshot 7: Mobile view
    println!("Taking screenshot: Mobile view");
    page.set_viewport(375, 812).await?;
    page.goto(BASE_URL).await?;
    tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    let screenshot = page.screenshot().await?;
    std::fs::write(format!("{}/07-mobile.png", SCREENSHOT_DIR), &screenshot)?;
    println!("  Saved: {}/07-mobile.png", SCREENSHOT_DIR);

    browser.close().await?;
    println!("\nAll screenshots saved to {}/", SCREENSHOT_DIR);
    Ok(())
}
