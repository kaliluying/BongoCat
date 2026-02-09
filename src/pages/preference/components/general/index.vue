<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core'
import { disable, enable, isEnabled } from '@tauri-apps/plugin-autostart'
import { message, Select, Switch } from 'ant-design-vue'
import { watch } from 'vue'

import MacosPermissions from './components/macos-permissions/index.vue'
import ThemeMode from './components/theme-mode/index.vue'

import ProList from '@/components/pro-list/index.vue'
import ProListItem from '@/components/pro-list-item/index.vue'
import { INVOKE_KEY } from '@/constants'
import { useGeneralStore } from '@/stores/general'

const generalStore = useGeneralStore()

watch(() => generalStore.app.autostart, async (value) => {
  const enabled = await isEnabled()

  if (value && !enabled) {
    return enable()
  }

  if (!value && enabled) {
    disable()
  }
}, { immediate: true })

watch(() => generalStore.app.runAsAdmin, async (value, oldValue) => {
  if (!value || value === oldValue) return

  try {
    await invoke(INVOKE_KEY.RESTART_AS_ADMIN)
  } catch (error) {
    generalStore.app.runAsAdmin = false

    message.error(String(error))
  }
})
</script>

<template>
  <MacosPermissions />

  <ProList :title="$t('pages.preference.general.labels.appSettings')">
    <ProListItem :title="$t('pages.preference.general.labels.launchOnStartup')">
      <Switch v-model:checked="generalStore.app.autostart" />
    </ProListItem>

    <ProListItem
      :description="$t('pages.preference.general.hints.showTaskbarIcon')"
      :title="$t('pages.preference.general.labels.showTaskbarIcon')"
    >
      <Switch v-model:checked="generalStore.app.taskbarVisible" />
    </ProListItem>

    <ProListItem
      :description="$t('pages.preference.general.hints.runAsAdmin')"
      :title="$t('pages.preference.general.labels.runAsAdmin')"
    >
      <Switch v-model:checked="generalStore.app.runAsAdmin" />
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.general.labels.appearanceSettings')">
    <ThemeMode />

    <ProListItem :title="$t('pages.preference.general.labels.language')">
      <Select v-model:value="generalStore.appearance.language">
        <Select.Option value="zh-CN">
          简体中文
        </Select.Option>
        <Select.Option value="en-US">
          English
        </Select.Option>
        <Select.Option value="vi-VN">
          Tiếng Việt
        </Select.Option>
        <Select.Option value="pt-BR">
          Português
        </Select.Option>
      </Select>
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.general.labels.updateSettings')">
    <ProListItem :title="$t('pages.preference.general.labels.autoCheckUpdate')">
      <Switch v-model:checked="generalStore.update.autoCheck" />
    </ProListItem>
  </ProList>

  <ProList :title="$t('pages.preference.general.labels.statisticsSettings')">
    <ProListItem :title="$t('pages.preference.general.labels.keyboardClickCount')">
      <span class="text-sm text-color-2">
        {{ generalStore.app.keyboardClickCount }}
      </span>
    </ProListItem>

    <ProListItem :title="$t('pages.preference.general.labels.mouseClickCount')">
      <span class="text-sm text-color-2">
        {{ generalStore.app.mouseClickCount }}
      </span>
    </ProListItem>
  </ProList>
</template>
