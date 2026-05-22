import { ref, nextTick } from 'vue'

export function useChatScroll() {
  const scrollContainer = ref<HTMLElement | null>(null)
  const messagesEnd = ref<HTMLElement | null>(null)

  function scrollToBottom() {
    nextTick(() => {
      if (scrollContainer.value) {
        scrollContainer.value.scrollTop = scrollContainer.value.scrollHeight
      }
      messagesEnd.value?.scrollIntoView({ behavior: 'smooth' })
    })
  }

  return { scrollContainer, messagesEnd, scrollToBottom }
}
