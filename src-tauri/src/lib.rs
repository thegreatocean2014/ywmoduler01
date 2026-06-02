
mod storage;
mod http;

use tauri::{
    App, Manager,
    menu::{Menu, MenuItem, Submenu, PredefinedMenuItem},
    Emitter,
};
use http::{make_http_request};

use storage::{clear_storage, get_storage, remove_storage, set_storage};


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        // .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        // 自定义命令示例
        .invoke_handler(tauri::generate_handler![
            get_user_data,
            save_preferences,
            greet,

            make_http_request,

            get_storage,
            set_storage,
            remove_storage,
            clear_storage,
        ])
        .setup(|app| {

            if let Ok(resource_dir) = app.path().resource_dir() {
                println!("📁 Resource dir: {}", resource_dir.display());
            }
            if let Ok(p) = app.path().resolve(
                "backend-mock/server/index.mjs",
                tauri::path::BaseDirectory::Resource
            ) {
                println!("📄 index.mjs exists: {}", p.exists());
                println!("📄 index.mjs path: {}", p.display());
            }

            // 开发模式启动 mock server
            #[cfg(debug_assertions)]
            {

                // start_mock_server(app);
                println!("运行在开发模式，Mock server 由 beforeDevCommand 启动");

            }

            if !cfg!(debug_assertions)
            {

                start_mock_server_production( app );
                // if let Some(window) = app.get_webview_window("main") {
                //     window.open_devtools();
                // }

            }


            // ✅ 构建菜单
            let menu = build_menu(app)?;
            app.set_menu(menu)?;
            // // ✅ 用 app.handle().clone() 替代直接捕获 app
            // let app_handle = app.handle().clone();

            // 应用启动时的初始化逻辑
            println!("业务数据建模助手已启动！");
            Ok(())
        })
        // ✅ 在 build 之后处理菜单事件
        .on_menu_event(|app_handle, event| {
            match event.id().as_ref() {
                "new_project" => {
                    println!("菜单: 新建项目");
                    let _ = app_handle.emit("menu-new-project", ());
                }
                "open_project" => {
                    println!("菜单: 打开项目");
                    let _ = app_handle.emit("menu-open-project", ());
                }
                "save_project" => {
                    println!("菜单: 保存项目");
                    let _ = app_handle.emit("menu-save-project", ());
                }
                "exit" => {
                    println!("菜单: 退出");
                    app_handle.exit(0);
                }
                "settings" => {
                    println!("菜单: 设置");
                    let _ = app_handle.emit("menu-open-settings", ());
                }
                // 打开开发工具
                "opendevtools" => {
                    println!("菜单: 打开开发工具");
                    // app_handle.emit("menu-opendevtools", ()).unwrap();

                    if let Some(window) = app_handle.get_webview_window("main") {
                        window.open_devtools();
                    }
                }
                "about" => {
                    println!("菜单: 关于");
                    let _ = app_handle.emit("menu-about", ());
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}


// ✅ 构建菜单函数
fn build_menu(app: &App) -> tauri::Result<Menu<tauri::Wry>> {
    // 文件子菜单
    let file_menu = Submenu::with_items(
        app,
        "文件",
        true,
        &[
            &MenuItem::with_id(app, "new_project", "新建项目", true, Some("CmdOrCtrl+N"))?,
            &MenuItem::with_id(app, "open_project", "打开项目", true, Some("CmdOrCtrl+O"))?,
            &MenuItem::with_id(app, "save_project", "保存项目", true, Some("CmdOrCtrl+S"))?,
            &PredefinedMenuItem::separator(app)?,   // 分隔线
            &MenuItem::with_id(app, "exit", "退出", true, Some("CmdOrCtrl+Q"))?,
        ],
    )?;

    // 编辑子菜单
    let edit_menu = Submenu::with_items(
        app,
        "编辑",
        true,
        &[
            &PredefinedMenuItem::copy(app, Some("复制"))?,
            &PredefinedMenuItem::paste(app, Some("粘贴"))?,
            &PredefinedMenuItem::cut(app, Some("剪切"))?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(app, "settings", "设置", true, Some("CmdOrCtrl+,"))?,
        ],
    )?;

    // 帮助子菜单
    let help_menu = Submenu::with_items(
        app,
        "帮助",
        true,
        &[
            &MenuItem::with_id(app, "opendevtools", "打开开发工具", true, None::<&str>)?,
            &MenuItem::with_id(app, "about", "关于", true, None::<&str>)?,
        ],
    )?;

    // 组合成完整菜单
    Menu::with_items(app, &[&file_menu, &edit_menu, &help_menu])
}


// /// 开发模式：启动本地 mock server
// fn start_mock_server(app: &App) {
//     use tauri_plugin_shell::ShellExt;
//     let app_handle = app.handle().clone();
//
//     std::thread::spawn(move || {
//         // 在开发模式，mock server 由 beforeDevCommand 启动
//         // 这里仅记录日志
//         println!("Mock server will be started by beforeDevCommand");
//     });
// }



fn start_mock_server_production(app: &App) {
    use tauri_plugin_shell::ShellExt;

    let app_handle = app.handle().clone();

    let log_path = std::env::current_exe()
        .map(|exe| exe.parent().unwrap_or(std::path::Path::new(".")).join("mock-server.log"))
        .unwrap_or_else(|_| std::path::PathBuf::from("mock-server.log"));

    std::thread::spawn(move || {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut log_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("无法创建日志文件");

        macro_rules! log_info {
            ($($arg:tt)*) => {{
                let msg = format!("[{}] INFO  {}\n",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    format!($($arg)*)
                );
                print!("{}", msg);
                let _ = log_file.write_all(msg.as_bytes());
                let _ = log_file.flush();
            }};
        }

        macro_rules! log_error {
            ($($arg:tt)*) => {{
                let msg = format!("[{}] ERROR {}\n",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    format!($($arg)*)
                );
                eprint!("{}", msg);
                let _ = log_file.write_all(msg.as_bytes());
                let _ = log_file.flush();
            }};
        }

        log_info!("====== Mock Server 启动流程开始 ======");
        log_info!("日志文件路径: {}", log_path.display());

        let resource_path = app_handle
            .path()
            .resolve("backend-mock/server/index.mjs", tauri::path::BaseDirectory::Resource);

        match resource_path {
            Ok(mock_server_path) => {
                log_info!("✅ 资源路径解析成功");
                log_info!("  📁 脚本路径: {}", mock_server_path.display());
                log_info!("  ✓ 文件存在: {}", mock_server_path.exists());

                if !mock_server_path.exists() {
                    log_error!("❌ Mock server 文件不存在!");
                    log_error!("  期望路径: {}", mock_server_path.display());
                    return;
                }

                let cmd = if cfg!(target_os = "windows") { "node.exe" } else { "node" };

                // ✅ 获取不带 \\?\ 前缀的规范路径
                let script_path_str = if cfg!(target_os = "windows") {
                    // Windows: 移除 \\?\ 前缀
                    let path_str = mock_server_path.to_string_lossy().to_string();
                    if path_str.starts_with(r"\\?\") {
                        path_str[4..].to_string()
                    } else {
                        path_str
                    }
                } else {
                    mock_server_path.to_string_lossy().to_string()
                };

                log_info!("");
                log_info!("📋 完整命令信息:");
                log_info!("  命令: {}", cmd);
                log_info!("  脚本: {}", script_path_str);
                log_info!("  环境变量:");
                log_info!("    - NITRO_PORT: 5320");
                log_info!("");
                log_info!("  完整命令行: {} \"{}\"", cmd, script_path_str);
                log_info!("  (NITRO_PORT=5320)");
                log_info!("");

                log_info!("🚀 正在启动 Mock Server...");

                let args = vec![script_path_str.as_str()];

                match app_handle
                    .shell()
                    .command(cmd)
                    .args(args)
                    .env("NITRO_PORT", "5320")
                    .spawn()
                {
                    Ok(child) => {
                        log_info!("✅ Mock server 进程启动成功!");
                        log_info!("  进程标志: {:?}", child);
                        log_info!("");
                        log_info!("📊 服务信息:");
                        log_info!("  - 服务地址: http://localhost:5320");
                        log_info!("  - API 地址: http://localhost:5320/api");
                        log_info!("  - 状态: 启动中...");
                    }
                    Err(e) => {
                        log_error!("❌ Mock server 启动失败!");
                        log_error!("  错误信息: {}", e);
                        log_error!("");
                        log_error!("🔍 故障排查:");
                        log_error!("  1. 确认 Node.js 已安装");
                        log_error!("  2. 运行: node --version");
                        log_error!("  3. 确认 Node.js 在 PATH 中");
                        log_error!("  4. 手动运行:");
                        log_error!("     {} \"{}\"", cmd, script_path_str);
                        log_error!("  5. 检查脚本文件: {}", mock_server_path.display());
                        return;
                    }
                }

                log_info!("⏳ 等待 Mock server 就绪 (2秒)...");
                std::thread::sleep(std::time::Duration::from_secs(2));

                log_info!("====== Mock Server 启动流程完成 ======");
                log_info!("");
            }
            Err(e) => {
                log_error!("❌ 资源路径解析失败!");
                log_error!("  错误: {}", e);
            }
        }
    });
}


// fn start_mock_server_production(app: &App) {
//     use tauri_plugin_shell::ShellExt;
//
//     let app_handle = app.handle().clone();
//
//     // 获取日志文件路径（应用数据目录下）
//     // let log_path = app_handle
//     //     .path()
//     //     .app_local_data_dir()
//     //     .map(|dir| dir.join("mock-server.log"))
//     //     .unwrap_or_else(|_| std::path::PathBuf::from("mock-server.log"));
//
//     let log_path = std::env::current_exe()
//         .map(|exe| exe.parent().unwrap_or(std::path::Path::new(".")).join("mock-server.log"))
//         .unwrap_or_else(|_| std::path::PathBuf::from("mock-server.log"));
//
//     std::thread::spawn(move || {
//         use std::fs::OpenOptions;
//         use std::io::Write;
//
//         // 打开或创建日志文件（追加模式）
//         let mut log_file = OpenOptions::new()
//             .create(true)
//             .append(true)
//             .open(&log_path)
//             .expect("无法创建日志文件");
//
//         macro_rules! log_info {
//             ($($arg:tt)*) => {{
//                 let msg = format!("[{}] INFO  {}\n",
//                     chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
//                     format!($($arg)*)
//                 );
//                 print!("{}", msg);
//                 let _ = log_file.write_all(msg.as_bytes());
//                 let _ = log_file.flush();
//             }};
//         }
//
//         macro_rules! log_error {
//             ($($arg:tt)*) => {{
//                 let msg = format!("[{}] ERROR {}\n",
//                     chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
//                     format!($($arg)*)
//                 );
//                 eprint!("{}", msg);
//                 let _ = log_file.write_all(msg.as_bytes());
//                 let _ = log_file.flush();
//             }};
//         }
//
//         log_info!("====== Mock Server 启动 ======");
//         log_info!("日志文件路径: {}", log_path.display());
//
//         let resource_path = app_handle
//             .path()
//             .resolve("backend-mock/server/index.mjs", tauri::path::BaseDirectory::Resource);
//
//         match resource_path {
//             Ok(mock_server_path) => {
//                 log_info!("解析资源路径成功: {}", mock_server_path.display());
//
//                 if !mock_server_path.exists() {
//                     log_error!("Mock server 文件不存在: {}", mock_server_path.display());
//                     return;
//                 }
//
//                 log_info!("Mock server 文件已找到，准备启动...");
//
//                 let cmd = if cfg!(target_os = "windows") { "node.exe" } else { "node" };
//                 log_info!("使用 Node 命令: {}", cmd);
//
//                 match app_handle
//                     .shell()
//                     .command(cmd)
//                     .args([mock_server_path.to_string_lossy().as_ref()])
//                     .env("NITRO_PORT", "5320")  // ✅ 添加这行，设置端口为 5320
//                     .spawn()
//                 {
//                     Ok(child) => {
//                         log_info!("Mock server 进程启动成功: {:?}", child);
//                     }
//                     Err(e) => {
//                         log_error!("Mock server 进程启动失败: {}", e);
//                     }
//                 }
//
//                 log_info!("等待 Mock server 就绪 (2s)...");
//                 std::thread::sleep(std::time::Duration::from_secs(2));
//                 log_info!("Mock server 启动流程完成");
//             }
//             Err(e) => {
//                 log_error!("资源路径解析失败: {}", e);
//             }
//         }
//     });
// }


// fn start_mock_server_production(app: &App) {
//
//     use tauri_plugin_shell::ShellExt;
//     // use std::path::PathBuf;
//
//     let app_handle = app.handle().clone();
//
//     std::thread::spawn(move || {
//         // 获取应用资源目录
//         let resource_path = app_handle
//             .path()
//             .resolve("backend-mock/server/index.mjs", tauri::path::BaseDirectory::Resource);
//
//         match resource_path {
//             Ok(mock_server_path) => {
//                 println!("Starting mock server from: {}", mock_server_path.display());
//
//                 // 检查文件是否存在
//                 if !mock_server_path.exists() {
//                     eprintln!("Mock server file not found: {}", mock_server_path.display());
//                     return;
//                 }
//
//                 // 使用 ShellExt 的 command API
//                 let cmd = if cfg!(target_os = "windows") {
//                     "node.exe"
//                 } else {
//                     "node"
//                 };
//
//                 match app_handle.shell().command(cmd)
//                     .args(&[mock_server_path.to_string_lossy().as_ref()])
//                     .spawn()
//                 {
//                     Ok(_) => {
//                         println!("Mock server started successfully");
//                     }
//                     Err(e) => {
//                         eprintln!("Failed to start mock server: {}", e);
//                     }
//                 }
//
//                 // 等待服务器启动
//                 std::thread::sleep(std::time::Duration::from_secs(2));
//             }
//             Err(e) => {
//                 eprintln!("Failed to resolve mock server path: {}", e);
//             }
//         }
//     });
//
// }



// 自定义Tauri命令 - 示例：获取用户数据
#[tauri::command]
fn get_user_data() -> String {
    "User data from Rust backend".to_string()
}

// 自定义Tauri命令 - 示例：保存用户偏好设置
#[tauri::command]
fn save_preferences(key: String, value: String) -> Result<(), String> {
    println!("Saving preference: {} = {}", key, value);
    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}



