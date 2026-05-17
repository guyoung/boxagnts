#!/bin/bash
# Boxagnts Release Script (Linux/macOS)
# 手动触发 GitHub Release Workflow

VERSION=""

# Parse arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        -v|--version) VERSION="$2"; shift ;;
        *) echo "Unknown parameter passed: $1"; exit 1 ;;
    esac
    shift
done

echo "Boxagnts Release Script"
echo "============================"
echo ""

# 如果没有提供版本，使用当前日期
if [ -z "$VERSION" ]; then
    VERSION=$(date +%Y%m%d)
    echo -e "\033[33m使用日期作为版本号: $VERSION\033[0m"
else
    echo -e "\033[32m使用指定版本号: $VERSION\033[0m"
fi

echo ""
echo -e "\033[36m请通过 GitHub 界面手动触发 Workflow:\033[0m"
echo -e "1. 访问: https://github.com/<your-username>/boxagnts-pub/actions"
echo -e "2. 选择 'Build & Release' 工作流"
echo -e "3. 点击 'Run workflow'"
echo -e "4. 输入版本号 (可选): $VERSION"
echo -e "5. 点击 'Run workflow' 开始构建"
echo ""
echo -e "\033[36m或者，您可以推送一个带日期的标签来触发自动发布:\033[0m"
echo "git tag -a $VERSION -m 'Release $VERSION'"
echo "git push origin $VERSION"
