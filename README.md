# 说明

## 前置环境

```bat
# install vcpkg
git clone https://github.com/microsoft/vcpkg.git
.\bootstrap-vcpkg.bat

# then you got a 'vcpkg.exe'

# install dependencies
.\vcpkg integrate install
.\vcpkg install leptonica:x64-windows-static-md
.\vcpkg install tesseract:x64-windows-static-md
```
