import { createRouter, createWebHashHistory } from 'vue-router'

const router = createRouter({
  history: createWebHashHistory('/dashboard/'),
  routes: [
    {
      path: '/',
      name: 'chat',
      component: () => import('@/views/ChatPage.vue'),
    },
    {
      path: '/usage',
      name: 'usage',
      component: () => import('@/views/UsagePage.vue'),
    },
    {
      path: '/mcp',
      name: 'mcp',
      component: () => import('@/views/McpPage.vue'),
    },
    {
      path: '/files',
      name: 'files',
      component: () => import('@/views/FilePage.vue'),
    },
    {
      path: '/sites',
      name: 'sites',
      component: () => import('@/views/SitesPage.vue'),
    },
    {
      path: '/crons',
      name: 'crons',
      component: () => import('@/views/CronsPage.vue'),
    },
    {
      path: '/agents',
      name: 'agents',
      component: () => import('@/views/AgentsPage.vue'),
    },
    {
      path: '/skills',
      name: 'skills',
      component: () => import('@/views/SkillsPage.vue'),
    },
    {
      path: '/tools',
      name: 'tools',
      component: () => import('@/views/ToolsPage.vue'),
    },
    {
      path: '/settings',
      component: () => import('@/views/SettingsPage.vue'),
      redirect: '/settings/model',
      children: [
        {
          path: 'agents-md',
          name: 'settings-agents-md',
          component: () => import('@/views/SettingsAgentsMdPage.vue'),
        },
        {
          path: 'model',
          name: 'settings-model',
          component: () => import('@/views/SettingsModelPage.vue'),
        },
        {
          path: 'security',
          name: 'settings-security',
          component: () => import('@/views/SettingsSecurityPage.vue'),
        },
      ],
    },
  ],
})

export default router
