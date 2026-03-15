/**
 * Retry utilities for API calls
 * Provides exponential backoff and configurable retry logic
 */

/**
 * Configuration for retry behavior
 */
export interface RetryConfig {
  /** Maximum number of retry attempts */
  maxRetries?: number
  /** Initial delay in milliseconds */
  initialDelay?: number
  /** Multiplier for exponential backoff */
  backoffMultiplier?: number
  /** Maximum delay between retries */
  maxDelay?: number
}

/**
 * Default retry configuration
 */
const DEFAULT_CONFIG: Required<RetryConfig> = {
  maxRetries: 3,
  initialDelay: 1000,
  backoffMultiplier: 2,
  maxDelay: 10000,
}

/**
 * Execute an async function with retry logic and exponential backoff.
 * 
 * @param fn - Async function to execute
 * @param config - Optional retry configuration
 * @returns Promise that resolves with the function result or rejects on final failure
 * 
 * @example
 * const result = await withRetry(
 *   () => api.fetchData(),
 *   { maxRetries: 5, initialDelay: 500 }
 * )
 */
export async function withRetry<T>(
  fn: () => Promise<T>,
  config: RetryConfig = {}
): Promise<T> {
  const mergedConfig = { ...DEFAULT_CONFIG, ...config }
  const { maxRetries, initialDelay, backoffMultiplier, maxDelay } = mergedConfig
  
  let lastError: Error | null = null
  
  for (let attempt = 0; attempt < maxRetries; attempt++) {
    try {
      return await fn()
    } catch (error) {
      lastError = error instanceof Error ? error : new Error(String(error))
      
      // Don't retry on the last attempt
      if (attempt < maxRetries - 1) {
        // Calculate delay with exponential backoff
        const delay = Math.min(
          initialDelay * Math.pow(backoffMultiplier, attempt),
          maxDelay
        )
        
        console.warn(`Attempt ${attempt + 1} failed, retrying in ${delay}ms:`, lastError.message)
        await new Promise(resolve => setTimeout(resolve, delay))
      }
    }
  }
  
  // All retries exhausted
  throw lastError || new Error('Operation failed after retries')
}
