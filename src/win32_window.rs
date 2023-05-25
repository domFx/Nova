use std::mem::size_of;

use windows::{
    Win32::{
        Foundation::{HWND, LPARAM, LRESULT, WPARAM, BOOL, RECT},
        System::{
            LibraryLoader::{GetModuleHandleA, LoadLibraryA, GetProcAddress}, 
            Memory::{MEM_COMMIT, PAGE_READWRITE, MEM_RELEASE}
        },
        System::Memory::{VirtualAlloc, VirtualFree},
        UI::WindowsAndMessaging::{ 
            HMENU, WNDCLASSA, WINDOW_EX_STYLE, MSG,
            CreateWindowExA, DefWindowProcA, RegisterClassA, PeekMessageA,
            TranslateMessage, DispatchMessageA, GetClientRect,
            WM_CLOSE, WM_SIZE, WM_DESTROY, WM_ACTIVATEAPP, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, WM_PAINT, PM_REMOVE, WM_QUIT
        }, 
        Graphics::Gdi::{ 
            PAINTSTRUCT, HDC, BITMAPINFO, BITMAPINFOHEADER,
            BeginPaint, EndPaint, StretchDIBits, ReleaseDC, GetDC,
            DIB_RGB_COLORS, BI_RGB, SRCCOPY, 
        }
    },
    s
};

use winapi::um::xinput::{
    XUSER_MAX_COUNT, XINPUT_GAMEPAD_DPAD_UP, 
    XINPUT_STATE, XINPUT_VIBRATION
};
use winapi::shared::minwindef::DWORD;

struct OffscreenBuffer {
    info: Option<BITMAPINFO>,
    memory: Option<*mut core::ffi::c_void>,
    width: i32,
    height: i32,
    bytes_per_pixel: i32,
    pitch: usize
}

struct WindowDimension {
    width: i32,
    height: i32
}

static mut IS_RUNNING: bool = true;
static mut BACKBUFFER: OffscreenBuffer = OffscreenBuffer {
    info: None,
    memory: None,
    width: 0,
    height: 0,
    bytes_per_pixel: 4,
    pitch: 0
};

type XInputGetStateDef = unsafe extern "system" fn(u32, *mut XINPUT_STATE) -> DWORD;
type XInputSetStateDef = unsafe extern "system" fn(DWORD, *mut XINPUT_VIBRATION) -> DWORD;

static mut XINPUT_GET_STATE: Option<XInputGetStateDef> = None;
static mut XINPUT_SET_STATE: Option<XInputSetStateDef> = None;

fn load_xinput() {
    let dll_paths = vec!["xinput1_4.dll", "xinput1_3.dll"];

    for dll_path in dll_paths {
        let hmodule = unsafe { LoadLibraryA(windows::core::PCSTR(dll_path.as_ptr())) };
        match hmodule {
            Ok(value) => {
                unsafe { 
                    XINPUT_GET_STATE = Some(std::mem::transmute(GetProcAddress(value, s!("XInputGetState"))));
                    XINPUT_SET_STATE = Some(std::mem::transmute(GetProcAddress(value, s!("XInputSetState"))));
                }
                break;
            },
            _ => continue
        };
    }
}

pub fn open_win32_window() {
    load_xinput();

    unsafe {
        resize_dib_section(&mut BACKBUFFER, 1280, 720);
    }

    let instance = unsafe { GetModuleHandleA(None).ok().unwrap() };
    let class = WNDCLASSA {
        hInstance: instance,
        lpszClassName: s!("Window Title"),
        lpfnWndProc: Some(main_window_callback),
        style: CS_HREDRAW|CS_VREDRAW,
        ..Default::default()
    };

    unsafe { 
        if RegisterClassA(&class) > 0 {
            let window_handle = 
                CreateWindowExA(            
                    WINDOW_EX_STYLE::default(), 
                    class.lpszClassName, 
                    s!("Nova Engine"), 
                    WS_OVERLAPPEDWINDOW|WS_VISIBLE,
                    CW_USEDEFAULT, 
                    CW_USEDEFAULT, 
                    CW_USEDEFAULT, 
                    CW_USEDEFAULT, 
                    HWND::default(), 
                    HMENU::default(), 
                    instance, 
                    None);

            let mut x_offset = 0;   
            let mut y_offset = 0;
                    
            while IS_RUNNING {
                let mut message: MSG = MSG::default();
                while PeekMessageA(&mut message, HWND::default(), 0, 0, PM_REMOVE) != BOOL::default() {
                    if message.message == WM_QUIT {
                        IS_RUNNING = false;
                    }

                    TranslateMessage(&message);
                    DispatchMessageA(&message);    
                }

                if XINPUT_GET_STATE.is_some() && XINPUT_SET_STATE.is_some()  {
                    for controller_index in 0..XUSER_MAX_COUNT {
                        let mut controller_state: XINPUT_STATE = std::mem::zeroed();
                        if XINPUT_GET_STATE.unwrap()(controller_index, &mut controller_state) == 0 {
                            // Retrieved controller state
                            // Check if dwPacketNumber increases too rapidly
                            let dpad_up = controller_state.Gamepad.wButtons & XINPUT_GAMEPAD_DPAD_UP;
                        
                        } else {
                            // Controller not available
                            println!("Error retrieving controller state");
                        }
                    }
                }

                render(&mut BACKBUFFER, x_offset, y_offset);

                let device_context = GetDC(window_handle);

                let window_dimension = get_window_dimension(window_handle);

                display_buffer_in_window(device_context, window_dimension.width, window_dimension.height,
                     &BACKBUFFER, 0, 0, window_dimension.width, window_dimension.height);
                ReleaseDC(window_handle, device_context);
                
                x_offset += 1;
                y_offset += 1;
            } 
        } else {
            //todo!("logging");
        }

        //todo!("logging");
    }
}

