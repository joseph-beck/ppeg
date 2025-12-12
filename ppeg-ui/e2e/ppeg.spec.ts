import { expect,test } from '@playwright/test'

test.describe('ppeg', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/')
  })

  test('has title ppeg', async ({ page }) => {
    await expect(page).toHaveTitle(/ppeg/)
  })
})
