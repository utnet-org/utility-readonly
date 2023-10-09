/// 读取用户输入的字符串 (不包括换行符) 并返回 String 对象 (堆上分配) 或 &str 对象 (栈上分配)
#[macro_export]
macro_rules! read_input {
    ($var:ident) => {
        let mut $var = String::new();
        io::stdin().read_line(&mut $var).expect("读取失败");
        let $var = $var.trim();
    };
}

/// 显示提示信息并刷新缓冲区
#[macro_export]
macro_rules! prompt {
    ($msg:expr) => {
        print!("{}", $msg);
        io::stdout().flush().unwrap();
    };
}

#[macro_export]
macro_rules! red {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().red()));
}

#[macro_export]
macro_rules! blue {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().blue()));
}

#[macro_export]
macro_rules! green {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().green()));
}

#[macro_export]
macro_rules! yellow {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().yellow()));
}

#[macro_export]
macro_rules! pink {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().magenta()));
}

#[macro_export]
macro_rules! blue_on_yellow {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().blue().on_yellow()));
}

#[macro_export]
macro_rules! bold {
    ($($arg:tt)*) => (format!("{}", format_args!($($arg)*).to_string().bold()));
}
