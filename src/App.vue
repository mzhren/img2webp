<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open, message, save } from "@tauri-apps/plugin-dialog";
import { platform } from "@tauri-apps/plugin-os";
import { path } from '@tauri-apps/api'
import './style.css'; // 导入提取出来的样式文件

interface ImageInfo {
  url: string;
  name: string;
  file?: File;
}

// 扩展ImageInfoExtended接口，增加原始文件路径属性
interface ImageInfoExtended extends ImageInfo {
  filePath?: string;     // 临时文件路径
  originalPath?: string; // 添加原始文件路径
}

const selectedImages = ref<ImageInfoExtended[]>([]);
const isDragging = ref(false);
const fileInput = ref<HTMLInputElement>();
const viewMode = ref<'grid' | 'list'>('grid');
const isConverting = ref(false);

// 修改变量名和默认值
const quality = ref(80); // 默认压缩质量为80
const useCustomOutputDir = ref(false); // 默认保存在原始文件所在目录

function removeImage(index: number) {
  selectedImages.value.splice(index, 1);
}

async function handleFileSelect() {
  try {
    // 使用Tauri的open对话框选择文件
    const selected = await open({
      multiple: true,
      filters: [{
        name: '图片文件',
        extensions: ['png', 'jpg', 'jpeg', 'gif', 'bmp', 'webp']
      }]
    });
    
    if (selected === null) return;
    
    // 确保selected是数组
    const filePaths = Array.isArray(selected) ? selected : [selected];
    await processSelectedFilesFromPaths(filePaths);
  } catch (err) {
    console.error('文件选择错误:', err);
    // 回退到浏览器文件选择
    fileInput.value?.click();
  }
}

async function handleFileChange(e: Event) {
  const input = e.target as HTMLInputElement;
  if (input.files && input.files.length > 0) {
    const files = Array.from(input.files);
    const imageFiles = files.filter(file => file.type.startsWith('image/'));
    
    // 仍然使用处理函数，但现在我们无法获取真实路径
    await processSelectedFiles(imageFiles);
  }
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
    // 使用新方法处理文件
    processSelectedFiles(files);
  }
}

// 添加新的函数来处理直接从Tauri获取的文件路径
async function processSelectedFilesFromPaths(filePaths: string[]) {
  const newImages: ImageInfoExtended[] = [];
  
  for (const originalPath of filePaths) {
    try {
      // 读取文件内容
      const fileContent = await invoke('read_file_binary', { path: originalPath });
      const uint8Array = new Uint8Array(fileContent as number[]);
      
      // 获取文件名
      const fileName = await path.basename(originalPath);
      
      // 生成临时文件路径
      const currentPlatform = await platform();
      const timestamp = Date.now();
      const randomId = Math.random().toString(36).substring(2);
      const tempFileName = `${timestamp}_${randomId}_${fileName}`;
      
      let tempPath: string;
      if (currentPlatform === 'windows') {
        tempPath = `C:\\Windows\\Temp\\${tempFileName}`;
      } else {
        tempPath = `/tmp/${tempFileName}`;
      }
      
      // 写入临时文件
      await invoke('write_temp_file', { 
        path: tempPath, 
        data: Array.from(uint8Array) 
      });
      
      // 创建Blob URL用于显示 - 修复扩展名处理
      // 获取扩展名并移除点号
      const ext = await path.extname(fileName);
      const extWithoutDot = typeof ext === 'string' ? ext.replace('.', '') : '';
      
      // 使用安全的方式创建Blob
      const blob = new Blob([uint8Array], { 
        type: extWithoutDot ? `image/${extWithoutDot}` : 'image/png' 
      });
      const url = URL.createObjectURL(blob);
      
      // 添加到图像列表，包括原始路径
      newImages.push({
        url: url,
        name: fileName,
        filePath: tempPath,
        originalPath: originalPath  // 保存原始文件路径
      });
    } catch (err) {
      console.error(`处理文件 ${originalPath} 时出错:`, err);
    }
  }
  
  selectedImages.value = [...selectedImages.value, ...newImages];
}

// 在这里添加一个新函数，用于获取文件的路径信息
async function getFilePathInfo(filePath: string) {
  try {
    // 使用Rust后端函数获取父目录
    const parentDir = await invoke('get_parent_directory', { filePath });
    const fileName = await path.basename(filePath);
    return {
      parentDir,
      fileName
    };
  } catch (err) {
    console.error('无法获取文件路径信息:', err);
    throw err;
  }
}

