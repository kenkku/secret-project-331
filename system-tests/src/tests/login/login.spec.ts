import {
  expect,
  PlaywrightTestArgs,
  PlaywrightTestOptions,
  PlaywrightWorkerArgs,
  PlaywrightWorkerOptions,
  test,
} from "@playwright/test"

import { logout } from "../../utils/logout"

test.describe("Login session with Playwright", async () => {
  /// Login state to use
  test.use({ storageState: "src/states/admin@example.com.json" })

  test.beforeAll(async (_args: PlaywrightWorkerArgs & PlaywrightWorkerOptions) => {
    // Executed once before tests
  })

  test.afterAll(async (_args: PlaywrightWorkerArgs & PlaywrightWorkerOptions) => {
    // Executed once after tests
  })

  test.beforeEach(
    async ({
      page,
    }: PlaywrightTestArgs &
      PlaywrightTestOptions &
      PlaywrightWorkerArgs &
      PlaywrightWorkerOptions) => {
      // Executed before each test
      await page.goto("http://project-331.local")
    },
  )

  test.afterEach(
    async (
      _args: PlaywrightTestArgs &
        PlaywrightTestOptions &
        PlaywrightWorkerArgs &
        PlaywrightWorkerOptions,
    ) => {
      // Executed after each test
    },
  )

  test("is succesful", async ({ page }) => {
    await page.click("id=main-navigation-menu")
    expect(await page.waitForSelector("text=Log out")).toBeTruthy()
  })

  test("able to logout", async ({ page }) => {
    await logout(page)
    expect(await page.waitForSelector("text=Log in")).toBeTruthy()
  })
})

test.describe("Login return_to", async () => {
  test.beforeEach(
    async ({
      page,
    }: PlaywrightTestArgs &
      PlaywrightTestOptions &
      PlaywrightWorkerArgs &
      PlaywrightWorkerOptions) => {
      // Executed before each test
      await page.goto("http://project-331.local")
    },
  )

  test("works after succesful login", async ({ page }) => {
    // Click text=University of Helsinki, Department of Computer Science
    await Promise.all([
      page.waitForNavigation(),
      page.click("text=University of Helsinki, Department of Computer Science"),
    ])
    expect(page.url().startsWith("http://project-331.local/org/")).toBe(true)
    // Click text=Login
    await page.click("id=main-navigation-menu")
    await Promise.all([page.waitForNavigation(), page.click("text=Log in")])
    await page.waitForSelector(`label:has-text("Password")`)
    expect(page.url().startsWith("http://project-331.local/login?return_to=")).toBe(true)
    // Click input[name="email"]
    await page.click(`label:has-text("Email")`)
    // Fill input[name="email"]
    await page.fill(`label:has-text("Email")`, "admin@example.com")
    // Click input[name="password"]
    await page.click(`label:has-text("Password")`)
    // Fill input[name="password"]
    await page.fill(`label:has-text("Password")`, "admin")
    // Click text=Submit
    await Promise.all([
      page.waitForNavigation(/*{ url: 'http://project-331.local/organizations/f242f19e-6d6f-43d5-9186-d0424864146e' }*/),
      page.click("id=login-button"),
    ])
    expect(page.url().startsWith("http://project-331.local/org/")).toBe(true)
  })
})
