import { test } from "@playwright/test"

import expectScreenshotsToMatchSnapshots from "../../utils/screenshot"
test.use({
  storageState: "src/states/admin@example.com.json",
})
test("test", async ({ page, headless }) => {
  // Go to http://project-331.local/manage/courses/7f36cf71-c2d2-41fc-b2ae-bbbcafab0ea5/pages
  await page.goto(
    "http://project-331.local/manage/courses/7f36cf71-c2d2-41fc-b2ae-bbbcafab0ea5/pages",
  )
  // Click a:has-text("In the second chapter...")
  await Promise.all([
    page.waitForNavigation(/*{ url: 'http://project-331.local/cms/pages/e89e3590-3280-4536-a980-5e0c4d039f86' }*/),
    page.click('a:has-text("In the second chapter...")'),
  ])
  // Click text=Save
  await page.click("text=Save")
  await expectScreenshotsToMatchSnapshots({
    page,
    headless,
    snapshotName: "success-notification-test",
    waitForThisToBeVisibleAndStable: "text=Success",
  })
})
