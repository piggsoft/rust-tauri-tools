/**
 * Date utilities with timezone-safe operations
 * All dates are stored in UTC format (ISO 8601) in the database
 * This module ensures consistent date comparisons regardless of timezone
 */

/**
 * Parse an ISO date string to a Date object, preserving the exact time.
 * 
 * @param dateStr - ISO 8601 formatted date string
 * @returns Date object representing the input date
 * @throws Error if the date string is invalid
 */
export function parseDate(dateStr: string): Date {
  const date = new Date(dateStr)
  if (isNaN(date.getTime())) {
    throw new Error(`Invalid date string: ${dateStr}`)
  }
  return date
}

/**
 * Get today's date at midnight in local timezone
 */
export function getToday(): Date {
  const now = new Date()
  return new Date(now.getFullYear(), now.getMonth(), now.getDate())
}

/**
 * Get tomorrow's date at midnight in local timezone
 */
export function getTomorrow(): Date {
  const today = getToday()
  return new Date(today.getTime() + 24 * 60 * 60 * 1000)
}

/**
 * Get a date at midnight in local timezone from an ISO string
 */
export function getDateAtMidnight(dateStr: string): Date {
  const date = parseDate(dateStr)
  return new Date(date.getFullYear(), date.getMonth(), date.getDate())
}

/**
 * Check if a date is today (timezone-safe)
 */
export function isToday(dateStr: string): boolean {
  const taskDate = getDateAtMidnight(dateStr)
  const today = getToday()
  return taskDate.getTime() === today.getTime()
}

/**
 * Check if a date is tomorrow (timezone-safe)
 */
export function isTomorrow(dateStr: string): boolean {
  const taskDate = getDateAtMidnight(dateStr)
  const tomorrow = getTomorrow()
  return taskDate.getTime() === tomorrow.getTime()
}

/**
 * Check if a date is overdue (timezone-safe)
 * A task is overdue if its due date is before today at midnight
 */
export function isOverdue(dateStr: string): boolean {
  const taskDate = getDateAtMidnight(dateStr)
  const today = getToday()
  return taskDate.getTime() < today.getTime()
}

/**
 * Format a date for display (timezone-safe)
 * Returns "今天", "明天", or "M/D" format
 */
export function formatDate(dateStr: string): string {
  if (isToday(dateStr)) {
    return '今天'
  }
  if (isTomorrow(dateStr)) {
    return '明天'
  }

  const date = parseDate(dateStr)
  return `${date.getMonth() + 1}/${date.getDate()}`
}

/**
 * Format a date to YYYY-MM-DD string (timezone-safe)
 * This is used for calendar comparisons and database operations
 */
export function formatDateToYYYYMMDD(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

/**
 * Parse a YYYY-MM-DD string to a Date object at midnight
 */
export function parseYYYYMMDD(dateStr: string): Date {
  const [year, month, day] = dateStr.split('-').map(Number)
  return new Date(year, month - 1, day)
}

/**
 * Format time from an ISO date string
 * Returns "HH:mm" format
 */
export function formatTime(dateStr: string): string {
  const date = parseDate(dateStr)
  return `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}`
}

/**
 * Check if a date is within the next 7 days
 */
export function isWithinWeek(dateStr: string): boolean {
  const taskDate = getDateAtMidnight(dateStr)
  const today = getToday()
  const weekFromNow = new Date(today.getTime() + 7 * 24 * 60 * 60 * 1000)
  return taskDate.getTime() <= weekFromNow.getTime()
}

/**
 * Compare two dates, ignoring time component
 * Returns -1 if date1 < date2, 0 if equal, 1 if date1 > date2
 */
export function compareDates(date1Str: string, date2Str: string): number {
  const d1 = getDateAtMidnight(date1Str)
  const d2 = getDateAtMidnight(date2Str)
  if (d1.getTime() < d2.getTime()) return -1
  if (d1.getTime() > d2.getTime()) return 1
  return 0
}
