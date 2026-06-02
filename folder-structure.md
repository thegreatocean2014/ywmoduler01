
# Folder Structure

## Summary
- **Total Folders**: 527
- **Total Files**: 1592
- **Root Directory**: OceanModulerTauri

### File Types
- **.rs**: 5
- **.png**: 25
- **.icns**: 1
- **.ico**: 7
- **.json**: 141
- **other**: 18
- **.lock**: 1
- **.toml**: 1
- **.ts**: 642
- **.mjs**: 11
- **.md**: 136
- **.sh**: 2
- **.conf**: 1
- **.css**: 16
- **.scss**: 4
- **.svg**: 21
- **.vue**: 525
- **.snap**: 1
- **.html**: 7
- **.mts**: 4
- **.analyze**: 5
- **.development**: 5
- **.production**: 5
- **.map**: 1
- **.js**: 1
- **.yml**: 2
- **.yaml**: 3
- **.code-workspace**: 1

## Table of Contents
- [Directory Structure](#directory-structure)

## Directory Structure

```
OceanModulerTauri/
├─ 📁 src-tauri
│  ├─ 📁 src
│  │  ├─ 📄 http.rs (1.1 KB)
│  │  ├─ 📄 lib.rs (1.7 KB)
│  │  ├─ 📄 main.rs (0.3 KB)
│  │  └─ 📄 storage.rs (1.4 KB)
│  ├─ 📁 icons
│  │  ├─ 🖼️ 128x128.png (10.8 KB)
│  │  ├─ 🖼️ 128x128@2x.png (22.6 KB)
│  │  ├─ 🖼️ 32x32.png (2.2 KB)
│  │  ├─ 📄 icon.icns (270.5 KB)
│  │  ├─ 📄 icon.ico (36.8 KB)
│  │  ├─ 🖼️ icon.png (48.8 KB)
│  │  ├─ 🖼️ Square107x107Logo.png (9.0 KB)
│  │  ├─ 🖼️ Square142x142Logo.png (12.2 KB)
│  │  ├─ 🖼️ Square150x150Logo.png (12.7 KB)
│  │  ├─ 🖼️ Square284x284Logo.png (25.3 KB)
│  │  ├─ 🖼️ Square30x30Logo.png (2.0 KB)
│  │  ├─ 🖼️ Square310x310Logo.png (27.8 KB)
│  │  ├─ 🖼️ Square44x44Logo.png (3.3 KB)
│  │  ├─ 🖼️ Square71x71Logo.png (5.9 KB)
│  │  ├─ 🖼️ Square89x89Logo.png (7.4 KB)
│  │  └─ 🖼️ StoreLogo.png (3.9 KB)
│  ├─ 📁 gen
│  │  └─ 📁 schemas
│  │     ├─ 📊 acl-manifests.json (139.8 KB)
│  │     ├─ 📊 capabilities.json (0.1 KB)
│  │     ├─ 📊 desktop-schema.json (405.2 KB)
│  │     └─ 📊 windows-schema.json (405.2 KB)
│  ├─ 📁 capabilities
│  │  └─ 📊 default.json (0.2 KB)
│  ├─ 📄 .gitignore (0.1 KB)
│  ├─ 📄 build.rs (0.0 KB)
│  ├─ 📄 Cargo.lock (172.1 KB)
│  ├─ 📄 Cargo.toml (1.0 KB)
│  └─ 📊 tauri.conf.json (1.0 KB)
├─ 📁 scripts
│  ├─ 📁 vsh
│  │  ├─ 📁 src
│  │  │  ├─ 📁 publint
│  │  │  │  └─ 💻 index.ts (4.2 KB)
│  │  │  ├─ 📁 lint
│  │  │  │  └─ 💻 index.ts (1.9 KB)
│  │  │  ├─ 📁 code-workspace
│  │  │  │  └─ 💻 index.ts (1.7 KB)
│  │  │  ├─ 📁 check-dep
│  │  │  │  └─ 💻 index.ts (3.4 KB)
│  │  │  ├─ 📁 check-circular
│  │  │  │  └─ 💻 index.ts (5.5 KB)
│  │  │  └─ 💻 index.ts (2.1 KB)
│  │  ├─ 📁 bin
│  │  │  └─ 📄 vsh.mjs (0.0 KB)
│  │  ├─ 💻 env.d.ts (0.0 KB)
│  │  ├─ 📊 package.json (0.6 KB)
│  │  ├─ 📜 README.md (0.9 KB)
│  │  ├─ 📊 tsconfig.json (0.2 KB)
│  │  └─ 💻 tsdown.config.ts (0.2 KB)
│  ├─ 📁 turbo-run
│  │  ├─ 📁 src
│  │  │  ├─ 💻 index.ts (0.4 KB)
│  │  │  └─ 💻 run.ts (1.7 KB)
│  │  ├─ 📁 bin
│  │  │  └─ 📄 turbo-run.mjs (0.0 KB)
│  │  ├─ 📊 package.json (0.5 KB)
│  │  ├─ 📜 README.md (1.1 KB)
│  │  ├─ 📊 tsconfig.json (0.1 KB)
│  │  └─ 💻 tsdown.config.ts (0.2 KB)
│  ├─ 📁 deploy
│  │  ├─ 📄 build-local-docker-image.sh (1.6 KB)
│  │  ├─ 📄 Dockerfile (0.9 KB)
│  │  └─ 📄 nginx.conf (1.6 KB)
│  ├─ 📄 build-tauri.sh (0.4 KB)
│  └─ 📄 clean.mjs (4.4 KB)
├─ 📁 packages
│  ├─ 📁 utils
│  │  ├─ 📁 src
│  │  │  ├─ 📁 helpers
│  │  │  │  ├─ 📁 __tests__
│  │  │  │  │  ├─ 💻 find-menu-by-path.test.ts (2.4 KB)
│  │  │  │  │  ├─ 💻 generate-menus.test.ts (5.6 KB)
│  │  │  │  │  ├─ 💻 generate-routes-frontend.test.ts (2.8 KB)
│  │  │  │  │  └─ 💻 merge-route-modules.test.ts (1.8 KB)
│  │  │  │  ├─ 💻 find-menu-by-path.ts (0.8 KB)
│  │  │  │  ├─ 💻 generate-menus.ts (2.2 KB)
│  │  │  │  ├─ 💻 generate-routes-backend.ts (2.8 KB)
│  │  │  │  ├─ 💻 generate-routes-frontend.ts (1.5 KB)
│  │  │  │  ├─ 💻 get-popup-container.ts (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (0.3 KB)
│  │  │  │  ├─ 💻 merge-route-modules.ts (0.7 KB)
│  │  │  │  ├─ 💻 reset-routes.ts (1.1 KB)
│  │  │  │  └─ 💻 unmount-global-loading.ts (1.2 KB)
│  │  │  └─ 💻 index.ts (0.1 KB)
│  │  ├─ 📊 package.json (0.6 KB)
│  │  ├─ 📜 README.md (0.4 KB)
│  │  └─ 📊 tsconfig.json (0.2 KB)
│  ├─ 📁 types
│  │  ├─ 📁 src
│  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  └─ 💻 user.ts (0.3 KB)
│  │  ├─ 💻 global.d.ts (0.7 KB)
│  │  ├─ 📊 package.json (0.6 KB)
│  │  ├─ 📜 README.md (0.4 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 styles
│  │  ├─ 📁 src
│  │  │  ├─ 📁 naive
│  │  │  │  └─ 📄 index.css (0.6 KB)
│  │  │  ├─ 📁 global
│  │  │  │  └─ 📄 index.scss (0.0 KB)
│  │  │  ├─ 📁 ele
│  │  │  │  └─ 📄 index.css (1.1 KB)
│  │  │  ├─ 📁 antdv-next
│  │  │  │  └─ 📄 index.css (1.5 KB)
│  │  │  ├─ 📁 antd
│  │  │  │  └─ 📄 index.css (1.5 KB)
│  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  └─ 💻 style-exports.d.ts (0.1 KB)
│  │  ├─ 📊 package.json (1.0 KB)
│  │  ├─ 📜 README.md (0.3 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 stores
│  │  ├─ 📁 src
│  │  │  ├─ 📁 modules
│  │  │  │  ├─ 💻 access.test.ts (1.4 KB)
│  │  │  │  ├─ 💻 access.ts (2.7 KB)
│  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  ├─ 💻 tabbar.test.ts (7.2 KB)
│  │  │  │  ├─ 💻 tabbar.ts (20.4 KB)
│  │  │  │  ├─ 💻 timezone.ts (2.9 KB)
│  │  │  │  ├─ 💻 user.test.ts (1.1 KB)
│  │  │  │  └─ 💻 user.ts (1.1 KB)
│  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  └─ 💻 setup.ts (1.9 KB)
│  │  ├─ 📊 package.json (0.8 KB)
│  │  ├─ 💻 shim-pinia.d.ts (0.2 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 preferences
│  │  ├─ 📁 src
│  │  │  └─ 💻 index.ts (0.7 KB)
│  │  ├─ 📊 package.json (0.6 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 locales
│  │  ├─ 📁 src
│  │  │  ├─ 📁 langs
│  │  │  │  ├─ 📁 zh-CN
│  │  │  │  │  ├─ 📊 authentication.json (2.4 KB)
│  │  │  │  │  ├─ 📊 common.json (0.5 KB)
│  │  │  │  │  ├─ 📊 preferences.json (7.7 KB)
│  │  │  │  │  ├─ 📊 profile.json (0.1 KB)
│  │  │  │  │  └─ 📊 ui.json (5.4 KB)
│  │  │  │  └─ 📁 en-US
│  │  │  │     ├─ 📊 authentication.json (2.6 KB)
│  │  │  │     ├─ 📊 common.json (0.5 KB)
│  │  │  │     ├─ 📊 preferences.json (7.7 KB)
│  │  │  │     ├─ 📊 profile.json (0.1 KB)
│  │  │  │     └─ 📊 ui.json (5.6 KB)
│  │  │  ├─ 💻 i18n.ts (3.7 KB)
│  │  │  ├─ 💻 index.ts (0.5 KB)
│  │  │  └─ 💻 typing.ts (0.6 KB)
│  │  ├─ 📊 package.json (0.6 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 icons
│  │  ├─ 📁 src
│  │  │  ├─ 📁 svg
│  │  │  │  ├─ 📁 icons
│  │  │  │  │  ├─ 📄 antdv-logo.svg (3.3 KB)
│  │  │  │  │  ├─ 📄 antdv-next-logo.svg (4.0 KB)
│  │  │  │  │  ├─ 📄 avatar-1.svg (23.3 KB)
│  │  │  │  │  ├─ 📄 avatar-2.svg (20.1 KB)
│  │  │  │  │  ├─ 📄 avatar-3.svg (31.8 KB)
│  │  │  │  │  ├─ 📄 avatar-4.svg (18.7 KB)
│  │  │  │  │  ├─ 📄 bell.svg (3.4 KB)
│  │  │  │  │  ├─ 📄 cake.svg (2.2 KB)
│  │  │  │  │  ├─ 📄 card.svg (2.8 KB)
│  │  │  │  │  ├─ 📄 dingding.svg (0.7 KB)
│  │  │  │  │  ├─ 📄 download.svg (2.1 KB)
│  │  │  │  │  ├─ 📄 github.svg (1.7 KB)
│  │  │  │  │  ├─ 📄 google.svg (1.1 KB)
│  │  │  │  │  ├─ 📄 qqchat.svg (3.3 KB)
│  │  │  │  │  ├─ 📄 tdesign-logo.svg (3.5 KB)
│  │  │  │  │  └─ 📄 wechat.svg (1.7 KB)
│  │  │  │  ├─ 💻 index.ts (1.3 KB)
│  │  │  │  └─ 💻 load.ts (2.1 KB)
│  │  │  ├─ 📁 icons
│  │  │  │  └─ 📄 empty-icon.vue (0.9 KB)
│  │  │  ├─ 📁 iconify
│  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  └─ 💻 index.ts (0.1 KB)
│  │  ├─ 📊 package.json (0.5 KB)
│  │  ├─ 📜 README.md (0.3 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 effects
│  │  ├─ 📁 request
│  │  │  ├─ 📁 src
│  │  │  │  ├─ 📁 request-client
│  │  │  │  │  ├─ 📁 modules
│  │  │  │  │  │  ├─ 💻 downloader.test.ts (4.5 KB)
│  │  │  │  │  │  ├─ 💻 downloader.ts (1.8 KB)
│  │  │  │  │  │  ├─ 💻 interceptor.ts (1.0 KB)
│  │  │  │  │  │  ├─ 💻 sse.test.ts (3.7 KB)
│  │  │  │  │  │  ├─ 💻 sse.ts (3.6 KB)
│  │  │  │  │  │  ├─ 💻 uploader.test.ts (3.5 KB)
│  │  │  │  │  │  └─ 💻 uploader.ts (1.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 💻 preset-interceptors.ts (5.1 KB)
│  │  │  │  │  ├─ 💻 request-client.test.ts (2.9 KB)
│  │  │  │  │  ├─ 💻 request-client.ts (4.5 KB)
│  │  │  │  │  └─ 💻 types.ts (2.4 KB)
│  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  ├─ 📊 package.json (0.7 KB)
│  │  │  └─ 📊 tsconfig.json (0.1 KB)
│  │  ├─ 📁 plugins
│  │  │  ├─ 📁 src
│  │  │  │  ├─ 📁 vxe-table
│  │  │  │  │  ├─ 💻 api.ts (4.3 KB)
│  │  │  │  │  ├─ 💻 extends.ts (1.8 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.3 KB)
│  │  │  │  │  ├─ 💻 init.ts (3.5 KB)
│  │  │  │  │  ├─ 📜 README.md (1.1 KB)
│  │  │  │  │  ├─ 📄 style.css (3.6 KB)
│  │  │  │  │  ├─ 💻 types.ts (4.9 KB)
│  │  │  │  │  ├─ 💻 use-viewed-row.ts (14.2 KB)
│  │  │  │  │  ├─ 💻 use-vxe-grid.ts (2.2 KB)
│  │  │  │  │  └─ 📄 use-vxe-grid.vue (13.9 KB)
│  │  │  │  ├─ 📁 tiptap
│  │  │  │  │  ├─ 💻 extensions.ts (12.5 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📄 preview.vue (0.7 KB)
│  │  │  │  │  ├─ 📄 style.css (2.2 KB)
│  │  │  │  │  ├─ 📄 tiptap.vue (9.9 KB)
│  │  │  │  │  ├─ 💻 toolbar.ts (11.0 KB)
│  │  │  │  │  ├─ 💻 types.ts (2.2 KB)
│  │  │  │  │  └─ 💻 use-tiptap-toolbar.ts (4.4 KB)
│  │  │  │  ├─ 📁 motion
│  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│  │  │  │  │  ├─ 📜 README.md (0.6 KB)
│  │  │  │  │  └─ 💻 types.ts (0.5 KB)
│  │  │  │  ├─ 📁 echarts
│  │  │  │  │  ├─ 📄 echarts-ui.vue (0.2 KB)
│  │  │  │  │  ├─ 💻 echarts.ts (0.8 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📜 README.md (0.7 KB)
│  │  │  │  │  ├─ 💻 types.ts (0.6 KB)
│  │  │  │  │  └─ 💻 use-echarts.ts (4.8 KB)
│  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  ├─ 💻 plugins-context.ts (1.1 KB)
│  │  │  │  └─ 💻 types.ts (0.5 KB)
│  │  │  ├─ 📊 package.json (2.0 KB)
│  │  │  ├─ 📜 README.md (0.7 KB)
│  │  │  └─ 📊 tsconfig.json (0.1 KB)
│  │  ├─ 📁 layouts
│  │  │  ├─ 📁 src
│  │  │  │  ├─ 📁 widgets
│  │  │  │  │  ├─ 📁 user-dropdown
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 📄 user-dropdown.vue (7.5 KB)
│  │  │  │  │  ├─ 📁 timezone
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 📄 timezone-button.vue (2.1 KB)
│  │  │  │  │  ├─ 📁 theme-toggle
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 theme-button.vue (4.1 KB)
│  │  │  │  │  │  └─ 📄 theme-toggle.vue (1.7 KB)
│  │  │  │  │  ├─ 📁 preferences
│  │  │  │  │  │  ├─ 📁 icons
│  │  │  │  │  │  │  ├─ 📄 content-compact.vue (2.6 KB)
│  │  │  │  │  │  │  ├─ 📄 full-content.vue (1.3 KB)
│  │  │  │  │  │  │  ├─ 📄 header-mixed-nav.vue (3.9 KB)
│  │  │  │  │  │  │  ├─ 📄 header-nav.vue (2.6 KB)
│  │  │  │  │  │  │  ├─ 📄 header-sidebar-nav.vue (4.6 KB)
│  │  │  │  │  │  │  ├─ 💻 index.ts (0.5 KB)
│  │  │  │  │  │  │  ├─ 📄 mixed-nav.vue (4.3 KB)
│  │  │  │  │  │  │  ├─ 📄 setting.vue (0.9 KB)
│  │  │  │  │  │  │  ├─ 📄 sidebar-mixed-nav.vue (3.4 KB)
│  │  │  │  │  │  │  └─ 📄 sidebar-nav.vue (3.1 KB)
│  │  │  │  │  │  ├─ 📁 blocks
│  │  │  │  │  │  │  ├─ 📁 theme
│  │  │  │  │  │  │  │  ├─ 📄 builtin.vue (4.3 KB)
│  │  │  │  │  │  │  │  ├─ 📄 color-mode.vue (0.6 KB)
│  │  │  │  │  │  │  │  ├─ 📄 font-size.vue (1.2 KB)
│  │  │  │  │  │  │  │  ├─ 📄 radius.vue (0.9 KB)
│  │  │  │  │  │  │  │  └─ 📄 theme.vue (2.8 KB)
│  │  │  │  │  │  │  ├─ 📁 shortcut-keys
│  │  │  │  │  │  │  │  └─ 📄 global.vue (1.9 KB)
│  │  │  │  │  │  │  ├─ 📁 layout
│  │  │  │  │  │  │  │  ├─ 📄 breadcrumb.vue (1.7 KB)
│  │  │  │  │  │  │  │  ├─ 📄 content.vue (1.2 KB)
│  │  │  │  │  │  │  │  ├─ 📄 copyright.vue (1.5 KB)
│  │  │  │  │  │  │  │  ├─ 📄 footer.vue (0.5 KB)
│  │  │  │  │  │  │  │  ├─ 📄 header.vue (1.6 KB)
│  │  │  │  │  │  │  │  ├─ 📄 layout.vue (2.7 KB)
│  │  │  │  │  │  │  │  ├─ 📄 navigation.vue (1.3 KB)
│  │  │  │  │  │  │  │  ├─ 📄 sidebar.vue (3.3 KB)
│  │  │  │  │  │  │  │  ├─ 📄 tabbar.vue (3.1 KB)
│  │  │  │  │  │  │  │  └─ 📄 widget.vue (2.4 KB)
│  │  │  │  │  │  │  ├─ 📁 general
│  │  │  │  │  │  │  │  ├─ 📄 animation.vue (1.4 KB)
│  │  │  │  │  │  │  │  └─ 📄 general.vue (2.2 KB)
│  │  │  │  │  │  │  ├─ 📁 custom
│  │  │  │  │  │  │  │  └─ 📄 custom.vue (3.2 KB)
│  │  │  │  │  │  │  ├─ 📄 block.vue (0.3 KB)
│  │  │  │  │  │  │  ├─ 📄 checkbox-item.vue (1.3 KB)
│  │  │  │  │  │  │  ├─ 💻 index.ts (1.2 KB)
│  │  │  │  │  │  │  ├─ 📄 input-item.vue (1.6 KB)
│  │  │  │  │  │  │  ├─ 📄 number-field-item.vue (1.6 KB)
│  │  │  │  │  │  │  ├─ 📄 select-item.vue (1.7 KB)
│  │  │  │  │  │  │  ├─ 📄 switch-item.vue (1.3 KB)
│  │  │  │  │  │  │  └─ 📄 toggle-item.vue (1.1 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│  │  │  │  │  │  ├─ 📄 preferences-button.vue (0.7 KB)
│  │  │  │  │  │  ├─ 📄 preferences-drawer.vue (19.6 KB)
│  │  │  │  │  │  ├─ 📄 preferences.vue (2.4 KB)
│  │  │  │  │  │  └─ 💻 use-open-preferences.ts (0.3 KB)
│  │  │  │  │  ├─ 📁 notification
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 notification.vue (5.5 KB)
│  │  │  │  │  │  └─ 💻 types.ts (0.4 KB)
│  │  │  │  │  ├─ 📁 lock-screen
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 lock-screen-modal.vue (2.6 KB)
│  │  │  │  │  │  └─ 📄 lock-screen.vue (4.9 KB)
│  │  │  │  │  ├─ 📁 global-search
│  │  │  │  │  │  ├─ 📄 global-search.vue (4.3 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 📄 search-panel.vue (7.4 KB)
│  │  │  │  │  ├─ 📁 check-updates
│  │  │  │  │  │  ├─ 📄 check-updates.vue (3.0 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📄 breadcrumb.vue (1.6 KB)
│  │  │  │  │  ├─ 📄 color-toggle.vue (1.8 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.5 KB)
│  │  │  │  │  ├─ 📄 language-toggle.vue (1.0 KB)
│  │  │  │  │  └─ 📄 layout-toggle.vue (1.5 KB)
│  │  │  │  ├─ 📁 route-cached
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📄 route-cached-page.vue (0.7 KB)
│  │  │  │  │  └─ 📄 route-cached-view.vue (2.5 KB)
│  │  │  │  ├─ 📁 iframe
│  │  │  │  │  ├─ 📄 iframe-router-view.vue (2.2 KB)
│  │  │  │  │  ├─ 📄 iframe-view.vue (0.0 KB)
│  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  ├─ 📁 hooks
│  │  │  │  │  └─ 💻 index.ts (2.4 KB)
│  │  │  │  ├─ 📁 basic
│  │  │  │  │  ├─ 📁 tabbar
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 tabbar.vue (2.0 KB)
│  │  │  │  │  │  └─ 💻 use-tabbar.ts (5.5 KB)
│  │  │  │  │  ├─ 📁 menu
│  │  │  │  │  │  ├─ 📄 extra-menu.vue (0.8 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│  │  │  │  │  │  ├─ 📄 menu.vue (0.9 KB)
│  │  │  │  │  │  ├─ 📄 mixed-menu.vue (1.0 KB)
│  │  │  │  │  │  ├─ 💻 use-extra-menu.ts (3.8 KB)
│  │  │  │  │  │  ├─ 💻 use-mixed-menu.ts (4.5 KB)
│  │  │  │  │  │  └─ 💻 use-navigation.ts (1.8 KB)
│  │  │  │  │  ├─ 📁 header
│  │  │  │  │  │  ├─ 📄 header.vue (6.0 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📁 footer
│  │  │  │  │  │  ├─ 📄 footer.vue (0.2 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📁 copyright
│  │  │  │  │  │  ├─ 📄 copyright.vue (0.9 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📁 content
│  │  │  │  │  │  ├─ 📄 content-spinner.vue (0.3 KB)
│  │  │  │  │  │  ├─ 📄 content.vue (2.4 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 💻 use-content-spinner.ts (1.2 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📄 layout.vue (12.7 KB)
│  │  │  │  │  └─ 📜 README.md (0.3 KB)
│  │  │  │  ├─ 📁 authentication
│  │  │  │  │  ├─ 📁 icons
│  │  │  │  │  │  └─ 📄 slogan.vue (162.3 KB)
│  │  │  │  │  ├─ 📄 authentication.vue (5.1 KB)
│  │  │  │  │  ├─ 📄 form.vue (0.9 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📄 toolbar.vue (1.4 KB)
│  │  │  │  │  └─ 💻 types.ts (0.1 KB)
│  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  ├─ 📊 package.json (1.2 KB)
│  │  │  └─ 📊 tsconfig.json (0.1 KB)
│  │  ├─ 📁 hooks
│  │  │  ├─ 📁 src
│  │  │  │  ├─ 💻 index.ts (0.3 KB)
│  │  │  │  ├─ 💻 use-app-config.ts (0.9 KB)
│  │  │  │  ├─ 💻 use-content-maximize.ts (0.5 KB)
│  │  │  │  ├─ 💻 use-design-tokens.ts (12.0 KB)
│  │  │  │  ├─ 💻 use-hover-toggle.ts (4.9 KB)
│  │  │  │  ├─ 💻 use-pagination.ts (1.9 KB)
│  │  │  │  ├─ 💻 use-refresh.ts (0.3 KB)
│  │  │  │  ├─ 💻 use-tabs.ts (3.6 KB)
│  │  │  │  └─ 💻 use-watermark.ts (1.8 KB)
│  │  │  ├─ 📊 package.json (0.8 KB)
│  │  │  ├─ 📜 README.md (0.3 KB)
│  │  │  └─ 📊 tsconfig.json (0.2 KB)
│  │  ├─ 📁 common-ui
│  │  │  ├─ 📁 src
│  │  │  │  ├─ 📁 ui
│  │  │  │  │  ├─ 📁 profile
│  │  │  │  │  │  ├─ 📄 base-setting.vue (1.2 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.4 KB)
│  │  │  │  │  │  ├─ 📄 notification-setting.vue (1.3 KB)
│  │  │  │  │  │  ├─ 📄 password-setting.vue (1.2 KB)
│  │  │  │  │  │  ├─ 📄 profile.vue (1.6 KB)
│  │  │  │  │  │  ├─ 📄 security-setting.vue (1.3 KB)
│  │  │  │  │  │  └─ 💻 types.ts (0.4 KB)
│  │  │  │  │  ├─ 📁 fallback
│  │  │  │  │  │  ├─ 📁 icons
│  │  │  │  │  │  │  ├─ 📄 icon-403.vue (9.6 KB)
│  │  │  │  │  │  │  ├─ 📄 icon-404.vue (24.2 KB)
│  │  │  │  │  │  │  ├─ 📄 icon-500.vue (7.5 KB)
│  │  │  │  │  │  │  ├─ 📄 icon-coming-soon.vue (13.5 KB)
│  │  │  │  │  │  │  ├─ 📄 icon-offline.vue (8.4 KB)
│  │  │  │  │  │  │  └─ 📄 warning.svg (8.7 KB)
│  │  │  │  │  │  ├─ 💻 fallback.ts (0.4 KB)
│  │  │  │  │  │  ├─ 📄 fallback.vue (3.7 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📁 dashboard
│  │  │  │  │  │  ├─ 📁 workbench
│  │  │  │  │  │  │  ├─ 💻 index.ts (0.3 KB)
│  │  │  │  │  │  │  ├─ 📄 workbench-header.vue (1.3 KB)
│  │  │  │  │  │  │  ├─ 📄 workbench-project.vue (1.7 KB)
│  │  │  │  │  │  │  ├─ 📄 workbench-quick-nav.vue (1.4 KB)
│  │  │  │  │  │  │  ├─ 📄 workbench-todo.vue (1.6 KB)
│  │  │  │  │  │  │  └─ 📄 workbench-trends.vue (1.6 KB)
│  │  │  │  │  │  ├─ 📁 analysis
│  │  │  │  │  │  │  ├─ 📄 analysis-chart-card.vue (0.4 KB)
│  │  │  │  │  │  │  ├─ 📄 analysis-charts-tabs.vue (0.9 KB)
│  │  │  │  │  │  │  ├─ 📄 analysis-overview.vue (1.3 KB)
│  │  │  │  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 💻 typing.ts (0.8 KB)
│  │  │  │  │  ├─ 📁 authentication
│  │  │  │  │  │  ├─ 📄 auth-title.vue (0.3 KB)
│  │  │  │  │  │  ├─ 📄 code-login.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 dingding-login.vue (2.7 KB)
│  │  │  │  │  │  ├─ 📄 forget-password.vue (2.3 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.5 KB)
│  │  │  │  │  │  ├─ 📄 login-expired-modal.vue (2.1 KB)
│  │  │  │  │  │  ├─ 📄 login.vue (4.4 KB)
│  │  │  │  │  │  ├─ 📄 qrcode-login.vue (2.0 KB)
│  │  │  │  │  │  ├─ 📄 register.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 third-party-login.vue (1.8 KB)
│  │  │  │  │  │  └─ 💻 types.ts (1.1 KB)
│  │  │  │  │  ├─ 📁 about
│  │  │  │  │  │  ├─ 💻 about.ts (0.2 KB)
│  │  │  │  │  │  ├─ 📄 about.vue (4.9 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.0 KB)
│  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  ├─ 📁 components
│  │  │  │  │  ├─ 📁 tree
│  │  │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  │  │  └─ 📄 tree.vue (0.8 KB)
│  │  │  │  │  ├─ 📁 tippy
│  │  │  │  │  │  ├─ 💻 directive.ts (2.7 KB)
│  │  │  │  │  │  └─ 💻 index.ts (1.6 KB)
│  │  │  │  │  ├─ 📁 resize
│  │  │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  │  │  └─ 📄 resize.vue (26.7 KB)
│  │  │  │  │  ├─ 📁 page
│  │  │  │  │  │  ├─ 📁 __tests__
│  │  │  │  │  │  │  └─ 💻 page.test.ts (2.1 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 page.vue (2.5 KB)
│  │  │  │  │  │  └─ 💻 types.ts (0.6 KB)
│  │  │  │  │  ├─ 📁 loading
│  │  │  │  │  │  ├─ 💻 directive.ts (3.4 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 loading.vue (0.8 KB)
│  │  │  │  │  │  └─ 📄 spinner.vue (0.6 KB)
│  │  │  │  │  ├─ 📁 json-viewer
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  ├─ 📄 index.vue (3.2 KB)
│  │  │  │  │  │  ├─ 📄 style.scss (1.6 KB)
│  │  │  │  │  │  └─ 💻 types.ts (0.9 KB)
│  │  │  │  │  ├─ 📁 icon-picker
│  │  │  │  │  │  ├─ 📄 icon-picker.vue (7.5 KB)
│  │  │  │  │  │  ├─ 💻 icons.ts (1.9 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 💻 types.ts (0.6 KB)
│  │  │  │  │  ├─ 📁 ellipsis-text
│  │  │  │  │  │  ├─ 📄 ellipsis-text.vue (5.1 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📁 cropper
│  │  │  │  │  │  ├─ 📄 cropper.vue (28.0 KB)
│  │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 📁 count-to
│  │  │  │  │  │  ├─ 📄 count-to.vue (2.5 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 💻 types.ts (1.3 KB)
│  │  │  │  │  ├─ 📁 col-page
│  │  │  │  │  │  ├─ 📄 col-page.vue (2.5 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  │  └─ 💻 types.ts (0.5 KB)
│  │  │  │  │  ├─ 📁 captcha
│  │  │  │  │  │  ├─ 📁 slider-translate-captcha
│  │  │  │  │  │  │  └─ 📄 index.vue (7.9 KB)
│  │  │  │  │  │  ├─ 📁 slider-rotate-captcha
│  │  │  │  │  │  │  └─ 📄 index.vue (4.9 KB)
│  │  │  │  │  │  ├─ 📁 slider-captcha
│  │  │  │  │  │  │  ├─ 📄 index.vue (6.7 KB)
│  │  │  │  │  │  │  ├─ 📄 slider-captcha-action.vue (1.4 KB)
│  │  │  │  │  │  │  ├─ 📄 slider-captcha-bar.vue (0.7 KB)
│  │  │  │  │  │  │  └─ 📄 slider-captcha-content.vue (1.0 KB)
│  │  │  │  │  │  ├─ 📁 point-selection-captcha
│  │  │  │  │  │  │  ├─ 📄 index.vue (4.2 KB)
│  │  │  │  │  │  │  └─ 📄 point-selection-captcha-card.vue (2.1 KB)
│  │  │  │  │  │  ├─ 📁 hooks
│  │  │  │  │  │  │  └─ 💻 useCaptchaPoints.ts (0.3 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.4 KB)
│  │  │  │  │  │  └─ 💻 types.ts (3.5 KB)
│  │  │  │  │  ├─ 📁 api-component
│  │  │  │  │  │  ├─ 📄 api-component.vue (6.2 KB)
│  │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│  │  │  │  │  │  └─ 💻 types.ts (2.5 KB)
│  │  │  │  │  └─ 💻 index.ts (0.9 KB)
│  │  │  │  └─ 💻 index.ts (0.1 KB)
│  │  │  ├─ 📊 package.json (1.6 KB)
│  │  │  └─ 📊 tsconfig.json (0.1 KB)
│  │  ├─ 📁 access
│  │  │  ├─ 📁 src
│  │  │  │  ├─ 📄 access-control.vue (1.1 KB)
│  │  │  │  ├─ 💻 accessible.ts (6.0 KB)
│  │  │  │  ├─ 💻 directive.ts (1.0 KB)
│  │  │  │  ├─ 💻 index.ts (0.2 KB)
│  │  │  │  └─ 💻 use-access.ts (1.4 KB)
│  │  │  ├─ 📊 package.json (0.7 KB)
│  │  │  └─ 📊 tsconfig.json (0.1 KB)
│  │  └─ 📜 README.md (0.8 KB)
│  ├─ 📁 constants
│  │  ├─ 📁 src
│  │  │  ├─ 💻 core.ts (0.4 KB)
│  │  │  └─ 💻 index.ts (0.1 KB)
│  │  ├─ 📊 package.json (0.5 KB)
│  │  ├─ 📜 README.md (0.4 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  └─ 📁 @core
│     ├─ 📁 ui-kit
│     │  ├─ 📁 tabs-ui
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 components
│     │  │  │  │  ├─ 📁 widgets
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 tool-more.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 tool-refresh.vue (0.4 KB)
│     │  │  │  │  │  └─ 📄 tool-screen.vue (0.5 KB)
│     │  │  │  │  ├─ 📁 tabs-chrome
│     │  │  │  │  │  └─ 📄 tabs.vue (6.4 KB)
│     │  │  │  │  ├─ 📁 tabs
│     │  │  │  │  │  └─ 📄 tabs.vue (4.8 KB)
│     │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  ├─ 📄 tabs-view.vue (2.6 KB)
│     │  │  │  ├─ 💻 types.ts (1.3 KB)
│     │  │  │  ├─ 💻 use-tabs-drag.ts (3.3 KB)
│     │  │  │  └─ 💻 use-tabs-view-scroll.ts (4.7 KB)
│     │  │  ├─ 📊 package.json (1.3 KB)
│     │  │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.4 KB)
│     │  ├─ 📁 shadcn-ui
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 ui
│     │  │  │  │  ├─ 📁 tree
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 tree.vue (14.1 KB)
│     │  │  │  │  │  └─ 💻 types.ts (1.7 KB)
│     │  │  │  │  ├─ 📁 tooltip
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 Tooltip.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 TooltipContent.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 TooltipProvider.vue (0.3 KB)
│     │  │  │  │  │  └─ 📄 TooltipTrigger.vue (0.3 KB)
│     │  │  │  │  ├─ 📁 toggle-group
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 ToggleGroup.vue (1.1 KB)
│     │  │  │  │  │  └─ 📄 ToggleGroupItem.vue (1.1 KB)
│     │  │  │  │  ├─ 📁 toggle
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 💻 toggle.ts (0.9 KB)
│     │  │  │  │  │  └─ 📄 Toggle.vue (1.0 KB)
│     │  │  │  │  ├─ 📁 textarea
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 Textarea.vue (0.8 KB)
│     │  │  │  │  ├─ 📁 tabs
│     │  │  │  │  │  ├─ 💻 index.ts (0.3 KB)
│     │  │  │  │  │  ├─ 📄 Tabs.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 TabsContent.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 TabsList.vue (0.6 KB)
│     │  │  │  │  │  └─ 📄 TabsTrigger.vue (1.0 KB)
│     │  │  │  │  ├─ 📁 switch
│     │  │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│     │  │  │  │  │  └─ 📄 Switch.vue (1.3 KB)
│     │  │  │  │  ├─ 📁 sheet
│     │  │  │  │  │  ├─ 💻 index.ts (0.5 KB)
│     │  │  │  │  │  ├─ 💻 sheet.ts (1.0 KB)
│     │  │  │  │  │  ├─ 📄 Sheet.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 SheetClose.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 SheetContent.vue (2.5 KB)
│     │  │  │  │  │  ├─ 📄 SheetDescription.vue (0.6 KB)
│     │  │  │  │  │  ├─ 📄 SheetFooter.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 SheetHeader.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 SheetOverlay.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 SheetTitle.vue (0.5 KB)
│     │  │  │  │  │  └─ 📄 SheetTrigger.vue (0.3 KB)
│     │  │  │  │  ├─ 📁 separator
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 Separator.vue (1.0 KB)
│     │  │  │  │  ├─ 📁 select
│     │  │  │  │  │  ├─ 💻 index.ts (0.7 KB)
│     │  │  │  │  │  ├─ 📄 Select.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 SelectContent.vue (2.0 KB)
│     │  │  │  │  │  ├─ 📄 SelectGroup.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 SelectItem.vue (1.1 KB)
│     │  │  │  │  │  ├─ 📄 SelectItemText.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 SelectLabel.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 SelectScrollDownButton.vue (0.8 KB)
│     │  │  │  │  │  ├─ 📄 SelectScrollUpButton.vue (0.8 KB)
│     │  │  │  │  │  ├─ 📄 SelectSeparator.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 SelectTrigger.vue (1.1 KB)
│     │  │  │  │  │  └─ 📄 SelectValue.vue (0.3 KB)
│     │  │  │  │  ├─ 📁 scroll-area
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 ScrollArea.vue (1.0 KB)
│     │  │  │  │  │  └─ 📄 ScrollBar.vue (0.9 KB)
│     │  │  │  │  ├─ 📁 resizable
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 ResizableHandle.vue (1.7 KB)
│     │  │  │  │  │  └─ 📄 ResizablePanelGroup.vue (0.8 KB)
│     │  │  │  │  ├─ 📁 radio-group
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 RadioGroup.vue (0.7 KB)
│     │  │  │  │  │  └─ 📄 RadioGroupItem.vue (1.0 KB)
│     │  │  │  │  ├─ 📁 popover
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 Popover.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 PopoverContent.vue (1.3 KB)
│     │  │  │  │  │  └─ 📄 PopoverTrigger.vue (0.3 KB)
│     │  │  │  │  ├─ 📁 pin-input
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 PinInput.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 PinInputGroup.vue (0.6 KB)
│     │  │  │  │  │  ├─ 📄 PinInputInput.vue (0.8 KB)
│     │  │  │  │  │  └─ 📄 PinInputSeparator.vue (0.4 KB)
│     │  │  │  │  ├─ 📁 pagination
│     │  │  │  │  │  ├─ 💻 index.ts (0.4 KB)
│     │  │  │  │  │  ├─ 📄 PaginationEllipsis.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 PaginationFirst.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 PaginationLast.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 PaginationNext.vue (0.7 KB)
│     │  │  │  │  │  └─ 📄 PaginationPrev.vue (0.7 KB)
│     │  │  │  │  ├─ 📁 number-field
│     │  │  │  │  │  ├─ 💻 index.ts (0.4 KB)
│     │  │  │  │  │  ├─ 📄 NumberField.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 NumberFieldContent.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 NumberFieldDecrement.vue (0.8 KB)
│     │  │  │  │  │  ├─ 📄 NumberFieldIncrement.vue (0.8 KB)
│     │  │  │  │  │  └─ 📄 NumberFieldInput.vue (0.5 KB)
│     │  │  │  │  ├─ 📁 label
│     │  │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│     │  │  │  │  │  └─ 📄 Label.vue (0.6 KB)
│     │  │  │  │  ├─ 📁 input
│     │  │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│     │  │  │  │  │  └─ 📄 Input.vue (1.2 KB)
│     │  │  │  │  ├─ 📁 hover-card
│     │  │  │  │  │  ├─ 📄 HoverCard.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 HoverCardContent.vue (1.2 KB)
│     │  │  │  │  │  ├─ 📄 HoverCardTrigger.vue (0.3 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.2 KB)
│     │  │  │  │  ├─ 📁 form
│     │  │  │  │  │  ├─ 📄 FormControl.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 FormDescription.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 FormItem.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 FormLabel.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 FormMessage.vue (0.3 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.4 KB)
│     │  │  │  │  │  ├─ 💻 injectionKeys.ts (0.1 KB)
│     │  │  │  │  │  └─ 💻 useFormField.ts (0.9 KB)
│     │  │  │  │  ├─ 📁 dropdown-menu
│     │  │  │  │  │  ├─ 📄 DropdownMenu.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuCheckboxItem.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuContent.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuGroup.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuItem.vue (0.9 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuLabel.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuRadioGroup.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuRadioItem.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuSeparator.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuShortcut.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuSub.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuSubContent.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuSubTrigger.vue (0.9 KB)
│     │  │  │  │  │  ├─ 📄 DropdownMenuTrigger.vue (0.4 KB)
│     │  │  │  │  │  └─ 💻 index.ts (1.1 KB)
│     │  │  │  │  ├─ 📁 dialog
│     │  │  │  │  │  ├─ 📄 Dialog.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 DialogClose.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 DialogContent.vue (3.5 KB)
│     │  │  │  │  │  ├─ 📄 DialogDescription.vue (0.6 KB)
│     │  │  │  │  │  ├─ 📄 DialogFooter.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 DialogHeader.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 DialogOverlay.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 DialogScrollContent.vue (2.0 KB)
│     │  │  │  │  │  ├─ 📄 DialogTitle.vue (0.6 KB)
│     │  │  │  │  │  ├─ 📄 DialogTrigger.vue (0.3 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.6 KB)
│     │  │  │  │  ├─ 📁 context-menu
│     │  │  │  │  │  ├─ 📄 ContextMenu.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuCheckboxItem.vue (1.2 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuContent.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuGroup.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuItem.vue (1.0 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuLabel.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuPortal.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuRadioGroup.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuRadioItem.vue (1.2 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuSeparator.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuShortcut.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuSub.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuSubContent.vue (1.3 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuSubTrigger.vue (1.0 KB)
│     │  │  │  │  │  ├─ 📄 ContextMenuTrigger.vue (0.4 KB)
│     │  │  │  │  │  └─ 💻 index.ts (1.0 KB)
│     │  │  │  │  ├─ 📁 checkbox
│     │  │  │  │  │  ├─ 📄 Checkbox.vue (1.3 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 card
│     │  │  │  │  │  ├─ 📄 Card.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 CardContent.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 CardDescription.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 CardFooter.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 CardHeader.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 CardTitle.vue (0.3 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.3 KB)
│     │  │  │  │  ├─ 📁 button
│     │  │  │  │  │  ├─ 💻 button.ts (1.4 KB)
│     │  │  │  │  │  ├─ 📄 Button.vue (0.6 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 💻 types.ts (0.3 KB)
│     │  │  │  │  ├─ 📁 breadcrumb
│     │  │  │  │  │  ├─ 📄 Breadcrumb.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 BreadcrumbEllipsis.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 BreadcrumbItem.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 BreadcrumbLink.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 BreadcrumbList.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 BreadcrumbPage.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 BreadcrumbSeparator.vue (0.4 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.5 KB)
│     │  │  │  │  ├─ 📁 badge
│     │  │  │  │  │  ├─ 💻 badge.ts (0.9 KB)
│     │  │  │  │  │  ├─ 📄 Badge.vue (0.4 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 avatar
│     │  │  │  │  │  ├─ 💻 avatar.ts (0.6 KB)
│     │  │  │  │  │  ├─ 📄 Avatar.vue (0.5 KB)
│     │  │  │  │  │  ├─ 📄 AvatarFallback.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 AvatarImage.vue (0.3 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.2 KB)
│     │  │  │  │  ├─ 📁 alert-dialog
│     │  │  │  │  │  ├─ 📄 AlertDialog.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 AlertDialogAction.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 AlertDialogCancel.vue (0.3 KB)
│     │  │  │  │  │  ├─ 📄 AlertDialogContent.vue (2.7 KB)
│     │  │  │  │  │  ├─ 📄 AlertDialogDescription.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 AlertDialogOverlay.vue (0.2 KB)
│     │  │  │  │  │  ├─ 📄 AlertDialogTitle.vue (0.7 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.4 KB)
│     │  │  │  │  ├─ 📁 accordion
│     │  │  │  │  │  ├─ 📄 Accordion.vue (0.4 KB)
│     │  │  │  │  │  ├─ 📄 AccordionContent.vue (0.7 KB)
│     │  │  │  │  │  ├─ 📄 AccordionItem.vue (0.6 KB)
│     │  │  │  │  │  ├─ 📄 AccordionTrigger.vue (1.0 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.3 KB)
│     │  │  │  │  └─ 💻 index.ts (0.8 KB)
│     │  │  │  ├─ 📁 components
│     │  │  │  │  ├─ 📁 tooltip
│     │  │  │  │  │  ├─ 📄 help-tooltip.vue (0.6 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 tooltip.vue (0.9 KB)
│     │  │  │  │  ├─ 📁 spinner
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 loading.vue (2.5 KB)
│     │  │  │  │  │  └─ 📄 spinner.vue (2.6 KB)
│     │  │  │  │  ├─ 📁 spine-text
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 spine-text.vue (1.1 KB)
│     │  │  │  │  ├─ 📁 select
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 select.vue (1.3 KB)
│     │  │  │  │  ├─ 📁 segmented
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 segmented.vue (1.8 KB)
│     │  │  │  │  │  ├─ 📄 tabs-indicator.vue (1.1 KB)
│     │  │  │  │  │  └─ 💻 types.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 scrollbar
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 scrollbar.vue (4.4 KB)
│     │  │  │  │  ├─ 📁 render-content
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 render-content.vue (1.4 KB)
│     │  │  │  │  ├─ 📁 popover
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 📄 popover.vue (1.2 KB)
│     │  │  │  │  ├─ 📁 pin-input
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 input.vue (2.5 KB)
│     │  │  │  │  │  └─ 💻 types.ts (0.5 KB)
│     │  │  │  │  ├─ 📁 logo
│     │  │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│     │  │  │  │  │  └─ 📄 logo.vue (1.9 KB)
│     │  │  │  │  ├─ 📁 input-password
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 📄 input-password.vue (1.3 KB)
│     │  │  │  │  │  └─ 📄 password-strength.vue (1.4 KB)
│     │  │  │  │  ├─ 📁 icon
│     │  │  │  │  │  ├─ 📄 icon.vue (0.9 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.0 KB)
│     │  │  │  │  ├─ 📁 hover-card
│     │  │  │  │  │  ├─ 📄 hover-card.vue (1.2 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 full-screen
│     │  │  │  │  │  ├─ 📄 full-screen.vue (1.0 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 expandable-arrow
│     │  │  │  │  │  ├─ 📄 expandable-arrow.vue (0.7 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 dropdown-menu
│     │  │  │  │  │  ├─ 📄 dropdown-menu.vue (1.4 KB)
│     │  │  │  │  │  ├─ 📄 dropdown-radio-menu.vue (1.5 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  └─ 💻 interface.ts (0.5 KB)
│     │  │  │  │  ├─ 📁 count-to-animator
│     │  │  │  │  │  ├─ 📄 count-to-animator.vue (2.5 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 context-menu
│     │  │  │  │  │  ├─ 📄 context-menu.vue (2.9 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 💻 interface.ts (0.6 KB)
│     │  │  │  │  ├─ 📁 collapsible
│     │  │  │  │  │  ├─ 📄 collapsible-params-item.vue (2.8 KB)
│     │  │  │  │  │  ├─ 📄 collapsible-params.vue (6.4 KB)
│     │  │  │  │  │  ├─ 📄 collapsible.vue (1.9 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  └─ 💻 type.ts (0.5 KB)
│     │  │  │  │  ├─ 📁 checkbox
│     │  │  │  │  │  ├─ 📄 checkbox.vue (0.7 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 📁 button
│     │  │  │  │  │  ├─ 📄 button-group.vue (1.7 KB)
│     │  │  │  │  │  ├─ 💻 button.ts (1.5 KB)
│     │  │  │  │  │  ├─ 📄 button.vue (0.9 KB)
│     │  │  │  │  │  ├─ 📄 check-button-group.vue (4.7 KB)
│     │  │  │  │  │  ├─ 📄 icon-button.vue (1.5 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.3 KB)
│     │  │  │  │  ├─ 📁 breadcrumb
│     │  │  │  │  │  ├─ 📄 breadcrumb-background.vue (2.2 KB)
│     │  │  │  │  │  ├─ 📄 breadcrumb-view.vue (0.9 KB)
│     │  │  │  │  │  ├─ 📄 breadcrumb.vue (2.8 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 💻 types.ts (0.4 KB)
│     │  │  │  │  ├─ 📁 back-top
│     │  │  │  │  │  ├─ 📄 back-top.vue (1.0 KB)
│     │  │  │  │  │  ├─ 💻 backtop.ts (0.6 KB)
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  └─ 💻 use-backtop.ts (1.1 KB)
│     │  │  │  │  ├─ 📁 avatar
│     │  │  │  │  │  ├─ 📄 avatar.vue (1.6 KB)
│     │  │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  │  └─ 💻 index.ts (0.7 KB)
│     │  │  │  ├─ 📁 assets
│     │  │  │  │  └─ 📄 index.css (0.0 KB)
│     │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  ├─ 📊 components.json (0.3 KB)
│     │  │  ├─ 📊 package.json (1.3 KB)
│     │  │  └─ 📊 tsconfig.json (0.2 KB)
│     │  ├─ 📁 popup-ui
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 modal
│     │  │  │  │  ├─ 📁 __tests__
│     │  │  │  │  │  └─ 💻 modal-api.test.ts (3.5 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 💻 modal-api.ts (4.1 KB)
│     │  │  │  │  ├─ 💻 modal.ts (3.5 KB)
│     │  │  │  │  ├─ 📄 modal.vue (9.3 KB)
│     │  │  │  │  ├─ 💻 use-modal-draggable.ts (3.8 KB)
│     │  │  │  │  └─ 💻 use-modal.ts (4.5 KB)
│     │  │  │  ├─ 📁 drawer
│     │  │  │  │  ├─ 📁 __tests__
│     │  │  │  │  │  └─ 💻 drawer-api.test.ts (3.5 KB)
│     │  │  │  │  ├─ 💻 drawer-api.ts (3.9 KB)
│     │  │  │  │  ├─ 💻 drawer.ts (3.2 KB)
│     │  │  │  │  ├─ 📄 drawer.vue (8.5 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  └─ 💻 use-drawer.ts (4.2 KB)
│     │  │  │  ├─ 📁 alert
│     │  │  │  │  ├─ 💻 alert.ts (2.6 KB)
│     │  │  │  │  ├─ 📄 alert.vue (5.8 KB)
│     │  │  │  │  ├─ 💻 AlertBuilder.ts (6.7 KB)
│     │  │  │  │  └─ 💻 index.ts (0.3 KB)
│     │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  ├─ 📊 package.json (1.3 KB)
│     │  │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.4 KB)
│     │  ├─ 📁 menu-ui
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 utils
│     │  │  │  │  └─ 💻 index.ts (1.3 KB)
│     │  │  │  ├─ 📁 hooks
│     │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  ├─ 💻 use-menu-context.ts (1.2 KB)
│     │  │  │  │  ├─ 💻 use-menu-scroll.ts (1.0 KB)
│     │  │  │  │  └─ 💻 use-menu.ts (1.0 KB)
│     │  │  │  ├─ 📁 components
│     │  │  │  │  ├─ 📁 normal-menu
│     │  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  │  │  ├─ 💻 normal-menu.ts (0.4 KB)
│     │  │  │  │  │  └─ 📄 normal-menu.vue (3.6 KB)
│     │  │  │  │  ├─ 📄 collapse-transition.vue (2.5 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  ├─ 📄 menu-badge-dot.vue (0.6 KB)
│     │  │  │  │  ├─ 📄 menu-badge.vue (1.3 KB)
│     │  │  │  │  ├─ 📄 menu-item.vue (3.4 KB)
│     │  │  │  │  ├─ 📄 menu.vue (22.1 KB)
│     │  │  │  │  ├─ 📄 sub-menu-content.vue (2.3 KB)
│     │  │  │  │  └─ 📄 sub-menu.vue (6.8 KB)
│     │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  ├─ 📄 menu.vue (0.6 KB)
│     │  │  │  ├─ 📄 sub-menu.vue (1.5 KB)
│     │  │  │  └─ 💻 types.ts (2.8 KB)
│     │  │  ├─ 📊 package.json (1.4 KB)
│     │  │  ├─ 📜 README.md (0.0 KB)
│     │  │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.4 KB)
│     │  ├─ 📁 layout-ui
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 hooks
│     │  │  │  │  ├─ 💻 use-layout.ts (1.2 KB)
│     │  │  │  │  └─ 💻 use-sidebar-drag.ts (3.7 KB)
│     │  │  │  ├─ 📁 components
│     │  │  │  │  ├─ 📁 widgets
│     │  │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  │  ├─ 📄 sidebar-collapse-button.vue (0.5 KB)
│     │  │  │  │  │  └─ 📄 sidebar-fixed-button.vue (0.6 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.3 KB)
│     │  │  │  │  ├─ 📄 layout-content.vue (1.4 KB)
│     │  │  │  │  ├─ 📄 layout-footer.vue (0.8 KB)
│     │  │  │  │  ├─ 📄 layout-header.vue (1.3 KB)
│     │  │  │  │  ├─ 📄 layout-sidebar.vue (8.6 KB)
│     │  │  │  │  └─ 📄 layout-tabbar.vue (0.5 KB)
│     │  │  │  ├─ 💻 index.ts (0.1 KB)
│     │  │  │  ├─ 💻 vben-layout.ts (3.1 KB)
│     │  │  │  └─ 📄 vben-layout.vue (15.7 KB)
│     │  │  ├─ 📊 package.json (1.3 KB)
│     │  │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.4 KB)
│     │  ├─ 📁 form-ui
│     │  │  ├─ 📁 __tests__
│     │  │  │  └─ 💻 form-api.test.ts (6.9 KB)
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 form-render
│     │  │  │  │  ├─ 💻 context.ts (0.7 KB)
│     │  │  │  │  ├─ 💻 dependencies.ts (4.3 KB)
│     │  │  │  │  ├─ 💻 expandable.ts (2.6 KB)
│     │  │  │  │  ├─ 📄 form-field.vue (11.9 KB)
│     │  │  │  │  ├─ 📄 form-label.vue (0.8 KB)
│     │  │  │  │  ├─ 📄 form.vue (5.2 KB)
│     │  │  │  │  ├─ 💻 helper.ts (1.5 KB)
│     │  │  │  │  └─ 💻 index.ts (0.2 KB)
│     │  │  │  ├─ 📁 components
│     │  │  │  │  └─ 📄 form-actions.vue (4.1 KB)
│     │  │  │  ├─ 💻 config.ts (2.2 KB)
│     │  │  │  ├─ 💻 field-name.ts (0.3 KB)
│     │  │  │  ├─ 💻 form-api.ts (18.8 KB)
│     │  │  │  ├─ 💻 index.ts (0.3 KB)
│     │  │  │  ├─ 💻 types.ts (12.7 KB)
│     │  │  │  ├─ 💻 use-form-context.ts (3.5 KB)
│     │  │  │  ├─ 💻 use-vben-form.ts (1.3 KB)
│     │  │  │  ├─ 📄 vben-form.vue (2.0 KB)
│     │  │  │  └─ 📄 vben-use-form.vue (4.6 KB)
│     │  │  ├─ 📊 package.json (1.4 KB)
│     │  │  ├─ 📊 tsconfig.json (0.2 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.4 KB)
│     │  └─ 📜 README.md (0.1 KB)
│     ├─ 📁 preferences
│     │  ├─ 📁 __tests__
│     │  │  ├─ 📁 __snapshots__
│     │  │  │  └─ 📄 config.test.ts.snap (3.5 KB)
│     │  │  ├─ 💻 config.test.ts (0.3 KB)
│     │  │  └─ 💻 preferences.test.ts (14.1 KB)
│     │  ├─ 📁 src
│     │  │  ├─ 💻 config.ts (3.2 KB)
│     │  │  ├─ 💻 constants.ts (2.1 KB)
│     │  │  ├─ 💻 index.ts (0.5 KB)
│     │  │  ├─ 💻 preferences.ts (12.1 KB)
│     │  │  ├─ 💻 types.ts (11.4 KB)
│     │  │  ├─ 💻 update-css-variables.ts (3.8 KB)
│     │  │  └─ 💻 use-preferences.ts (7.0 KB)
│     │  ├─ 📊 package.json (1.1 KB)
│     │  ├─ 📊 tsconfig.json (0.2 KB)
│     │  └─ 💻 tsdown.config.ts (0.2 KB)
│     ├─ 📁 composables
│     │  ├─ 📁 src
│     │  │  ├─ 📁 __tests__
│     │  │  │  └─ 💻 use-sortable.test.ts (1.4 KB)
│     │  │  ├─ 📁 use-simple-locale
│     │  │  │  ├─ 💻 index.ts (0.6 KB)
│     │  │  │  ├─ 💻 messages.ts (0.5 KB)
│     │  │  │  └─ 📜 README.md (0.0 KB)
│     │  │  ├─ 💻 index.ts (0.3 KB)
│     │  │  ├─ 💻 use-is-mobile.ts (0.2 KB)
│     │  │  ├─ 💻 use-layout-style.ts (2.4 KB)
│     │  │  ├─ 💻 use-namespace.ts (2.8 KB)
│     │  │  ├─ 💻 use-priority-value.ts (2.6 KB)
│     │  │  ├─ 💻 use-scroll-lock.ts (1.4 KB)
│     │  │  └─ 💻 use-sortable.ts (0.7 KB)
│     │  ├─ 📊 package.json (1.1 KB)
│     │  ├─ 📊 tsconfig.json (0.2 KB)
│     │  └─ 💻 tsdown.config.ts (0.2 KB)
│     ├─ 📁 base
│     │  ├─ 📁 typings
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 💻 app.d.ts (2.3 KB)
│     │  │  │  ├─ 💻 basic.d.ts (0.6 KB)
│     │  │  │  ├─ 💻 helper.d.ts (2.8 KB)
│     │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  ├─ 💻 menu-record.ts (1.3 KB)
│     │  │  │  ├─ 💻 tabs.ts (0.2 KB)
│     │  │  │  └─ 💻 vue-router.d.ts (3.0 KB)
│     │  │  ├─ 📊 package.json (1.0 KB)
│     │  │  ├─ 📊 tsconfig.json (0.2 KB)
│     │  │  ├─ 💻 tsdown.config.ts (0.2 KB)
│     │  │  └─ 💻 vue-router.d.ts (0.2 KB)
│     │  ├─ 📁 shared
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 utils
│     │  │  │  │  ├─ 📁 __tests__
│     │  │  │  │  │  ├─ 💻 date.test.ts (4.2 KB)
│     │  │  │  │  │  ├─ 💻 diff.test.ts (1.6 KB)
│     │  │  │  │  │  ├─ 💻 dom.test.ts (2.8 KB)
│     │  │  │  │  │  ├─ 💻 inference.test.ts (5.1 KB)
│     │  │  │  │  │  ├─ 💻 letter.test.ts (3.6 KB)
│     │  │  │  │  │  ├─ 💻 resources.test.ts (2.5 KB)
│     │  │  │  │  │  ├─ 💻 stack.test.ts (2.3 KB)
│     │  │  │  │  │  ├─ 💻 state-handler.test.ts (1.9 KB)
│     │  │  │  │  │  ├─ 💻 tree.test.ts (4.6 KB)
│     │  │  │  │  │  ├─ 💻 unique.test.ts (1.6 KB)
│     │  │  │  │  │  ├─ 💻 update-css-variables.test.ts (1.1 KB)
│     │  │  │  │  │  ├─ 💻 util.test.ts (3.9 KB)
│     │  │  │  │  │  └─ 💻 window.test.ts (0.8 KB)
│     │  │  │  │  ├─ 💻 cn.ts (0.2 KB)
│     │  │  │  │  ├─ 💻 date.ts (1.6 KB)
│     │  │  │  │  ├─ 💻 diff.ts (2.4 KB)
│     │  │  │  │  ├─ 💻 dom.ts (2.4 KB)
│     │  │  │  │  ├─ 💻 download.ts (3.9 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.6 KB)
│     │  │  │  │  ├─ 💻 inference.ts (4.6 KB)
│     │  │  │  │  ├─ 💻 letter.ts (1.0 KB)
│     │  │  │  │  ├─ 💻 merge.ts (0.3 KB)
│     │  │  │  │  ├─ 💻 nprogress.ts (1.2 KB)
│     │  │  │  │  ├─ 💻 resources.ts (0.6 KB)
│     │  │  │  │  ├─ 💻 stack.ts (2.0 KB)
│     │  │  │  │  ├─ 💻 state-handler.ts (1.2 KB)
│     │  │  │  │  ├─ 💻 to.ts (0.6 KB)
│     │  │  │  │  ├─ 💻 tree.ts (3.3 KB)
│     │  │  │  │  ├─ 💻 unique.ts (0.4 KB)
│     │  │  │  │  ├─ 💻 update-css-variables.ts (0.9 KB)
│     │  │  │  │  ├─ 💻 util.ts (1.3 KB)
│     │  │  │  │  └─ 💻 window.ts (1.0 KB)
│     │  │  │  ├─ 📁 constants
│     │  │  │  │  ├─ 💻 globals.ts (0.6 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.0 KB)
│     │  │  │  │  └─ 💻 vben.ts (0.7 KB)
│     │  │  │  ├─ 📁 color
│     │  │  │  │  ├─ 📁 __tests__
│     │  │  │  │  │  └─ 💻 convert.test.ts (1.8 KB)
│     │  │  │  │  ├─ 💻 color.ts (0.2 KB)
│     │  │  │  │  ├─ 💻 convert.ts (1.9 KB)
│     │  │  │  │  ├─ 💻 generator.ts (1.1 KB)
│     │  │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  │  ├─ 📁 cache
│     │  │  │  │  ├─ 📁 __tests__
│     │  │  │  │  │  └─ 💻 storage-manager.test.ts (4.9 KB)
│     │  │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  │  ├─ 💻 indexeddb-driver.ts (4.1 KB)
│     │  │  │  │  ├─ 💻 local-storage-driver.ts (1.7 KB)
│     │  │  │  │  ├─ 💻 memory-storage-driver.ts (0.7 KB)
│     │  │  │  │  ├─ 📜 README.md (12.0 KB)
│     │  │  │  │  ├─ 💻 storage-manager.ts (4.1 KB)
│     │  │  │  │  └─ 💻 types.ts (1.0 KB)
│     │  │  │  ├─ 💻 global-state.ts (1.0 KB)
│     │  │  │  └─ 💻 store.ts (0.0 KB)
│     │  │  ├─ 📊 package.json (2.8 KB)
│     │  │  ├─ 📊 tsconfig.json (0.2 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.4 KB)
│     │  ├─ 📁 icons
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 💻 create-icon.ts (0.3 KB)
│     │  │  │  ├─ 💻 index.ts (0.2 KB)
│     │  │  │  └─ 💻 lucide.ts (1.3 KB)
│     │  │  ├─ 📊 package.json (0.9 KB)
│     │  │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  │  └─ 💻 tsdown.config.ts (0.2 KB)
│     │  ├─ 📁 design
│     │  │  ├─ 📁 src
│     │  │  │  ├─ 📁 scss-bem
│     │  │  │  │  ├─ 📄 bem.scss (0.4 KB)
│     │  │  │  │  └─ 📄 constants.scss (0.2 KB)
│     │  │  │  ├─ 📁 design-tokens
│     │  │  │  │  ├─ 📄 dark.css (12.3 KB)
│     │  │  │  │  ├─ 📄 default.css (13.3 KB)
│     │  │  │  │  └─ 💻 index.ts (0.0 KB)
│     │  │  │  ├─ 📁 css
│     │  │  │  │  ├─ 📄 global.css (0.0 KB)
│     │  │  │  │  ├─ 📄 nprogress.css (1.1 KB)
│     │  │  │  │  ├─ 📄 transition.css (3.4 KB)
│     │  │  │  │  └─ 📄 ui.css (1.4 KB)
│     │  │  │  └─ 💻 index.ts (0.1 KB)
│     │  │  ├─ 📊 package.json (1.1 KB)
│     │  │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  │  └─ 💻 vite.config.ts (0.2 KB)
│     │  └─ 📜 README.md (0.1 KB)
│     └─ 📜 README.md (0.2 KB)
├─ 📁 internal
│  ├─ 📁 vite-config
│  │  ├─ 📁 src
│  │  │  ├─ 📁 utils
│  │  │  │  └─ 💻 env.ts (2.8 KB)
│  │  │  ├─ 📁 plugins
│  │  │  │  ├─ 📁 inject-app-loading
│  │  │  │  │  ├─ 📄 default-loading-antd.html (1.9 KB)
│  │  │  │  │  ├─ 📄 default-loading.html (2.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (1.9 KB)
│  │  │  │  │  └─ 📜 README.md (0.1 KB)
│  │  │  │  ├─ 💻 archiver.ts (1.8 KB)
│  │  │  │  ├─ 💻 dayjs.ts (2.0 KB)
│  │  │  │  ├─ 💻 extra-app-config.ts (2.2 KB)
│  │  │  │  ├─ 💻 html.ts (0.8 KB)
│  │  │  │  ├─ 💻 importmap.ts (6.5 KB)
│  │  │  │  ├─ 💻 index.ts (6.5 KB)
│  │  │  │  ├─ 💻 inject-metadata.ts (2.8 KB)
│  │  │  │  ├─ 💻 license.ts (1.3 KB)
│  │  │  │  ├─ 💻 nitro-mock.ts (2.5 KB)
│  │  │  │  ├─ 💻 print.ts (0.7 KB)
│  │  │  │  ├─ 💻 tailwind-reference.ts (1.2 KB)
│  │  │  │  └─ 💻 vxe-table.ts (0.4 KB)
│  │  │  ├─ 📁 config
│  │  │  │  ├─ 💻 application.ts (3.5 KB)
│  │  │  │  ├─ 💻 common.ts (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (1.0 KB)
│  │  │  │  └─ 💻 library.ts (1.6 KB)
│  │  │  ├─ 💻 index.ts (0.2 KB)
│  │  │  ├─ 💻 options.ts (1.3 KB)
│  │  │  └─ 💻 typing.ts (7.4 KB)
│  │  ├─ 📊 package.json (1.6 KB)
│  │  ├─ 📊 tsconfig.json (0.2 KB)
│  │  └─ 💻 tsdown.config.ts (0.9 KB)
│  ├─ 📁 tsconfig
│  │  ├─ 📊 base.json (1.0 KB)
│  │  ├─ 📊 library.json (0.3 KB)
│  │  ├─ 📊 node.json (0.3 KB)
│  │  ├─ 📊 package.json (0.5 KB)
│  │  ├─ 📊 web-app.json (0.2 KB)
│  │  └─ 📊 web.json (0.4 KB)
│  ├─ 📁 tailwind-config
│  │  ├─ 📁 src
│  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  └─ 📄 theme.css (16.5 KB)
│  │  ├─ 📊 package.json (0.9 KB)
│  │  └─ 📊 tsconfig.json (0.1 KB)
│  ├─ 📁 node-utils
│  │  ├─ 📁 src
│  │  │  ├─ 📁 __tests__
│  │  │  │  ├─ 💻 hash.test.ts (1.7 KB)
│  │  │  │  └─ 💻 path.test.ts (2.3 KB)
│  │  │  ├─ 💻 constants.ts (0.1 KB)
│  │  │  ├─ 💻 date.ts (0.2 KB)
│  │  │  ├─ 💻 formatter.ts (0.2 KB)
│  │  │  ├─ 💻 fs.ts (1.0 KB)
│  │  │  ├─ 💻 git.ts (0.8 KB)
│  │  │  ├─ 💻 hash.ts (0.4 KB)
│  │  │  ├─ 💻 index.ts (0.6 KB)
│  │  │  ├─ 💻 monorepo.ts (1.2 KB)
│  │  │  ├─ 💻 path.ts (0.3 KB)
│  │  │  └─ 💻 spinner.ts (0.5 KB)
│  │  ├─ 📁 scripts
│  │  │  └─ 📄 build.mjs (0.8 KB)
│  │  ├─ 📊 package.json (1.0 KB)
│  │  ├─ 📊 tsconfig.build.json (0.2 KB)
│  │  ├─ 📊 tsconfig.json (0.1 KB)
│  │  └─ 💻 tsdown.config.ts (0.2 KB)
│  └─ 📁 lint-configs
│     ├─ 📁 stylelint-config
│     │  ├─ 📄 index.mjs (3.3 KB)
│     │  └─ 📊 package.json (1.1 KB)
│     ├─ 📁 oxlint-config
│     │  ├─ 📁 src
│     │  │  ├─ 📁 configs
│     │  │  │  ├─ 💻 command.ts (0.2 KB)
│     │  │  │  ├─ 💻 comments.ts (0.5 KB)
│     │  │  │  ├─ 💻 ignores.ts (0.3 KB)
│     │  │  │  ├─ 💻 import.ts (0.6 KB)
│     │  │  │  ├─ 💻 index.ts (2.4 KB)
│     │  │  │  ├─ 💻 javascript.ts (3.2 KB)
│     │  │  │  ├─ 💻 node.ts (0.2 KB)
│     │  │  │  ├─ 💻 overrides.ts (2.3 KB)
│     │  │  │  ├─ 💻 plugins.ts (0.2 KB)
│     │  │  │  ├─ 💻 tailwindcss.ts (1.5 KB)
│     │  │  │  ├─ 💻 test.ts (0.6 KB)
│     │  │  │  ├─ 💻 typescript.ts (1.1 KB)
│     │  │  │  ├─ 💻 unicorn.ts (0.4 KB)
│     │  │  │  └─ 💻 vue.ts (0.1 KB)
│     │  │  └─ 💻 index.ts (0.6 KB)
│     │  ├─ 📊 package.json (0.9 KB)
│     │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  └─ 💻 tsdown.config.ts (0.2 KB)
│     ├─ 📁 oxfmt-config
│     │  ├─ 📁 src
│     │  │  └─ 💻 index.ts (0.8 KB)
│     │  ├─ 📊 package.json (0.7 KB)
│     │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  └─ 💻 tsdown.config.ts (0.2 KB)
│     ├─ 📁 eslint-config
│     │  ├─ 📁 src
│     │  │  ├─ 📁 configs
│     │  │  │  ├─ 💻 ignores.ts (1.3 KB)
│     │  │  │  ├─ 💻 index.ts (0.3 KB)
│     │  │  │  ├─ 💻 javascript.ts (4.6 KB)
│     │  │  │  ├─ 💻 jsonc.ts (7.3 KB)
│     │  │  │  ├─ 💻 node.ts (2.0 KB)
│     │  │  │  ├─ 💻 perfectionist.ts (2.9 KB)
│     │  │  │  ├─ 💻 pnpm.ts (0.9 KB)
│     │  │  │  ├─ 💻 typescript.ts (1.9 KB)
│     │  │  │  ├─ 💻 unicorn.ts (1.3 KB)
│     │  │  │  ├─ 💻 vue.ts (4.6 KB)
│     │  │  │  └─ 💻 yaml.ts (2.6 KB)
│     │  │  ├─ 💻 custom-config.ts (3.7 KB)
│     │  │  ├─ 💻 index.ts (0.7 KB)
│     │  │  └─ 💻 util.ts (0.2 KB)
│     │  ├─ 📊 package.json (1.3 KB)
│     │  ├─ 📊 tsconfig.json (0.1 KB)
│     │  └─ 💻 tsdown.config.ts (0.3 KB)
│     └─ 📁 commitlint-config
│        ├─ 💻 index.d.ts (0.2 KB)
│        ├─ 📄 index.mjs (4.5 KB)
│        └─ 📊 package.json (0.8 KB)
├─ 📁 docs
│  ├─ 📁 src
│  │  ├─ 📁 _env
│  │  │  ├─ 📁 node
│  │  │  │  └─ 📁 adapter
│  │  │  │     ├─ 💻 form.ts (0.1 KB)
│  │  │  │     └─ 💻 vxe-table.ts (0.1 KB)
│  │  │  └─ 📁 adapter
│  │  │     ├─ 💻 component.ts (3.2 KB)
│  │  │     ├─ 💻 form.ts (1.2 KB)
│  │  │     └─ 💻 vxe-table.ts (1.8 KB)
│  │  ├─ 📁 sponsor
│  │  │  └─ 📜 personal.md (0.5 KB)
│  │  ├─ 📁 public
│  │  │  ├─ 📁 logos
│  │  │  │  ├─ 📄 nitro.svg (3.3 KB)
│  │  │  │  ├─ 📄 shadcn-ui.svg (0.5 KB)
│  │  │  │  ├─ 📄 turborepo.svg (4.0 KB)
│  │  │  │  └─ 📄 vite.svg (1.5 KB)
│  │  │  ├─ 📁 guide
│  │  │  │  ├─ 🖼️ devtools.png (392.7 KB)
│  │  │  │  ├─ 🖼️ loading.png (87.2 KB)
│  │  │  │  ├─ 🖼️ locale.png (471.0 KB)
│  │  │  │  ├─ 🖼️ login-expired.png (556.0 KB)
│  │  │  │  ├─ 🖼️ login.png (469.0 KB)
│  │  │  │  ├─ 🖼️ preferences.png (123.2 KB)
│  │  │  │  ├─ 🖼️ qq.png (446.7 KB)
│  │  │  │  ├─ 🖼️ qq_channel.png (447.7 KB)
│  │  │  │  ├─ 🖼️ report.png (1000.2 KB)
│  │  │  │  ├─ 🖼️ test.png (249.0 KB)
│  │  │  │  └─ 🖼️ update-notice.png (400.5 KB)
│  │  │  └─ 📄 favicon.ico (5.3 KB)
│  │  ├─ 📁 guide
│  │  │  ├─ 📁 project
│  │  │  │  ├─ 📜 changeset.md (0.5 KB)
│  │  │  │  ├─ 📜 cli.md (2.3 KB)
│  │  │  │  ├─ 📜 dir.md (3.0 KB)
│  │  │  │  ├─ 📜 standard.md (6.9 KB)
│  │  │  │  ├─ 📜 tailwindcss.md (1.4 KB)
│  │  │  │  ├─ 📜 test.md (1.1 KB)
│  │  │  │  └─ 📜 vite.md (0.6 KB)
│  │  │  ├─ 📁 other
│  │  │  │  ├─ 📜 faq.md (5.4 KB)
│  │  │  │  ├─ 📜 project-update.md (1.5 KB)
│  │  │  │  └─ 📜 remove-code.md (0.4 KB)
│  │  │  ├─ 📁 introduction
│  │  │  │  ├─ 📜 changelog.md (0.0 KB)
│  │  │  │  ├─ 📜 quick-start.md (2.4 KB)
│  │  │  │  ├─ 📜 roadmap.md (0.0 KB)
│  │  │  │  ├─ 📜 thin.md (3.5 KB)
│  │  │  │  ├─ 📜 vben.md (4.0 KB)
│  │  │  │  └─ 📜 why.md (2.3 KB)
│  │  │  ├─ 📁 in-depth
│  │  │  │  ├─ 📜 access.md (10.1 KB)
│  │  │  │  ├─ 📜 check-updates.md (2.6 KB)
│  │  │  │  ├─ 📜 features.md (1.7 KB)
│  │  │  │  ├─ 📜 layout.md (0.0 KB)
│  │  │  │  ├─ 📜 loading.md (1.2 KB)
│  │  │  │  ├─ 📜 locale.md (6.3 KB)
│  │  │  │  ├─ 📜 login.md (4.6 KB)
│  │  │  │  ├─ 📜 theme.md (32.8 KB)
│  │  │  │  └─ 📜 ui-framework.md (0.9 KB)
│  │  │  └─ 📁 essentials
│  │  │     ├─ 📜 build.md (6.2 KB)
│  │  │     ├─ 📜 concept.md (2.0 KB)
│  │  │     ├─ 📜 development.md (8.9 KB)
│  │  │     ├─ 📜 external-module.md (1.1 KB)
│  │  │     ├─ 📜 icons.md (1.8 KB)
│  │  │     ├─ 📜 route.md (15.3 KB)
│  │  │     ├─ 📜 server.md (11.4 KB)
│  │  │     ├─ 📜 settings.md (20.7 KB)
│  │  │     └─ 📜 styles.md (2.6 KB)
│  │  ├─ 📁 friend-links
│  │  │  └─ 📜 index.md (1.0 KB)
│  │  ├─ 📁 en
│  │  │  ├─ 📁 guide
│  │  │  │  ├─ 📁 project
│  │  │  │  │  ├─ 📜 changeset.md (0.6 KB)
│  │  │  │  │  ├─ 📜 cli.md (2.4 KB)
│  │  │  │  │  ├─ 📜 dir.md (3.3 KB)
│  │  │  │  │  ├─ 📜 standard.md (7.3 KB)
│  │  │  │  │  ├─ 📜 tailwindcss.md (1.5 KB)
│  │  │  │  │  ├─ 📜 test.md (1.2 KB)
│  │  │  │  │  └─ 📜 vite.md (0.7 KB)
│  │  │  │  ├─ 📁 other
│  │  │  │  │  ├─ 📜 faq.md (6.4 KB)
│  │  │  │  │  ├─ 📜 project-update.md (1.7 KB)
│  │  │  │  │  └─ 📜 remove-code.md (0.4 KB)
│  │  │  │  ├─ 📁 introduction
│  │  │  │  │  ├─ 📜 changelog.md (0.0 KB)
│  │  │  │  │  ├─ 📜 quick-start.md (2.2 KB)
│  │  │  │  │  ├─ 📜 roadmap.md (0.0 KB)
│  │  │  │  │  ├─ 📜 thin.md (2.3 KB)
│  │  │  │  │  ├─ 📜 vben.md (4.4 KB)
│  │  │  │  │  └─ 📜 why.md (1.4 KB)
│  │  │  │  ├─ 📁 in-depth
│  │  │  │  │  ├─ 📜 access.md (11.4 KB)
│  │  │  │  │  ├─ 📜 check-updates.md (1.5 KB)
│  │  │  │  │  ├─ 📜 features.md (2.0 KB)
│  │  │  │  │  ├─ 📜 layout.md (0.0 KB)
│  │  │  │  │  ├─ 📜 loading.md (1.3 KB)
│  │  │  │  │  ├─ 📜 locale.md (6.9 KB)
│  │  │  │  │  ├─ 📜 login.md (2.4 KB)
│  │  │  │  │  ├─ 📜 theme.md (33.2 KB)
│  │  │  │  │  └─ 📜 ui-framework.md (1.0 KB)
│  │  │  │  └─ 📁 essentials
│  │  │  │     ├─ 📜 build.md (7.0 KB)
│  │  │  │     ├─ 📜 concept.md (2.3 KB)
│  │  │  │     ├─ 📜 development.md (9.8 KB)
│  │  │  │     ├─ 📜 external-module.md (1.3 KB)
│  │  │  │     ├─ 📜 icons.md (2.0 KB)
│  │  │  │     ├─ 📜 route.md (15.0 KB)
│  │  │  │     ├─ 📜 server.md (11.4 KB)
│  │  │  │     ├─ 📜 settings.md (22.7 KB)
│  │  │  │     └─ 📜 styles.md (2.8 KB)
│  │  │  ├─ 📁 components
│  │  │  │  ├─ 📁 layout-ui
│  │  │  │  │  └─ 📜 page.md (1.1 KB)
│  │  │  │  ├─ 📁 common-ui
│  │  │  │  │  ├─ 📜 vben-alert.md (1.9 KB)
│  │  │  │  │  ├─ 📜 vben-api-component.md (2.2 KB)
│  │  │  │  │  ├─ 📜 vben-count-to-animator.md (1.8 KB)
│  │  │  │  │  ├─ 📜 vben-cropper.md (4.5 KB)
│  │  │  │  │  ├─ 📜 vben-drawer.md (2.2 KB)
│  │  │  │  │  ├─ 📜 vben-ellipsis-text.md (1.5 KB)
│  │  │  │  │  ├─ 📜 vben-form.md (5.4 KB)
│  │  │  │  │  ├─ 📜 vben-modal.md (2.3 KB)
│  │  │  │  │  ├─ 📜 vben-tiptap.md (5.5 KB)
│  │  │  │  │  └─ 📜 vben-vxe-table.md (2.7 KB)
│  │  │  │  └─ 📜 introduction.md (0.7 KB)
│  │  │  └─ 📜 index.md (2.7 KB)
│  │  ├─ 📁 demos
│  │  │  ├─ 📁 vben-vxe-table
│  │  │  │  ├─ 📁 virtual
│  │  │  │  │  └─ 📄 index.vue (1.2 KB)
│  │  │  │  ├─ 📁 tree
│  │  │  │  │  └─ 📄 index.vue (1.6 KB)
│  │  │  │  ├─ 📁 remote
│  │  │  │  │  └─ 📄 index.vue (2.6 KB)
│  │  │  │  ├─ 📁 form
│  │  │  │  │  └─ 📄 index.vue (2.9 KB)
│  │  │  │  ├─ 📁 fixed
│  │  │  │  │  └─ 📄 index.vue (1.4 KB)
│  │  │  │  ├─ 📁 edit-row
│  │  │  │  │  └─ 📄 index.vue (2.2 KB)
│  │  │  │  ├─ 📁 edit-cell
│  │  │  │  │  └─ 📄 index.vue (1.2 KB)
│  │  │  │  ├─ 📁 custom-cell
│  │  │  │  │  └─ 📄 index.vue (2.4 KB)
│  │  │  │  ├─ 📁 basic
│  │  │  │  │  └─ 📄 index.vue (2.1 KB)
│  │  │  │  ├─ 💻 mock-api.ts (0.8 KB)
│  │  │  │  └─ 💻 table-data.ts (9.8 KB)
│  │  │  ├─ 📁 vben-tiptap
│  │  │  │  ├─ 📁 image-upload
│  │  │  │  │  └─ 📄 index.vue (1.3 KB)
│  │  │  │  └─ 📁 basic
│  │  │  │     └─ 📄 index.vue (0.4 KB)
│  │  │  ├─ 📁 vben-modal
│  │  │  │  ├─ 📁 shared-data
│  │  │  │  │  ├─ 📄 index.vue (0.5 KB)
│  │  │  │  │  └─ 📄 modal.vue (0.5 KB)
│  │  │  │  ├─ 📁 extra
│  │  │  │  │  ├─ 📄 index.vue (0.4 KB)
│  │  │  │  │  └─ 📄 modal.vue (0.2 KB)
│  │  │  │  ├─ 📁 dynamic
│  │  │  │  │  ├─ 📄 index.vue (0.6 KB)
│  │  │  │  │  └─ 📄 modal.vue (0.9 KB)
│  │  │  │  ├─ 📁 draggable
│  │  │  │  │  ├─ 📄 index.vue (0.4 KB)
│  │  │  │  │  └─ 📄 modal.vue (0.2 KB)
│  │  │  │  ├─ 📁 basic
│  │  │  │  │  └─ 📄 index.vue (0.3 KB)
│  │  │  │  ├─ 📁 auto-height
│  │  │  │  │  ├─ 📄 index.vue (0.4 KB)
│  │  │  │  │  └─ 📄 modal.vue (0.9 KB)
│  │  │  │  └─ 📁 animation-type
│  │  │  │     └─ 📄 index.vue (0.9 KB)
│  │  │  ├─ 📁 vben-form
│  │  │  │  ├─ 📁 value-format
│  │  │  │  │  └─ 📄 index.vue (3.5 KB)
│  │  │  │  ├─ 📁 rules
│  │  │  │  │  └─ 📄 index.vue (4.1 KB)
│  │  │  │  ├─ 📁 query
│  │  │  │  │  └─ 📄 index.vue (2.0 KB)
│  │  │  │  ├─ 📁 dynamic
│  │  │  │  │  └─ 📄 index.vue (3.7 KB)
│  │  │  │  ├─ 📁 custom
│  │  │  │  │  └─ 📄 index.vue (1.6 KB)
│  │  │  │  ├─ 📁 basic
│  │  │  │  │  └─ 📄 index.vue (4.9 KB)
│  │  │  │  └─ 📁 api
│  │  │  │     └─ 📄 index.vue (6.1 KB)
│  │  │  ├─ 📁 vben-ellipsis-text
│  │  │  │  ├─ 📁 tooltip
│  │  │  │  │  └─ 📄 index.vue (0.5 KB)
│  │  │  │  ├─ 📁 line
│  │  │  │  │  └─ 📄 index.vue (2.4 KB)
│  │  │  │  ├─ 📁 expand
│  │  │  │  │  └─ 📄 index.vue (2.4 KB)
│  │  │  │  └─ 📁 auto-display
│  │  │  │     └─ 📄 index.vue (0.4 KB)
│  │  │  ├─ 📁 vben-drawer
│  │  │  │  ├─ 📁 shared-data
│  │  │  │  │  ├─ 📄 drawer.vue (0.5 KB)
│  │  │  │  │  └─ 📄 index.vue (0.5 KB)
│  │  │  │  ├─ 📁 extra
│  │  │  │  │  ├─ 📄 drawer.vue (0.2 KB)
│  │  │  │  │  └─ 📄 index.vue (0.4 KB)
│  │  │  │  ├─ 📁 dynamic
│  │  │  │  │  ├─ 📄 drawer.vue (0.6 KB)
│  │  │  │  │  └─ 📄 index.vue (0.6 KB)
│  │  │  │  ├─ 📁 basic
│  │  │  │  │  └─ 📄 index.vue (0.3 KB)
│  │  │  │  └─ 📁 auto-height
│  │  │  │     ├─ 📄 drawer.vue (0.9 KB)
│  │  │  │     └─ 📄 index.vue (0.4 KB)
│  │  │  ├─ 📁 vben-cropper
│  │  │  │  ├─ 📁 basic
│  │  │  │  │  └─ 📄 index.vue (2.0 KB)
│  │  │  │  └─ 📁 aspect-ratio
│  │  │  │     └─ 📄 index.vue (2.7 KB)
│  │  │  ├─ 📁 vben-count-to-animator
│  │  │  │  ├─ 📁 custom
│  │  │  │  │  └─ 📄 index.vue (0.2 KB)
│  │  │  │  └─ 📁 basic
│  │  │  │     └─ 📄 index.vue (0.2 KB)
│  │  │  ├─ 📁 vben-api-component
│  │  │  │  └─ 📁 cascader
│  │  │  │     └─ 📄 index.vue (1.9 KB)
│  │  │  └─ 📁 vben-alert
│  │  │     ├─ 📁 prompt
│  │  │     │  └─ 📄 index.vue (3.2 KB)
│  │  │     ├─ 📁 confirm
│  │  │     │  └─ 📄 index.vue (2.0 KB)
│  │  │     └─ 📁 alert
│  │  │        └─ 📄 index.vue (0.8 KB)
│  │  ├─ 📁 components
│  │  │  ├─ 📁 layout-ui
│  │  │  │  └─ 📜 page.md (1.6 KB)
│  │  │  ├─ 📁 common-ui
│  │  │  │  ├─ 📜 vben-alert.md (3.2 KB)
│  │  │  │  ├─ 📜 vben-api-component.md (6.6 KB)
│  │  │  │  ├─ 📜 vben-count-to-animator.md (2.1 KB)
│  │  │  │  ├─ 📜 vben-cropper.md (4.8 KB)
│  │  │  │  ├─ 📜 vben-drawer.md (7.8 KB)
│  │  │  │  ├─ 📜 vben-ellipsis-text.md (2.2 KB)
│  │  │  │  ├─ 📜 vben-form.md (19.6 KB)
│  │  │  │  ├─ 📜 vben-modal.md (8.3 KB)
│  │  │  │  ├─ 📜 vben-tiptap.md (5.4 KB)
│  │  │  │  └─ 📜 vben-vxe-table.md (6.8 KB)
│  │  │  └─ 📜 introduction.md (0.7 KB)
│  │  ├─ 📁 commercial
│  │  │  ├─ 📜 community.md (1.2 KB)
│  │  │  ├─ 📜 customized.md (0.5 KB)
│  │  │  └─ 📜 technical-support.md (0.3 KB)
│  │  └─ 📜 index.md (3.1 KB)
│  ├─ 📁 .vitepress
│  │  ├─ 📁 theme
│  │  │  ├─ 📁 styles
│  │  │  │  ├─ 📄 base.css (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  └─ 📄 variables.css (4.2 KB)
│  │  │  ├─ 📁 plugins
│  │  │  │  └─ 💻 hm.ts (0.7 KB)
│  │  │  ├─ 📁 components
│  │  │  │  ├─ 📄 site-layout.vue (1.9 KB)
│  │  │  │  └─ 📄 vben-contributors.vue (0.6 KB)
│  │  │  └─ 💻 index.ts (0.9 KB)
│  │  ├─ 📁 config
│  │  │  ├─ 📁 plugins
│  │  │  │  └─ 💻 demo-preview.ts (4.6 KB)
│  │  │  ├─ 📄 en.mts (7.6 KB)
│  │  │  ├─ 📄 index.mts (0.5 KB)
│  │  │  ├─ 📄 shared.mts (4.4 KB)
│  │  │  └─ 📄 zh.mts (9.7 KB)
│  │  ├─ 📁 components
│  │  │  ├─ 📄 demo-preview.vue (1.2 KB)
│  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  └─ 📄 preview-group.vue (2.8 KB)
│  │  └─ 📁 build
│  │     ├─ 📄 dayjs-loader.mjs (0.1 KB)
│  │     └─ 📄 dayjs-resolve-hook.mjs (0.3 KB)
│  ├─ 📊 package.json (1.0 KB)
│  └─ 📊 tsconfig.json (0.4 KB)
├─ 📁 apps
│  ├─ 📁 web-tdesign
│  │  ├─ 📁 src
│  │  │  ├─ 📁 views
│  │  │  │  ├─ 📁 _core
│  │  │  │  │  ├─ 📁 profile
│  │  │  │  │  │  ├─ 📄 base-setting.vue (1.3 KB)
│  │  │  │  │  │  ├─ 📄 index.vue (1.1 KB)
│  │  │  │  │  │  ├─ 📄 notification-setting.vue (0.7 KB)
│  │  │  │  │  │  ├─ 📄 password-setting.vue (1.5 KB)
│  │  │  │  │  │  └─ 📄 security-setting.vue (1.0 KB)
│  │  │  │  │  ├─ 📁 fallback
│  │  │  │  │  │  ├─ 📄 coming-soon.vue (0.1 KB)
│  │  │  │  │  │  ├─ 📄 forbidden.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 internal-error.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 not-found.vue (0.2 KB)
│  │  │  │  │  │  └─ 📄 offline.vue (0.2 KB)
│  │  │  │  │  ├─ 📁 authentication
│  │  │  │  │  │  ├─ 📄 code-login.vue (1.7 KB)
│  │  │  │  │  │  ├─ 📄 forget-password.vue (0.9 KB)
│  │  │  │  │  │  ├─ 📄 login.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 qrcode-login.vue (0.3 KB)
│  │  │  │  │  │  └─ 📄 register.vue (2.5 KB)
│  │  │  │  │  ├─ 📁 about
│  │  │  │  │  │  └─ 📄 index.vue (0.1 KB)
│  │  │  │  │  └─ 📜 README.md (0.1 KB)
│  │  │  │  ├─ 📁 demos
│  │  │  │  │  └─ 📁 tdesign
│  │  │  │  │     └─ 📄 index.vue (1.8 KB)
│  │  │  │  └─ 📁 dashboard
│  │  │  │     ├─ 📁 workspace
│  │  │  │     │  └─ 📄 index.vue (6.9 KB)
│  │  │  │     └─ 📁 analytics
│  │  │  │        ├─ 📄 analytics-trends.vue (2.0 KB)
│  │  │  │        ├─ 📄 analytics-visits-data.vue (1.6 KB)
│  │  │  │        ├─ 📄 analytics-visits-sales.vue (1.1 KB)
│  │  │  │        ├─ 📄 analytics-visits-source.vue (1.5 KB)
│  │  │  │        ├─ 📄 analytics-visits.vue (1.1 KB)
│  │  │  │        └─ 📄 index.vue (2.1 KB)
│  │  │  ├─ 📁 store
│  │  │  │  ├─ 💻 auth.ts (2.9 KB)
│  │  │  │  └─ 💻 index.ts (0.0 KB)
│  │  │  ├─ 📁 router
│  │  │  │  ├─ 📁 routes
│  │  │  │  │  ├─ 📁 modules
│  │  │  │  │  │  ├─ 💻 dashboard.ts (0.9 KB)
│  │  │  │  │  │  ├─ 💻 demos.ts (0.5 KB)
│  │  │  │  │  │  └─ 💻 vben.ts (2.7 KB)
│  │  │  │  │  ├─ 💻 core.ts (2.4 KB)
│  │  │  │  │  └─ 💻 index.ts (1.5 KB)
│  │  │  │  ├─ 💻 access.ts (1.1 KB)
│  │  │  │  ├─ 💻 guard.ts (3.6 KB)
│  │  │  │  └─ 💻 index.ts (0.9 KB)
│  │  │  ├─ 📁 locales
│  │  │  │  ├─ 📁 langs
│  │  │  │  │  ├─ 📁 zh-CN
│  │  │  │  │  │  ├─ 📊 demos.json (0.3 KB)
│  │  │  │  │  │  └─ 📊 page.json (0.3 KB)
│  │  │  │  │  └─ 📁 en-US
│  │  │  │  │     ├─ 📊 demos.json (0.3 KB)
│  │  │  │  │     └─ 📊 page.json (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (1.7 KB)
│  │  │  │  └─ 📜 README.md (0.2 KB)
│  │  │  ├─ 📁 layouts
│  │  │  │  ├─ 📄 auth.vue (0.6 KB)
│  │  │  │  ├─ 📄 basic.vue (6.1 KB)
│  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  ├─ 📁 api
│  │  │  │  ├─ 📁 core
│  │  │  │  │  ├─ 💻 auth.ts (1.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 💻 menu.ts (0.3 KB)
│  │  │  │  │  └─ 💻 user.ts (0.2 KB)
│  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  └─ 💻 request.ts (3.2 KB)
│  │  │  ├─ 📁 adapter
│  │  │  │  ├─ 📁 component
│  │  │  │  │  └─ 💻 index.ts (8.3 KB)
│  │  │  │  ├─ 💻 form.ts (1.4 KB)
│  │  │  │  ├─ 💻 tdesign.ts (0.1 KB)
│  │  │  │  └─ 💻 vxe-table.ts (2.0 KB)
│  │  │  ├─ 📄 app.vue (0.9 KB)
│  │  │  ├─ 💻 bootstrap.ts (2.0 KB)
│  │  │  ├─ 💻 main.ts (0.9 KB)
│  │  │  └─ 💻 preferences.ts (0.4 KB)
│  │  ├─ 📁 public
│  │  │  └─ 📄 favicon.ico (5.3 KB)
│  │  ├─ 📄 .env (0.3 KB)
│  │  ├─ 📄 .env.analyze (0.1 KB)
│  │  ├─ 📄 .env.development (0.3 KB)
│  │  ├─ 📄 .env.production (0.3 KB)
│  │  ├─ 📄 index.html (1.2 KB)
│  │  ├─ 📊 package.json (1.4 KB)
│  │  ├─ 📊 tsconfig.json (0.3 KB)
│  │  ├─ 📊 tsconfig.node.json (0.3 KB)
│  │  └─ 💻 vite.config.ts (0.4 KB)
│  ├─ 📁 web-naive
│  │  ├─ 📁 src
│  │  │  ├─ 📁 views
│  │  │  │  ├─ 📁 _core
│  │  │  │  │  ├─ 📁 profile
│  │  │  │  │  │  ├─ 📄 base-setting.vue (1.3 KB)
│  │  │  │  │  │  ├─ 📄 index.vue (1.1 KB)
│  │  │  │  │  │  ├─ 📄 notification-setting.vue (0.7 KB)
│  │  │  │  │  │  ├─ 📄 password-setting.vue (1.5 KB)
│  │  │  │  │  │  └─ 📄 security-setting.vue (1.0 KB)
│  │  │  │  │  ├─ 📁 fallback
│  │  │  │  │  │  ├─ 📄 coming-soon.vue (0.1 KB)
│  │  │  │  │  │  ├─ 📄 forbidden.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 internal-error.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 not-found.vue (0.2 KB)
│  │  │  │  │  │  └─ 📄 offline.vue (0.2 KB)
│  │  │  │  │  ├─ 📁 authentication
│  │  │  │  │  │  ├─ 📄 code-login.vue (1.7 KB)
│  │  │  │  │  │  ├─ 📄 forget-password.vue (0.9 KB)
│  │  │  │  │  │  ├─ 📄 login.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 qrcode-login.vue (0.3 KB)
│  │  │  │  │  │  └─ 📄 register.vue (2.5 KB)
│  │  │  │  │  ├─ 📁 about
│  │  │  │  │  │  └─ 📄 index.vue (0.1 KB)
│  │  │  │  │  └─ 📜 README.md (0.1 KB)
│  │  │  │  ├─ 📁 demos
│  │  │  │  │  ├─ 📁 table
│  │  │  │  │  │  └─ 📄 index.vue (0.7 KB)
│  │  │  │  │  ├─ 📁 naive
│  │  │  │  │  │  └─ 📄 index.vue (2.1 KB)
│  │  │  │  │  └─ 📁 form
│  │  │  │  │     ├─ 📄 basic.vue (4.4 KB)
│  │  │  │  │     └─ 📄 modal.vue (1.4 KB)
│  │  │  │  └─ 📁 dashboard
│  │  │  │     ├─ 📁 workspace
│  │  │  │     │  └─ 📄 index.vue (6.9 KB)
│  │  │  │     └─ 📁 analytics
│  │  │  │        ├─ 📄 analytics-trends.vue (2.0 KB)
│  │  │  │        ├─ 📄 analytics-visits-data.vue (1.6 KB)
│  │  │  │        ├─ 📄 analytics-visits-sales.vue (1.1 KB)
│  │  │  │        ├─ 📄 analytics-visits-source.vue (1.5 KB)
│  │  │  │        ├─ 📄 analytics-visits.vue (1.1 KB)
│  │  │  │        └─ 📄 index.vue (2.1 KB)
│  │  │  ├─ 📁 store
│  │  │  │  ├─ 💻 auth.ts (3.0 KB)
│  │  │  │  └─ 💻 index.ts (0.0 KB)
│  │  │  ├─ 📁 router
│  │  │  │  ├─ 📁 routes
│  │  │  │  │  ├─ 📁 modules
│  │  │  │  │  │  ├─ 💻 dashboard.ts (0.9 KB)
│  │  │  │  │  │  ├─ 💻 demos.ts (0.9 KB)
│  │  │  │  │  │  └─ 💻 vben.ts (2.7 KB)
│  │  │  │  │  ├─ 💻 core.ts (2.4 KB)
│  │  │  │  │  └─ 💻 index.ts (1.5 KB)
│  │  │  │  ├─ 💻 access.ts (1.1 KB)
│  │  │  │  ├─ 💻 guard.ts (3.6 KB)
│  │  │  │  └─ 💻 index.ts (0.9 KB)
│  │  │  ├─ 📁 locales
│  │  │  │  ├─ 📁 langs
│  │  │  │  │  ├─ 📁 zh-CN
│  │  │  │  │  │  ├─ 📊 demos.json (0.4 KB)
│  │  │  │  │  │  └─ 📊 page.json (0.3 KB)
│  │  │  │  │  └─ 📁 en-US
│  │  │  │  │     ├─ 📊 demos.json (0.4 KB)
│  │  │  │  │     └─ 📊 page.json (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (0.9 KB)
│  │  │  │  └─ 📜 README.md (0.2 KB)
│  │  │  ├─ 📁 layouts
│  │  │  │  ├─ 📄 auth.vue (0.7 KB)
│  │  │  │  ├─ 📄 basic.vue (6.1 KB)
│  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  ├─ 📁 api
│  │  │  │  ├─ 📁 core
│  │  │  │  │  ├─ 💻 auth.ts (1.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 💻 menu.ts (0.3 KB)
│  │  │  │  │  └─ 💻 user.ts (0.2 KB)
│  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  └─ 💻 request.ts (3.2 KB)
│  │  │  ├─ 📁 adapter
│  │  │  │  ├─ 📁 component
│  │  │  │  │  └─ 💻 index.ts (7.6 KB)
│  │  │  │  ├─ 💻 form.ts (1.3 KB)
│  │  │  │  ├─ 💻 naive.ts (0.8 KB)
│  │  │  │  └─ 💻 vxe-table.ts (2.0 KB)
│  │  │  ├─ 📄 app.vue (1.2 KB)
│  │  │  ├─ 💻 bootstrap.ts (1.9 KB)
│  │  │  ├─ 💻 main.ts (0.9 KB)
│  │  │  └─ 💻 preferences.ts (0.4 KB)
│  │  ├─ 📁 public
│  │  │  └─ 📄 favicon.ico (5.3 KB)
│  │  ├─ 📄 .env (0.3 KB)
│  │  ├─ 📄 .env.analyze (0.1 KB)
│  │  ├─ 📄 .env.development (0.3 KB)
│  │  ├─ 📄 .env.production (0.3 KB)
│  │  ├─ 📄 index.html (1.2 KB)
│  │  ├─ 📊 package.json (1.4 KB)
│  │  ├─ 📊 tsconfig.json (0.3 KB)
│  │  ├─ 📊 tsconfig.node.json (0.3 KB)
│  │  └─ 💻 vite.config.ts (0.4 KB)
│  ├─ 📁 web-ele
│  │  ├─ 📁 src
│  │  │  ├─ 📁 views
│  │  │  │  ├─ 📁 _core
│  │  │  │  │  ├─ 📁 profile
│  │  │  │  │  │  ├─ 📄 base-setting.vue (1.3 KB)
│  │  │  │  │  │  ├─ 📄 index.vue (1.1 KB)
│  │  │  │  │  │  ├─ 📄 notification-setting.vue (0.7 KB)
│  │  │  │  │  │  ├─ 📄 password-setting.vue (1.5 KB)
│  │  │  │  │  │  └─ 📄 security-setting.vue (1.0 KB)
│  │  │  │  │  ├─ 📁 fallback
│  │  │  │  │  │  ├─ 📄 coming-soon.vue (0.1 KB)
│  │  │  │  │  │  ├─ 📄 forbidden.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 internal-error.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 not-found.vue (0.2 KB)
│  │  │  │  │  │  └─ 📄 offline.vue (0.2 KB)
│  │  │  │  │  ├─ 📁 authentication
│  │  │  │  │  │  ├─ 📄 code-login.vue (1.9 KB)
│  │  │  │  │  │  ├─ 📄 forget-password.vue (0.9 KB)
│  │  │  │  │  │  ├─ 📄 login.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 qrcode-login.vue (0.3 KB)
│  │  │  │  │  │  └─ 📄 register.vue (2.5 KB)
│  │  │  │  │  ├─ 📁 about
│  │  │  │  │  │  └─ 📄 index.vue (0.1 KB)
│  │  │  │  │  └─ 📜 README.md (0.1 KB)
│  │  │  │  ├─ 📁 demos
│  │  │  │  │  ├─ 📁 form
│  │  │  │  │  │  └─ 📄 basic.vue (4.7 KB)
│  │  │  │  │  └─ 📁 element
│  │  │  │  │     └─ 📄 index.vue (3.3 KB)
│  │  │  │  └─ 📁 dashboard
│  │  │  │     ├─ 📁 workspace
│  │  │  │     │  └─ 📄 index.vue (6.9 KB)
│  │  │  │     └─ 📁 analytics
│  │  │  │        ├─ 📄 analytics-trends.vue (2.0 KB)
│  │  │  │        ├─ 📄 analytics-visits-data.vue (1.6 KB)
│  │  │  │        ├─ 📄 analytics-visits-sales.vue (1.1 KB)
│  │  │  │        ├─ 📄 analytics-visits-source.vue (1.5 KB)
│  │  │  │        ├─ 📄 analytics-visits.vue (1.1 KB)
│  │  │  │        └─ 📄 index.vue (2.1 KB)
│  │  │  ├─ 📁 types
│  │  │  │  └─ 💻 element-plus-style-css.d.ts (0.1 KB)
│  │  │  ├─ 📁 store
│  │  │  │  ├─ 💻 auth.ts (3.0 KB)
│  │  │  │  └─ 💻 index.ts (0.0 KB)
│  │  │  ├─ 📁 router
│  │  │  │  ├─ 📁 routes
│  │  │  │  │  ├─ 📁 modules
│  │  │  │  │  │  ├─ 💻 dashboard.ts (0.9 KB)
│  │  │  │  │  │  ├─ 💻 demos.ts (0.7 KB)
│  │  │  │  │  │  └─ 💻 vben.ts (2.7 KB)
│  │  │  │  │  ├─ 💻 core.ts (2.4 KB)
│  │  │  │  │  └─ 💻 index.ts (1.5 KB)
│  │  │  │  ├─ 💻 access.ts (1.1 KB)
│  │  │  │  ├─ 💻 guard.ts (3.6 KB)
│  │  │  │  └─ 💻 index.ts (0.9 KB)
│  │  │  ├─ 📁 locales
│  │  │  │  ├─ 📁 langs
│  │  │  │  │  ├─ 📁 zh-CN
│  │  │  │  │  │  ├─ 📊 demos.json (0.4 KB)
│  │  │  │  │  │  └─ 📊 page.json (0.3 KB)
│  │  │  │  │  └─ 📁 en-US
│  │  │  │  │     ├─ 📊 demos.json (0.4 KB)
│  │  │  │  │     └─ 📊 page.json (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (2.3 KB)
│  │  │  │  └─ 📜 README.md (0.2 KB)
│  │  │  ├─ 📁 layouts
│  │  │  │  ├─ 📄 auth.vue (0.7 KB)
│  │  │  │  ├─ 📄 basic.vue (6.1 KB)
│  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  ├─ 📁 api
│  │  │  │  ├─ 📁 core
│  │  │  │  │  ├─ 💻 auth.ts (1.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 💻 menu.ts (0.3 KB)
│  │  │  │  │  └─ 💻 user.ts (0.2 KB)
│  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  └─ 💻 request.ts (3.2 KB)
│  │  │  ├─ 📁 adapter
│  │  │  │  ├─ 📁 component
│  │  │  │  │  └─ 💻 index.ts (11.0 KB)
│  │  │  │  ├─ 💻 form.ts (1.2 KB)
│  │  │  │  └─ 💻 vxe-table.ts (2.1 KB)
│  │  │  ├─ 📄 app.vue (0.4 KB)
│  │  │  ├─ 💻 bootstrap.ts (2.1 KB)
│  │  │  ├─ 💻 main.ts (0.9 KB)
│  │  │  └─ 💻 preferences.ts (0.4 KB)
│  │  ├─ 📁 public
│  │  │  └─ 📄 favicon.ico (5.3 KB)
│  │  ├─ 📄 .env (0.3 KB)
│  │  ├─ 📄 .env.analyze (0.1 KB)
│  │  ├─ 📄 .env.development (0.3 KB)
│  │  ├─ 📄 .env.production (0.3 KB)
│  │  ├─ 📄 index.html (1.2 KB)
│  │  ├─ 📊 package.json (1.5 KB)
│  │  ├─ 📊 tsconfig.json (0.3 KB)
│  │  ├─ 📊 tsconfig.node.json (0.3 KB)
│  │  └─ 💻 vite.config.ts (0.6 KB)
│  ├─ 📁 web-antdv-next
│  │  ├─ 📁 src
│  │  │  ├─ 📁 views
│  │  │  │  ├─ 📁 _core
│  │  │  │  │  ├─ 📁 profile
│  │  │  │  │  │  ├─ 📄 base-setting.vue (1.3 KB)
│  │  │  │  │  │  ├─ 📄 index.vue (1.1 KB)
│  │  │  │  │  │  ├─ 📄 notification-setting.vue (0.7 KB)
│  │  │  │  │  │  ├─ 📄 password-setting.vue (1.5 KB)
│  │  │  │  │  │  └─ 📄 security-setting.vue (1.0 KB)
│  │  │  │  │  ├─ 📁 fallback
│  │  │  │  │  │  ├─ 📄 coming-soon.vue (0.1 KB)
│  │  │  │  │  │  ├─ 📄 forbidden.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 internal-error.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 not-found.vue (0.2 KB)
│  │  │  │  │  │  └─ 📄 offline.vue (0.2 KB)
│  │  │  │  │  ├─ 📁 authentication
│  │  │  │  │  │  ├─ 📄 code-login.vue (1.7 KB)
│  │  │  │  │  │  ├─ 📄 forget-password.vue (0.9 KB)
│  │  │  │  │  │  ├─ 📄 login.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 qrcode-login.vue (0.3 KB)
│  │  │  │  │  │  └─ 📄 register.vue (2.5 KB)
│  │  │  │  │  ├─ 📁 about
│  │  │  │  │  │  └─ 📄 index.vue (0.1 KB)
│  │  │  │  │  └─ 📜 README.md (0.1 KB)
│  │  │  │  ├─ 📁 demos
│  │  │  │  │  └─ 📁 antd
│  │  │  │  │     └─ 📄 index.vue (1.7 KB)
│  │  │  │  └─ 📁 dashboard
│  │  │  │     ├─ 📁 workspace
│  │  │  │     │  └─ 📄 index.vue (6.9 KB)
│  │  │  │     └─ 📁 analytics
│  │  │  │        ├─ 📄 analytics-trends.vue (2.0 KB)
│  │  │  │        ├─ 📄 analytics-visits-data.vue (1.6 KB)
│  │  │  │        ├─ 📄 analytics-visits-sales.vue (1.1 KB)
│  │  │  │        ├─ 📄 analytics-visits-source.vue (1.5 KB)
│  │  │  │        ├─ 📄 analytics-visits.vue (1.1 KB)
│  │  │  │        └─ 📄 index.vue (2.1 KB)
│  │  │  ├─ 📁 store
│  │  │  │  ├─ 💻 auth.ts (2.9 KB)
│  │  │  │  └─ 💻 index.ts (0.0 KB)
│  │  │  ├─ 📁 router
│  │  │  │  ├─ 📁 routes
│  │  │  │  │  ├─ 📁 modules
│  │  │  │  │  │  ├─ 💻 dashboard.ts (0.9 KB)
│  │  │  │  │  │  ├─ 💻 demos.ts (0.6 KB)
│  │  │  │  │  │  └─ 💻 vben.ts (2.6 KB)
│  │  │  │  │  ├─ 💻 core.ts (2.4 KB)
│  │  │  │  │  └─ 💻 index.ts (1.5 KB)
│  │  │  │  ├─ 💻 access.ts (1.1 KB)
│  │  │  │  ├─ 💻 guard.ts (3.6 KB)
│  │  │  │  └─ 💻 index.ts (0.9 KB)
│  │  │  ├─ 📁 locales
│  │  │  │  ├─ 📁 langs
│  │  │  │  │  ├─ 📁 zh-CN
│  │  │  │  │  │  ├─ 📊 demos.json (0.3 KB)
│  │  │  │  │  │  └─ 📊 page.json (0.3 KB)
│  │  │  │  │  └─ 📁 en-US
│  │  │  │  │     ├─ 📊 demos.json (0.3 KB)
│  │  │  │  │     └─ 📊 page.json (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (2.3 KB)
│  │  │  │  └─ 📜 README.md (0.2 KB)
│  │  │  ├─ 📁 layouts
│  │  │  │  ├─ 📄 auth.vue (0.7 KB)
│  │  │  │  ├─ 📄 basic.vue (6.1 KB)
│  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  ├─ 📁 api
│  │  │  │  ├─ 📁 core
│  │  │  │  │  ├─ 💻 auth.ts (1.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 💻 menu.ts (0.3 KB)
│  │  │  │  │  └─ 💻 user.ts (0.2 KB)
│  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  └─ 💻 request.ts (3.2 KB)
│  │  │  ├─ 📁 adapter
│  │  │  │  ├─ 📁 component
│  │  │  │  │  └─ 💻 index.ts (19.1 KB)
│  │  │  │  ├─ 💻 form.ts (1.4 KB)
│  │  │  │  └─ 💻 vxe-table.ts (2.0 KB)
│  │  │  ├─ 📄 app.vue (0.9 KB)
│  │  │  ├─ 💻 bootstrap.ts (1.9 KB)
│  │  │  ├─ 💻 main.ts (0.9 KB)
│  │  │  └─ 💻 preferences.ts (0.4 KB)
│  │  ├─ 📁 public
│  │  │  └─ 📄 favicon.ico (5.3 KB)
│  │  ├─ 📄 .env (0.3 KB)
│  │  ├─ 📄 .env.analyze (0.1 KB)
│  │  ├─ 📄 .env.development (0.3 KB)
│  │  ├─ 📄 .env.production (0.3 KB)
│  │  ├─ 📄 index.html (1.2 KB)
│  │  ├─ 📊 package.json (1.4 KB)
│  │  ├─ 📊 tsconfig.json (0.3 KB)
│  │  ├─ 📊 tsconfig.node.json (0.3 KB)
│  │  └─ 💻 vite.config.ts (0.4 KB)
│  ├─ 📁 web-antd
│  │  ├─ 📁 src
│  │  │  ├─ 📁 views
│  │  │  │  ├─ 📁 _core
│  │  │  │  │  ├─ 📁 profile
│  │  │  │  │  │  ├─ 📄 base-setting.vue (1.3 KB)
│  │  │  │  │  │  ├─ 📄 index.vue (1.1 KB)
│  │  │  │  │  │  ├─ 📄 notification-setting.vue (0.7 KB)
│  │  │  │  │  │  ├─ 📄 password-setting.vue (1.5 KB)
│  │  │  │  │  │  └─ 📄 security-setting.vue (1.0 KB)
│  │  │  │  │  ├─ 📁 fallback
│  │  │  │  │  │  ├─ 📄 coming-soon.vue (0.1 KB)
│  │  │  │  │  │  ├─ 📄 forbidden.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 internal-error.vue (0.2 KB)
│  │  │  │  │  │  ├─ 📄 not-found.vue (0.2 KB)
│  │  │  │  │  │  └─ 📄 offline.vue (0.2 KB)
│  │  │  │  │  ├─ 📁 authentication
│  │  │  │  │  │  ├─ 📄 code-login.vue (1.7 KB)
│  │  │  │  │  │  ├─ 📄 forget-password.vue (0.9 KB)
│  │  │  │  │  │  ├─ 📄 login.vue (2.4 KB)
│  │  │  │  │  │  ├─ 📄 qrcode-login.vue (0.3 KB)
│  │  │  │  │  │  └─ 📄 register.vue (2.5 KB)
│  │  │  │  │  ├─ 📁 about
│  │  │  │  │  │  └─ 📄 index.vue (0.1 KB)
│  │  │  │  │  └─ 📜 README.md (0.1 KB)
│  │  │  │  ├─ 📁 demos
│  │  │  │  │  └─ 📁 antd
│  │  │  │  │     └─ 📄 index.vue (2.3 KB)
│  │  │  │  └─ 📁 dashboard
│  │  │  │     ├─ 📁 workspace
│  │  │  │     │  └─ 📄 index.vue (6.9 KB)
│  │  │  │     └─ 📁 analytics
│  │  │  │        ├─ 📄 analytics-trends.vue (2.0 KB)
│  │  │  │        ├─ 📄 analytics-visits-data.vue (1.6 KB)
│  │  │  │        ├─ 📄 analytics-visits-sales.vue (1.1 KB)
│  │  │  │        ├─ 📄 analytics-visits-source.vue (1.5 KB)
│  │  │  │        ├─ 📄 analytics-visits.vue (1.1 KB)
│  │  │  │        └─ 📄 index.vue (2.1 KB)
│  │  │  ├─ 📁 utils
│  │  │  │  └─ 💻 tauri-detector.ts (0.3 KB)
│  │  │  ├─ 📁 store
│  │  │  │  ├─ 💻 auth.ts (2.9 KB)
│  │  │  │  └─ 💻 index.ts (0.0 KB)
│  │  │  ├─ 📁 services
│  │  │  │  ├─ 💻 api.ts (1.8 KB)
│  │  │  │  ├─ 💻 storage.ts (1.2 KB)
│  │  │  │  └─ 💻 tauri.ts (1.0 KB)
│  │  │  ├─ 📁 router
│  │  │  │  ├─ 📁 routes
│  │  │  │  │  ├─ 📁 modules
│  │  │  │  │  │  ├─ 💻 dashboard.ts (0.9 KB)
│  │  │  │  │  │  ├─ 💻 demos.ts (0.5 KB)
│  │  │  │  │  │  └─ 💻 vben.ts (2.7 KB)
│  │  │  │  │  ├─ 💻 core.ts (2.4 KB)
│  │  │  │  │  └─ 💻 index.ts (1.5 KB)
│  │  │  │  ├─ 💻 access.ts (1.1 KB)
│  │  │  │  ├─ 💻 guard.ts (3.6 KB)
│  │  │  │  └─ 💻 index.ts (0.9 KB)
│  │  │  ├─ 📁 locales
│  │  │  │  ├─ 📁 langs
│  │  │  │  │  ├─ 📁 zh-CN
│  │  │  │  │  │  ├─ 📊 demos.json (0.3 KB)
│  │  │  │  │  │  └─ 📊 page.json (0.3 KB)
│  │  │  │  │  └─ 📁 en-US
│  │  │  │  │     ├─ 📊 demos.json (0.3 KB)
│  │  │  │  │     └─ 📊 page.json (0.3 KB)
│  │  │  │  ├─ 💻 index.ts (2.3 KB)
│  │  │  │  └─ 📜 README.md (0.2 KB)
│  │  │  ├─ 📁 layouts
│  │  │  │  ├─ 📄 auth.vue (0.7 KB)
│  │  │  │  ├─ 📄 basic.vue (6.1 KB)
│  │  │  │  └─ 💻 index.ts (0.2 KB)
│  │  │  ├─ 📁 api
│  │  │  │  ├─ 📁 core
│  │  │  │  │  ├─ 💻 auth.ts (1.0 KB)
│  │  │  │  │  ├─ 💻 index.ts (0.1 KB)
│  │  │  │  │  ├─ 💻 menu.ts (0.3 KB)
│  │  │  │  │  └─ 💻 user.ts (0.2 KB)
│  │  │  │  ├─ 💻 index.ts (0.0 KB)
│  │  │  │  └─ 💻 request.ts (3.2 KB)
│  │  │  ├─ 📁 adapter
│  │  │  │  ├─ 📁 component
│  │  │  │  │  └─ 💻 index.ts (20.5 KB)
│  │  │  │  ├─ 💻 form.ts (1.5 KB)
│  │  │  │  └─ 💻 vxe-table.ts (2.0 KB)
│  │  │  ├─ 📄 app.vue (0.8 KB)
│  │  │  ├─ 💻 bootstrap.ts (2.2 KB)
│  │  │  ├─ 💻 main.ts (1.0 KB)
│  │  │  └─ 💻 preferences.ts (2.0 KB)
│  │  ├─ 📁 public
│  │  │  └─ 📄 favicon.ico (5.3 KB)
│  │  ├─ 📄 .env (0.3 KB)
│  │  ├─ 📄 .env.analyze (0.1 KB)
│  │  ├─ 📄 .env.development (0.3 KB)
│  │  ├─ 📄 .env.production (0.3 KB)
│  │  ├─ 📄 index.html (1.2 KB)
│  │  ├─ 📊 package.json (1.6 KB)
│  │  ├─ 📊 tsconfig.json (0.3 KB)
│  │  ├─ 📊 tsconfig.node.json (0.3 KB)
│  │  └─ 💻 vite.config.ts (0.4 KB)
│  ├─ 📁 tauri-app
│  │  └─ 📊 package.json (0.2 KB)
│  └─ 📁 backend-mock
│     ├─ 📁 utils
│     │  ├─ 💻 cookie-utils.ts (0.7 KB)
│     │  ├─ 💻 jwt-utils.ts (1.9 KB)
│     │  ├─ 💻 mock-data.ts (9.2 KB)
│     │  ├─ 💻 response.ts (1.5 KB)
│     │  └─ 💻 timezone-utils.ts (0.2 KB)
│     ├─ 📁 routes
│     │  └─ 💻 [...].ts (0.4 KB)
│     ├─ 📁 middleware
│     │  └─ 💻 1.api.ts (0.6 KB)
│     ├─ 📁 api
│     │  ├─ 📁 user
│     │  │  └─ 💻 info.ts (0.4 KB)
│     │  ├─ 📁 timezone
│     │  │  ├─ 💻 getTimezone.ts (0.4 KB)
│     │  │  ├─ 💻 getTimezoneOptions.ts (0.4 KB)
│     │  │  └─ 💻 setTimezone.ts (0.8 KB)
│     │  ├─ 📁 table
│     │  │  └─ 💻 list.ts (3.7 KB)
│     │  ├─ 📁 system
│     │  │  ├─ 📁 user
│     │  │  │  └─ 💻 list.ts (2.2 KB)
│     │  │  ├─ 📁 role
│     │  │  │  └─ 💻 list.ts (2.2 KB)
│     │  │  ├─ 📁 menu
│     │  │  │  ├─ 💻 list.ts (0.4 KB)
│     │  │  │  ├─ 💻 name-exists.ts (0.8 KB)
│     │  │  │  └─ 💻 path-exists.ts (0.8 KB)
│     │  │  └─ 📁 dept
│     │  │     ├─ 💻 .post.ts (0.4 KB)
│     │  │     ├─ 💻 list.ts (1.7 KB)
│     │  │     ├─ 💻 [id].delete.ts (0.4 KB)
│     │  │     └─ 💻 [id].put.ts (0.4 KB)
│     │  ├─ 📁 menu
│     │  │  └─ 💻 all.ts (0.5 KB)
│     │  ├─ 📁 demo
│     │  │  └─ 💻 bigint.ts (0.9 KB)
│     │  ├─ 📁 auth
│     │  │  ├─ 💻 codes.ts (0.5 KB)
│     │  │  ├─ 💻 login.post.ts (1.1 KB)
│     │  │  ├─ 💻 logout.post.ts (0.4 KB)
│     │  │  └─ 💻 refresh.post.ts (0.9 KB)
│     │  ├─ 💻 status.ts (0.3 KB)
│     │  ├─ 💻 test.get.ts (0.1 KB)
│     │  ├─ 💻 test.post.ts (0.1 KB)
│     │  └─ 💻 upload.ts (0.5 KB)
│     ├─ 📁 .nitro
│     │  ├─ 📁 types
│     │  │  ├─ 💻 nitro-config.d.ts (0.2 KB)
│     │  │  ├─ 💻 nitro-imports.d.ts (20.5 KB)
│     │  │  ├─ 💻 nitro-routes.d.ts (3.6 KB)
│     │  │  ├─ 💻 nitro.d.ts (0.1 KB)
│     │  │  └─ 📊 tsconfig.json (0.7 KB)
│     │  ├─ 📁 dev
│     │  │  ├─ 📄 index.mjs (77.3 KB)
│     │  │  └─ 📄 index.mjs.map (276.2 KB)
│     │  └─ 📊 nitro.json (0.3 KB)
│     ├─ 📄 .env (0.1 KB)
│     ├─ 💻 error.ts (0.2 KB)
│     ├─ 💻 nitro.config.ts (0.7 KB)
│     ├─ 📊 package.json (0.4 KB)
│     ├─ 📜 README.md (0.7 KB)
│     ├─ 📊 tsconfig.build.json (0.1 KB)
│     └─ 📊 tsconfig.json (0.0 KB)
├─ 📁 .changeset
│  ├─ 📊 config.json (0.5 KB)
│  └─ 📜 README.md (0.5 KB)
├─ 📄 .browserslistrc (0.0 KB)
├─ 💻 .commitlintrc.js (0.0 KB)
├─ 📄 .dockerignore (0.1 KB)
├─ 📄 .editorconfig (0.3 KB)
├─ 📄 .gitattributes (0.4 KB)
├─ 📄 .gitconfig (0.0 KB)
├─ 📄 .gitignore (0.6 KB)
├─ 📄 .gitpod.yml (0.1 KB)
├─ 📄 .node-version (0.0 KB)
├─ 📄 .npmrc (0.0 KB)
├─ 📄 .stylelintignore (0.1 KB)
├─ 📊 cspell.json (1.4 KB)
├─ 📄 eslint.config.mjs (0.1 KB)
├─ 📜 folder-structure.md (111.6 KB)
├─ 📄 lefthook.yml (1.1 KB)
├─ 📄 LICENSE (1.0 KB)
├─ 💻 oxfmt.config.ts (0.4 KB)
├─ 💻 oxlint.config.ts (0.1 KB)
├─ 📊 package.json (4.1 KB)
├─ 📄 pnpm-lock.yaml (699.6 KB)
├─ 📄 pnpm-workspace.yaml (5.6 KB)
├─ 📜 README.ja-JP.md (6.8 KB)
├─ 📜 README.md (6.5 KB)
├─ 📜 README.zh-CN.md (5.9 KB)
├─ 📄 stylelint.config.mjs (0.1 KB)
├─ 📄 tea.yaml (0.1 KB)
├─ 📊 turbo.json (1.0 KB)
├─ 📄 vben-admin.code-workspace (3.8 KB)
└─ 💻 vitest.config.ts (0.8 KB)

```
