# Boxagnts Release Script (Windows)
# 手动触发 GitHub Release Workflow

param(
    [string]$Version = ""
)

Write-Host "Boxagnts Release Script" -ForegroundColor Cyan
Write-Host "============================" -ForegroundColor Cyan
Write-Host ""

# 如果没有提供版本，使用当前日期
if ([string]::IsNullOrEmpty($Version)) {
    $Version = Get-Date -Format "yyyyMMdd"
    Write-Host "使用日期作为版本号: $Version" -ForegroundColor Yellow
} else {
    Write-Host "使用指定版本号: $Version" -ForegroundColor Green
}

Write-Host ""
Write-Host "请通过 GitHub 界面手动触发 Workflow:" -ForegroundColor Cyan
Write-Host "1. 访问: https://github.com/<your-username>/boxagnts-pub/actions" -ForegroundColor White
Write-Host "2. 选择 'Build & Release' 工作流" -ForegroundColor White
Write-Host "3. 点击 'Run workflow'" -ForegroundColor White
Write-Host "4. 输入版本号 (可选): $Version" -ForegroundColor White
Write-Host "5. 点击 'Run workflow' 开始构建" -ForegroundColor White
Write-Host ""
Write-Host "或者，您可以推送一个带日期的标签来触发自动发布:" -ForegroundColor Cyan
Write-Host "git tag -a $Version -m 'Release $Version'" -ForegroundColor White
Write-Host "git push origin $Version" -ForegroundColor White
