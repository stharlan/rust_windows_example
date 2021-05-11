
use bindings::{Windows::Win32::Gdi::*, Windows::Win32::{SystemServices::*, WindowsAndMessaging}, Windows::Win32::WindowsAndMessaging::*, Windows::Win32::MenusAndResources::*};

fn create_button(pwnd:HWND, control_id:isize)
{

    // control id is 101

    let class_name = std::ffi::CString::new("BUTTON").expect("BUTTON");
    let pstr_class_name:PSTR = PSTR(class_name.as_ptr() as *mut _);

    let wnd_name = std::ffi::CString::new("Press Me!").expect("BUTTON");
    let pstr_wnd_name:PSTR = PSTR(wnd_name.as_ptr() as *mut _);

    let control_id_as_hmenu = HMENU(control_id);

    unsafe 
    {
        let module_handle = GetModuleHandleA(None);
        let h_inst:HINSTANCE = HINSTANCE(module_handle);

        CreateWindowExA(
            WINDOW_EX_STYLE(0),
            pstr_class_name,
            pstr_wnd_name,
            WINDOW_STYLE::WS_VISIBLE|WINDOW_STYLE::WS_CHILD,
            5, 5, 100, 30,
            pwnd,
            control_id_as_hmenu,
            h_inst,
            0 as *mut _);
    }
}

extern "system" fn window_proc(hwnd:HWND, msg:u32, w_param:WPARAM, l_param:LPARAM) -> LRESULT {
    match msg {
        WM_CREATE => {
            create_button(hwnd, 101);
            LRESULT(0)
        },
        WM_DESTROY => {
            unsafe { PostQuitMessage(0); }
            LRESULT(0)
        } ,
        WM_QUIT => {
            unsafe { DestroyWindow(hwnd); }
            LRESULT(0)
        },
        WM_COMMAND=> {
            //let hiword = (w_param.0 & 0xff00) >> 16;
            let loword = w_param.0 & 0xff;
            match loword {
                101 => {
                    println!("Button was pressed!");
                    LRESULT(0)
                },
                _ => LRESULT(0)
            }
        },
        _ => {
            unsafe { DefWindowProcA(hwnd, msg, w_param, l_param) }
        },
    }
}

fn main() -> windows::Result<()> {

    unsafe {
        let module_handle = GetModuleHandleA(None);
        println!("hInstance = {:#X}", module_handle);
        let h_inst:HINSTANCE = HINSTANCE(module_handle);
        let h_brush:HBRUSH = HBRUSH(6);

        let class_name = std::ffi::CString::new("RUST_WND_CLASS").expect("CString::new failed");
        let pstr_class_name:PSTR = PSTR(class_name.as_ptr() as *mut _);
        
        let window_name = std::ffi::CString::new("Rust Window").expect("CString::new failed");
        let pstr_window_name:PSTR = PSTR(window_name.as_ptr() as *mut _);

        let mut wc = WNDCLASSA::default();
        wc.style = WNDCLASS_STYLES::CS_HREDRAW|WNDCLASS_STYLES::CS_VREDRAW;
        wc.lpfnWndProc = Some(window_proc);
        wc.lpszClassName = pstr_class_name;
        wc.hInstance = h_inst;
        wc.hbrBackground = h_brush;
    
        let result = RegisterClassA(&wc);
        println!("register class = {:#X}", result);

        let _h_wnd = CreateWindowExA(
            WINDOW_EX_STYLE::WS_EX_OVERLAPPEDWINDOW,
            pstr_class_name,
            pstr_window_name,
            WINDOW_STYLE::WS_OVERLAPPEDWINDOW|WINDOW_STYLE::WS_VISIBLE,
            WindowsAndMessaging::CW_USEDEFAULT,
            WindowsAndMessaging::CW_USEDEFAULT,
            640, 480,
            HWND::NULL,
            HMENU::NULL,
            h_inst,
            0 as *mut _);
        
        let mut msg = MSG::default();
        let h_wnd2 = HWND::default();

        println!("Beginning message loop...");

        loop {
            let result = GetMessageA(&mut msg, h_wnd2, 0, 0).0;
            if result < 1 {
                break;
            } else {
                match msg.message {
                    WM_QUIT => (),
                    _ => {
                        TranslateMessage(&msg);
                        DispatchMessageA(&msg);
                    }
                }
            }
        }
    }

    println!("Done.");

    Ok(())
}
