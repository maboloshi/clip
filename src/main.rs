// GUI 版本：使用 windows 子系统，阻止控制台窗口
#![cfg_attr(all(windows, feature = "gui"), windows_subsystem = "windows")]
use chardet::{charset2encoding, detect};
use std::env;
use std::fs;
use std::io::Read;

// 条件输出宏
macro_rules! println_info {
    ($silent:expr, $($arg:tt)*) => {
        if !$silent {
            println!($($arg)*);
        }
    };
}

macro_rules! eprintln_info {
    ($silent:expr, $($arg:tt)*) => {
        if !$silent {
            eprintln!($($arg)*);
        }
    };
}

// Windows API 剪贴板操作
#[cfg(windows)]
mod clipboard {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr;

    #[link(name = "user32")]
    extern "system" {
        fn OpenClipboard(hwnd: *mut std::ffi::c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn EmptyClipboard() -> i32;
        fn SetClipboardData(format: u32, mem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GlobalAlloc(flags: u32, size: usize) -> *mut std::ffi::c_void;
        fn GlobalLock(mem: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
        fn GlobalUnlock(mem: *mut std::ffi::c_void) -> i32;
    }

    const CF_UNICODETEXT: u32 = 13;
    const GMEM_MOVEABLE: u32 = 0x0002;

    pub fn set_text(text: &str) -> Result<(), String> {
        unsafe {
            if OpenClipboard(ptr::null_mut()) == 0 {
                return Err("无法打开剪贴板".into());
            }

            EmptyClipboard();

            // 转换为 UTF-16
            let wide: Vec<u16> = OsStr::new(text)
                .encode_wide()
                .chain(once(0))
                .collect();
            let size = wide.len() * 2;

            let hmem = GlobalAlloc(GMEM_MOVEABLE, size);
            if hmem.is_null() {
                CloseClipboard();
                return Err("内存分配失败".into());
            }

            let locked = GlobalLock(hmem);
            if locked.is_null() {
                CloseClipboard();
                return Err("无法锁定内存".into());
            }

            ptr::copy_nonoverlapping(wide.as_ptr(), locked as *mut u16, wide.len());
            GlobalUnlock(hmem);

            if SetClipboardData(CF_UNICODETEXT, hmem).is_null() {
                CloseClipboard();
                return Err("设置剪贴板数据失败".into());
            }

            CloseClipboard();
            Ok(())
        }
    }
}

// Windows API 编码转换
#[cfg(windows)]
mod encoding {
    use std::collections::HashMap;
    use std::ptr;

    #[link(name = "kernel32")]
    extern "system" {
        fn MultiByteToWideChar(
            code_page: u32,
            flags: u32,
            str: *const u8,
            str_len: i32,
            wide_str: *mut u16,
            wide_str_len: i32,
        ) -> i32;
    }

    // 编码名称到 Windows 代码页的映射
    fn get_codepage(encoding: &str) -> Option<u32> {
        let mut map = HashMap::new();

        // UTF 系列
        map.insert("utf-8", 65001);
        map.insert("utf8", 65001);
        map.insert("utf-16le", 1200);
        map.insert("utf-16be", 1201);

        // 简体中文
        map.insert("gbk", 936);
        map.insert("gb2312", 936);
        map.insert("gb18030", 54936);
        map.insert("cp936", 936);

        // 繁体中文
        map.insert("big5", 950);
        map.insert("cp950", 950);

        // 日文
        map.insert("shift-jis", 932);
        map.insert("shift_jis", 932);
        map.insert("sjis", 932);
        map.insert("cp932", 932);
        map.insert("euc-jp", 51932);
        map.insert("iso-2022-jp", 50220);

        // 韩文
        map.insert("euc-kr", 51949);
        map.insert("cp949", 949);
        map.insert("ks_c_5601-1987", 949);

        // 西欧
        map.insert("windows-1252", 1252);
        map.insert("iso-8859-1", 28591);
        map.insert("latin1", 28591);

        // 其他
        map.insert("ascii", 20127);

        map.get(encoding.to_lowercase().as_str()).copied()
    }

    pub fn decode_with_encoding(bytes: &[u8], encoding: &str) -> Result<String, String> {
        // 特殊处理 UTF-16
        if encoding.to_lowercase().contains("utf-16") || encoding.to_lowercase().contains("utf16") {
            return decode_utf16(bytes, encoding);
        }

        let codepage = get_codepage(encoding)
            .ok_or_else(|| format!("不支持的编码: {}", encoding))?;

        unsafe {
            // 第一次调用获取所需缓冲区大小
            let size = MultiByteToWideChar(
                codepage,
                0,
                bytes.as_ptr(),
                bytes.len() as i32,
                ptr::null_mut(),
                0,
            );

            if size <= 0 {
                return Err(format!("编码转换失败: {} (codepage: {})", encoding, codepage));
            }

            // 分配缓冲区并进行转换
            let mut wide_buf = vec![0u16; size as usize];
            let result = MultiByteToWideChar(
                codepage,
                0,
                bytes.as_ptr(),
                bytes.len() as i32,
                wide_buf.as_mut_ptr(),
                size,
            );

            if result <= 0 {
                return Err(format!("编码转换失败: {}", encoding));
            }

            String::from_utf16(&wide_buf[..result as usize])
                .map_err(|_| "UTF-16 转换失败".into())
        }
    }

    fn decode_utf16(bytes: &[u8], encoding: &str) -> Result<String, String> {
        if bytes.is_empty() {
            return Err("UTF-16 文件为空".into());
        }

        let is_le = encoding.to_lowercase().contains("le")
            || (!encoding.to_lowercase().contains("be"));

        // 处理奇数长度（补0）
        let mut byte_vec = bytes.to_vec();
        if byte_vec.len() % 2 != 0 {
            byte_vec.push(0);
        }

        let u16_data: Vec<u16> = if is_le {
            byte_vec.chunks_exact(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect()
        } else {
            byte_vec.chunks_exact(2)
                .map(|c| u16::from_be_bytes([c[0], c[1]]))
                .collect()
        };

        String::from_utf16(&u16_data).map_err(|e| format!("UTF-16 解码失败: {:?}", e))
    }
}

// 检测并读取文件编码
fn read_file_with_encoding(path: &str, silent: bool) -> Result<String, String> {
    let mut file = fs::File::open(path).map_err(|e| format!("无法打开文件: {}", e))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    // 检测 BOM
    let (bytes, detected_encoding) = if buffer.starts_with(&[0xEF, 0xBB, 0xBF]) {
        (&buffer[3..], Some("UTF-8 (with BOM)"))
    } else if buffer.starts_with(&[0xFF, 0xFE]) {
        (&buffer[2..], Some("UTF-16 LE (with BOM)"))
    } else if buffer.starts_with(&[0xFE, 0xFF]) {
        (&buffer[2..], Some("UTF-16 BE (with BOM)"))
    } else {
        (&buffer[..], None)
    };

    // 使用 chardet 检测编码
    let encoding_name = if let Some(enc) = detected_encoding {
        println_info!(silent, "✓ 检测到 BOM: {}", enc);
        enc.split_whitespace().next().unwrap().to_string()
    } else {
        println_info!(silent, "⧗ 正在检测文件编码...");

        let result = detect(bytes);
        let charset = result.0;
        let confidence = result.1;
        let _language = result.2;

        // 转换 charset 名称到标准编码名
        let encoding = charset2encoding(&charset);

        println_info!(silent, "✓ 检测到编码: {} (置信度: {:.0}%)", encoding, confidence * 100.0);

        encoding.to_string()
    };

    // 使用 Windows API 转码
    encoding::decode_with_encoding(bytes, &encoding_name)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // 检查是否启用静默模式
    let silent = if cfg!(feature = "gui") {
        true // GUI 版本默认静默
    } else {
        args.iter().any(|arg| arg == "-s" || arg == "--silent")
    };

    // 检查是否请求帮助
    let show_help = args.iter().any(|arg| arg == "-h" || arg == "--help");

    // 过滤掉标志参数，获取文件路径
    let file_args: Vec<&String> = args.iter()
        .skip(1)
        .filter(|arg| !arg.starts_with('-'))
        .collect();

    // 主动请求帮助或缺少参数
    if show_help || file_args.is_empty() {
        println_info!(silent, "Clip - 文本文件到剪贴板工具");
        println_info!(silent, "版本 0.1.1");
        println_info!(silent, "");
        println_info!(silent, "用法:");
        println_info!(silent, "  {} [选项] <文件路径>", args[0]);
        println_info!(silent, "");
        println_info!(silent, "示例:");
        println_info!(silent, "  {} text.txt", args[0]);
        println_info!(silent, "  {} -s text.txt  (静默模式)", args[0]);
        println_info!(silent, "");
        println_info!(silent, "选项:");
        println_info!(silent, "  -s, --silent    静默模式，不输出任何信息");
        println_info!(silent, "  -h, --help      显示此帮助信息");
        println_info!(silent, "");
        println_info!(silent, "功能:");
        println_info!(silent, "  - 自动检测文件编码 (使用 chardet)");
        println_info!(silent, "  - 支持 UTF-8, GBK, Shift-JIS, Big5, EUC-KR 等");
        println_info!(silent, "  - 显示检测置信度");

        // 主动请求帮助 → 退出码 0
        // 缺少参数 → 退出码 1
        std::process::exit(if show_help { 0 } else { 1 });
    }

    let file_path = file_args[0];

    match read_file_with_encoding(file_path, silent) {
        Ok(content) => match clipboard::set_text(&content) {
            Ok(_) => {
                println_info!(silent, "✓ 已复制到剪贴板 ({} 字符)", content.len());
            }
            Err(e) => {
                eprintln_info!(silent, "✗ 复制失败: {}", e);
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln_info!(silent, "✗ 处理失败: {}", e);
            std::process::exit(1);
        }
    }
}