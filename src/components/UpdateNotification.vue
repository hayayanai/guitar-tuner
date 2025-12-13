<script setup lang="ts">
import { ref, onMounted } from "vue";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { relaunch } from "@tauri-apps/plugin-process";

const updateAvailable = ref(false);
const updateInfo = ref<Update | null>(null);
const downloading = ref(false);
const downloadProgress = ref(0);
const downloadTotal = ref(0);
const error = ref<string | null>(null);
const checkingUpdate = ref(false);

async function checkForUpdates() {
  try {
    checkingUpdate.value = true;
    error.value = null;
    const update = await check();

    if (update) {
      updateAvailable.value = true;
      updateInfo.value = update;
      console.log(`Found update: ${update.version} (${update.date})`);
    } else {
      console.log("No updates available");
    }
  } catch (err) {
    error.value = `Update check error: ${err}`;
    console.error("Update check failed:", err);
  } finally {
    checkingUpdate.value = false;
  }
}

async function downloadAndInstall() {
  if (!updateInfo.value) return;

  try {
    downloading.value = true;
    downloadProgress.value = 0;
    downloadTotal.value = 0;
    error.value = null;

    await updateInfo.value.downloadAndInstall((event) => {
      switch (event.event) {
        case "Started":
          downloadTotal.value = event.data.contentLength ?? 0;
          console.log(`Download started: ${downloadTotal.value} bytes`);
          break;
        case "Progress":
          downloadProgress.value += event.data.chunkLength;
          console.log(`Downloaded: ${downloadProgress.value} / ${downloadTotal.value}`);
          break;
        case "Finished":
          console.log("Download finished");
          break;
      }
    });

    console.log("Update installed, relaunching...");
    await relaunch();
  } catch (err) {
    error.value = `Installation error: ${err}`;
    console.error("Update installation failed:", err);
    downloading.value = false;
  }
}

function dismissUpdate() {
  updateAvailable.value = false;
  updateInfo.value = null;
  error.value = null;
}

onMounted(() => {
  // Automatically check for updates on startup
  checkForUpdates();
});
</script>

<template>
  <div v-if="checkingUpdate" class="update-notification checking">
    <p>Checking for updates...</p>
  </div>

  <div v-else-if="error" class="update-notification error">
    <p>{{ error }}</p>
    <button @click="dismissUpdate">Close</button>
  </div>

  <div v-else-if="updateAvailable && updateInfo" class="update-notification available">
    <div class="update-header">
      <h3>New version available</h3>
      <button class="close-btn" @click="dismissUpdate" :disabled="downloading">×</button>
    </div>

    <div class="update-content">
      <p class="version">Version {{ updateInfo.version }}</p>
      <p v-if="updateInfo.date" class="date">
        {{ new Date(updateInfo.date).toLocaleDateString("en-US") }}
      </p>
      <p v-if="updateInfo.body" class="notes">{{ updateInfo.body }}</p>
    </div>

    <div v-if="downloading" class="download-progress">
      <div class="progress-bar">
        <div
          class="progress-fill"
          :style="{
            width: downloadTotal > 0 ? `${(downloadProgress / downloadTotal) * 100}%` : '0%',
          }"
        ></div>
      </div>
      <p class="progress-text">
        Downloading... {{ Math.round((downloadProgress / downloadTotal) * 100) }}%
      </p>
    </div>

    <div v-else class="update-actions">
      <button @click="downloadAndInstall" class="install-btn">Update & Install</button>
      <button @click="dismissUpdate" class="later-btn">Later</button>
    </div>
  </div>
</template>

<style scoped>

.update-notification {
  position: fixed;
  left: 50%;
  bottom: var(--space-md);
  transform: translateX(-50%);
  background: var(--color-background);
  color: var(--color-text);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  box-shadow: 0 6px 16px rgba(0, 0, 0, 0.12);
  padding: var(--space-md);
  width: 100%;
  max-width: 400px;
  z-index: 1000;
  animation: slideInUp 0.2s ease-out;
}

@keyframes slideInUp {
  from {
    transform: translateX(-50%) translateY(100%);
    opacity: 0;
  }
  to {
    transform: translateX(-50%) translateY(0);
    opacity: 1;
  }
}

.update-notification.checking {
  background: var(--color-background-secondary);
  border-color: var(--color-border);
  padding: var(--space-sm) var(--space-md);
}

.update-notification.checking p {
  margin: 0;
  color: #666;
  font-size: 14px;
}

.update-notification.error {
  background: var(--color-error-light);
  border: 1px solid var(--color-error);
}

.update-notification.error p {
  margin: 0 0 8px 0;
  color: #c33;
}

.update-notification.available {
  background: var(--color-background-secondary);
  border: 1px solid var(--color-divider);
}

.update-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 12px;
}

.update-header h3 {
  margin: 0;
  font-size: var(--font-size-base);
  font-weight: 700;
  color: var(--color-primary);
}

.close-btn {
  background: none;
  border: none;
  font-size: 24px;
  line-height: 1;
  cursor: pointer;
  color: #666;
  padding: 0;
  width: 24px;
  height: 24px;
}

.close-btn:hover:not(:disabled) {
  color: #333;
}

.close-btn:disabled {
  cursor: not-allowed;
  opacity: 0.5;
}

.update-content {
  margin-bottom: 16px;
}

.version {
  font-size: var(--font-size-sm);
  font-weight: 600;
  margin: 0 0 var(--space-xs) 0;
  color: var(--color-text);
}

.date {
  font-size: var(--font-size-xs);
  color: var(--color-text-secondary);
  margin: 0 0 var(--space-sm) 0;
}

.notes {
  font-size: var(--font-size-sm);
  color: var(--color-text);
  margin: 0;
  white-space: pre-wrap;
  max-height: 120px;
  overflow-y: auto;
}

.download-progress {
  margin-bottom: 16px;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: var(--color-background);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  overflow: hidden;
  margin-bottom: var(--space-xs);
}

.progress-fill {
  height: 100%;
  background: var(--color-primary);
  transition: width 0.3s ease;
}

.progress-text {
  font-size: var(--font-size-sm);
  color: var(--color-text-secondary);
  margin: 0;
  text-align: center;
}

.update-actions {
  display: flex;
  gap: var(--space-sm);
  justify-content: flex-end;
}

button {
  padding: var(--space-sm) var(--space-md);
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border);
  font-size: var(--font-size-sm);
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.install-btn {
  background: var(--color-primary);
  color: white;
  border-color: transparent;
}

.install-btn:hover {
  filter: brightness(0.95);
}

.later-btn {
  background: var(--color-background);
  color: var(--color-text-secondary);
}

.later-btn:hover {
  background: var(--color-background-secondary);
}

button:active {
  transform: scale(0.98);
}
</style>
