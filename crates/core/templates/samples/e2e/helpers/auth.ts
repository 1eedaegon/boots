import { Page, test as base } from '@playwright/test';

export type Role = 'admin' | 'writer' | 'reader';

interface Credentials {
  email: string;
  password: string;
}

const CREDENTIALS: Record<Role, Credentials> = {
  admin: { email: 'admin@example.com', password: 'admin123' },
  writer: { email: 'writer@example.com', password: 'writer123' },
  reader: { email: 'reader@example.com', password: 'reader123' },
};

/**
 * Create test fixture with pre-authenticated state
 */
export const testAsRole = (role: Role) =>
  base.extend({
    storageState: `e2e/.auth/${role}.json`,
  });

/**
 * Login as a specific role
 */
export async function loginAs(page: Page, role: Role) {
  const creds = CREDENTIALS[role];
  await page.goto('/login');
  await page.fill('[name="email"]', creds.email);
  await page.fill('[name="password"]', creds.password);
  await page.click('button[type="submit"]');
  await page.waitForURL('/');
}

/**
 * Logout current user
 */
export async function logout(page: Page) {
  await page.click('button[aria-label="Logout"]');
  await page.waitForURL('/login');
}

/**
 * Get current user info from API
 */
export async function getCurrentUser(page: Page) {
  const response = await page.request.get('/api/auth/me');
  return response.json();
}
