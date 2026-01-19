// Screenshot tests for SWE Cloud UI
// Takes screenshots of main pages for visual verification

use rsc_test::prelude::*;

const BASE_URL: &str = "http://localhost:3000";
const SCREENSHOT_DIR: &str = "tests/browser/screenshots";

#[browser]
async fn screenshot_landing_page() {
    let page = browser.new_page().await;
    page.set_viewport(1920, 1080).await;
    page.goto(BASE_URL).await;
    page.wait_for(".dashboard-page, .landing-page").await;

    // Wait for any animations to complete
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::create_dir_all(SCREENSHOT_DIR).ok();
    std::fs::write(format!("{}/01-landing.png", SCREENSHOT_DIR), &screenshot).unwrap();

    assert!(page.query(".feature-grid, .features").exists().await);
}

#[browser]
async fn screenshot_cloudemu_page() {
    let page = browser.new_page().await;
    page.set_viewport(1920, 1080).await;
    page.goto(&format!("{}/cloudemu", BASE_URL)).await;
    page.wait_for(".cloudemu-layout, .cloudemu-landing").await;
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::write(format!("{}/02-cloudemu.png", SCREENSHOT_DIR), &screenshot).unwrap();

    assert!(page.url().await.contains("/cloudemu"));
}

#[browser]
async fn screenshot_cloudkit_page() {
    let page = browser.new_page().await;
    page.set_viewport(1920, 1080).await;
    page.goto(&format!("{}/cloudkit", BASE_URL)).await;
    page.wait_for(".cloudkit-layout, .cloudkit-landing").await;
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::write(format!("{}/03-cloudkit.png", SCREENSHOT_DIR), &screenshot).unwrap();

    assert!(page.url().await.contains("/cloudkit"));
}

#[browser]
async fn screenshot_iac_page() {
    let page = browser.new_page().await;
    page.set_viewport(1920, 1080).await;
    page.goto(&format!("{}/iac", BASE_URL)).await;
    page.wait_for(".iac-layout, .iac-landing").await;
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::write(format!("{}/04-iac.png", SCREENSHOT_DIR), &screenshot).unwrap();

    assert!(page.url().await.contains("/iac"));
}

#[browser]
async fn screenshot_settings_page() {
    let page = browser.new_page().await;
    page.set_viewport(1920, 1080).await;
    page.goto(&format!("{}/settings", BASE_URL)).await;
    page.wait_for(".settings-layout, .settings-page").await;
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::write(format!("{}/05-settings.png", SCREENSHOT_DIR), &screenshot).unwrap();

    assert!(page.url().await.contains("/settings"));
}

#[browser]
async fn screenshot_404_page() {
    let page = browser.new_page().await;
    page.set_viewport(1920, 1080).await;
    page.goto(&format!("{}/nonexistent", BASE_URL)).await;
    page.wait_for(".not-found-page, .not-found").await;
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::write(format!("{}/06-not-found.png", SCREENSHOT_DIR), &screenshot).unwrap();

    assert!(page.query("h1").text().await.contains("404"));
}

#[browser]
async fn screenshot_mobile_landing() {
    let page = browser.new_page().await;
    page.set_viewport(375, 812).await;  // iPhone X size
    page.goto(BASE_URL).await;
    page.wait_for(".dashboard-page, .landing-page").await;
    page.wait(500).await;

    let screenshot = page.screenshot().await;
    std::fs::write(format!("{}/07-mobile-landing.png", SCREENSHOT_DIR), &screenshot).unwrap();
}
