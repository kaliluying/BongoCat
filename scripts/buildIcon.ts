import { execSync } from 'node:child_process'

(() => {
  const command = 'tauri icon src-tauri/assets/logo.png'

  execSync(command, { stdio: 'inherit' })
})()
