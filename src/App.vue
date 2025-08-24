<script setup lang="ts">
import { ref } from "vue";

const selectedImages = ref<string[]>([]);
const isDragging = ref(false);
const fileInput = ref<HTMLInputElement>();

function handleFileSelect() {
  fileInput.value?.click();
}

function handleFileChange(e: Event) {
  const input = e.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    const files = Array.from(input.files);
    const imageFiles = files.filter(file => file.type.startsWith('image/'));
    selectedImages.value = [...selectedImages.value, ...imageFiles.map(file => URL.createObjectURL(file))];
  }
  // 重置input以允许选择相同文件
  input.value = '';
}

function handleDragOver(e: DragEvent) {
  e.preventDefault();
  isDragging.value = true;
}

function handleDragLeave(e: DragEvent) {
  e.preventDefault();
  isDragging.value = false;
}

function handleDrop(e: DragEvent) {
  e.preventDefault();
  isDragging.value = false;
  
  const files = e.dataTransfer?.files;
  if (files && files.length > 0) {
    const fileList = Array.from(files);
    const imageFiles = fileList.filter(file => file.type.startsWith('image/'));
    selectedImages.value = [...selectedImages.value, ...imageFiles.map(file => URL.createObjectURL(file))];
  }
}
</script>

<template>
  <main class="container">
    <h1>图片转WebP工具</h1>

    <input
      ref="fileInput"
      type="file"
      multiple
      accept="image/png, image/jpeg, image/jpg, image/gif, image/bmp, image/webp"
      @change="handleFileChange"
      style="display: none"
    />

    <div 
      class="drop-zone"
      :class="{ 'dragging': isDragging }"
      @click="handleFileSelect"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @drop="handleDrop"
    >
      <div class="drop-content">
        <p>点击选择图片或拖拽图片到此处</p>
        <p class="hint">支持 PNG, JPG, JPEG, GIF, BMP, WebP 格式</p>
      </div>
    </div>

    <div v-if="selectedImages.length > 0" class="image-list">
      <h3>已选图片列表</h3>
      <div class="image-grid">
        <div v-for="(image, index) in selectedImages" :key="index" class="image-item">
          <img :src="image" alt="Selected image" class="thumbnail" />
        </div>
      </div>
    </div>
  </main>
</template>

<style scoped>
.drop-zone {
  border: 2px dashed #007bff;
  border-radius: 12px;
  padding: 40px;
  text-align: center;
  cursor: pointer;
  transition: all 0.3s ease;
  background-color: #f8f9fa;
  margin: 20px auto;
  width: 80%;
}

.drop-zone:hover {
  background-color: #e9ecef;
  border-color: #0056b3;
}

.drop-zone.dragging {
  background-color: #d1ecf1;
  border-color: #17a2b8;
  transform: scale(1.02);
}

.drop-content p {
  margin: 0;
  font-size: 18px;
  color: #495057;
}

.hint {
  font-size: 14px !important;
  color: #6c757d !important;
  margin-top: 8px !important;
}

.image-list {
  margin-top: 30px;
  max-width: 800px;
  margin-left: auto;
  margin-right: auto;
}

.image-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(250px, 1fr));
  gap: 16px;
  margin-top: 16px;
}

.image-item {
  background-color: #f8f9fa;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 8px;
  text-align: center;
  overflow: hidden;
  transition: transform 0.2s ease;
}

.image-item:hover {
  transform: scale(1.05);
}

.thumbnail {
  width: 100%;
  height: 150px;
  object-fit: cover;
  border-radius: 4px;
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 5px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>