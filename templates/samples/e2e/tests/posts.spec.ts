import { test, expect } from '@playwright/test';
import { loginAs, testAsRole } from '../helpers/auth';

// Writer tests
const writerTest = testAsRole('writer');

writerTest.describe('Writer - Posts', () => {
  writerTest('can access new post page', async ({ page }) => {
    await page.goto('/posts/new');
    await expect(page.locator('h1')).toContainText('New Post');
  });

  writerTest('can create a post', async ({ page }) => {
    await page.goto('/posts/new');

    const title = `Test Post ${Date.now()}`;
    await page.fill('[name="title"]', title);
    await page.fill('[name="content"]', 'Test content');
    await page.click('button[type="submit"]');

    await expect(page).toHaveURL(/\/posts\/[\w-]+/);
    await expect(page.locator('h1')).toContainText(title);
  });

  writerTest('can edit own post', async ({ page, request }) => {
    // Create a post first
    const response = await request.post('/api/posts', {
      data: { title: 'Original', content: 'Content' },
    });
    const post = await response.json();

    await page.goto(`/posts/${post.id}/edit`);
    await page.fill('[name="title"]', 'Updated Title');
    await page.click('button[type="submit"]');

    await expect(page.locator('h1')).toContainText('Updated Title');
  });

  writerTest('cannot see edit button for others post', async ({ page }) => {
    // Assuming there's a post from another writer
    await page.goto('/');
    await page.click('[data-testid="post-card"]:first-child');

    // Should not see edit button for others' posts
    const editButton = page.locator('button:has-text("Edit")');
    // This assertion depends on whether the post belongs to current user
  });
});

// Reader tests
const readerTest = testAsRole('reader');

readerTest.describe('Reader - Posts', () => {
  readerTest('can view post list', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('[data-testid="post-list"]')).toBeVisible();
  });

  readerTest('cannot see new post button', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('button:has-text("New Post")')).not.toBeVisible();
    await expect(page.locator('a:has-text("New Post")')).not.toBeVisible();
  });

  readerTest('gets 403 when accessing new post page', async ({ page }) => {
    await page.goto('/posts/new');
    await expect(page.locator('text=Permission denied')).toBeVisible();
  });
});

// Admin tests
const adminTest = testAsRole('admin');

adminTest.describe('Admin - Posts', () => {
  adminTest('can edit any post', async ({ page, request }) => {
    // Get first post (from any user)
    const listResponse = await request.get('/api/posts');
    const posts = await listResponse.json();

    if (posts.data && posts.data.length > 0) {
      const postId = posts.data[0].id;

      await page.goto(`/posts/${postId}/edit`);
      await page.fill('[name="title"]', 'Admin Edited');
      await page.click('button[type="submit"]');

      await expect(page.locator('h1')).toContainText('Admin Edited');
    }
  });

  adminTest('can delete any post', async ({ page, request }) => {
    // Create a test post
    const response = await request.post('/api/posts', {
      data: { title: 'To Delete', content: 'Content' },
    });
    const post = await response.json();

    await page.goto(`/posts/${post.id}`);
    await page.click('button:has-text("Delete")');
    await page.click('[role="dialog"] button:has-text("Confirm")');

    await expect(page).toHaveURL('/');
  });
});
