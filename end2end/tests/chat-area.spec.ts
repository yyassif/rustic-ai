import { test, expect, Page } from '@playwright/test';

test.describe('ChatArea Component', () => {
  let page: Page;

  test.beforeEach(async ({ browser }) => {
    page = await browser.newPage();
    await page.goto('/');
    await page.waitForLoadState('networkidle');
  });

  test('renders empty ChatArea initially', async () => {
    const chatArea = await page.locator('.py-2\\.5.flex.flex-col');
    await expect(chatArea).toBeVisible();
    
    const messages = await page.locator('.group.w-full.text-gray-800.dark\\:text-gray-100');
    await expect(messages).toHaveCount(0);
  });

  test('displays message after sending through prompt area', async () => {
    // Locate and fill the textarea
    const textarea = await page.locator('textarea');
    await textarea.fill('Test message');

    // Submit the message
    await page.keyboard.press('Enter');

    // Wait for the message to appear
    await page.waitForSelector('.group.w-full.text-gray-800.dark\\:text-gray-100');

    // Check if the message is displayed
    const messages = await page.locator('.group.w-full.text-gray-800.dark\\:text-gray-100');
    await expect(messages).toHaveCount(1);
  });

  test('renders markdown content correctly after sending message', async () => {
    // Send a message with markdown
    const textarea = await page.locator('textarea');
    await textarea.fill('# Heading\n**bold** and *italic*');
    await page.keyboard.press('Enter');

    // Wait for the message to appear
    await page.waitForSelector('.markdown-body');

    const markdownBody = await page.locator('.markdown-body').first();
    await expect(markdownBody).toBeVisible();
    
    // Check for specific markdown elements
    await expect(markdownBody.locator('h1')).toHaveText('Heading');
    await expect(markdownBody.locator('strong')).toHaveText('bold');
    await expect(markdownBody.locator('em')).toHaveText('italic');
  });

  test('copy button functionality', async () => {
    // Send a message first
    const textarea = await page.locator('textarea');
    await textarea.fill('Test message for copy');
    await page.keyboard.press('Enter');

    // Wait for the message to appear
    await page.waitForSelector('button:has(.w-4.h-4)');

    const copyButton = await page.locator('button:has(.w-4.h-4)').first();
    
    // Initial state
    await expect(copyButton.locator('svg.text-gray-400')).toBeVisible();
    
    // Clicked state
    await copyButton.click();
    await expect(copyButton.locator('svg.text-green-400')).toBeVisible();
    
    // Wait for the button to revert back
    await expect(copyButton.locator('svg.text-gray-400')).toBeVisible({ timeout: 3000 });
  });

  test('message structure', async () => {
    // Send a message first
    const textarea = await page.locator('textarea');
    await textarea.fill('Test message for structure');
    await page.keyboard.press('Enter');

    // Wait for the message to appear
    await page.waitForSelector('.group.w-full.text-gray-800.dark\\:text-gray-100');

    const message = await page.locator('.group.w-full.text-gray-800.dark\\:text-gray-100').first();
    
    await expect(message.locator('.w-8 .rounded-full')).toBeVisible(); // Avatar
    await expect(message.locator('.markdown-body')).toBeVisible(); // Message content
    await expect(message.locator('button:has(.w-4.h-4)')).toBeVisible(); // Copy button
  });
});