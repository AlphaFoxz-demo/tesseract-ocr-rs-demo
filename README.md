# 说明

## 前置环境

### 安装[tesseract](https://tesseract-ocr.github.io/)

### 安装运行时依赖（Windows）

```sh
# install vcpkg
git clone https://github.com/microsoft/vcpkg.git
.\bootstrap-vcpkg.bat

# then you got a 'vcpkg.exe'

# install dependencies
.\vcpkg integrate install
.\vcpkg install leptonica:x64-windows-static-md
.\vcpkg install tesseract:x64-windows-static-md
```