unsafe extern "system" fn main_window_callback(
    window: HWND,
    message: u32, 
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {    
    let mut result = LRESULT::default();

    match message {
        WM_SIZE => { 
            println!("WM_SIZE");
        },
        WM_CLOSE => { 
            // DestroyWindow(window);
            IS_RUNNING = false;
            println!("WM_CLOSE"); 
        },
        WM_DESTROY => { 
            // PostQuitMessage(0);
            IS_RUNNING = false;
            println!("WM_DESTROY");
        },
        WM_ACTIVATEAPP => { 
            println!("WM_ACTIVATEAPP");
        },
        WM_PAINT => {
            let mut paint_struct = PAINTSTRUCT::default();
            let device_context = BeginPaint(window, &mut paint_struct);

            let x = paint_struct.rcPaint.left; 
            let y = paint_struct.rcPaint.top;
            let width = paint_struct.rcPaint.right - paint_struct.rcPaint.left;
            let height = paint_struct.rcPaint.bottom - paint_struct.rcPaint.top;

            let window_dimension = get_window_dimension(window);

            display_buffer_in_window(device_context, window_dimension.width, window_dimension.height, 
                &BACKBUFFER, x, y, width, height);

            EndPaint(window, &paint_struct);
        },
        _ => { 
            result = DefWindowProcA(window, message, wparam, lparam);
        }
    }

    result
}

fn render(offscreen_buffer: &OffscreenBuffer, x_offset: i32, y_offset: i32) {
    unsafe {
        
        match offscreen_buffer.memory {
            Some(value) => {               
                let mut row = value as *mut u8;

                for y in 0..offscreen_buffer.height {
                    let mut pixel = row as *mut u32;

                    for x in 0..offscreen_buffer.width {
                        let blue: u8 = (x + x_offset) as u8;
                        let green: u8 = (y + y_offset) as u8;
                        let red: u8 = 0;
                        
                        *pixel = (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue);
                        pixel = pixel.add(1);
                    }

                    row = row.add(offscreen_buffer.pitch);
                }
            },
            None => return
        }
    }
}

fn resize_dib_section(offscreen_buffer: &mut OffscreenBuffer, width: i32, height: i32) {
    unsafe {
        if offscreen_buffer.memory.is_some() {
            VirtualFree(offscreen_buffer.memory.unwrap(), 0, MEM_RELEASE);
        }

        let mut bitmap_info: BITMAPINFO = BITMAPINFO::default();

        offscreen_buffer.width = width;
        offscreen_buffer.height = height;
        offscreen_buffer.bytes_per_pixel = 4;
    
        let mut bitmap_header_info = BITMAPINFOHEADER::default();
        bitmap_header_info.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_header_info.biWidth = offscreen_buffer.width;
        bitmap_header_info.biHeight = -offscreen_buffer.height;
        bitmap_header_info.biPlanes = 1;
        bitmap_header_info.biBitCount = 32;
        bitmap_header_info.biCompression =  BI_RGB.0 as u32; // 0 or uncompressed
        bitmap_info.bmiHeader = bitmap_header_info;

        offscreen_buffer.info = Some(bitmap_info);
    }

    let bitmap_memory_size: usize = (offscreen_buffer.width * offscreen_buffer.height * offscreen_buffer.bytes_per_pixel).try_into().unwrap();

    unsafe {
        offscreen_buffer.memory = Some(VirtualAlloc(None, bitmap_memory_size, MEM_COMMIT, PAGE_READWRITE));
    }

    offscreen_buffer.pitch = (offscreen_buffer.width * offscreen_buffer.bytes_per_pixel) as usize;
} 

fn display_buffer_in_window(device_context: HDC, window_width: i32, window_height: i32, offscreen_buffer: &OffscreenBuffer, x: i32, y: i32, width: i32, height: i32) {
    unsafe {
        let bit_memory = offscreen_buffer.memory.map(|p| p as *const core::ffi::c_void);
        StretchDIBits(
            device_context, 
            0, 0, window_width, window_height,
            0, 0, offscreen_buffer.width, offscreen_buffer.height,
            bit_memory, &offscreen_buffer.info.unwrap(), DIB_RGB_COLORS, SRCCOPY);
    }
}

fn get_window_dimension(window: HWND) -> WindowDimension {
    let mut client_rect: RECT = RECT::default();
    unsafe {
        GetClientRect(window, &mut client_rect);
    }
    
    let width = client_rect.right - client_rect.left;
    let height = client_rect.bottom - client_rect.top;

    return WindowDimension {
        width,
        height
    }
}