import { marked } from 'marked'
import DOMPurify from 'dompurify'

function escapeHtml(str: string): string {
  return str
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
}

interface CacheEntry {
  text: string
  html: string
}

export function useMarkdownRender() {
  const cache = new Map<string, CacheEntry>()

  function renderMarkdown(uuid: string, text: string): string {
    if (!text) return ''

    const entry = cache.get(uuid)
    if (entry && entry.text === text) {
      return entry.html
    }

    try {
      const html = DOMPurify.sanitize(marked.parse(text, { async: false }) as string || '') as string
      cache.set(uuid, { text, html })
      return html
    } catch {
      const fallback = escapeHtml(text)
      cache.set(uuid, { text, html: fallback })
      return fallback
    }
  }

  function clearCache() {
    cache.clear()
  }

  return { renderMarkdown }
}
