const DAY_NAMES: Record<string, number> = {
  sun: 0, mon: 1, tue: 2, wed: 3, thu: 4, fri: 5, sat: 6,
}

const MONTH_NAMES: Record<string, number> = {
  jan: 1, feb: 2, mar: 3, apr: 4, may: 5, jun: 6,
  jul: 7, aug: 8, sep: 9, oct: 10, nov: 11, dec: 12,
}

function parseField(field: string, min: number, max: number, nameMap?: Record<string, number>): Set<number> {
  const result = new Set<number>()
  const parts = field.toLowerCase().split(',')

  for (const rawPart of parts) {
    let part = rawPart.trim()
    let step = 1

    if (part.includes('/')) {
      const [range, stepStr] = part.split('/')
      part = range
      step = parseInt(stepStr, 10)
      if (isNaN(step) || step <= 0) step = 1
    }

    let start: number
    let end: number

    if (part === '*') {
      start = min
      end = max
    } else if (part.includes('-')) {
      const [s, e] = part.split('-')
      const sLower = s.trim().toLowerCase()
      const eLower = e.trim().toLowerCase()
      start = nameMap?.[sLower] ?? parseInt(s, 10)
      end = nameMap?.[eLower] ?? parseInt(e, 10)
    } else {
      const valLower = part.trim().toLowerCase()
      start = nameMap?.[valLower] ?? parseInt(part, 10)
      end = start
    }

    if (isNaN(start) || isNaN(end)) continue

    for (let i = start; i <= end; i += step) {
      if (i >= min && i <= max) {
        result.add(i)
      }
    }
  }

  return result
}

export function getNextRunTimes(expr: string, count: number = 3): Date[] {
  const fields = expr.trim().split(/\s+/)
  if (fields.length !== 6) return []

  const seconds = parseField(fields[0], 0, 59)
  const minutes = parseField(fields[1], 0, 59)
  const hours = parseField(fields[2], 0, 23)
  const days = parseField(fields[3], 1, 31)
  const months = parseField(fields[4], 1, 12, MONTH_NAMES)
  const dows = parseField(fields[5], 0, 6, DAY_NAMES)

  const results: Date[] = []
  const maxIter = 366 * 24 * 60

  const now = new Date()
  let current = new Date(now.getFullYear(), now.getMonth(), now.getDate(), now.getHours(), now.getMinutes() + 1, 0, 0)
  let iter = 0

  while (results.length < count && iter < maxIter) {
    iter++

    if (!months.has(current.getMonth() + 1)) {
      current.setMonth(current.getMonth() + 1, 1)
      current.setHours(0, 0, 0, 0)
      continue
    }

    const dayValid = days.has(current.getDate())
    const dowValid = dows.has(current.getDay())
    const bothWildcard = fields[3].trim() === '*' && fields[5].trim() === '*'

    if (!bothWildcard && !(dayValid && dowValid) && !((fields[3].trim() !== '*' && dayValid) || (fields[5].trim() !== '*' && dowValid))) {
      current.setDate(current.getDate() + 1)
      current.setHours(0, 0, 0, 0)
      continue
    }

    if (!dayValid && !dowValid) {
      current.setDate(current.getDate() + 1)
      current.setHours(0, 0, 0, 0)
      continue
    }

    if (!hours.has(current.getHours())) {
      current.setHours(current.getHours() + 1, 0, 0, 0)
      continue
    }

    if (!minutes.has(current.getMinutes())) {
      current.setMinutes(current.getMinutes() + 1, 0, 0)
      continue
    }

    let secFound = false
    for (let s = current.getSeconds(); s < 60; s++) {
      if (seconds.has(s)) {
        current.setSeconds(s)
        results.push(new Date(current))
        secFound = true
        break
      }
    }

    if (!secFound) {
      current.setMinutes(current.getMinutes() + 1, 0, 0)
      continue
    }

    current.setSeconds(current.getSeconds() + 1)
    if (current.getSeconds() === 0) {
      current.setMinutes(current.getMinutes() + 1, 0, 0)
    }
  }

  return results
}

export function isValidCronExpr(expr: string): boolean {
  const fields = expr.trim().split(/\s+/)
  return fields.length === 6
}

export interface CronFieldOption {
  title: string
  value: string
}