// 添加缺失的processSelectedFiles函数
async function processSelectedFiles(files: File[]) {
  const newImages: ImageInfoExtended[] = [];
  
  for (const file of files) {
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    
    // 生成唯一的临时文件名
    const currentPlatform = await platform();
    const timestamp = Date.now();
    const randomId = Math.random().toString(36).substring(2);
    const tempFileName = `${timestamp}_${randomId}_${file.name}`;
    
    // 决定临时文件路径
    let tempPath: string;
    if (currentPlatform === 'windows') {
      tempPath = `C:\\Windows\\Temp\\${tempFileName}`;
    } else {
      tempPath = `/tmp/${tempFileName}`;
    }
    
    try {
      // 写入临时文件
      await invoke('write_temp_file', { 
        path: tempPath, 
        data: Array.from(uint8Array) 
      });
      
      // 添加到图像列表
      newImages.push({
        url: URL.createObjectURL(file),
        name: file.name,
        file: file,
        filePath: tempPath  // 存储临时文件路径
        // 注意：这里没有originalPath，因为浏览器API无法获取本地文件路径
      });
    } catch (err) {
      console.error('无法创建临时文件:', err);
    }
  }
  
  selectedImages.value = [...selectedImages.value, ...newImages];
}

async function convertImages() {
  console.log("转换中，请稍候...");
  if (selectedImages.value.length === 0) {
    await message('请先选择要转换的图片', { title: '提示', kind: 'warning' });
    return;
  }

  isConverting.value = true;
  
  try {
    let customOutputDir: string | null = null;
    
    if (useCustomOutputDir.value) {
      customOutputDir = await open({
        directory: true,
        multiple: false,
        title: "选择WebP图片保存位置"
      });

      if (!customOutputDir) {
        isConverting.value = false;
        return;
      }
    }

    const conversionTasks = [];

    for (const image of selectedImages.value) {
      if ((image.file || image.filePath)) {
        try {
          // 确定输出文件名
          const outputFileName = image.name.replace(/\.[^/.]+$/, "") + ".webp";
          let outputPath: string;
          
          if (!useCustomOutputDir.value) {
            if (image.originalPath) {
              // 如果有原始路径，获取其目录
              const originalDir = await getFilePathInfo(image.originalPath);
              outputPath = await path.join(originalDir.parentDir as string, outputFileName);
            } else {
              // 回退到临时文件路径
              const tempPathInfo = await getFilePathInfo(image.filePath as string);
              outputPath = await path.join(tempPathInfo.parentDir as string, outputFileName);
            }
            
            console.log(`自动保存到：${outputPath}`);
          } else {
            // 使用自定义输出目录
            outputPath = await path.join(customOutputDir!, outputFileName);
          }
          
          // 添加转换任务
          const task = async () => {
            // 转换文件，传递质量参数
            await invoke('convert_to_webp', { 
              imagePath: image.filePath,
              outputPath: outputPath,
              quality: Number(quality.value)
            });
            console.log(`已将 ${image.name} 转换为 ${outputFileName}，质量为 ${quality.value}`);
          };
          
          conversionTasks.push(task());
        } catch (err) {
          console.error(`处理图片 ${image.name} 时出错:`, err);
          await message(
            `处理图片 ${image.name} 时出错: ${err}\n跳过该图片并继续转换。`,
            { title: '转换错误', kind: 'error' }
          );
        }
      }
    }
    
    // 等待所有转换任务完成
    await Promise.all(conversionTasks);

    await message('转换完成！', { title: '成功', kind: 'info' });
  } catch (error: unknown) {
    console.error("转换失败:", error);
    
    // 检查是否是 cwebp 程序不存在的错误
    const errorMessage = typeof error === 'string' ? error : (error instanceof Error ? error.message : String(error));
    if (errorMessage.includes('program not found') || errorMessage.includes('cwebp')) {
      await message(
        '转换失败：系统中未找到 cwebp 程序。\n\n请安装 WebP 工具：\n' +
        '• Windows: 下载 libwebp 并添加到 PATH\n' +
        '• macOS: brew install webp\n' +
        '• Ubuntu: sudo apt-get install webp',
        { 
          title: '缺少依赖程序', 
          kind: 'error' 
        }
      );
    } else {
      await message(
        `转换失败：${errorMessage}\n\n请检查：\n` +
        '• 输出目录是否有写入权限\n' +
        '• 图片文件是否损坏\n' +
        '• 磁盘空间是否充足',
        { 
          title: '转换错误', 
          kind: 'error' 
        }
      );
    }
  } finally {
    isConverting.value = false;
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

    <div class="options">
      <div class="option-group">
        <label for="quality-slider">压缩质量: {{ quality }}</label>
        <input 
          type="range" 
          id="quality-slider" 
          v-model.number="quality" 
          min="0" 
          max="100" 
          step="1"
          class="quality-slider"
        >
      </div>
      <div class="option-group checkbox-group">
        <input 
          type="checkbox" 
          id="use-custom-output-dir" 
          v-model="useCustomOutputDir"
        >
        <label for="use-custom-output-dir">
          使用自定义输出目录
          <span class="hint-text">（不选择时将保存在原始图片所在目录）</span>
        </label>
      </div>
    </div>

    <div class="actions">
      <button class="convert-btn" @click="convertImages" :disabled="isConverting || selectedImages.length === 0">
        {{ isConverting ? '转换中...' : '转换' }}
      </button>
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