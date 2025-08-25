<script setup lang="ts">
import { ref } from "vue";

interface ImageInfo {
  url: string;
  name: string;
}

const selectedImages = ref<ImageInfo[]>([]);
const isDragging = ref(false);
const fileInput = ref<HTMLInputElement>();
const viewMode = ref<'grid' | 'list'>('grid');

function removeImage(index: number) {
  selectedImages.value.splice(index, 1);
}

function handleFileSelect() {
  fileInput.value?.click();
}

function handleFileChange(e: Event) {
  const input = e.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    const files = Array.from(input.files);
    const imageFiles = files.filter(file => file.type.startsWith('image/'));
    const newImages = imageFiles.map(file => ({
      url: URL.createObjectURL(file),
      name: file.name
    }));
    selectedImages.value = [...selectedImages.value, ...newImages];
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
  console.log(e.dataTransfer?.items);
  e.preventDefault();
  isDragging.value = false;
  
  if (!e.dataTransfer) return;

  const files: File[] = [];
  const items = e.dataTransfer.items;
  
  for (let i = 0; i < items.length; i++) {
    const item = items[i];
    if (item.kind === 'file') {
      const file = item.getAsFile();
      if (file && file.type.startsWith('image/')) {
        files.push(file);
      }
    }
  }

  if (files.length > 0) {
    const newImages = files.map(file => ({
      url: URL.createObjectURL(file),
      name: file.name
    }));
    selectedImages.value = [...selectedImages.value, ...newImages];
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

    <div data-tauri-drag-region
      class="drop-zone drag-area"
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
      <div class="view-controls">
        <!-- <h3>已选图片列表</h3> -->
        <div class="view-buttons">
          <button 
            :class="{ active: viewMode === 'grid' }" 
            @click="viewMode = 'grid'"
          >
            网格视图
          </button>
          <button 
            :class="{ active: viewMode === 'list' }" 
            @click="viewMode = 'list'"
          >
            列表视图
          </button>
        </div>
      </div>
      
      <div v-if="viewMode === 'grid'" class="image-grid">
        <div v-for="(image, index) in selectedImages" :key="index" class="image-item">
          <img :src="image.url" alt="Selected image" class="thumbnail" />
          <button class="delete-btn" @click.stop="removeImage(index)">×</button>
        </div>
      </div>
      
      <div v-if="viewMode === 'list'" class="image-list-view">
        <div v-for="(image, index) in selectedImages" :key="index" class="list-item">
          <img :src="image.url" alt="Selected image" class="list-thumbnail" />
          <span class="file-name">{{ image.name }}</span>
          <button class="delete-btn" @click.stop="removeImage(index)">×</button>
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
  min-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.view-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.view-buttons {
  display: flex;
  gap: 8px;
}

.view-buttons button {
  padding: 6px 12px;
  border: 1px solid #dee2e6;
  background-color: white;
  cursor: pointer;
  border-radius: 4px;
  font-size: 14px;
}

.view-buttons button.active {
  background-color: #007bff;
  color: white;
  border-color: #007bff;
}

.image-grid {
  display: grid;
  min-width: 600px;
  grid-template-columns: repeat(auto-fill, minmax(100px, 1fr));
  gap: 12px;
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
  width: 100px;
  height: 100px;
}

.image-item:hover {
  transform: scale(1.05);
}

.thumbnail {
  width: 100%;
  height: 100%;
  object-fit: cover;
  border-radius: 4px;
}

.image-list-view {
  margin-top: 16px;
}

.list-item {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border: 1px solid #dee2e6;
  border-radius: 6px;
  margin-bottom: 8px;
  background-color: #f8f9fa;
}

.list-thumbnail {
  width: 40px;
  height: 40px;
  object-fit: cover;
  border-radius: 4px;
  margin-right: 12px;
}

.file-name {
  font-size: 14px;
  color: #495057;
  flex: 1;
}

.delete-btn {
  position: absolute;
  top: 4px;
  right: 4px;
  width: 20px;
  height: 20px;
  border: none;
  color: red;
  font-size: 14px;
  font-weight: bold;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  opacity: 0;
  transition: opacity 0.2s ease;
}

.image-item:hover .delete-btn,
.list-item:hover .delete-btn {
  opacity: 1;
}

.image-item {
  position: relative;
}

.list-item {
  position: relative;
  padding-right: 40px;
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