export const CRON_FIELD_DEFS: Record<string, CronFieldOption[]> = {
  second: [
    { title: 'Every second', value: '*' },
    { title: 'At :00', value: '0' },
    { title: 'At :30', value: '30' },
    { title: 'Every 5s', value: '*/5' },
    { title: 'Every 15s', value: '*/15' },
    { title: 'Every 30s', value: '*/30' },
  ],
  minute: [
    { title: 'Every minute', value: '*' },
    { title: 'At :00', value: '0' },
    { title: 'At :15', value: '15' },
    { title: 'At :30', value: '30' },
    { title: 'At :45', value: '45' },
    { title: 'Every 5 min', value: '*/5' },
    { title: 'Every 15 min', value: '*/15' },
    { title: 'Every 30 min', value: '*/30' },
  ],
  hour: [
    { title: 'Every hour', value: '*' },
    { title: 'Midnight (0)', value: '0' },
    { title: '6 AM', value: '6' },
    { title: '8 AM', value: '8' },
    { title: '9 AM', value: '9' },
    { title: '12 PM', value: '12' },
    { title: '6 PM', value: '18' },
    { title: 'Every 2 hrs', value: '*/2' },
    { title: 'Every 6 hrs', value: '*/6' },
  ],
  day: [
    { title: 'Every day', value: '*' },
    { title: '1st', value: '1' },
    { title: '15th', value: '15' },
    { title: 'Last (28)', value: '28' },
  ],
  month: [
    { title: 'Every month', value: '*' },
    { title: 'Jan', value: '1' },
    { title: 'Feb', value: '2' },
    { title: 'Mar', value: '3' },
    { title: 'Apr', value: '4' },
    { title: 'May', value: '5' },
    { title: 'Jun', value: '6' },
    { title: 'Jul', value: '7' },
    { title: 'Aug', value: '8' },
    { title: 'Sep', value: '9' },
    { title: 'Oct', value: '10' },
    { title: 'Nov', value: '11' },
    { title: 'Dec', value: '12' },
  ],
  dow: [
    { title: 'Every day', value: '*' },
    { title: 'Mon–Fri', value: 'Mon-Fri' },
    { title: 'Sat–Sun', value: 'Sun,Sat' },
    { title: 'Mon', value: 'Mon' },
    { title: 'Tue', value: 'Tue' },
    { title: 'Wed', value: 'Wed' },
    { title: 'Thu', value: 'Thu' },
    { title: 'Fri', value: 'Fri' },
    { title: 'Sat', value: 'Sat' },
    { title: 'Sun', value: 'Sun' },
  ],
}

export function describeCron(expr: string): string {
  const fields = expr.trim().split(/\s+/)
  if (fields.length !== 6) return ''

  const parts: string[] = []

  const secLabel = lookUpLabel('second', fields[0])
  const minLabel = lookUpLabel('minute', fields[1])
  const hourLabel = lookUpLabel('hour', fields[2])
  const dayLabel = lookUpLabel('day', fields[3])
  const monthLabel = lookUpLabel('month', fields[4])
  const dowLabel = lookUpLabel('dow', fields[5])

  if (secLabel !== '-' && secLabel !== 'Every second') {
    parts.push(`Seconds: ${secLabel}`)
  }

  const timeParts: string[] = []
  if (minLabel !== '-' && minLabel !== 'Every minute') timeParts.push(minLabel.toLowerCase())
  if (hourLabel !== '-' && hourLabel !== 'Every hour') timeParts.push(`at ${hourLabel.toLowerCase()}`)
  if (timeParts.length > 0) parts.push(timeParts.join(' '))

  const dateParts: string[] = []
  if (dowLabel !== '-' && dowLabel !== 'Every day') dateParts.push(dowLabel)
  if (dayLabel !== '-' && dayLabel !== 'Every day') dateParts.push(`day ${dayLabel}`)
  if (monthLabel !== '-' && monthLabel !== 'Every month') dateParts.push(monthLabel)
  if (dateParts.length > 0) parts.push(`on ${dateParts.join(', ')}`)

  return parts.length > 0 ? parts.join(' — ') : 'Every second of every day'
}

function lookUpLabel(field: string, value: string): string {
  const opts = CRON_FIELD_DEFS[field]
  if (!opts) return value
  const found = opts.find(o => o.value.toLowerCase() === value.toLowerCase())
  return found ? found.title : value
}

export const DEFAULT_BUILDER = {
  second: '*',
  minute: '*',
  hour: '*',
  day: '*',
  month: '*',
  dow: '*',
}

export function builderToExpr(b: typeof DEFAULT_BUILDER): string {
  return `${b.second} ${b.minute} ${b.hour} ${b.day} ${b.month} ${b.dow}`
}

export function exprToBuilder(expr: string): typeof DEFAULT_BUILDER {
  const fields = expr.trim().split(/\s+/)
  if (fields.length !== 6) return { ...DEFAULT_BUILDER }
  return {
    second: fields[0],
    minute: fields[1],
    hour: fields[2],
    day: fields[3],
    month: fields[4],
    dow: fields[5],
  }
}
