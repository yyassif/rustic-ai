import { test, expect } from '@playwright/test';

test.describe('PromptArea Component', () => {
  test.beforeEach(async ({ page }) => {
    // Navigate to the page containing the PromptArea component
    await page.goto('/');
  });

  test('renders correctly', async ({ page }) => {
    const promptArea = await page.locator('form');
    await expect(promptArea).toBeVisible();
    await expect(page.locator('textarea')).toBeVisible();
    await expect(page.locator('button[type="submit"]')).toBeVisible();
  });

  test('allows input and clears on submit', async ({ page }) => {
    const textarea = page.locator('textarea');
    const submitButton = page.locator('button[type="submit"]');

    await textarea.fill('Test message');
    await expect(textarea).toHaveValue('Test message');

    await submitButton.click();
    await expect(textarea).toHaveValue('');
  });

  test('submits on Enter key', async ({ page }) => {
    const textarea = page.locator('textarea');

    await textarea.fill('Test message');
    await textarea.press('Enter');
    await expect(textarea).toHaveValue('');
  });

  test('does not submit on Shift+Enter', async ({ page }) => {
    const textarea = page.locator('textarea');

    await textarea.fill('Test message');
    await textarea.press('Shift+Enter');
    await expect(textarea).toHaveValue('Test message\n');
  });

  test('disables input when loading', async ({ page }) => {
    // You'll need to implement a way to trigger the loading state
    await page.evaluate(() => {
      // Assuming you have a global function to set loading state
      window.setLoading(true);
    });

    const textarea = page.locator('textarea');
    const submitButton = page.locator('button[type="submit"]');

    await expect(textarea).toBeDisabled();
    await expect(submitButton).toBeDisabled();
  });

  test('submit button is disabled when input is empty', async ({ page }) => {
    const submitButton = page.locator('button[type="submit"]');
    await expect(submitButton).toBeDisabled();

    const textarea = page.locator('textarea');
    await textarea.fill('Test');
    await expect(submitButton).toBeEnabled();

    await textarea.fill('');
    await expect(submitButton).toBeDisabled();
  });
});