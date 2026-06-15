#!/usr/bin/env bash
#
# 部署 website/ 到 Cloudflare Pages。
# 流程:安装依赖 -> 构建(astro build + pagefind)-> wrangler 部署 dist/
#
# 用法:
#   ./scripts/deploy.sh                 # 部署到生产分支
#   ./scripts/deploy.sh -p preview      # 部署到预览分支 preview
#
# 前置要求:
#   - 已安装 pnpm、npx
#   - 已登录 Cloudflare:  npx wrangler login
#     或设置环境变量 CLOUDFLARE_API_TOKEN(和 CLOUDFLARE_ACCOUNT_ID)
set -euo pipefail

# Cloudflare Pages 项目名,可用环境变量覆盖
PROJECT_NAME="${CF_PAGES_PROJECT:-rust-tutorials}"
BRANCH=""

while getopts "p:n:h" opt; do
  case "$opt" in
    p) BRANCH="$OPTARG" ;;
    n) PROJECT_NAME="$OPTARG" ;;
    h)
      grep '^#' "$0" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) echo "未知参数,使用 -h 查看帮助" >&2; exit 1 ;;
  esac
done

# 切换到脚本所在仓库的 website 目录
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
WEBSITE_DIR="$SCRIPT_DIR/../website"
cd "$WEBSITE_DIR"

echo "==> 安装依赖 (pnpm install --frozen-lockfile)"
pnpm install --frozen-lockfile

echo "==> 构建站点 (pnpm build)"
pnpm build

if [[ ! -d dist ]]; then
  echo "构建失败:未找到 dist/ 目录" >&2
  exit 1
fi

echo "==> 部署 dist/ 到 Cloudflare Pages 项目: $PROJECT_NAME"
deploy_args=(pages deploy dist --project-name "$PROJECT_NAME")
if [[ -n "$BRANCH" ]]; then
  deploy_args+=(--branch "$BRANCH")
fi

npx wrangler "${deploy_args[@]}"

echo "==> 完成 ✅"
