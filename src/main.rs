use std::mem::size_of;

use windows:: {
    core::{
        CanInto
    },
    Win32::{
        Foundation::{
            HWND, LPARAM, LRESULT, WPARAM, BOOL, RECT
        },
        System::{LibraryLoader::{
            GetModuleHandleA
        }, Memory::{MEM_COMMIT, PAGE_READWRITE, MEM_RELEASE}},
        System::Memory::{
            VirtualAlloc, VirtualFree
        },
        UI::WindowsAndMessaging::{ 
            HMENU, WNDCLASSA, WINDOW_EX_STYLE, MSG,
            CreateWindowExA, DefWindowProcA, RegisterClassA,
            TranslateMessage, DispatchMessageA, GetClientRect,
            WM_CLOSE, WM_SIZE, WM_DESTROY, WM_ACTIVATEAPP, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
            CS_OWNDC, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, WM_PAINT, PeekMessageA, PM_REMOVE, WM_QUIT
        }, 
        Graphics::Gdi::{ 
            PAINTSTRUCT, HDC, BITMAPINFO, BITMAPINFOHEADER,
            BeginPaint, EndPaint, StretchDIBits,
            DIB_RGB_COLORS, BI_RGB, SRCCOPY, ReleaseDC, GetDC
        }
    },
    s
};


static mut IS_RUNNING: bool = true;
static mut BITMAP_INFO: Option<BITMAPINFO> = None;
static mut BITMAP_MEMORY: Option<*mut core::ffi::c_void> = None;
static mut BITMAP_WIDTH: i32 = 0;
static mut BITMAP_HEIGHT: i32 = 0;

fn main() {

    let instance = unsafe { GetModuleHandleA(None).ok().unwrap() };
    let class = WNDCLASSA {
        hInstance: instance,
        lpszClassName: s!("Window Title"),
        lpfnWndProc: Some(main_window_callback),
        style: CS_OWNDC|CS_HREDRAW|CS_VREDRAW,
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

            let mut message: MSG = MSG::default();

            let mut x_offset = 0;   
            let mut y_offset = 0;

            while IS_RUNNING {
                while PeekMessageA(&mut message, HWND::default(), 0, 0, PM_REMOVE) != BOOL::default() {
                    if message.message == WM_QUIT {
                        IS_RUNNING = false;
                    }

                    TranslateMessage(&message);
                    DispatchMessageA(&message);    
                }

                render(x_offset, y_offset);

                let  device_context = GetDC(window_handle);

                let mut client_rect = RECT::default();
                GetClientRect(window_handle, &mut client_rect);

                let window_width = client_rect.right - client_rect.left;
                let window_height = client_rect.bottom - client_rect.top;

                update_window(device_context, &client_rect, 0, 0, window_width, window_height);
                ReleaseDC(window_handle, device_context);
                
                x_offset += 1;
                y_offset += 1;
            } 
        } else {
            println!("Failed to Register Window");
        }
    }
}

unsafe extern "system" fn main_window_callback(
    window: HWND,
    message: u32, 
    wparam: WPARAM,
    lparam: LPARAM
) -> LRESULT {    
    match message {
        WM_SIZE => { 
            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);
            let width = rect.right - rect.left;
            let height = rect.bottom - rect.top;
            resize_dib_section(width, height);
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

            let mut rect = RECT::default();
            GetClientRect(window, &mut rect);

            update_window(device_context, &rect, x, y, width, height);

            EndPaint(window, &paint_struct);
        },
        _ => { 
            // println!("Unconfigured"); 
            return DefWindowProcA(window, message, wparam, lparam);
        }
    }

    LRESULT::default()
}

fn render(x_offset: i32, y_offset: i32) {
    unsafe {
        let pitch: usize = (BITMAP_WIDTH * 4) as usize;

        match BITMAP_MEMORY {
            Some(value) => {
                
                let mut row = value as *mut u8;

                for y in 0..BITMAP_HEIGHT {
                    let mut pixel = row as *mut u32;

                    for x in 0..BITMAP_WIDTH {
                        let blue: u8 = (x + x_offset) as u8;
                        let green: u8 = (y + y_offset) as u8;
                        let red: u8 = 0;
                        
                        *pixel = (u32::from(red) << 16) | (u32::from(green) << 8) | u32::from(blue);
                        pixel = pixel.add(1);
                    }

                    row = row.add(pitch);
                }
            },
            None => return
        }
    }
}

fn resize_dib_section(width: i32, height: i32) {
    unsafe {
        if BITMAP_MEMORY.is_some() {
            VirtualFree(BITMAP_MEMORY.unwrap(), 0, MEM_RELEASE);
        }

        let mut bitmap_info: BITMAPINFO = BITMAPINFO::default();

        BITMAP_WIDTH = width;
        BITMAP_HEIGHT = height;
    
        let mut bitmap_header_info = BITMAPINFOHEADER::default();
        bitmap_header_info.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        bitmap_header_info.biWidth = width;
        bitmap_header_info.biHeight = -height;
        bitmap_header_info.biPlanes = 1;
        bitmap_header_info.biBitCount = 32;
        bitmap_header_info.biCompression =  BI_RGB.0 as u32; // 0 or uncompressed
        bitmap_info.bmiHeader = bitmap_header_info;

        BITMAP_INFO = Some(bitmap_info);
    }

    
    let bytes_per_pixel = 4;
    let bitmap_memory_size: usize = (width * height * bytes_per_pixel).try_into().unwrap();

    unsafe {
        BITMAP_MEMORY = Some(VirtualAlloc(None, bitmap_memory_size, MEM_COMMIT, PAGE_READWRITE));
    }
} 

fn update_window(device_context: HDC, window_rect: &RECT, x: i32, y: i32, width: i32, height: i32) {
    let window_width = window_rect.right - window_rect.left;
    let window_height = window_rect.bottom - window_rect.top;

    unsafe {
        let bit_memory = BITMAP_MEMORY.map(|p| p as *const core::ffi::c_void);
        StretchDIBits(
            device_context, 
            /*
            x, y, width, height, 
            x, y, width, height, 
             */
            0, 0, BITMAP_WIDTH, BITMAP_HEIGHT,
            0, 0, window_width, window_height,
            bit_memory, &BITMAP_INFO.unwrap(), DIB_RGB_COLORS, SRCCOPY);
    }
}